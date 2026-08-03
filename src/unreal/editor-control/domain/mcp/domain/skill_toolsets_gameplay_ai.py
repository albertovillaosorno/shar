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
#   - Skill toolsets gameplay ai domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill toolsets gameplay ai domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill toolsets gameplay ai domain module."""

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
