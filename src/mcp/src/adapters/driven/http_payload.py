# File:
#   - http_payload.py
# Path:
#   - src/mcp/src/adapters/driven/http_payload.py
#
# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE
# Path-Rule:
#   - All paths in this header are repository-root relative.
#
# Boundary-Contract:
# - Owns:
#   - Bounded HTTP body, JSON, and Server-Sent Event decoding.
# - Must-Not:
#   - Open sockets, manage MCP sessions, or interpret tool semantics.
# - Allows:
#   - Content-length checks, streamed byte ceilings, and JSON validation.
# - Split-When:
#   - SSE reconnection or content-block streaming requires separate state.
# - Merge-When:
#   - Another adapter module owns the same response-decoding invariants.
# - Summary:
#   - Decodes bounded native MCP HTTP payloads.
# - Description:
#   - Prevents unbounded JSON and SSE responses from exhausting memory.
# - Usage:
#   - Called by the loopback HTTP exchange adapter.
# - Defaults:
#   - Limits each response body to 64 MiB.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: bounded MCP HTTP payload decoding
#   - reason: JSON and SSE readers share one byte-limit contract
#   - split: extract SSE state if reconnection or event replay is added
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Bounded native MCP HTTP payload decoding."""

from __future__ import annotations

import json
from typing import NoReturn, Protocol, TypeVar, cast, overload

from mcp.src.adapters.driven.response_validation import (
    matches_integer_request_id,
)
from mcp.src.domain.errors import (
    ProtocolError,
    fail_configuration,
    fail_protocol,
)
from mcp.src.domain.json_types import (
    DuplicateJsonKeyError,
    JsonObject,
    reject_duplicate_json_object,
    require_json_object,
)

_HeaderDefault = TypeVar("_HeaderDefault")

DEFAULT_MAX_RESPONSE_BYTES = 64 * 1024 * 1024
_CONTENT_TYPE_EVENT_STREAM = "text/event-stream"
_CONTENT_TYPE_JSON = "application/json"


class HttpPayloadResponse(Protocol):
    """Minimum response surface required by payload decoding."""

    @overload
    def getheader(self, name: str) -> str | None: ...

    @overload
    def getheader(
        self,
        name: str,
        default: _HeaderDefault,
    ) -> str | _HeaderDefault: ...

    def read(self, amt: int | None = None) -> bytes:
        """Read up to `amt` bytes from the response body."""
        ...

    def readline(self, limit: int = -1) -> bytes:
        """Read one response line with an optional byte limit."""
        ...


def validate_max_response_bytes(value: int) -> int:
    """Return one positive response byte ceiling.

    Returns:
        The validated byte ceiling.
    """
    if value <= 0:
        fail_configuration("max_response_bytes must be positive")
    return value


def read_http_payload(
    response: HttpPayloadResponse,
    request_id: int | None,
    *,
    max_response_bytes: int,
) -> JsonObject | None:
    """Decode one bounded JSON or SSE response payload.

    Args:
        response: Readable HTTP response.
        request_id: Expected JSON-RPC response identity.
        max_response_bytes: Maximum allowed response body size.

    Returns:
        A strict JSON object, or `None` for an empty body.
    """
    limit = validate_max_response_bytes(max_response_bytes)
    content_type, media_type = _response_content_type(response)
    if media_type == _CONTENT_TYPE_EVENT_STREAM:
        if request_id is None:
            fail_protocol("notification unexpectedly returned an SSE stream")
        return _read_sse_payload(response, request_id, limit)
    body = read_bounded_body(response, max_response_bytes=limit)
    if not body:
        return None
    if request_id is None:
        fail_protocol("notification response must be empty")
    if media_type != _CONTENT_TYPE_JSON:
        displayed_type = content_type or "<missing>"
        fail_protocol(
            f"unsupported Content-Type for HTTP response: {displayed_type}"
        )
    return _decode_json(body, context="HTTP response")


def read_http_error_payload(
    response: HttpPayloadResponse,
    *,
    max_response_bytes: int,
) -> JsonObject | None:
    """Decode one optional JSON object from an HTTP error response.

    Unsupported media types and malformed JSON return `None` so the caller can
    preserve the stable HTTP status fallback. Framing and byte-limit violations
    remain protocol errors.

    Returns:
        A strict JSON error object, or `None` when no valid object is available.
    """
    limit = validate_max_response_bytes(max_response_bytes)
    _, media_type = _response_content_type(response)
    body = read_bounded_body(response, max_response_bytes=limit)
    if not body or media_type != _CONTENT_TYPE_JSON:
        return None
    try:
        return _decode_json(body, context="HTTP error response")
    except ProtocolError:
        return None


def _response_content_type(
    response: HttpPayloadResponse,
) -> tuple[str, str]:
    content_type = response.getheader("Content-Type", "") or ""
    if content_type and not all(
        character.isascii() and character.isprintable()
        for character in content_type
    ):
        fail_protocol("HTTP response has invalid Content-Type header")
    media_type = content_type.partition(";")[0].strip().casefold()
    return content_type, media_type


def read_bounded_body(
    response: HttpPayloadResponse,
    *,
    max_response_bytes: int,
) -> bytes:
    """Read one HTTP body without crossing its byte ceiling.

    Returns:
        The complete body when it fits within the configured limit.
    """
    limit = validate_max_response_bytes(max_response_bytes)
    declared_length = _validate_content_length(response, limit)
    body = response.read(limit + 1)
    if len(body) > limit:
        fail_protocol(f"HTTP response exceeded {limit} bytes")
    if declared_length is not None and len(body) != declared_length:
        fail_protocol("HTTP response Content-Length does not match body size")
    return body


def _validate_content_length(
    response: HttpPayloadResponse,
    limit: int,
) -> int | None:
    raw_length = response.getheader("Content-Length")
    if raw_length is None:
        return None
    if raw_length.startswith("-"):
        magnitude = raw_length[1:]
        if magnitude.isascii() and magnitude.isdigit():
            fail_protocol("HTTP response Content-Length is negative")
    if not raw_length.isascii() or not raw_length.isdigit():
        fail_protocol("HTTP response Content-Length is invalid")
    try:
        length = int(raw_length)
    except ValueError as error:
        fail_protocol("HTTP response Content-Length is invalid", cause=error)
    if length > limit:
        fail_protocol(f"HTTP response exceeded {limit} bytes")
    return length


def _read_sse_payload(
    response: HttpPayloadResponse,
    request_id: int,
    limit: int,
) -> JsonObject:
    data_lines: list[str] = []
    total_bytes = 0
    while True:
        raw_line = response.readline(limit - total_bytes + 1)
        if not raw_line:
            break
        total_bytes += len(raw_line)
        if total_bytes > limit:
            fail_protocol(f"SSE response exceeded {limit} bytes")
        try:
            line = raw_line.decode("utf-8").rstrip("\r\n")
        except UnicodeDecodeError as error:
            fail_protocol("SSE data is not valid UTF-8", cause=error)
        if not line:
            payload = _finish_sse_event(data_lines, request_id)
            if payload is not None:
                return payload
            continue
        if line.startswith("data:"):
            data_lines.append(line.removeprefix("data:").lstrip())
    payload = _finish_sse_event(data_lines, request_id)
    if payload is not None:
        return payload
    return fail_protocol(f"SSE stream ended before response id {request_id}")


def _finish_sse_event(
    data_lines: list[str],
    request_id: int,
) -> JsonObject | None:
    if not data_lines:
        return None
    if not any(data_lines):
        data_lines.clear()
        return None
    payload = _decode_json(
        "\n".join(data_lines).encode(),
        context="SSE data",
    )
    data_lines.clear()
    if matches_integer_request_id(payload.get("id"), request_id):
        return payload
    return None


def _decode_json(body: bytes, *, context: str) -> JsonObject:
    try:
        text = body.decode("utf-8")
        parsed = cast(
            "object",
            json.loads(
                text,
                object_pairs_hook=reject_duplicate_json_object,
                parse_constant=_reject_non_finite_constant,
            ),
        )
    except DuplicateJsonKeyError as error:
        fail_protocol(str(error), cause=error)
    except ValueError as error:
        fail_protocol(f"{context} is not valid JSON", cause=error)
    return require_json_object(parsed, context=context)


def _reject_non_finite_constant(value: str) -> NoReturn:
    fail_protocol(f"JSON contains non-finite number {value}")
