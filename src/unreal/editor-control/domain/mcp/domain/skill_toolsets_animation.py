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
#   - Skill toolsets animation domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill toolsets animation domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill toolsets animation domain module."""

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
