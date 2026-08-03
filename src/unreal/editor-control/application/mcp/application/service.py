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
#   - Service application service.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Service application service.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Service application service."""

from __future__ import annotations

from typing import NamedTuple
from typing import Self
from typing import TYPE_CHECKING

from mcp.domain.catalog import ToolsetDefinition
from mcp.domain.catalog import parse_toolset_catalog
from mcp.domain.catalog import parse_toolset_definition
from mcp.domain.errors import UnrealMcpError
from mcp.domain.errors import fail_protocol
from mcp.domain.tool_identity import native_tool_leaf

if TYPE_CHECKING:
    from types import TracebackType

    from mcp.domain.catalog import ToolsetSummary
    from mcp.domain.json_types import JsonObject
    from mcp.domain.session import McpSession
    from mcp.domain.tool_outcome import ToolCallOutcome
    from mcp.port_outbound.transport import McpTransport

_REQUIRED_META_TOOLS = frozenset(
    {"call_tool", "describe_toolset", "list_toolsets"}
)


class DoctorReport(NamedTuple):
    """Native server readiness observed through one initialized session."""

    protocol_version: str
    server_name: str
    server_version: str
    top_level_tools: tuple[str, ...]
    missing_meta_tools: tuple[str, ...]
    toolset_count: int

    @property
    def ready(self) -> bool:
        """Whether the native tool-search surface is complete.

        Returns:
            `True` when all required native meta-tools are available.

        """
        return not self.missing_meta_tools and self.toolset_count > 0


class UnrealMcpTranslator:
    """Stateful application service for one native MCP session."""

    def __init__(self, transport: McpTransport) -> None:
        """Create a translator around one supplied transport port.

        Args:
            transport: Driven adapter used for all protocol operations.

        """
        self._transport = transport
        self._session: McpSession | None = None

    def __enter__(self) -> Self:
        """Initialize this translator for use as a context manager.

        Returns:
            This connected translator instance.

        """
        _ = self.connect()
        return self

    def __exit__(
        self,
        exception_type: type[BaseException] | None,
        exception: BaseException | None,
        traceback: TracebackType | None,
    ) -> None:
        """Close the active session when leaving a context manager.

        Args:
            exception_type: Exception class raised by the managed block.
            exception: Exception instance raised by the managed block.
            traceback: Traceback raised by the managed block.

        Raises:
            UnrealMcpError: The close failure when no primary failure exists.

        """
        del exception_type, traceback
        try:
            self.close()
        except UnrealMcpError as close_error:
            if exception is None:
                raise
            exception.add_note(f"MCP session cleanup failed: {close_error}")

    def connect(self) -> McpSession:
        """Initialize the session once and return its negotiated identity.

        Returns:
            The active initialized session.

        """
        if self._session is None:
            self._session = self._transport.initialize()
        return self._session

    def close(self) -> None:
        """Close the active session when one exists."""
        if self._session is None:
            return
        session = self._session
        self._transport.close(session)
        self._session = None

    def doctor(self) -> DoctorReport:
        """Verify protocol health and required meta-tool availability.

        Returns:
            A complete readiness report for the connected native server.

        """
        session = self._require_session()
        self._transport.ping(session)
        tools = tuple(sorted(self._transport.list_tools(session)))
        missing = tuple(sorted(_REQUIRED_META_TOOLS.difference(tools)))
        toolset_count = 0 if missing else len(self.list_toolsets())
        return DoctorReport(
            protocol_version=session.protocol_version,
            server_name=session.server_name,
            server_version=session.server_version,
            top_level_tools=tools,
            missing_meta_tools=missing,
            toolset_count=toolset_count,
        )

    def list_toolsets(self) -> tuple[ToolsetSummary, ...]:
        """Return every toolset discoverable through tool-search mode.

        Returns:
            Native toolset summaries in registry order.

        """
        outcome = self._transport.call_tool(
            self._require_session(),
            "list_toolsets",
            {},
        ).require_success()
        return parse_toolset_catalog(outcome.text)

    def describe_toolset(self, name: str) -> ToolsetDefinition:
        """Return the complete native schema for one toolset.

        Args:
            name: Native Toolset Registry identity.

        Returns:
            The complete validated toolset definition.

        """
        normalized_name = name.strip()
        if not normalized_name:
            fail_protocol("toolset name must not be empty")
        outcome = self._transport.call_tool(
            self._require_session(),
            "describe_toolset",
            {"toolset_name": normalized_name},
        ).require_success()
        return parse_toolset_definition(normalized_name, outcome.text)

    def discover_catalog(self) -> tuple[ToolsetDefinition, ...]:
        """Return a stable complete catalog sorted by toolset name.

        Returns:
            Every discoverable native toolset and its complete schema.

        """
        definitions: list[ToolsetDefinition] = []
        for summary in sorted(self.list_toolsets(), key=lambda item: item.name):
            definition = self.describe_toolset(summary.name)
            if not definition.description and summary.description:
                definition = ToolsetDefinition(
                    name=definition.name,
                    description=summary.description,
                    tools=definition.tools,
                    raw_schema=definition.raw_schema,
                )
            definitions.append(definition)
        return tuple(definitions)

    def call_tool(
        self,
        toolset_name: str,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        """Dispatch one native Toolset Registry tool through `call_tool`.

        Args:
            toolset_name: Native toolset identity, or empty for global lookup.
            tool_name: Native tool identity.
            arguments: Strict JSON arguments forwarded without lossy mapping.

        Returns:
            The successful normalized native tool outcome.

        """
        normalized_toolset = toolset_name.strip()
        native_name = native_tool_leaf(normalized_toolset, tool_name)
        params: JsonObject = {
            "tool_name": native_name,
            "arguments": arguments,
        }
        if normalized_toolset:
            params["toolset_name"] = normalized_toolset
        return self._transport.call_tool(
            self._require_session(),
            "call_tool",
            params,
        ).require_success()

    def raw_call(
        self,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        """Invoke one top-level MCP tool without Toolset Registry wrapping.

        Args:
            tool_name: Top-level MCP tool identity.
            arguments: Strict JSON arguments forwarded without lossy mapping.

        Returns:
            The successful normalized native tool outcome.

        """
        if not tool_name.strip():
            fail_protocol("top-level tool name must not be empty")
        return self._transport.call_tool(
            self._require_session(),
            tool_name,
            arguments,
        ).require_success()

    def _require_session(self) -> McpSession:
        if self._session is None:
            fail_protocol("translator session is not initialized")
        return self._session
