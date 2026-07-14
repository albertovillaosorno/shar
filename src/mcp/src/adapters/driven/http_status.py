# File:
#   - http_status.py
# Path:
#   - src/mcp/src/adapters/driven/http_status.py
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
#   - Translation of unsuccessful MCP HTTP responses into protocol failures.
# - Must-Not:
#   - Open connections, read response streams, or construct MCP requests.
# - Allows:
#   - Extracting a bounded JSON-RPC error message from an HTTP response body.
# - Split-When:
#   - Status interpretation gains a second independent error protocol.
# - Merge-When:
#   - Another module owns the same HTTP-to-domain failure invariant.
# - Summary:
#   - Converts unsuccessful HTTP outcomes into typed protocol failures.
# - Description:
#   - Separates response-status interpretation from wire exchange mechanics.
# - Usage:
#   - Called by the HTTP exchange client after decoding the response body.
# - Defaults:
#   - Uses a stable fallback when no valid JSON-RPC error message exists.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""HTTP status interpretation for native Unreal MCP exchanges."""

from __future__ import annotations

from typing import Never

from mcp.src.domain.errors import fail_protocol
from mcp.src.domain.json_types import JsonObject, require_json_object

_JSON_RPC_VERSION = "2.0"
_HTTP_SUCCESS_MINIMUM = 200
_HTTP_SUCCESS_LIMIT = 300


def require_http_success(
    status: int,
    payload: JsonObject | None,
) -> None:
    """Require one successful HTTP status or raise its protocol failure."""
    if _HTTP_SUCCESS_MINIMUM <= status < _HTTP_SUCCESS_LIMIT:
        return
    raise_http_status_error(status, payload)


def raise_http_status_error(
    status: int,
    payload: JsonObject | None,
) -> Never:
    """Raise one protocol failure for an unsuccessful HTTP response.

    Args:
        status: Unsuccessful HTTP status code.
        payload: Optional decoded JSON-RPC response body.
    """
    message = _validated_error_message(payload)
    if message is None:
        fail_protocol(f"MCP server returned HTTP {status}")
    fail_protocol(f"HTTP {status}: {message}")


def _validated_error_message(payload: JsonObject | None) -> str | None:
    if (
        payload is None
        or payload.get("jsonrpc") != _JSON_RPC_VERSION
        or "result" in payload
    ):
        return None
    error_value = payload.get("error")
    if not isinstance(error_value, dict):
        return None
    error = require_json_object(error_value, context="HTTP error")
    code = error.get("code")
    if not isinstance(code, int) or isinstance(code, bool):
        return None
    message = error.get("message")
    return message if isinstance(message, str) else None
