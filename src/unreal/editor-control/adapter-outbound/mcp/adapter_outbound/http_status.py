# Copyright:
#   - Copyright © 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - Http status outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Http status outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Http status outbound adapter."""

from __future__ import annotations

from typing import Never

from mcp.adapter_outbound.response_validation import matches_integer_request_id
from mcp.adapter_outbound.response_validation import (
    validated_json_rpc_error_message,
)
from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import require_json_object

_JSON_RPC_VERSION = "2.0"
_HTTP_SUCCESS_MINIMUM = 200
_HTTP_SUCCESS_LIMIT = 300


def is_http_success(status: int) -> bool:
    """Return whether one HTTP status is in the successful 2xx range."""
    return _HTTP_SUCCESS_MINIMUM <= status < _HTTP_SUCCESS_LIMIT


def require_http_success(
    status: int,
    payload: JsonObject | None,
    *,
    request_id: int | None,
) -> None:
    """Require one successful HTTP status or raise its protocol failure."""
    if is_http_success(status):
        return
    raise_http_status_error(status, payload, request_id=request_id)


def raise_http_status_error(
    status: int,
    payload: JsonObject | None,
    *,
    request_id: int | None,
) -> Never:
    """Raise one protocol failure for an unsuccessful HTTP response.

    Args:
        status: Unsuccessful HTTP status code.
        payload: Optional decoded JSON-RPC response body.
        request_id: Originating integer request identity, or `None`.

    """
    message = _validated_error_message(payload, request_id)
    if message is None:
        fail_protocol(f"MCP server returned HTTP {status}")
    fail_protocol(f"HTTP {status}: {message}")


def _validated_error_message(
    payload: JsonObject | None,
    request_id: int | None,
) -> str | None:
    if (
        payload is None
        or request_id is None
        or payload.get("jsonrpc") != _JSON_RPC_VERSION
        or "result" in payload
        or not matches_integer_request_id(payload.get("id"), request_id)
    ):
        return None
    error_value = payload.get("error")
    if not isinstance(error_value, dict):
        return None
    error = require_json_object(error_value, context="HTTP error")
    code = error.get("code")
    if not isinstance(code, int) or isinstance(code, bool):
        return None
    return validated_json_rpc_error_message(error.get("message"))
