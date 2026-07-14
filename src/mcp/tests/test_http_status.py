# File:
#   - test_http_status.py
# Path:
#   - src/mcp/tests/test_http_status.py
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
#   - Regression evidence for unsuccessful MCP HTTP status interpretation.
# - Must-Not:
#   - Open sockets, invoke Unreal, or test HTTP framing mechanics.
# - Allows:
#   - Verifying stable protocol failures from decoded response bodies.
# - Split-When:
#   - HTTP error interpretation gains a second independent protocol.
# - Merge-When:
#   - Another test module owns the same pure status-translation evidence.
# - Summary:
#   - Guards HTTP-to-protocol failure translation.
# - Description:
#   - Proves the extracted status interpreter preserves failure behavior.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Uses deterministic decoded JSON objects only.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Regression tests for unsuccessful MCP HTTP status interpretation."""

from __future__ import annotations

from typing import TYPE_CHECKING

import pytest
from mcp.src.adapters.driven.http_status import (
    raise_http_status_error,
)
from mcp.src.domain.errors import ProtocolError

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject


def test_missing_payload_reports_status() -> None:
    """An empty error response reports its HTTP status."""
    with pytest.raises(
        ProtocolError,
        match=r"^MCP server returned HTTP 500$",
    ):
        raise_http_status_error(500, None)


def test_rpc_error_reports_server_message() -> None:
    """A valid JSON-RPC error exposes its bounded text message."""
    payload: JsonObject = {
        "error": {
            "code": -32600,
            "message": "invalid request",
        }
    }

    with pytest.raises(
        ProtocolError,
        match=r"^HTTP 400: invalid request$",
    ):
        raise_http_status_error(400, payload)


def test_non_text_message_uses_stable_fallback() -> None:
    """A non-text JSON-RPC message never leaks an arbitrary value."""
    payload: JsonObject = {
        "error": {
            "code": -32603,
            "message": 17,
        }
    }

    with pytest.raises(
        ProtocolError,
        match=r"^HTTP 500: unknown MCP error$",
    ):
        raise_http_status_error(500, payload)
