# File:
#   - response_validation.py
# Path:
#   - src/mcp/src/adapters/driven/response_validation.py
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
#   - Pure validation of MCP JSON-RPC and initialization responses.
# - Must-Not:
#   - Open sockets, execute tools, parse CLI input, or mutate editor state.
# - Allows:
#   - Constructing domain session values from validated wire evidence.
# - Split-When:
#   - Tool-list parsing and initialization validation evolve independently.
# - Merge-When:
#   - Another module owns the same wire-response invariants.
# - Summary:
#   - Validates native Unreal MCP response contracts fail closed.
# - Description:
#   - Separates protocol evidence validation from HTTP exchange mechanics.
# - Usage:
#   - Called by the Streamable HTTP adapter after response decoding.
# - Defaults:
#   - Rejects unsupported versions, capabilities, IDs, and JSON-RPC forms.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: native MCP response validation boundary
#   - reason: initialization and result checks share one JSON-RPC trust boundary
#   - split: extract initialization checks if capability negotiation expands
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Pure response validation for the native Unreal MCP transport."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.src.domain.errors import fail_protocol
from mcp.src.domain.json_types import JsonObject, require_json_object
from mcp.src.domain.session import McpSession

if TYPE_CHECKING:
    from mcp.src.adapters.driven.http_exchange import HttpExchange

_JSON_RPC_VERSION = "2.0"
_VISIBLE_ASCII_MINIMUM = 0x21
_VISIBLE_ASCII_MAXIMUM = 0x7E
_MAX_SESSION_ID_LENGTH = 4_096


def parse_initialized_session(
    exchange: HttpExchange,
    request_id: int,
    *,
    expected_protocol_version: str,
) -> McpSession:
    """Validate one initialization exchange and return its session.

    Args:
        exchange: Decoded HTTP initialization exchange.
        request_id: JSON-RPC request identity sent by the client.
        expected_protocol_version: The only protocol version this client
            supports.

    Returns:
        A fully validated native Unreal MCP session.
    """
    session_id = require_visible_ascii_session_id(exchange.session_id)
    outcome = require_json_rpc_result(exchange, request_id)
    protocol_version = outcome.get("protocolVersion")
    if protocol_version != expected_protocol_version:
        fail_protocol("initialize negotiated an unsupported protocol version")
    capabilities = require_json_object(
        outcome.get("capabilities"),
        context="initialize result.capabilities",
    )
    _ = require_json_object(
        capabilities.get("tools"),
        context="initialize result.capabilities.tools",
    )
    server_info = require_json_object(
        outcome.get("serverInfo"),
        context="initialize result.serverInfo",
    )
    server_name = server_info.get("name")
    server_version = server_info.get("version")
    if not isinstance(server_name, str):
        fail_protocol("serverInfo.name must be text")
    if any(not character.isprintable() for character in server_name):
        fail_protocol("serverInfo.name must be printable text")
    if not isinstance(server_version, str):
        fail_protocol("serverInfo.version must be text")
    if any(not character.isprintable() for character in server_version):
        fail_protocol("serverInfo.version must be printable text")
    return McpSession(
        session_id=session_id,
        protocol_version=expected_protocol_version,
        server_name=server_name,
        server_version=server_version,
    )


def matches_integer_request_id(value: object, request_id: int) -> bool:
    """Return whether a response id exactly matches one integer request."""
    return (
        isinstance(value, int)
        and not isinstance(value, bool)
        and value == request_id
    )


def require_json_rpc_result(
    exchange: HttpExchange,
    request_id: int,
) -> JsonObject:
    """Return one validated JSON-RPC result object.

    Args:
        exchange: Decoded HTTP exchange.
        request_id: JSON-RPC request identity sent by the client.

    Returns:
        The strict JSON object stored in the response result member.
    """
    payload = exchange.payload
    if payload is None:
        fail_protocol(f"response id {request_id} had no JSON body")
    if payload.get("jsonrpc") != _JSON_RPC_VERSION:
        fail_protocol("response omitted JSON-RPC version 2.0")
    response_id = payload.get("id")
    if not matches_integer_request_id(response_id, request_id):
        fail_protocol(f"response id mismatch: expected {request_id}")
    has_result = "result" in payload
    has_error = "error" in payload
    if has_result == has_error:
        fail_protocol("response must contain exactly one of result or error")
    if has_error:
        error = require_json_object(
            payload["error"],
            context="JSON-RPC error",
        )
        code = error.get("code")
        if not isinstance(code, int) or isinstance(code, bool):
            fail_protocol("JSON-RPC error.code must be an integer")
        message = error.get("message")
        if (
            not isinstance(message, str)
            or not message
            or not message.isprintable()
        ):
            fail_protocol(
                "JSON-RPC error.message must be non-empty printable text"
            )
        fail_protocol(message)
    return require_json_object(
        payload["result"],
        context="JSON-RPC result",
    )


def parse_tool_names(result: JsonObject) -> tuple[str, ...]:
    """Return validated names from one `tools/list` result.

    Args:
        result: Strict JSON-RPC result object.

    Returns:
        Tool names in server-provided order.
    """
    raw_tools = result.get("tools")
    if not isinstance(raw_tools, list):
        fail_protocol("tools/list result omitted tools array")
    names: list[str] = []
    seen: set[str] = set()
    for index, raw_tool in enumerate(raw_tools):
        tool = require_json_object(
            raw_tool,
            context=f"tools/list.tools[{index}]",
        )
        name = tool.get("name")
        if (
            not isinstance(name, str)
            or not name
            or name != name.strip()
            or not name.isprintable()
            or any(character.isspace() for character in name)
        ):
            fail_protocol(f"tools/list.tools[{index}].name is invalid")
        if name in seen:
            fail_protocol(f"duplicate tool identity: {name}")
        seen.add(name)
        names.append(name)
    return tuple(names)


def require_visible_ascii_session_id(value: str | None) -> str:
    """Return one safe session header identity.

    Args:
        value: Candidate MCP session header value.

    Returns:
        A non-empty visible-ASCII session identifier.
    """
    if (
        value is None
        or not value
        or len(value) > _MAX_SESSION_ID_LENGTH
        or not all(
            _VISIBLE_ASCII_MINIMUM <= ord(character) <= _VISIBLE_ASCII_MAXIMUM
            for character in value
        )
    ):
        fail_protocol("initialize response omitted a valid MCP-Session-Id")
    return value
