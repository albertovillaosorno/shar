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
#   - Skill categories domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill categories domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill categories domain module."""

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
