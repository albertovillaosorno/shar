# File:
#   - skill_toolsets_gameplay_ai.py
# Path:
#   - src/mcp/src/domain/skill_toolsets_gameplay_ai.py
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
#   - Native Unreal MCP toolset identities for 40 gameplay and ai.
# - Must-Not:
#   - Categorize other domains, render Markdown, access files, or invoke tools.
# - Allows:
#   - Keeping one category assignment independently reviewable.
# - Split-When:
#   - The category gains two unrelated capability families.
# - Merge-When:
#   - The taxonomy removes this category boundary.
# - Summary:
#   - Defines 40 gameplay and ai toolset identities.
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
"""40 gameplay and ai native Unreal MCP toolset identities."""

from __future__ import annotations

GAMEPLAY_AI_TOOLSETS = (
    (
        "DataRegistryToolset",
        "DataRegistryTools",
    ),
    (
        "GASToolsets",
        "AbilitySystemInspectorToolset",
    ),
    (
        "GASToolsets",
        "AttributeSetToolset",
    ),
    (
        "GASToolsets",
        "GameplayCueToolset",
    ),
    (
        "GameFeaturesToolset",
        "GameFeaturesToolset",
    ),
    (
        "GameplayTagsToolset",
        "GameplayTagsToolset",
    ),
    (
        "WorldConditionsToolset",
        "WorldConditionTools",
    ),
    (
        "aimodule_toolset",
        "toolsets",
        "behavior_tree",
        "BehaviorTreeTools",
    ),
    (
        "conversation_toolset",
        "toolsets",
        "conversation",
        "ConversationTools",
    ),
    (
        "state_tree_toolset",
        "toolsets",
        "state_tree",
        "StateTreeTools",
    ),
)
