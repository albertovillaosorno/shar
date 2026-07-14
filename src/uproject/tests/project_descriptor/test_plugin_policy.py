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
#   - The policy that keeps VisualStudioTools optional at startup.
# - Must-Not:
#   - Invoke live Unreal processes or depend on network services.
# - Allows:
#   - Deterministic local fixtures and observable assertions.
# - Split-When:
#   - Another behavior belongs to a different tooling boundary.
# - Merge-When:
#   - Another test module proves the same behavior contract.
# - Summary:
#   - Regression test for the disabled Visual Studio plugin.
# - Description:
#   - Locks one disabled Win64 plugin descriptor entry.
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


"""Optional-plugin startup policy for the tracked Unreal descriptor."""

from __future__ import annotations

import json
from pathlib import Path
from typing import cast

PROJECT_ROOT = Path(__file__).resolve().parents[2]


def test_visual_studio_tools_plugin_is_explicitly_disabled() -> None:
    project = cast(
        "dict[str, object]",
        json.loads(
            (PROJECT_ROOT / "shar.uproject").read_text(encoding="utf-8")
        ),
    )
    plugins = cast("list[dict[str, object]]", project["Plugins"])
    matching_plugins = [
        plugin
        for plugin in plugins
        if plugin.get("Name") == "VisualStudioTools"
    ]

    assert matching_plugins == [
        {
            "Name": "VisualStudioTools",
            "Enabled": False,
            "SupportedTargetPlatforms": ["Win64"],
        }
    ]
