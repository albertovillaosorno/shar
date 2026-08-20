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
#   - Streamable http outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Streamable http outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Streamable http outbound adapter."""

from __future__ import annotations

from threading import Lock
from typing import TYPE_CHECKING

from mcp.adapter_outbound.http_exchange import HttpExchangeClient
from mcp.adapter_outbound.http_payload import DEFAULT_MAX_RESPONSE_BYTES
from mcp.adapter_outbound.http_request import DEFAULT_MAX_REQUEST_BYTES
from mcp.adapter_outbound.json_rpc_request import build_json_rpc_request
from mcp.adapter_outbound.package_version import package_version
from mcp.adapter_outbound.request_cancellation import cancel_timed_out_request
from mcp.adapter_outbound.response_validation import parse_initialized_session
from mcp.adapter_outbound.response_validation import parse_tool_names
from mcp.adapter_outbound.response_validation import require_json_rpc_result
from mcp.adapter_outbound.response_validation import (
    require_visible_ascii_session_id,
)
from mcp.domain.errors import RequestTimeoutError
from mcp.domain.errors import UnrealMcpError
from mcp.domain.errors import fail_protocol
from mcp.domain.tool_outcome import parse_tool_outcome

if TYPE_CHECKING:
    from mcp.domain.endpoint import McpEndpoint
    from mcp.domain.json_types import JsonObject
    from mcp.domain.session import McpSession
    from mcp.domain.tool_outcome import ToolCallOutcome

_PROTOCOL_VERSION = "2025-11-25"
_CLIENT_NAME = "shar-unreal-mcp-cli"
_CLIENT_VERSION = package_version()
_HTTP_ACCEPTED = 202
_MAX_TOOL_LIST_PAGES = 256
_MAX_TOOL_NAMES = 100_000
_MAX_TOOL_NAME_BYTES = DEFAULT_MAX_RESPONSE_BYTES
_MAX_SINGLE_CURSOR_BYTES = 4_096
_MAX_PAGINATION_CURSOR_BYTES = DEFAULT_MAX_RESPONSE_BYTES


def _extend_tool_names(
    tools: list[str],
    page_tools: tuple[str, ...],
    tool_name_bytes: int,
) -> int:
    """Append one page within aggregate tool count and byte budgets.

    Returns:
        Updated aggregate UTF-8 tool-name byte count.

    """
    if len(tools) + len(page_tools) > _MAX_TOOL_NAMES:
        fail_protocol("tools/list exceeded its tool limit")
    page_tool_name_bytes = sum(len(name.encode()) for name in page_tools)
    updated_tool_name_bytes = tool_name_bytes + page_tool_name_bytes
    if updated_tool_name_bytes > _MAX_TOOL_NAME_BYTES:
        fail_protocol("tools/list exceeded its tool name byte limit")
    tools.extend(page_tools)
    return updated_tool_name_bytes


class StreamableHttpTransport:
    """Serialized MCP client transport over loopback HTTP."""

    def __init__(
        self,
        endpoint: McpEndpoint,
        *,
        timeout_seconds: float = 30.0,
        max_request_bytes: int = DEFAULT_MAX_REQUEST_BYTES,
        max_response_bytes: int = DEFAULT_MAX_RESPONSE_BYTES,
    ) -> None:
        """Create one serialized native MCP transport.

        Args:
            endpoint: Validated loopback native MCP endpoint.
            timeout_seconds: Positive timeout for each HTTP exchange.
            max_request_bytes: Positive per-request byte ceiling.
            max_response_bytes: Positive per-response byte ceiling.

        """
        self._exchange = HttpExchangeClient(
            endpoint,
            timeout_seconds=timeout_seconds,
            max_request_bytes=max_request_bytes,
            max_response_bytes=max_response_bytes,
        )
        self._lock = Lock()
        self._next_request_id = 1

    def initialize(self) -> McpSession:
        """Create and initialize one negotiated MCP session.

        Returns:
            The initialized session identity and negotiated server metadata.

        """
        with self._lock:
            request_id = self._take_request_id()
            exchange = self._exchange.post(
                payload=build_json_rpc_request(
                    method="initialize",
                    params={
                        "protocolVersion": _PROTOCOL_VERSION,
                        "capabilities": {},
                        "clientInfo": {
                            "name": _CLIENT_NAME,
                            "version": _CLIENT_VERSION,
                        },
                    },
                    request_id=request_id,
                ),
                request_id=request_id,
                session=None,
            )
            session_id = require_visible_ascii_session_id(exchange.session_id)
            try:
                session = parse_initialized_session(
                    exchange,
                    request_id,
                    expected_protocol_version=_PROTOCOL_VERSION,
                )
                notification = self._exchange.post(
                    payload=build_json_rpc_request(
                        method="notifications/initialized",
                        params={},
                        request_id=None,
                    ),
                    request_id=None,
                    session=session,
                )
                if notification.status != _HTTP_ACCEPTED:
                    fail_protocol(
                        "initialized notification did not return HTTP 202"
                    )
            except UnrealMcpError as initialization_error:
                try:
                    self._delete_session_identity(
                        session_id,
                        _PROTOCOL_VERSION,
                    )
                except UnrealMcpError as cleanup_error:
                    initialization_error.add_note(
                        f"MCP session cleanup failed: {cleanup_error}"
                    )
                raise
            return session

    def ping(self, session: McpSession) -> None:
        """Verify that one initialized session remains responsive.

        Args:
            session: Active initialized native MCP session.

        """
        with self._lock:
            request_id = self._take_request_id()
            exchange = self._exchange.post(
                payload=build_json_rpc_request(
                    method="ping",
                    params={},
                    request_id=request_id,
                ),
                request_id=request_id,
                session=session,
            )
            _ = require_json_rpc_result(exchange, request_id)

    def list_tools(self, session: McpSession) -> tuple[str, ...]:
        """Return every top-level MCP tool name with pagination.

        Args:
            session: Active initialized native MCP session.

        Returns:
            Every unique top-level tool name in server order.

        """
        with self._lock:
            tools: list[str] = []
            cursor: str | None = None
            seen_cursors: set[str] = set()
            cursor_bytes = 0
            tool_name_bytes = 0
            page_count = 0
            while True:
                if page_count >= _MAX_TOOL_LIST_PAGES:
                    fail_protocol("tools/list exceeded its page limit")
                page_count += 1
                params: JsonObject = {}
                if cursor is not None:
                    params["cursor"] = cursor
                request_id = self._take_request_id()
                exchange = self._exchange.post(
                    payload=build_json_rpc_request(
                        method="tools/list",
                        params=params,
                        request_id=request_id,
                    ),
                    request_id=request_id,
                    session=session,
                )
                outcome = require_json_rpc_result(exchange, request_id)
                page_tools = parse_tool_names(outcome)
                tool_name_bytes = _extend_tool_names(
                    tools,
                    page_tools,
                    tool_name_bytes,
                )
                next_cursor = outcome.get("nextCursor")
                if next_cursor is None:
                    break
                if not isinstance(next_cursor, str) or not next_cursor:
                    fail_protocol(
                        "tools/list nextCursor must be non-empty text"
                    )
                if next_cursor in seen_cursors:
                    fail_protocol("tools/list returned a repeated cursor")
                next_cursor_bytes = len(next_cursor.encode())
                if next_cursor_bytes > _MAX_SINGLE_CURSOR_BYTES:
                    fail_protocol(
                        "tools/list exceeded its single cursor byte limit"
                    )
                if (
                    cursor_bytes + next_cursor_bytes
                    > _MAX_PAGINATION_CURSOR_BYTES
                ):
                    fail_protocol("tools/list exceeded its cursor byte limit")
                cursor_bytes += next_cursor_bytes
                seen_cursors.add(next_cursor)
                cursor = next_cursor
            if len(set(tools)) != len(tools):
                fail_protocol("tools/list returned duplicate tool names")
            return tuple(tools)

    def call_tool(
        self,
        session: McpSession,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        """Invoke one top-level MCP tool through a serialized call.

        Args:
            session: Active initialized native MCP session.
            tool_name: Top-level MCP tool identity.
            arguments: Strict JSON arguments.

        Returns:
            The normalized native tool outcome.

        Raises:
            RequestTimeoutError: If the serialized native call times out.

        """
        with self._lock:
            request_id = self._take_request_id()
            try:
                exchange = self._exchange.post(
                    payload=build_json_rpc_request(
                        method="tools/call",
                        params={"name": tool_name, "arguments": arguments},
                        request_id=request_id,
                    ),
                    request_id=request_id,
                    session=session,
                )
            except RequestTimeoutError as error:
                cancel_timed_out_request(
                    self._exchange,
                    session,
                    request_id,
                    error,
                )
                raise
            outcome = require_json_rpc_result(exchange, request_id)
            return parse_tool_outcome(outcome)

    def close(self, session: McpSession) -> None:
        """Delete one native MCP session.

        Args:
            session: Active initialized native MCP session.

        """
        with self._lock:
            self._delete_session(session)

    def _delete_session(self, session: McpSession) -> None:
        """Delete one full session while the caller owns the transport lock."""
        self._delete_session_identity(
            session.session_id,
            session.protocol_version,
        )

    def _delete_session_identity(
        self,
        session_id: str,
        protocol_version: str,
    ) -> None:
        """Delete one validated session header identity."""
        status = self._exchange.delete(
            session_id=session_id,
            protocol_version=protocol_version,
        )
        if status != _HTTP_ACCEPTED:
            fail_protocol("session delete did not return HTTP 202")

    def _take_request_id(self) -> int:
        request_id = self._next_request_id
        self._next_request_id += 1
        return request_id
