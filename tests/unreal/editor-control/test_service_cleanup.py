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
#   - Test service cleanup test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test service cleanup test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test service cleanup test module."""

from __future__ import annotations

from typing import TYPE_CHECKING
from typing import override

from mcp.application.service import UnrealMcpTranslator
from mcp.domain.errors import ProtocolError
from mcp.domain.errors import ToolCallError
from mcp.domain.session import McpSession
from mcp.port_outbound.transport import McpTransport
import pytest

if TYPE_CHECKING:
    from mcp.domain.json_types import JsonObject
    from mcp.domain.tool_outcome import ToolCallOutcome

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


class _RetryCleanupTransport(_CleanupTransport):
    """Synthetic transport whose first close attempt fails."""

    def __init__(self) -> None:
        super().__init__(ProtocolError("close failed"))
        self.initialize_calls = 0
        self.close_attempts = 0

    @override
    def initialize(self) -> McpSession:
        self.initialize_calls += 1
        return _SESSION

    @override
    def close(self, session: McpSession) -> None:
        assert session == _SESSION
        self.close_attempts += 1
        if self.close_attempts == 1:
            raise self._close_error
        self.closed = True


def test_raw_call_rejects_native_mutation_meta_tool_programmatically() -> None:
    transport = _CleanupTransport(ProtocolError("close failed"))
    translator = UnrealMcpTranslator(transport)

    with pytest.raises(ProtocolError, match="mutation meta-tool"):
        _ = translator.raw_call(
            "call_tool",
            {"tool_name": "create_asset", "arguments": {"name": "Asset"}},
        )


def test_failed_close_retains_session_for_retry() -> None:
    transport = _RetryCleanupTransport()
    translator = UnrealMcpTranslator(transport)
    assert translator.connect() == _SESSION

    with pytest.raises(ProtocolError, match="close failed"):
        translator.close()

    assert translator.connect() == _SESSION
    assert transport.initialize_calls == 1
    translator.close()
    assert transport.close_attempts == 2
    assert transport.closed


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
