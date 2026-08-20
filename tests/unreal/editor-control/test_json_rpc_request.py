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
#   - Test json rpc request test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test json rpc request test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test json rpc request test module."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.adapter_outbound.json_rpc_request import build_json_rpc_request

if TYPE_CHECKING:
    from mcp.domain.json_types import JsonObject


def test_request_includes_numeric_id() -> None:
    """A request includes its caller-provided numeric identifier."""
    params: JsonObject = {"name": "list_toolsets"}

    payload = build_json_rpc_request(
        method="tools/call",
        params=params,
        request_id=7,
    )

    assert payload == {
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": params,
        "id": 7,
    }
    assert payload["params"] is params


def test_notification_omits_id() -> None:
    """A notification has no JSON-RPC identifier member."""
    payload = build_json_rpc_request(
        method="notifications/initialized",
        params={},
        request_id=None,
    )

    assert payload == {
        "jsonrpc": "2.0",
        "method": "notifications/initialized",
        "params": {},
    }
    assert "id" not in payload
