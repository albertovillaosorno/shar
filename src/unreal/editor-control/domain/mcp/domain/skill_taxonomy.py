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
#   - Skill taxonomy domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill taxonomy domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill taxonomy domain module."""

from __future__ import annotations

import re

from mcp.domain.errors import fail_protocol
from mcp.domain.skill_categories import CATEGORIES
from mcp.domain.skill_categories import SkillCategory
from mcp.domain.skill_toolsets_animation import ANIMATION_TOOLSETS
from mcp.domain.skill_toolsets_assets import ASSETS_TOOLSETS
from mcp.domain.skill_toolsets_core import CORE_TOOLSETS
from mcp.domain.skill_toolsets_effects import EFFECTS_TOOLSETS
from mcp.domain.skill_toolsets_gameplay_ai import GAMEPLAY_AI_TOOLSETS
from mcp.domain.skill_toolsets_world_ui import WORLD_UI_TOOLSETS

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
