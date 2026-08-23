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
#   - Test project configuration test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test project configuration test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test project configuration test module."""

from __future__ import annotations

from importlib.metadata import PackageNotFoundError
import json
from pathlib import Path
import tomllib
from typing import cast
from unittest.mock import patch

from mcp.adapter_outbound.package_version import package_version
from mcp.domain.errors import ConfigurationError
from mcp.domain.json_types import require_json_object
import pytest

_REPOSITORY_ROOT = Path(__file__).resolve().parents[3]


def test_unreal_project_has_one_canonical_descriptor_root() -> None:
    """Generated Unreal state must not recreate the obsolete project root."""
    canonical_root = (
        _REPOSITORY_ROOT
        / "src/unreal/project/composition/uproject"
    )
    obsolete_roots = (
        _REPOSITORY_ROOT / "uproject",
        _REPOSITORY_ROOT / "shar-uproject",
    )
    descriptors = tuple(sorted(_REPOSITORY_ROOT.rglob("*.uproject")))

    assert descriptors == (canonical_root / "shar.uproject",)
    assert all(not path.exists() for path in obsolete_roots)


def test_unreal_project_enables_inbound_server_and_all_toolsets() -> None:
    project_path = (
        _REPOSITORY_ROOT
        / "src/unreal/project/composition/uproject/shar.uproject"
    )
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
    translator_readme_path = (
        _REPOSITORY_ROOT
        / "src/unreal/editor-control/composition/mcp/README.md"
    )
    translator_readme = translator_readme_path.read_text(encoding="utf-8")

    ignore_lines = ignore_text.splitlines()
    assert "src/unreal/project/composition/uproject/Plugins/" in ignore_lines
    logs_segment = "[Ll]ogs/"
    prefix = "!skills/unreal/capabilities/**/"
    assert f"{prefix}{logs_segment}" in ignore_lines
    assert f"{prefix}{logs_segment}**" in ignore_lines
    assert "not an MCP server" in translator_readme
    assert "future fallback" in translator_readme


def test_persistent_project_state_links_are_ignored_explicitly() -> None:
    """Git must ignore the persistent links, not only real directories."""
    ignore_lines = (_REPOSITORY_ROOT / ".gitignore").read_text(
        encoding="utf-8"
    ).splitlines()
    project_root = "src/unreal/project/composition/uproject"
    for name in ("Binaries", "DerivedDataCache", "Intermediate", "Saved"):
        assert f"{project_root}/{name}" in ignore_lines


def test_repository_pytest_configuration_owns_import_discovery() -> None:
    """Python tests must share one repository-owned Jig config authority."""
    config_lines = (
        _REPOSITORY_ROOT / ".jig/lang/python/pytest.ini"
    ).read_text(
        encoding="utf-8"
    ).splitlines()

    assert "    --strict-config" in config_lines
    assert "    --strict-markers" in config_lines
    assert "    --import-mode=importlib" in config_lines
    for kind_root in (
        "domain",
        "application",
        "port-outbound",
        "adapter-inbound",
        "adapter-outbound",
    ):
        expected_root = (
            f"    ../../../src/unreal/editor-control/{kind_root}"
        )
        assert expected_root in config_lines
    assert (
        "    ../../../src/unreal/project/composition/uproject"
        in config_lines
    )
    assert "    ../../../tests/unreal/editor-control" in config_lines
    assert "    ../../../tests/unreal/project" in config_lines
    assert "filterwarnings =" in config_lines
    assert "    error" in config_lines
    assert not (
        _REPOSITORY_ROOT
        / "src/unreal/editor-control/adapter-inbound/mcp/pytest.ini"
    ).exists()
    assert not (
        _REPOSITORY_ROOT
        / "src/unreal/project/composition/uproject/pytest.ini"
    ).exists()


def test_translator_version_matches_package_metadata() -> None:
    """Wire metadata and skill revisions use the package Calendar Version."""
    pyproject = require_json_object(
        cast(
            "object",
            tomllib.loads(
                (
                    _REPOSITORY_ROOT
                    / "src/unreal/editor-control/composition/mcp/pyproject.toml"
                ).read_text(encoding="utf-8")
            ),
        ),
        context="src/unreal/editor-control/composition/mcp/pyproject.toml",
    )
    project = require_json_object(
        pyproject.get("project"),
        # jig-ignore-next-line: exact value is indivisible
        context="src/unreal/editor-control/composition/mcp/pyproject.toml.project",
    )

    assert project["version"] == package_version()


@pytest.mark.parametrize(
    "invalid_version",
    ["1.0.0\ninjected", " 1.0.0 "],
)
def test_translator_version_rejects_noncanonical_text(
    invalid_version: str,
) -> None:
    """Client metadata cannot retain controls or surrounding whitespace."""
    package_version.cache_clear()
    try:
        with (
            patch(
                "mcp.adapter_outbound.package_version.distribution_version",
                return_value=invalid_version,
            ),
            pytest.raises(
                ConfigurationError,
                match="package version is invalid",
            ),
        ):
            _ = package_version()
    finally:
        package_version.cache_clear()


def test_translator_source_version_wraps_invalid_utf8(tmp_path: Path) -> None:
    """Unreadable source metadata remains a typed configuration failure."""
    metadata = tmp_path / "pyproject.toml"
    _ = metadata.write_bytes(b"\xff")
    package_version.cache_clear()
    try:
        with (
            patch(
                "mcp.adapter_outbound.package_version._SOURCE_PYPROJECT",
                metadata,
            ),
            patch(
                "mcp.adapter_outbound.package_version.distribution_version",
                side_effect=PackageNotFoundError,
            ),
            pytest.raises(ConfigurationError, match="cannot read translator"),
        ):
            _ = package_version()
    finally:
        package_version.cache_clear()


def test_native_mcp_server_autostarts_with_tool_search() -> None:
    settings_path = (
        _REPOSITORY_ROOT
        / "src/unreal/project/composition/uproject/Config"
        / "DefaultEditorPerProjectUserSettings.ini"
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


def test_translator_uses_canonical_kind_layout_and_stable_entrypoint() -> None:
    """Every Python package surface must live under its owning kind."""
    function_root = _REPOSITORY_ROOT / "src/unreal/editor-control"
    package_root = function_root / "composition/mcp"
    pyproject_lines = (
        (package_root / "pyproject.toml")
        .read_text(encoding="utf-8")
        .splitlines()
    )
    expected_packages = {
        "domain": function_root / "domain/mcp/domain",
        "application": function_root / "application/mcp/application",
        "port_outbound": (
            function_root / "port-outbound/mcp/port_outbound"
        ),
        "adapter_inbound": (
            function_root / "adapter-inbound/mcp/adapter_inbound"
        ),
        "adapter_outbound": (
            function_root / "adapter-outbound/mcp/adapter_outbound"
        ),
    }

    assert all(path.is_dir() for path in expected_packages.values())
    assert not tuple(function_root.rglob("src"))
    assert not tuple(function_root.rglob("tests"))
    assert (function_root / "contract/mcp/py.typed").is_file()
    assert (
        'shar-unreal-mcp = "mcp.adapter_inbound.cli:main"'
        in pyproject_lines
    )
    for package in expected_packages:
        assert any(
            f'"mcp/{package}"' in line
            for line in pyproject_lines
        )
    assert any(
        '"mcp/py.typed"' in line
        for line in pyproject_lines
    )


def test_game_feature_data_primary_asset_type_is_always_cooked() -> None:
    """Game Feature plugins require a non-default asset management rule."""
    settings_path = (
        _REPOSITORY_ROOT
        / "src/unreal/project/composition/uproject/Config/DefaultGame.ini"
    )
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
