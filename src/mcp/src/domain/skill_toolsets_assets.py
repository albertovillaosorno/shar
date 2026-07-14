# File:
#   - skill_toolsets_assets.py
# Path:
#   - src/mcp/src/domain/skill_toolsets_assets.py
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
#   - Native Unreal MCP toolset identities for 10 assets and data.
# - Must-Not:
#   - Categorize other domains, render Markdown, access files, or invoke tools.
# - Allows:
#   - Keeping one category assignment independently reviewable.
# - Split-When:
#   - The category gains two unrelated capability families.
# - Merge-When:
#   - The taxonomy removes this category boundary.
# - Summary:
#   - Defines 10 assets and data toolset identities.
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
#   - true
# LARGE-FILE:
#   - owner: asset and data toolset identity assignments
#   - reason: one category requires thirteen exact registry identities
#   - split: divide asset types only if category ownership changes
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess after asset toolset plugin upgrades
#
"""10 assets and data native Unreal MCP toolset identities."""

from __future__ import annotations

ASSETS_TOOLSETS = (
    (
        "editor_toolset",
        "toolsets",
        "asset",
        "AssetTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "blueprint",
        "BlueprintTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "curve_table",
        "CurveTableTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "data_asset",
        "DataAssetTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "data_table",
        "DataTableTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "material",
        "MaterialTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "material_instance",
        "MaterialInstanceTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "object",
        "ObjectTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "primitive",
        "PrimitiveTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "skeletal_mesh",
        "SkeletalMeshTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "static_mesh",
        "StaticMeshTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "string_table",
        "StringTableTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "texture",
        "TextureTools",
    ),
)
