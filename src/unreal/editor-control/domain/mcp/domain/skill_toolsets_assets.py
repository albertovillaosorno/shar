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
#   - Skill toolsets assets domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill toolsets assets domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill toolsets assets domain module."""

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
