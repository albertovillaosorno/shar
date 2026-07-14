# File:
#   - test_service_cleanup.py
# Path:
#   - src/mcp/tests/test_service_cleanup.py
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
#   - Application-session cleanup failure precedence tests.
# - Must-Not:
#   - Open sockets, invoke Unreal, or depend on HTTP adapters.
# - Allows:
#   - Synthetic transport failures during context-manager exit.
# - Split-When:
#   - Session lifecycle gains independently testable recovery policies.
# - Merge-When:
#   - Another application test module owns cleanup precedence.
# - Summary:
#   - Preserves primary failures while retaining cleanup evidence.
# - Description:
#   - Verifies close failures propagate only when no earlier failure exists.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Uses one immutable synthetic MCP session.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
#
# Large file:
#   - false
#
"""Application cleanup failure precedence tests."""

from __future__ import annotations

from typing import TYPE_CHECKING, override

import pytest
from mcp.src.application.service import UnrealMcpTranslator
from mcp.src.domain.errors import ProtocolError, ToolCallError
from mcp.src.domain.session import McpSession
from mcp.src.ports.transport import McpTransport

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject
    from mcp.src.domain.tool_outcome import ToolCallOutcome

_SESSION = McpSession(
    session_id="0123456789abcdef0123456789abcdef",
    protocol_version="2025-11-25",
    server_name="",
    server_version="",
)


class _CleanupTransport(McpTransport):
    """Synthetic transport that can fail only during session close."""

    def __init__(self, close_error: ProtocolError) -> None:
        self._close_error = close_error
        self.closed = False

    @override
    def initialize(self) -> McpSession:
        return _SESSION

    @override
    def ping(self, session: McpSession) -> None:
        assert session == _SESSION

    @override
    def list_tools(self, session: McpSession) -> tuple[str, ...]:
        assert session == _SESSION
        return ()

    @override
    def call_tool(
        self,
        session: McpSession,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        del tool_name, arguments
        assert session == _SESSION
        raise AssertionError

    @override
    def close(self, session: McpSession) -> None:
        assert session == _SESSION
        self.closed = True
        raise self._close_error


def test_close_failure_propagates_without_primary_failure() -> None:
    transport = _CleanupTransport(ProtocolError("close failed"))

    with (
        pytest.raises(ProtocolError, match="close failed"),
        UnrealMcpTranslator(transport),
    ):
        pass

    assert transport.closed


def test_close_failure_does_not_replace_primary_failure() -> None:
    transport = _CleanupTransport(ProtocolError("close failed"))

    primary_error = ToolCallError("tool failed")
    with (
        pytest.raises(ToolCallError, match="tool failed") as captured,
        UnrealMcpTranslator(transport),
    ):
        raise primary_error

    assert transport.closed
    assert captured.value.__notes__ == [
        "MCP session cleanup failed: close failed"
    ]
