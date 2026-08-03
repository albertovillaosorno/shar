# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - Json rpc request outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Json rpc request outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Json rpc request outbound adapter."""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from mcp.domain.json_types import JsonObject

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
