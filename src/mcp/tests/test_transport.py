# File:
#   - test_transport.py
# Path:
#   - src/mcp/tests/test_transport.py
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
#   - Black-box tests for native MCP HTTP lifecycle and SSE framing.
# - Must-Not:
#   - Require Unreal binaries, plugin code, or external networks.
# - Allows:
#   - Synthetic loopback protocol integration tests.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Guards transport lifecycle, pagination, and SSE outcomes.
# - Description:
#   - Exercises the real Python HTTP adapter end to end.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Each test uses an ephemeral loopback server.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
from __future__ import annotations

import pytest
from mcp.src.adapters.driven.streamable_http import (
    StreamableHttpTransport,
)
from mcp.src.domain.endpoint import McpEndpoint
from mcp.src.domain.errors import (
    ConfigurationError,
    ProtocolError,
    RequestTimeoutError,
    TransportError,
)

from tests.fake_unreal_server import FakeUnrealServer


def test_transport_rejects_non_finite_timeout_before_connection() -> None:
    """Socket timeouts must be finite positive deadlines."""
    for value in (float("nan"), float("inf"), float("-inf")):
        with pytest.raises(
            ConfigurationError,
            match="exchange timeout must be finite and positive",
        ):
            _ = StreamableHttpTransport(
                McpEndpoint.default(),
                timeout_seconds=value,
            )


def test_transport_rejects_redirected_rpc_response() -> None:
    """Redirects cannot masquerade as successful protocol responses."""
    with FakeUnrealServer.with_redirected_ping() as server:
        transport = StreamableHttpTransport(
            McpEndpoint.parse(server.endpoint),
            timeout_seconds=2.0,
        )
        session = transport.initialize()

        with pytest.raises(ProtocolError, match=r"HTTP 302"):
            transport.ping(session)

        transport.close(session)


def test_transport_completes_native_lifecycle_and_final_sse_event() -> None:
    with FakeUnrealServer() as server:
        transport = StreamableHttpTransport(
            McpEndpoint.parse(server.endpoint),
            timeout_seconds=2.0,
        )
        session = transport.initialize()
        transport.ping(session)
        assert transport.list_tools(session) == (
            "list_toolsets",
            "describe_toolset",
            "call_tool",
        )
        outcome = transport.call_tool(
            session,
            "call_tool",
            {"tool_name": "create_asset", "arguments": {"name": "A"}},
        )
        assert outcome.text == "native-ok:create_asset"
        transport.close(session)

        methods = tuple(request.get("method") for request in server.requests)
        assert methods == (
            "initialize",
            "notifications/initialized",
            "ping",
            "tools/list",
            "tools/list",
            "tools/call",
        )
        assert server.session_closed


def test_transport_deletes_session_when_initialize_result_is_invalid() -> None:
    """A valid session header is cleaned up when its body is malformed."""
    with FakeUnrealServer.with_malformed_initialize_result() as server:
        transport = StreamableHttpTransport(
            McpEndpoint.parse(server.endpoint),
            timeout_seconds=2.0,
        )

        with pytest.raises(
            ProtocolError,
            match=r"initialize result\.capabilities\.tools",
        ):
            _ = transport.initialize()

        assert server.session_closed


def test_transport_deletes_session_when_initialized_rejected() -> None:
    """A negotiated session is not leaked when initialization cannot finish."""
    with FakeUnrealServer.with_initialization_failure() as server:
        transport = StreamableHttpTransport(
            McpEndpoint.parse(server.endpoint),
            timeout_seconds=2.0,
        )

        with pytest.raises(
            ProtocolError,
            match="initialized notification did not return HTTP 202",
        ):
            _ = transport.initialize()

        assert server.session_closed


def test_transport_preserves_initialization_error_when_cleanup_fails() -> None:
    """Cleanup failure annotates rather than replaces the negotiation error."""
    with FakeUnrealServer.with_initialization_failure(
        reject_session_delete=True,
    ) as server:
        transport = StreamableHttpTransport(
            McpEndpoint.parse(server.endpoint),
            timeout_seconds=2.0,
        )

        with pytest.raises(
            ProtocolError,
            match="initialized notification did not return HTTP 202",
        ) as captured:
            _ = transport.initialize()

        notes = getattr(captured.value, "__notes__", [])
        assert notes == [
            "MCP session cleanup failed: session delete did not return HTTP 202"
        ]
        assert not server.session_closed


def test_transport_rejects_oversized_request_before_network_send() -> None:
    """Request-size failure occurs before the server receives a payload."""
    with FakeUnrealServer() as server:
        transport = StreamableHttpTransport(
            McpEndpoint.parse(server.endpoint),
            timeout_seconds=2.0,
            max_request_bytes=1,
        )

        with pytest.raises(TransportError, match="request exceeded"):
            _ = transport.initialize()

        assert not server.requests


def test_transport_rejects_repeated_pagination_cursor() -> None:
    with FakeUnrealServer(repeat_cursor=True) as server:
        transport = StreamableHttpTransport(
            McpEndpoint.parse(server.endpoint),
            timeout_seconds=2.0,
        )
        session = transport.initialize()
        with pytest.raises(ProtocolError, match="repeated cursor"):
            _ = transport.list_tools(session)
        transport.close(session)


def test_transport_cancels_timed_out_tool_request() -> None:
    """A timed-out native tool call is cancelled by its original request ID."""
    with FakeUnrealServer(
        delay_tool_calls=True,
        cancellation_delay_seconds=0.2,
    ) as server:
        transport = StreamableHttpTransport(
            McpEndpoint.parse(server.endpoint),
            timeout_seconds=0.1,
        )
        session = transport.initialize()

        with pytest.raises(RequestTimeoutError, match=r"request 2.*timed out"):
            _ = transport.call_tool(
                session,
                "call_tool",
                {"tool_name": "slow_tool", "arguments": {}},
            )

        assert server.cancelled_request_ids == (2,)
        methods = tuple(request.get("method") for request in server.requests)
        assert methods == (
            "initialize",
            "notifications/initialized",
            "tools/call",
            "notifications/cancelled",
        )
        transport.close(session)
