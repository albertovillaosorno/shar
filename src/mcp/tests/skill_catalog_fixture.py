# File:
#   - skill_catalog_fixture.py
# Path:
#   - src/mcp/tests/skill_catalog_fixture.py
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
#   - Synthetic complete native Unreal MCP catalogs for generated-skill tests.
# - Must-Not:
#   - Render documents, access files, connect to Unreal, or assert behavior.
# - Allows:
#   - Reusing one deterministic capability schema across focused test modules.
# - Split-When:
#   - Tests require independently varied tool and toolset schemas.
# - Merge-When:
#   - A single test module becomes the sole consumer of this fixture.
# - Summary:
#   - Builds deterministic complete skill-generation test catalogs.
# - Description:
#   - Uses one synthetic capability for each of the 52 known toolsets.
# - Usage:
#   - Imported by rendering, storage, and export tests.
# - Defaults:
#   - Embeds sentinel prose that generated output must omit.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: generated skill synthetic catalog fixture
#   - reason: complete toolset and schema construction form one test fixture
#   - split: extract schema variants if more than one capability shape is needed
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess after taxonomy or generated schema changes
#
"""Synthetic complete catalogs for Unreal MCP skill generation tests."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.src.domain.catalog import ToolDefinition, ToolsetDefinition
from mcp.src.domain.skill_taxonomy import known_toolset_names

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject

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
