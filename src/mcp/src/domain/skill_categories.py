# File:
#   - skill_categories.py
# Path:
#   - src/mcp/src/domain/skill_categories.py
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
#   - Ordered category identities and purposes for Unreal MCP skills.
# - Must-Not:
#   - Assign toolsets, render Markdown, access files, or invoke tools.
# - Allows:
#   - Defining the stable top-level navigation order.
# - Split-When:
#   - Category metadata gains behavior beyond immutable values.
# - Merge-When:
#   - Another domain module owns the same ordered categories.
# - Summary:
#   - Defines the Unreal MCP skill category order.
# - Description:
#   - Keeps navigation identity separate from toolset assignment.
# - Usage:
#   - Imported by taxonomy and generated skill renderers.
# - Defaults:
#   - Six categories ordered by stable semantic taxonomy.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: Unreal MCP skill category definitions
#   - reason: category identity and purpose form one navigation contract
#   - split: extract category prose if localization is introduced
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess when categories are added, removed, or reordered
#
"""Ordered categories for native Unreal MCP skills."""

from __future__ import annotations

from typing import NamedTuple


class SkillCategory(NamedTuple):
    """One ordered capability category."""

    slug: str
    title: str
    purpose: str


CATEGORIES = (
    SkillCategory(
        "core-and-governance",
        "Core and governance",
        "Editor health, configuration, plugins, logs, tests, and search.",
    ),
    SkillCategory(
        "assets-and-data",
        "Assets and data",
        "Assets, Blueprints, tables, materials, textures, and meshes.",
    ),
    SkillCategory(
        "world-and-ui",
        "World and UI",
        "Actors, levels, Slate inspection, and UMG authoring.",
    ),
    SkillCategory(
        "animation-and-cinematics",
        "Animation and cinematics",
        "Sequencer, keyframing, bindings, Control Rig, and animation exchange.",
    ),
    SkillCategory(
        "gameplay-and-ai",
        "Gameplay and AI",
        "Game Features, tags, abilities, AI graphs, and world conditions.",
    ),
    SkillCategory(
        "effects-physics-and-procedural",
        "Effects, physics, and procedural",
        "Niagara, PCG, Dataflow, and Physics Asset workflows.",
    ),
)
