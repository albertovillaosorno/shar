# File:
#   - transport.py
# Path:
#   - src/mcp/src/ports/transport.py
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
#   - The inbound application contract for MCP transport behavior.
# - Must-Not:
#   - Contain HTTP, socket, retry, CLI, or Unreal implementations.
# - Allows:
#   - Typed operations required by translator use cases.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Defines the replaceable native MCP transport port.
# - Description:
#   - Keeps protocol mechanics outside the application core.
# - Usage:
#   - Implemented by driven adapters and consumed by the service.
# - Defaults:
#   - No transport implementation is selected implicitly.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Transport port for the native Unreal MCP server."""

from __future__ import annotations

from typing import TYPE_CHECKING, Protocol

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject
    from mcp.src.domain.session import McpSession
    from mcp.src.domain.tool_outcome import ToolCallOutcome


class McpTransport(Protocol):
    """Protocol operations required by the terminal translator."""

    def initialize(self) -> McpSession:
        """Create and initialize one MCP session."""
        ...

    def ping(self, session: McpSession) -> None:
        """Verify that one initialized session remains responsive."""
        ...

    def list_tools(self, session: McpSession) -> tuple[str, ...]:
        """Return native top-level MCP tool names."""
        ...

    def call_tool(
        self,
        session: McpSession,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        """Invoke one top-level MCP tool and normalize its result."""
        ...

    def close(self, session: McpSession) -> None:
        """Close one session without hiding cleanup failures."""
        ...
