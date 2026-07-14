# File:
#   - test_unreal_mcp_version.py
# Path:
#   - src/mcp/tests/test_unreal_mcp_version.py
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
#   - Regression tests for installed Unreal MCP plugin version discovery.
# - Must-Not:
#   - Inspect the operator's real engine, connect to Unreal, or render skills.
# - Allows:
#   - Temporary project and engine descriptor fixtures.
# - Split-When:
#   - Engine resolution and version normalization need independent suites.
# - Merge-When:
#   - Another module owns the complete plugin version discovery contract.
# - Summary:
#   - Guards the single Unreal MCP version authority used by skill revisions.
# - Description:
#   - Proves descriptor `1.0` normalizes to public SemVer `1.0.0`.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Uses `UNREAL_ENGINE_ROOT` fixtures without external dependencies.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Installed Unreal MCP plugin version discovery regression tests."""

from __future__ import annotations

import json
from typing import TYPE_CHECKING

import pytest
from mcp.src.adapters.driven.unreal_mcp_version import (
    FilesystemUnrealMcpVersionProvider,
)
from mcp.src.domain.errors import ConfigurationError
from mcp.src.domain.skill_revision import (
    build_skill_revision,
    normalize_unreal_mcp_version,
)

if TYPE_CHECKING:
    from pathlib import Path

_PLUGIN_RELATIVE_PATH = (
    "Engine/Plugins/Experimental/ModelContextProtocol/"
    "ModelContextProtocol.uplugin"
)
_TEST_DIGEST = "a" * 64


def test_two_part_plugin_version_normalizes_to_semver() -> None:
    """Unreal's `1.0` VersionName becomes the public `1.0.0` version."""
    assert normalize_unreal_mcp_version("1.0") == "1.0.0"
    assert build_skill_revision("1.0", _TEST_DIGEST).token == (
        f"1.0.0/{_TEST_DIGEST}"
    )


def test_provider_reads_version_from_explicit_engine_root(
    tmp_path: Path,
) -> None:
    """The project association resolves one installed plugin descriptor."""
    project = tmp_path / "project" / "shar.uproject"
    engine = tmp_path / "UE_5.8"
    _write_json(project, {"EngineAssociation": "5.8"})
    _write_json(
        engine / _PLUGIN_RELATIVE_PATH,
        {"VersionName": "1.0"},
    )

    version = FilesystemUnrealMcpVersionProvider(
        project,
        environment={"UNREAL_ENGINE_ROOT": str(engine)},
    ).read_version()

    assert version == "1.0.0"


def test_provider_uses_program_files_association_fallback(
    tmp_path: Path,
) -> None:
    """Launcher-style engine paths resolve without a duplicated version."""
    project = tmp_path / "project" / "shar.uproject"
    program_files = tmp_path / "Program Files"
    engine = program_files / "Epic Games" / "UE_5.8"
    _write_json(project, {"EngineAssociation": "5.8"})
    _write_json(
        engine / _PLUGIN_RELATIVE_PATH,
        {"VersionName": "1.0.0"},
    )

    version = FilesystemUnrealMcpVersionProvider(
        project,
        environment={"PROGRAMFILES": str(program_files)},
    ).read_version()

    assert version == "1.0.0"


def test_provider_rejects_missing_plugin_descriptor(tmp_path: Path) -> None:
    """Missing installed metadata cannot silently invent a version."""
    project = tmp_path / "project" / "shar.uproject"
    _write_json(project, {"EngineAssociation": "5.8"})

    with pytest.raises(ConfigurationError, match="descriptor was not found"):
        _ = FilesystemUnrealMcpVersionProvider(
            project,
            environment={"UNREAL_ENGINE_ROOT": str(tmp_path / "missing")},
        ).read_version()


def test_provider_rejects_non_numeric_version_name(tmp_path: Path) -> None:
    """Unexpected plugin version syntax fails before skill generation."""
    project = tmp_path / "project" / "shar.uproject"
    engine = tmp_path / "UE_5.8"
    _write_json(project, {"EngineAssociation": "5.8"})
    _write_json(
        engine / _PLUGIN_RELATIVE_PATH,
        {"VersionName": "preview"},
    )

    with pytest.raises(ConfigurationError, match="one to three numeric parts"):
        _ = FilesystemUnrealMcpVersionProvider(
            project,
            environment={"UNREAL_ENGINE_ROOT": str(engine)},
        ).read_version()


def _write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    _ = path.write_text(
        json.dumps(value, indent=2) + "\n",
        encoding="utf-8",
        newline="\n",
    )
