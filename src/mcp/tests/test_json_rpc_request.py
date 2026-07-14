# File:
#   - test_json_rpc_request.py
# Path:
#   - src/mcp/tests/test_json_rpc_request.py
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
#   - Regression evidence for strict JSON-RPC request construction.
# - Must-Not:
#   - Open sockets, invoke Unreal, or depend on external services.
# - Allows:
#   - Verifying requests, notifications, and caller-owned parameters.
# - Split-When:
#   - Request construction gains a second independent protocol contract.
# - Merge-When:
#   - Another test module owns the same pure request-construction evidence.
# - Summary:
#   - Guards JSON-RPC request and notification object shape.
# - Description:
#   - Proves the extracted request builder preserves transport behavior.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Uses only deterministic in-memory JSON objects.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Regression tests for strict JSON-RPC request construction."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.src.adapters.driven.json_rpc_request import (
    build_json_rpc_request,
)

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject


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
