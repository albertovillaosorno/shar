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
#   - Test http status test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test http status test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test http status test module."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.adapter_outbound.http_status import raise_http_status_error
from mcp.domain.errors import ProtocolError
import pytest

if TYPE_CHECKING:
    from mcp.domain.json_types import JsonObject


def test_missing_payload_reports_status() -> None:
    """An empty error response reports its HTTP status."""
    with pytest.raises(
        ProtocolError,
        match=r"^MCP server returned HTTP 500$",
    ):
        raise_http_status_error(500, None, request_id=1)


def test_rpc_error_reports_server_message() -> None:
    """A valid JSON-RPC error exposes its bounded text message."""
    payload: JsonObject = {
        "jsonrpc": "2.0",
        "id": 1,
        "error": {
            "code": -32600,
            "message": "invalid request",
        },
    }

    with pytest.raises(
        ProtocolError,
        match=r"^HTTP 400: invalid request$",
    ):
        raise_http_status_error(400, payload, request_id=1)


def test_oversized_rpc_error_uses_status_fallback(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """An oversized server message cannot replace the stable HTTP status."""
    monkeypatch.setattr(
        # jig-ignore-next-line: exact value is indivisible
        "mcp.adapter_outbound.response_validation._MAX_JSON_RPC_ERROR_MESSAGE_BYTES",
        4,
        raising=False,
    )
    payload: JsonObject = {
        "jsonrpc": "2.0",
        "id": 1,
        "error": {
            "code": -32600,
            "message": "abcde",
        },
    }

    with pytest.raises(
        ProtocolError,
        match=r"^MCP server returned HTTP 400$",
    ):
        raise_http_status_error(400, payload, request_id=1)


def test_malformed_rpc_error_uses_status_fallback() -> None:
    """Only a complete JSON-RPC error may replace the HTTP status message."""
    payloads: tuple[JsonObject, ...] = (
        {"error": {"code": -32603, "message": "unversioned"}},
        {
            "jsonrpc": "2.0",
            "error": {"code": -32603, "message": "missing id"},
        },
        {
            "jsonrpc": "2.0",
            "id": 2,
            "error": {"code": -32603, "message": "wrong id"},
        },
        {
            "jsonrpc": "2.0",
            "id": True,
            "error": {"code": -32603, "message": "boolean id"},
        },
        {
            "jsonrpc": "2.0",
            "id": 1.0,
            "error": {"code": -32603, "message": "floating id"},
        },
        {
            "jsonrpc": "2.0",
            "id": 1,
            "error": {"code": True, "message": "failed"},
        },
        {
            "jsonrpc": "2.0",
            "id": 1,
            "error": {"code": 1.0, "message": "failed"},
        },
        {
            "jsonrpc": "2.0",
            "id": 1,
            "error": {"code": -32603, "message": 17},
        },
        {
            "jsonrpc": "2.0",
            "id": 1,
            "error": {"code": -32603, "message": ""},
        },
        {
            "jsonrpc": "2.0",
            "id": 1,
            "error": {"code": -32603, "message": "failed\ninjected"},
        },
        {
            "jsonrpc": "2.0",
            "id": 1,
            "error": {"code": -32603, "message": "failed\x07"},
        },
        {
            "jsonrpc": "2.0",
            "id": 1,
            "result": {},
            "error": {"code": -32603, "message": "failed"},
        },
    )

    for payload in payloads:
        with pytest.raises(
            ProtocolError,
            match=r"^MCP server returned HTTP 500$",
        ):
            raise_http_status_error(500, payload, request_id=1)
