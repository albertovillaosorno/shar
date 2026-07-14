# File:
#   - http_request.py
# Path:
#   - src/mcp/src/adapters/driven/http_request.py
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
#   - Deterministic bounded JSON encoding for MCP HTTP requests.
# - Must-Not:
#   - Open sockets, manage sessions, or interpret tool semantics.
# - Allows:
#   - UTF-8 serialization and positive request byte ceilings.
# - Split-When:
#   - Streaming uploads or compression require independent policies.
# - Merge-When:
#   - Another adapter module owns the same request-encoding invariant.
# - Summary:
#   - Encodes bounded native MCP HTTP request bodies.
# - Description:
#   - Rejects oversized serialized JSON before opening a connection.
# - Usage:
#   - Called by the loopback HTTP exchange adapter.
# - Defaults:
#   - Limits each serialized request body to 64 MiB.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
#
# Large file:
#   - false
#
"""Bounded native MCP HTTP request serialization."""

from __future__ import annotations

import json
from typing import TYPE_CHECKING

from mcp.src.domain.errors import fail_configuration, fail_transport

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject

DEFAULT_MAX_REQUEST_BYTES = 64 * 1024 * 1024


def validate_max_request_bytes(value: int) -> int:
    """Return one positive request byte ceiling.

    Returns:
        The validated request byte ceiling.
    """
    if value <= 0:
        fail_configuration("max_request_bytes must be positive")
    return value


def encode_json_request(
    payload: JsonObject,
    *,
    max_request_bytes: int,
) -> bytes:
    """Encode one strict JSON request within its byte ceiling.

    Args:
        payload: Strict JSON request or notification object.
        max_request_bytes: Maximum serialized UTF-8 body size.

    Returns:
        Deterministic compact UTF-8 JSON bytes.
    """
    limit = validate_max_request_bytes(max_request_bytes)
    try:
        serialized = json.dumps(
            payload,
            ensure_ascii=False,
            allow_nan=False,
            separators=(",", ":"),
        )
    except (TypeError, ValueError) as error:
        fail_transport(
            "MCP request contains a non-finite number or cycle",
            cause=error,
        )
    body = serialized.encode("utf-8")
    if len(body) > limit:
        fail_transport(f"MCP request exceeded {limit} bytes")
    return body
