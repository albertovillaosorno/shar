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
#   - Skill catalog fixture test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill catalog fixture test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill catalog fixture test module."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.domain.catalog import ToolDefinition
from mcp.domain.catalog import ToolsetDefinition
from mcp.domain.skill_taxonomy import known_toolset_names

if TYPE_CHECKING:
    from mcp.domain.json_types import JsonObject

SENTINEL_NATIVE_PROSE = "PRIVATE_NATIVE_DESCRIPTION_MUST_NOT_BE_RENDERED"
EXPECTED_TOOLSET_COUNT = 52
EXPECTED_CATEGORY_COUNT = 6
EXPECTED_DOCUMENT_COUNT = 53
TEST_UNREAL_MCP_VERSION = "1.0.0"


def complete_catalog() -> tuple[ToolsetDefinition, ...]:
    """Return one deterministic toolset definition per taxonomy identity."""
    return tuple(toolset(name) for name in sorted(known_toolset_names()))


def toolset(name: str) -> ToolsetDefinition:
    """Return one synthetic toolset definition with stable interface fields."""
    capability_name = f"{name}.synthetic_capability"
    input_schema: JsonObject = {
        "properties": {
            "optionalValue": {"type": "integer"},
            "requiredValue": {"type": "string"},
        },
        "required": ["requiredValue"],
        "type": "object",
    }
    output_schema: JsonObject = {
        "properties": {},
        "type": "object",
    }
    capability = ToolDefinition(
        name=capability_name,
        description=SENTINEL_NATIVE_PROSE,
        input_schema=input_schema,
        output_schema=output_schema,
    )
    raw_capability: JsonObject = {
        "description": SENTINEL_NATIVE_PROSE,
        "inputSchema": input_schema,
        "name": capability_name,
        "outputSchema": output_schema,
    }
    raw_schema: JsonObject = {
        "description": SENTINEL_NATIVE_PROSE,
        "name": name,
        "tools": [raw_capability],
    }
    return ToolsetDefinition(
        name=name,
        description=SENTINEL_NATIVE_PROSE,
        tools=(capability,),
        raw_schema=raw_schema,
    )
