# File:
#   - streamable_http.py
# Path:
#   - src/mcp/src/adapters/driven/streamable_http.py
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
#   - Native MCP lifecycle and tool operations over HTTP exchanges.
# - Must-Not:
#   - Accept remote endpoints, expose a server, or bypass sessions.
# - Allows:
#   - Serialized lifecycle, pagination, and native tool calls.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Implements the driven MCP transport port.
# - Description:
#   - Separates MCP operations from low-level HTTP framing.
# - Usage:
#   - Injected through the MCP transport port by the driving CLI.
# - Defaults:
#   - Uses bounded timeouts and protocol version 2025-11-25.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: serialized MCP operation transport
#   - reason: lifecycle, pagination, and tool calls share one active session
#   - split: split lifecycle from operations if reconnection support is added
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Streamable HTTP transport for the native Unreal MCP server."""

from __future__ import annotations

from threading import Lock
from typing import TYPE_CHECKING

from mcp.src.adapters.driven.http_exchange import HttpExchangeClient
from mcp.src.adapters.driven.http_payload import DEFAULT_MAX_RESPONSE_BYTES
from mcp.src.adapters.driven.http_request import DEFAULT_MAX_REQUEST_BYTES
from mcp.src.adapters.driven.json_rpc_request import (
    build_json_rpc_request,
)
from mcp.src.adapters.driven.package_version import package_version
from mcp.src.adapters.driven.request_cancellation import (
    cancel_timed_out_request,
)
from mcp.src.adapters.driven.response_validation import (
    parse_initialized_session,
    parse_tool_names,
    require_json_rpc_result,
)
from mcp.src.domain.errors import RequestTimeoutError, fail_protocol
from mcp.src.domain.tool_outcome import parse_tool_outcome

if TYPE_CHECKING:
    from mcp.src.domain.endpoint import McpEndpoint
    from mcp.src.domain.json_types import JsonObject
    from mcp.src.domain.session import McpSession
    from mcp.src.domain.tool_outcome import ToolCallOutcome

_PROTOCOL_VERSION = "2025-11-25"
_CLIENT_NAME = "shar-unreal-mcp-cli"
_CLIENT_VERSION = package_version()
_HTTP_ACCEPTED = 202


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
            while True:
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
                tools.extend(parse_tool_names(outcome))
                next_cursor = outcome.get("nextCursor")
                if next_cursor is None:
                    break
                if not isinstance(next_cursor, str) or not next_cursor:
                    fail_protocol(
                        "tools/list nextCursor must be non-empty text"
                    )
                if next_cursor in seen_cursors:
                    fail_protocol("tools/list returned a repeated cursor")
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
            status = self._exchange.delete(session)
            if status != _HTTP_ACCEPTED:
                fail_protocol("session delete did not return HTTP 202")

    def _take_request_id(self) -> int:
        request_id = self._next_request_id
        self._next_request_id += 1
        return request_id
