# File:
#   - skill_toolsets_effects.py
# Path:
#   - src/mcp/src/domain/skill_toolsets_effects.py
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
#   - Native Unreal MCP identities for effects, physics, and procedural work.
# - Must-Not:
#   - Categorize other domains, render Markdown, access files, or invoke tools.
# - Allows:
#   - Keeping one category assignment independently reviewable.
# - Split-When:
#   - The category gains two unrelated capability families.
# - Merge-When:
#   - The taxonomy removes this category boundary.
# - Summary:
#   - Defines 50 effects physics and procedural toolset identities.
# - Description:
#   - Supplies path segments to the canonical taxonomy aggregator.
# - Usage:
#   - Imported only by skill_taxonomy.py.
# - Defaults:
#   - Contains exact live Toolset Registry identities.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""50 effects physics and procedural native Unreal MCP toolset identities."""

from __future__ import annotations

EFFECTS_TOOLSETS = (
    (
        "DataflowAgent",
        "DataflowAgentToolset",
    ),
    (
        "NiagaraToolsets",
        "NiagaraToolset_Assets",
    ),
    (
        "NiagaraToolsets",
        "NiagaraToolset_Blueprint",
    ),
    (
        "NiagaraToolsets",
        "NiagaraToolset_Component",
    ),
    (
        "NiagaraToolsets",
        "NiagaraToolset_Info",
    ),
    (
        "NiagaraToolsets",
        "NiagaraToolset_System",
    ),
    (
        "PCGToolset",
        "PCGSpatialToolset",
    ),
    (
        "PCGToolset",
        "PCGToolset",
    ),
    (
        "PhysicsToolsets",
        "PhysicsAssetToolset",
    ),
)
