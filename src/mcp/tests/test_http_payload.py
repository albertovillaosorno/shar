# File:
#   - test_http_payload.py
# Path:
#   - src/mcp/tests/test_http_payload.py
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
#   - Regression evidence for bounded JSON and SSE response decoding.
# - Must-Not:
#   - Open sockets, invoke Unreal, or depend on external services.
# - Allows:
#   - In-memory HTTP body and header fixtures.
# - Split-When:
#   - JSON and SSE fixtures require independent support classes.
# - Merge-When:
#   - Another test module owns the same payload byte-limit contract.
# - Summary:
#   - Guards bounded native MCP response decoding.
# - Description:
#   - Proves content-length and streamed overflow fail before allocation growth.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Uses deliberately small byte ceilings for deterministic tests.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: bounded HTTP payload regression tests
#   - reason: JSON, SSE, and header limits share one response boundary
#   - split: split SSE fixtures if replay or reconnection is implemented
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Regression tests for bounded native MCP HTTP payload decoding."""

from __future__ import annotations

from io import BytesIO
from typing import TypeVar, overload

import pytest
from mcp.src.adapters.driven.http_payload import (
    read_bounded_body,
    read_http_payload,
    validate_max_response_bytes,
)
from mcp.src.domain.errors import ConfigurationError, ProtocolError

_HeaderDefault = TypeVar("_HeaderDefault")


class MemoryResponse:
    """Minimal in-memory response matching the payload reader protocol."""

    def __init__(
        self,
        body: bytes,
        *,
        headers: dict[str, str] | None = None,
    ) -> None:
        self._body = BytesIO(body)
        self._headers = {} if headers is None else headers

    @overload
    def getheader(self, name: str) -> str | None: ...

    @overload
    def getheader(
        self,
        name: str,
        default: _HeaderDefault,
    ) -> str | _HeaderDefault: ...

    def getheader(
        self,
        name: str,
        default: object = None,
    ) -> str | object:
        """Return one case-sensitive synthetic header."""
        return self._headers.get(name, default)

    def read(self, amt: int | None = None) -> bytes:
        """Read response bytes.

        Returns:
            The requested bytes or the remaining body.
        """
        return self._body.read() if amt is None else self._body.read(amt)

    def readline(self, limit: int = -1) -> bytes:
        """Read one bounded response line.

        Returns:
            One line or the remaining bounded fragment.
        """
        return self._body.readline(limit)


def test_json_payload_respects_content_length_and_stream_limit() -> None:
    """JSON bodies are accepted only when every byte fits the ceiling."""
    body = b'{"jsonrpc":"2.0","id":1,"result":{}}'
    response = MemoryResponse(
        body,
        headers={
            "Content-Type": "application/json",
            "Content-Length": str(len(body)),
        },
    )

    assert read_http_payload(
        response,
        1,
        max_response_bytes=len(body),
    ) == {"jsonrpc": "2.0", "id": 1, "result": {}}

    with pytest.raises(ProtocolError, match="exceeded 4 bytes"):
        _ = read_bounded_body(
            MemoryResponse(b"12345"),
            max_response_bytes=4,
        )


def test_json_payload_rejects_non_finite_numbers() -> None:
    """JSON-RPC responses cannot contain NaN or infinity literals."""
    for literal in (b"NaN", b"Infinity", b"-Infinity"):
        body = b'{"jsonrpc":"2.0","id":1,"result":{"value":' + literal + b"}}"
        with pytest.raises(ProtocolError, match="non-finite"):
            _ = read_http_payload(
                MemoryResponse(
                    body,
                    headers={"Content-Type": "application/json"},
                ),
                1,
                max_response_bytes=len(body),
            )


def test_json_payload_rejects_duplicate_object_keys() -> None:
    """Ambiguous JSON objects cannot overwrite an earlier member."""
    body = b'{"jsonrpc":"2.0","id":1,"result":{"value":1,"value":2}}'
    with pytest.raises(ProtocolError, match="duplicate JSON key: value"):
        _ = read_http_payload(
            MemoryResponse(body, headers={"Content-Type": "application/json"}),
            1,
            max_response_bytes=len(body),
        )


def test_declared_content_length_fails_before_body_read() -> None:
    """An oversized or malformed Content-Length fails immediately."""
    with pytest.raises(ProtocolError, match="exceeded 4 bytes"):
        _ = read_bounded_body(
            MemoryResponse(b"x", headers={"Content-Length": "5"}),
            max_response_bytes=4,
        )
    with pytest.raises(ProtocolError, match="Content-Length is invalid"):
        _ = read_bounded_body(
            MemoryResponse(b"x", headers={"Content-Length": "many"}),
            max_response_bytes=4,
        )
    with pytest.raises(ProtocolError, match="Content-Length is negative"):
        _ = read_bounded_body(
            MemoryResponse(b"", headers={"Content-Length": "-1"}),
            max_response_bytes=4,
        )


def test_sse_progress_is_skipped_until_matching_final_response() -> None:
    """Accept a final SSE event at EOF after progress events."""
    body = (
        b'data: {"jsonrpc":"2.0","method":"notifications/progress"}\n\n'
        b'data: {"jsonrpc":"2.0","id":7,"result":{}}\n'
    )
    response = MemoryResponse(
        body,
        headers={"Content-Type": "text/event-stream"},
    )

    assert read_http_payload(
        response,
        7,
        max_response_bytes=len(body),
    ) == {"jsonrpc": "2.0", "id": 7, "result": {}}


def test_sse_overflow_and_notification_stream_fail_closed() -> None:
    """SSE cannot cross the byte ceiling or answer a notification."""
    body = b'data: {"jsonrpc":"2.0","id":2,"result":{}}\n\n'
    headers = {"Content-Type": "text/event-stream"}
    with pytest.raises(ProtocolError, match="SSE response exceeded"):
        _ = read_http_payload(
            MemoryResponse(body, headers=headers),
            2,
            max_response_bytes=len(body) - 1,
        )
    with pytest.raises(ProtocolError, match="notification unexpectedly"):
        _ = read_http_payload(
            MemoryResponse(body, headers=headers),
            None,
            max_response_bytes=len(body),
        )


def test_response_limit_must_be_positive() -> None:
    """Invalid byte ceilings fail before any response is read."""
    with pytest.raises(ConfigurationError, match="must be positive"):
        _ = validate_max_response_bytes(0)
