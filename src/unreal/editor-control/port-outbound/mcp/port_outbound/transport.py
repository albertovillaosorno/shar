# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - Transport outbound port.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Transport outbound port.
# - Description:
#   - Implements the declared outbound port responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Transport outbound port."""

from __future__ import annotations

from typing import Protocol
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from mcp.domain.json_types import JsonObject
    from mcp.domain.session import McpSession
    from mcp.domain.tool_outcome import ToolCallOutcome


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
