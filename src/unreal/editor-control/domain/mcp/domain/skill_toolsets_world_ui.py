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
#   - Skill toolsets world ui domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill toolsets world ui domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill toolsets world ui domain module."""

from __future__ import annotations

WORLD_UI_TOOLSETS = (
    (
        "SlateInspectorToolset",
        "SlateInspectorToolset",
    ),
    (
        "UMGToolSet",
        "UMGToolSet",
    ),
    (
        "editor_toolset",
        "toolsets",
        "actor",
        "ActorTools",
    ),
    (
        "editor_toolset",
        "toolsets",
        "scene",
        "SceneTools",
    ),
)
