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
#   - Skill toolsets core domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill toolsets core domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill toolsets core domain module."""

from __future__ import annotations

CORE_TOOLSETS = (
    (
        "AutomationTestToolset",
        "AutomationTestToolset",
    ),
    (
        "ConfigSettingsToolset",
        "ConfigSettingsToolset",
    ),
    (
        "EditorToolset",
        "EditorAppToolset",
    ),
    (
        "EditorToolset",
        "LogsToolset",
    ),
    (
        "PluginToolset",
        "PluginToolset",
    ),
    (
        "SemanticSearchToolset",
        "SemanticSearchToolset",
    ),
    (
        "ToolsetRegistry",
        "AgentSkillToolset",
    ),
    (
        "editor_toolset",
        "toolsets",
        "programmatic",
        "ProgrammaticToolset",
    ),
)
