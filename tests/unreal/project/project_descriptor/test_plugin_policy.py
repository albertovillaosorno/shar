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
#   - Test plugin policy test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test plugin policy test module.
# - Description:
#   - Implements the declared test module responsibility for project.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test plugin policy test module."""

from __future__ import annotations

import json
from pathlib import Path
from typing import cast

PROJECT_ROOT = (
    Path(__file__).resolve().parents[4]
    / "src/unreal/project/composition/uproject"
)


def _project_plugins() -> list[dict[str, object]]:
    """Read the typed plugin entries from the tracked project descriptor.

    Returns:
        The plugin descriptor entries in declaration order.

    """
    project = cast(
        "dict[str, object]",
        json.loads(
            (PROJECT_ROOT / "shar.uproject").read_text(encoding="utf-8")
        ),
    )
    return cast("list[dict[str, object]]", project["Plugins"])


def _matching_plugins(name: str) -> list[dict[str, object]]:
    """Return all descriptor entries matching one exact plugin name.

    Returns:
        Matching plugin entries in declaration order.

    """
    return [
        plugin for plugin in _project_plugins() if plugin.get("Name") == name
    ]


def test_visual_studio_tools_plugin_is_explicitly_disabled() -> None:
    assert _matching_plugins("VisualStudioTools") == [
        {
            "Name": "VisualStudioTools",
            "Enabled": False,
            "SupportedTargetPlatforms": ["Win64"],
        }
    ]


def test_python_script_plugin_is_explicitly_disabled() -> None:
    """Keep editor automation on native MCP tools instead of embedded Python."""
    assert _matching_plugins("PythonScriptPlugin") == [
        {
            "Name": "PythonScriptPlugin",
            "Enabled": False,
        }
    ]


def test_native_world_plugins_are_explicitly_enabled() -> None:
    """Keep the published base map's Water and Landmass dependencies stable."""
    assert _matching_plugins("Water") == [
        {
            "Name": "Water",
            "Enabled": True,
        }
    ]
    assert _matching_plugins("Landmass") == [
        {
            "Name": "Landmass",
            "Enabled": True,
        }
    ]
