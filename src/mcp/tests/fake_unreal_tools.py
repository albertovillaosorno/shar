# File:
#   - fake_unreal_tools.py
# Path:
#   - src/mcp/tests/fake_unreal_tools.py
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
#   - Synthetic Toolset Registry text and schema payloads.
# - Must-Not:
#   - Start HTTP servers, manage sessions, or implement request routing.
# - Allows:
#   - Deterministic public-safe responses for fake native tools.
# - Split-When:
#   - Multiple independent synthetic toolset families are required.
# - Merge-When:
#   - The protocol handler becomes the only consumer and remains bounded.
# - Summary:
#   - Supplies deterministic fake Unreal tool responses.
# - Description:
#   - Keeps tool payload fixtures separate from HTTP protocol mechanics.
# - Usage:
#   - Called by the synthetic Unreal MCP request handler.
# - Defaults:
#   - Exposes one editor toolset and one synthetic asset tool.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Synthetic native Unreal MCP tool payloads."""

from __future__ import annotations

import json
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject


def tool_text(
    tool_name: object,
    arguments: JsonObject,
    *,
    empty_toolsets: bool,
) -> str:
    """Return one deterministic native tool response text."""
    if tool_name == "list_toolsets":
        if empty_toolsets:
            return ""
        return (
            "- EditorToolset.EditorToolset: Editor operations\n\n"
            "Provides:\n"
            "- create_asset: synthetic asset creation\n"
        )
    if tool_name == "describe_toolset":
        return json.dumps(_editor_schema(), separators=(",", ":"))
    if tool_name == "call_tool":
        native_name = arguments.get("tool_name", "unknown")
        return f"native-ok:{native_name}"
    return "raw-ok"


def _editor_schema() -> JsonObject:
    return {
        "description": "Editor operations",
        "tools": [
            {
                "name": "create_asset",
                "description": "Create one synthetic asset.",
                "inputSchema": {
                    "type": "object",
                    "properties": {"name": {"type": "string"}},
                },
                "outputSchema": {"type": "object"},
            }
        ],
    }
