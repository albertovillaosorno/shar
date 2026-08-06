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
#   - Fake unreal tools test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Fake unreal tools test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Fake unreal tools test module."""

from __future__ import annotations

import json
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from mcp.domain.json_types import JsonObject


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
                    "properties": {
                        "name": {
                            "type": "string",
                            "minLength": 1,
                            "pattern": "^[A-Za-z][A-Za-z0-9_]*$",
                        }
                    },
                    "required": ["name"],
                    "additionalProperties": False,
                },
                "outputSchema": {"type": "object"},
            }
        ],
    }
