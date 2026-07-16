# File:
#   - test_response_validation.py
# Path:
#   - src/mcp/tests/test_response_validation.py
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
#   - Regression evidence for native MCP response validation.
# - Must-Not:
#   - Open sockets, invoke Unreal, or exercise terminal parsing.
# - Allows:
#   - Pure JSON-RPC, session, identity, version, and capability fixtures.
# - Split-When:
#   - Initialization and general JSON-RPC validation need separate fixtures.
# - Merge-When:
#   - Another test module owns the same pure response evidence.
# - Summary:
#   - Guards fail-closed native Unreal MCP response validation.
# - Description:
#   - Proves invalid wire identities cannot cross into application sessions.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Uses protocol version 2025-11-25 and server name unreal-mcp.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: native MCP response validation tests
#   - reason: protocol identity and session fixtures share one boundary
#   - split: split initialization from general result tests if either expands
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Regression tests for native Unreal MCP response validation."""

from __future__ import annotations

from typing import TYPE_CHECKING

import pytest
from mcp.src.adapters.driven.http_exchange import HttpExchange
from mcp.src.adapters.driven.response_validation import (
    parse_initialized_session,
    parse_tool_names,
    require_json_rpc_result,
)
from mcp.src.domain.errors import ProtocolError

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject, JsonValue

_PROTOCOL_VERSION = "2025-11-25"
_SERVER_NAME = ""
_SESSION_ID = "0123456789abcdef"
_REQUEST_ID = 1


def test_valid_initialize_response_creates_unreal_session() -> None:
    """Complete native initialization evidence creates one session."""
    session = parse_initialized_session(
        _initialize_exchange(),
        _REQUEST_ID,
        expected_protocol_version=_PROTOCOL_VERSION,
    )

    assert session.session_id == _SESSION_ID
    assert session.protocol_version == _PROTOCOL_VERSION
    assert session.server_name == _SERVER_NAME


def test_initialize_rejects_non_visible_ascii_session_id() -> None:
    """Control characters cannot enter subsequent HTTP headers."""
    with pytest.raises(ProtocolError, match="valid MCP-Session-Id"):
        _ = parse_initialized_session(
            _initialize_exchange(session_id="bad\nsession"),
            _REQUEST_ID,
            expected_protocol_version=_PROTOCOL_VERSION,
        )


def test_initialize_rejects_excessive_session_id_length(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """A server cannot force oversized session headers onto every request."""
    monkeypatch.setattr(
        "mcp.src.adapters.driven.response_validation._MAX_SESSION_ID_LENGTH",
        4,
        raising=False,
    )

    with pytest.raises(ProtocolError, match="valid MCP-Session-Id"):
        _ = parse_initialized_session(
            _initialize_exchange(session_id="abcde"),
            _REQUEST_ID,
            expected_protocol_version=_PROTOCOL_VERSION,
        )


def test_initialize_rejects_wrong_protocol_and_malformed_metadata() -> None:
    """Protocol negotiation and server metadata remain strictly typed."""
    wrong_protocol = _initialize_result(protocol_version="2025-06-18")
    with pytest.raises(ProtocolError, match="unsupported protocol version"):
        _ = parse_initialized_session(
            _exchange(wrong_protocol),
            _REQUEST_ID,
            expected_protocol_version=_PROTOCOL_VERSION,
        )

    malformed_server = _initialize_result()
    malformed_server["serverInfo"] = {"name": 17, "version": ""}
    with pytest.raises(ProtocolError, match=r"serverInfo\.name must be text"):
        _ = parse_initialized_session(
            _exchange(malformed_server),
            _REQUEST_ID,
            expected_protocol_version=_PROTOCOL_VERSION,
        )


@pytest.mark.parametrize("field", ["name", "version"])
def test_initialize_rejects_controls_in_server_metadata(field: str) -> None:
    """Server metadata cannot retain JSON-escaped control characters."""
    result = _initialize_result(server_name="unreal-mcp")
    server_info: JsonObject = {
        "name": "unreal-mcp",
        "version": "1.0.0",
    }
    server_info[field] = "bad\ninjected"
    result["serverInfo"] = server_info

    with pytest.raises(
        ProtocolError,
        match=rf"serverInfo\.{field} must be printable text",
    ):
        _ = parse_initialized_session(
            _exchange(result),
            _REQUEST_ID,
            expected_protocol_version=_PROTOCOL_VERSION,
        )


@pytest.mark.parametrize("field", ["name", "version"])
def test_initialize_rejects_excessive_server_metadata_length(
    field: str,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """Initialization metadata cannot retain oversized printable text."""
    monkeypatch.setattr(
        "mcp.src.adapters.driven.response_validation._MAX_SERVER_METADATA_LENGTH",
        4,
        raising=False,
    )
    result = _initialize_result(server_name="name")
    server_info: JsonObject = {
        "name": "name",
        "version": "1.0",
    }
    server_info[field] = "abcde"
    result["serverInfo"] = server_info

    with pytest.raises(
        ProtocolError,
        match=rf"serverInfo\.{field} exceeds its length limit",
    ):
        _ = parse_initialized_session(
            _exchange(result),
            _REQUEST_ID,
            expected_protocol_version=_PROTOCOL_VERSION,
        )


def test_initialize_protocol_error_does_not_reflect_control_text() -> None:
    """Malformed negotiation text cannot add lines to diagnostics."""
    result = _initialize_result(protocol_version="bad\ninjected")

    with pytest.raises(ProtocolError) as caught:
        _ = parse_initialized_session(
            _exchange(result),
            _REQUEST_ID,
            expected_protocol_version=_PROTOCOL_VERSION,
        )

    assert str(caught.value) == (
        "initialize negotiated an unsupported protocol version"
    )


def test_initialize_requires_tools_capability() -> None:
    """A server without tools cannot satisfy the translator contract."""
    result = _initialize_result()
    result["capabilities"] = {}

    with pytest.raises(ProtocolError, match=r"capabilities\.tools"):
        _ = parse_initialized_session(
            _exchange(result),
            _REQUEST_ID,
            expected_protocol_version=_PROTOCOL_VERSION,
        )


def test_tool_names_reject_whitespace_identities() -> None:
    """Discovery names cannot hide padding or embedded whitespace."""
    for name in (" list_toolsets", "list_toolsets ", "list toolsets"):
        with pytest.raises(
            ProtocolError,
            match=r"tools/list\.tools\[0\]\.name",
        ):
            _ = parse_tool_names({"tools": [{"name": name}]})


def test_tool_names_reject_control_characters() -> None:
    """Discovery names cannot contain hidden terminal or routing controls."""
    for name in ("list\x00toolsets", "list\x07toolsets"):
        with pytest.raises(
            ProtocolError,
            match=r"tools/list\.tools\[0\]\.name",
        ):
            _ = parse_tool_names({"tools": [{"name": name}]})


def test_tool_names_reject_excessive_identity_bytes(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """One discovery identity cannot consume the aggregate name budget."""
    monkeypatch.setattr(
        "mcp.src.adapters.driven.response_validation._MAX_TOOL_NAME_BYTES",
        4,
        raising=False,
    )

    with pytest.raises(ProtocolError, match="name exceeds its byte limit"):
        _ = parse_tool_names({"tools": [{"name": "abcde"}]})


def test_tool_names_reject_excessive_page_entries(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """One tools/list page cannot force unbounded normalization work."""
    monkeypatch.setattr(
        "mcp.src.adapters.driven.response_validation._MAX_TOOLS_PER_PAGE",
        2,
        raising=False,
    )
    result: JsonObject = {
        "tools": [
            {"name": "first"},
            {"name": "second"},
            {"name": "third"},
        ]
    }

    with pytest.raises(ProtocolError, match="page tool limit"):
        _ = parse_tool_names(result)


def test_tool_names_reject_duplicate_identities() -> None:
    """Top-level capability discovery cannot contain duplicate names."""
    with pytest.raises(ProtocolError, match="duplicate tool identity"):
        _ = parse_tool_names(
            {
                "tools": [
                    {"name": "list_toolsets"},
                    {"name": "list_toolsets"},
                ]
            }
        )


def test_result_requires_exclusive_result_or_error_member() -> None:
    """Null or populated error members cannot coexist with result."""
    error_values: tuple[JsonValue, ...] = (
        None,
        {"code": -1, "message": "failed"},
    )
    for error_value in error_values:
        payload: JsonObject = {
            "jsonrpc": "2.0",
            "id": _REQUEST_ID,
            "result": {},
            "error": error_value,
        }
        exchange = HttpExchange(
            status=200,
            session_id=None,
            payload=payload,
        )

        with pytest.raises(
            ProtocolError,
            match="exactly one of result or error",
        ):
            _ = require_json_rpc_result(exchange, _REQUEST_ID)


def test_result_requires_well_formed_json_rpc_error() -> None:
    """Error responses require one integer code and text message."""
    malformed_errors: tuple[JsonValue, ...] = (
        {"code": True, "message": "failed"},
        {"code": 1.0, "message": "failed"},
        {"code": -1, "message": 17},
        {"code": -1, "message": ""},
        {"code": -1, "message": "failed\ninjected"},
        {"code": -1, "message": "failed\x07"},
        {"message": "failed"},
    )
    for error_value in malformed_errors:
        payload: JsonObject = {
            "jsonrpc": "2.0",
            "id": _REQUEST_ID,
            "error": error_value,
        }
        exchange = HttpExchange(
            status=200,
            session_id=None,
            payload=payload,
        )

        with pytest.raises(
            ProtocolError,
            match=r"JSON-RPC error\.(code|message)",
        ):
            _ = require_json_rpc_result(exchange, _REQUEST_ID)

    valid_error = HttpExchange(
        status=200,
        session_id=None,
        payload={
            "jsonrpc": "2.0",
            "id": _REQUEST_ID,
            "error": {"code": -32000, "message": "native failure"},
        },
    )
    with pytest.raises(ProtocolError, match="native failure"):
        _ = require_json_rpc_result(valid_error, _REQUEST_ID)


def test_result_rejects_excessive_json_rpc_error_message(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """One server error cannot become an oversized exception diagnostic."""
    monkeypatch.setattr(
        "mcp.src.adapters.driven.response_validation._MAX_JSON_RPC_ERROR_MESSAGE_BYTES",
        4,
        raising=False,
    )
    exchange = HttpExchange(
        status=200,
        session_id=None,
        payload={
            "jsonrpc": "2.0",
            "id": _REQUEST_ID,
            "error": {"code": -32000, "message": "abcde"},
        },
    )

    with pytest.raises(ProtocolError, match=r"JSON-RPC error\.message"):
        _ = require_json_rpc_result(exchange, _REQUEST_ID)


def test_result_requires_exact_integer_request_id() -> None:
    """Boolean and floating identifiers cannot alias an integer request."""
    for response_id in (True, 1.0):
        exchange = HttpExchange(
            status=200,
            session_id=None,
            payload={"jsonrpc": "2.0", "id": response_id, "result": {}},
        )

        with pytest.raises(ProtocolError, match="response id mismatch"):
            _ = require_json_rpc_result(exchange, _REQUEST_ID)


def test_response_id_error_does_not_reflect_control_text() -> None:
    """Malformed response identities cannot add lines to diagnostics."""
    exchange = HttpExchange(
        status=200,
        session_id=None,
        payload={
            "jsonrpc": "2.0",
            "id": "bad\ninjected",
            "result": {},
        },
    )

    with pytest.raises(ProtocolError) as caught:
        _ = require_json_rpc_result(exchange, _REQUEST_ID)

    assert str(caught.value) == (
        f"response id mismatch: expected {_REQUEST_ID}"
    )


def test_result_requires_json_rpc_version_two() -> None:
    """A lookalike response without JSON-RPC 2.0 is rejected."""
    exchange = HttpExchange(
        status=200,
        session_id=None,
        payload={"id": _REQUEST_ID, "result": {}},
    )

    with pytest.raises(ProtocolError, match=r"JSON-RPC version 2\.0"):
        _ = require_json_rpc_result(exchange, _REQUEST_ID)


def _initialize_exchange(
    *,
    session_id: str = _SESSION_ID,
) -> HttpExchange:
    return _exchange(_initialize_result(), session_id=session_id)


def _exchange(
    result: JsonObject,
    *,
    session_id: str = _SESSION_ID,
) -> HttpExchange:
    return HttpExchange(
        status=200,
        session_id=session_id,
        payload={
            "jsonrpc": "2.0",
            "id": _REQUEST_ID,
            "result": result,
        },
    )


def _initialize_result(
    *,
    protocol_version: str = _PROTOCOL_VERSION,
    server_name: str = _SERVER_NAME,
) -> JsonObject:
    return {
        "protocolVersion": protocol_version,
        "capabilities": {"tools": {"listChanged": True}},
        "serverInfo": {
            "name": server_name,
            "version": "",
        },
    }
