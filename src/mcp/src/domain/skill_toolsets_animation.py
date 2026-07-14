# File:
#   - skill_toolsets_animation.py
# Path:
#   - src/mcp/src/domain/skill_toolsets_animation.py
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
#   - Native Unreal MCP toolset identities for 30 animation and cinematics.
# - Must-Not:
#   - Categorize other domains, render Markdown, access files, or invoke tools.
# - Allows:
#   - Keeping one category assignment independently reviewable.
# - Split-When:
#   - The category gains two unrelated capability families.
# - Merge-When:
#   - The taxonomy removes this category boundary.
# - Summary:
#   - Defines 30 animation and cinematics toolset identities.
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
"""30 animation and cinematics native Unreal MCP toolset identities."""

from __future__ import annotations

ANIMATION_TOOLSETS = (
    (
        "animation_toolset",
        "toolsets",
        "conditions",
        "SequencerConditionTools",
    ),
    (
        "animation_toolset",
        "toolsets",
        "controlrig",
        "ControlRigTools",
    ),
    (
        "animation_toolset",
        "toolsets",
        "controlrig_sequencer",
        "SequencerControlRigTools",
    ),
    (
        "animation_toolset",
        "toolsets",
        "custom_bindings",
        "SequencerCustomBindingTools",
    ),
    (
        "animation_toolset",
        "toolsets",
        "import_export",
        "SequencerImportExportTools",
    ),
    (
        "animation_toolset",
        "toolsets",
        "keyframing",
        "SequencerKeyframingTools",
    ),
    (
        "animation_toolset",
        "toolsets",
        "outliner",
        "SequencerOutlinerTools",
    ),
    (
        "animation_toolset",
        "toolsets",
        "sequencer",
        "SequencerTools",
    ),
)
