# File:
#   - json_rpc_request.py
# Path:
#   - src/mcp/src/adapters/driven/json_rpc_request.py
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
#   - Construction of strict JSON-RPC request and notification objects.
# - Must-Not:
#   - Perform HTTP exchanges, allocate sessions, or interpret responses.
# - Allows:
#   - Adding the protocol marker, method, parameters, and optional request id.
# - Split-When:
#   - Request construction gains a second independently testable protocol.
# - Merge-When:
#   - Another module owns the same JSON-RPC construction invariant.
# - Summary:
#   - Builds transport-independent JSON-RPC request objects.
# - Description:
#   - Keeps protocol object construction outside the HTTP transport lifecycle.
# - Usage:
#   - Called by the Streamable HTTP transport before an exchange.
# - Defaults:
#   - Emits JSON-RPC version 2.0 and omits ids for notifications.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Strict JSON-RPC request construction for the Unreal MCP client."""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject

_JSON_RPC_VERSION = "2.0"


def build_json_rpc_request(
    *,
    method: str,
    params: JsonObject,
    request_id: int | None,
) -> JsonObject:
    """Build one JSON-RPC request or notification object.

    Args:
        method: JSON-RPC method name.
        params: Strict JSON object containing method parameters.
        request_id: Numeric request id, or `None` for a notification.

    Returns:
        A new JSON object with the protocol marker and optional id.
    """
    payload: JsonObject = {
        "jsonrpc": _JSON_RPC_VERSION,
        "method": method,
        "params": params,
    }
    if request_id is not None:
        payload["id"] = request_id
    return payload
