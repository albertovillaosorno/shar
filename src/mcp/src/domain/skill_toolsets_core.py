# File:
#   - skill_toolsets_core.py
# Path:
#   - src/mcp/src/domain/skill_toolsets_core.py
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
#   - Native Unreal MCP toolset identities for 00 core and governance.
# - Must-Not:
#   - Categorize other domains, render Markdown, access files, or invoke tools.
# - Allows:
#   - Keeping one category assignment independently reviewable.
# - Split-When:
#   - The category gains two unrelated capability families.
# - Merge-When:
#   - The taxonomy removes this category boundary.
# - Summary:
#   - Defines 00 core and governance toolset identities.
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
"""00 core and governance native Unreal MCP toolset identities."""

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
