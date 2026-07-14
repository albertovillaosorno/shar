# File:
#   - test_project_configuration.py
# Path:
#   - src/mcp/tests/test_project_configuration.py
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
#   - Regression tests for Unreal MCP project configuration and layout.
# - Must-Not:
#   - Start Unreal, edit plugins, or inspect installed engine source.
# - Allows:
#   - Read-only checks of project declarations, layout, and ignore policy.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Guards canonical project layout, plugin direction, and local exclusion.
# - Description:
#   - Prevents duplicate project roots, outbound defaults, or vendored plugins.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Checks repository files only.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: Unreal project configuration guards
#   - reason: plugin, ignore, and auto-start checks share one project boundary
#   - split: split plugin and settings checks if either surface expands
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
from __future__ import annotations

import json
import tomllib
from pathlib import Path
from typing import cast

from mcp.src.adapters.driven.package_version import package_version
from mcp.src.domain.json_types import require_json_object

_REPOSITORY_ROOT = Path(__file__).resolve().parents[3]


def test_unreal_project_has_one_canonical_descriptor_root() -> None:
    """Generated Unreal state must not recreate the obsolete project root."""
    canonical_root = _REPOSITORY_ROOT / "src/uproject"
    obsolete_root = _REPOSITORY_ROOT / "src/shar-uproject"
    descriptors = tuple(sorted((_REPOSITORY_ROOT / "src").rglob("*.uproject")))

    assert descriptors == (canonical_root / "shar.uproject",)
    assert not obsolete_root.exists()


def test_unreal_project_enables_inbound_server_and_all_toolsets() -> None:
    project_path = _REPOSITORY_ROOT / "src/uproject/shar.uproject"
    project = require_json_object(
        cast("object", json.loads(project_path.read_text(encoding="utf-8"))),
        context="shar.uproject",
    )
    raw_plugins = project.get("Plugins")
    assert isinstance(raw_plugins, list)
    plugins: dict[str, bool] = {}
    for index, raw_plugin in enumerate(raw_plugins):
        plugin = require_json_object(
            raw_plugin,
            context=f"shar.uproject.Plugins[{index}]",
        )
        name = plugin.get("Name")
        enabled = plugin.get("Enabled")
        if isinstance(name, str) and isinstance(enabled, bool):
            plugins[name] = enabled

    assert plugins["ModelContextProtocol"] is True
    assert plugins["AllToolsets"] is True
    assert plugins["MCPClientToolset"] is False


def test_project_plugins_remain_local_and_translator_is_not_a_server() -> None:
    ignore_text = (_REPOSITORY_ROOT / ".gitignore").read_text(encoding="utf-8")
    translator_readme = (_REPOSITORY_ROOT / "src/mcp/README.md").read_text(
        encoding="utf-8"
    )

    ignore_lines = ignore_text.splitlines()
    assert "src/uproject/Plugins/" in ignore_lines
    logs_segment = "[Ll]ogs/"  # cspell:disable-line -- ogs
    prefix = "!skills/unreal/capabilities/**/"
    assert f"{prefix}{logs_segment}" in ignore_lines
    assert f"{prefix}{logs_segment}**" in ignore_lines
    assert "not an MCP server" in translator_readme
    assert "future fallback" in translator_readme


def test_translator_pytest_configuration_owns_import_discovery() -> None:
    """Package tests must not depend on the caller's working directory."""
    config_lines = (
        (_REPOSITORY_ROOT / "src/mcp/pytest.ini")
        .read_text(encoding="utf-8")
        .splitlines()
    )

    assert "--strict-config" in config_lines[2]
    assert "--strict-markers" in config_lines[2]
    assert "--import-mode=importlib" in config_lines[2]
    assert "pythonpath = .." in config_lines
    assert "testpaths = tests" in config_lines
    assert "filterwarnings = error" in config_lines


def test_translator_version_matches_package_metadata() -> None:
    """Wire metadata and skill revisions use the package Calendar Version."""
    pyproject = require_json_object(
        cast(
            "object",
            tomllib.loads(
                (_REPOSITORY_ROOT / "src/mcp/pyproject.toml").read_text(
                    encoding="utf-8"
                )
            ),
        ),
        context="src/mcp/pyproject.toml",
    )
    project = require_json_object(
        pyproject.get("project"),
        context="src/mcp/pyproject.toml.project",
    )

    assert project["version"] == package_version()


def test_native_mcp_server_autostarts_with_tool_search() -> None:
    settings_path = (
        _REPOSITORY_ROOT
        / "src/uproject/Config/DefaultEditorPerProjectUserSettings.ini"
    )
    settings = settings_path.read_text(encoding="utf-8")

    assert (
        "[/Script/ModelContextProtocolEngine.ModelContextProtocolSettings]"
        in settings
    )
    assert "ServerUrlPath=/mcp" in settings.splitlines()
    assert "ServerPortNumber=8000" in settings.splitlines()
    assert "bAutoStartServer=True" in settings.splitlines()
    assert "bEnableToolSearch=True" in settings.splitlines()


def test_translator_uses_flat_source_layout_and_stable_entrypoint() -> None:
    """The package must not recreate a redundant named source directory."""
    package_root = _REPOSITORY_ROOT / "src/mcp"
    source_root = package_root / "src"
    pyproject_lines = (
        (package_root / "pyproject.toml")
        .read_text(encoding="utf-8")
        .splitlines()
    )

    assert (source_root / "domain").is_dir()
    assert (source_root / "application").is_dir()
    assert (source_root / "ports").is_dir()
    assert (source_root / "adapters").is_dir()
    assert not (source_root / "shar_unreal_mcp").exists()
    assert not (package_root / "shar_unreal_mcp").exists()
    assert (
        'shar-unreal-mcp = "mcp.src.adapters.driving.cli:main"'
        in pyproject_lines
    )
    assert '"src" = "mcp/src"' in pyproject_lines


def test_game_feature_data_primary_asset_type_is_always_cooked() -> None:
    """Game Feature plugins require a non-default asset management rule."""
    settings_path = _REPOSITORY_ROOT / "src/uproject/Config/DefaultGame.ini"
    settings_lines = settings_path.read_text(encoding="utf-8").splitlines()
    expected_entry = (
        '+PrimaryAssetTypesToScan=(PrimaryAssetType="GameFeatureData",'
        "AssetBaseClass=/Script/GameFeatures.GameFeatureData,"
        "bHasBlueprintClasses=False,bIsEditorOnly=False,Directories=,"
        "SpecificAssets=,Rules=(Priority=-1,ChunkId=-1,"
        "bApplyRecursively=True,CookRule=AlwaysCook))"
    )

    assert "[/Script/Engine.AssetManagerSettings]" in settings_lines
    assert settings_lines.count(expected_entry) == 1
