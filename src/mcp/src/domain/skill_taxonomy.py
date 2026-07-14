# File:
#   - skill_taxonomy.py
# Path:
#   - src/mcp/src/domain/skill_taxonomy.py
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
#   - Canonical routing from native Unreal toolset identity to skill category.
# - Must-Not:
#   - Own category-specific identity lists, render Markdown, or access files.
# - Allows:
#   - Joining path segments and failing closed on missing taxonomy ownership.
# - Split-When:
#   - Routing and slug generation evolve independently.
# - Merge-When:
#   - Another domain module owns the same taxonomy aggregation contract.
# - Summary:
#   - Aggregates ordered Unreal MCP skill taxonomy assignments.
# - Description:
#   - Keeps category-specific identities in separate SRP modules.
# - Usage:
#   - Used by generated skill renderers and taxonomy coverage tests.
# - Defaults:
#   - Unknown toolsets are rejected instead of becoming unclassified.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: native Unreal MCP skill taxonomy routing
#   - reason: category aggregation and slug generation form one domain contract
#   - split: extract slug generation if another consumer requires it
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess after every Unreal Engine or toolset plugin upgrade
#
"""Canonical routing for native Unreal MCP skill taxonomy."""

from __future__ import annotations

import re

from mcp.src.domain.errors import fail_protocol
from mcp.src.domain.skill_categories import CATEGORIES, SkillCategory
from mcp.src.domain.skill_toolsets_animation import ANIMATION_TOOLSETS
from mcp.src.domain.skill_toolsets_assets import ASSETS_TOOLSETS
from mcp.src.domain.skill_toolsets_core import CORE_TOOLSETS
from mcp.src.domain.skill_toolsets_effects import EFFECTS_TOOLSETS
from mcp.src.domain.skill_toolsets_gameplay_ai import GAMEPLAY_AI_TOOLSETS
from mcp.src.domain.skill_toolsets_world_ui import WORLD_UI_TOOLSETS

_CATEGORY_GROUPS = (
    ("core-and-governance", CORE_TOOLSETS),
    ("assets-and-data", ASSETS_TOOLSETS),
    ("world-and-ui", WORLD_UI_TOOLSETS),
    ("animation-and-cinematics", ANIMATION_TOOLSETS),
    ("gameplay-and-ai", GAMEPLAY_AI_TOOLSETS),
    ("effects-physics-and-procedural", EFFECTS_TOOLSETS),
)
_TOOLSET_CATEGORY = {
    ".".join(parts): category_slug
    for category_slug, names in _CATEGORY_GROUPS
    for parts in names
}
_CATEGORY_BY_SLUG = {category.slug: category for category in CATEGORIES}
_SLUG_PARTS = re.compile(r"[^a-z0-9]+")


def category_for_toolset(toolset_name: str) -> SkillCategory:
    """Return the single category that owns a native toolset."""
    category_slug = _TOOLSET_CATEGORY.get(toolset_name)
    if category_slug is None:
        fail_protocol(f"toolset lacks skill taxonomy ownership: {toolset_name}")
    category = _CATEGORY_BY_SLUG.get(category_slug)
    if category is None:
        fail_protocol(f"unknown skill category: {category_slug}")
    return category


def toolset_slug(toolset_name: str) -> str:
    """Return a deterministic filesystem-safe toolset slug."""
    slug = _SLUG_PARTS.sub("-", toolset_name.casefold()).strip("-")
    if not slug:
        fail_protocol("toolset name cannot produce an empty skill slug")
    return slug


def known_toolset_names() -> frozenset[str]:
    """Return every toolset identity owned by the taxonomy."""
    return frozenset(_TOOLSET_CATEGORY)
