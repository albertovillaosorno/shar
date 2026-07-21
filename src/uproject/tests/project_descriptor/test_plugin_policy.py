# File:
#   - test_plugin_policy.py
# Path:
#   - src/uproject/tests/project_descriptor/test_plugin_policy.py
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
#   - Startup policy for optional and required Unreal project plugins.
# - Must-Not:
#   - Invoke live Unreal processes or depend on network services.
# - Allows:
#   - Deterministic descriptor parsing and exact plugin assertions.
# - Split-When:
#   - Another plugin family gains an independent policy boundary.
# - Merge-When:
#   - Another module proves the same project-descriptor contract.
# - Summary:
#   - Regression tests for editor safety and world plugin dependencies.
# - Description:
#   - Locks disabled integration plugins and required native world plugins.
# - Usage:
#   - Run by pytest before project configuration is committed.
# - Defaults:
#   - The tracked project descriptor is the fixture.
#
# ADRs:
# - docs/adr/unreal/runtime/runtime-parity-test-boundary.md
#
# Large file:
#   - false
#

"""Plugin startup policy for the tracked Unreal project descriptor."""

from __future__ import annotations

import json
from pathlib import Path
from typing import cast

PROJECT_ROOT = Path(__file__).resolve().parents[2]


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
