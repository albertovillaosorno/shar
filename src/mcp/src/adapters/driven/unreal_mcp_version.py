# File:
#   - unreal_mcp_version.py
# Path:
#   - src/mcp/src/adapters/driven/unreal_mcp_version.py
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
#   - Resolving and reading the installed Unreal MCP plugin descriptor version.
# - Must-Not:
#   - Define plugin versions, render skills, or invoke Unreal MCP tools.
# - Allows:
#   - Project association, environment override, and Program Files discovery.
# - Split-When:
#   - Engine installation discovery becomes a shared repository service.
# - Merge-When:
#   - Another adapter owns the same project-to-plugin resolution contract.
# - Summary:
#   - Reads the single authoritative Unreal MCP version from Unreal Engine.
# - Description:
#   - Normalizes the descriptor `VersionName` to three-part SemVer.
# - Usage:
#   - Composed by the `skills` CLI command before Markdown rendering.
# - Defaults:
#   - Uses `UNREAL_ENGINE_ROOT`, then the associated launcher installation.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Filesystem adapter for the installed Unreal MCP plugin version."""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import TYPE_CHECKING, cast

from mcp.src.domain.errors import ProtocolError, fail_configuration
from mcp.src.domain.json_types import (
    DuplicateJsonKeyError,
    reject_duplicate_json_object,
    require_json_object,
)
from mcp.src.domain.skill_revision import normalize_unreal_mcp_version

if TYPE_CHECKING:
    from collections.abc import Mapping

    from mcp.src.domain.json_types import JsonObject

_MISSING_DESCRIPTOR = "installed Unreal MCP plugin descriptor was not found"
_ENGINE_ROOT_HINT = "set UNREAL_ENGINE_ROOT to the associated engine root"
_PLUGIN_DESCRIPTOR = Path(
    "Engine/Plugins/Experimental/ModelContextProtocol/ModelContextProtocol.uplugin"
)


class FilesystemUnrealMcpVersionProvider:
    """Resolve Unreal MCP version through the project engine association."""

    def __init__(
        self,
        project_descriptor: Path,
        *,
        environment: Mapping[str, str] | None = None,
    ) -> None:
        """Create one project-scoped plugin version provider."""
        self._project_descriptor = project_descriptor
        self._environment = os.environ if environment is None else environment

    def read_version(self) -> str:
        """Return the installed plugin version normalized to SemVer.

        Returns:
            Canonical `major.minor.patch` Unreal MCP plugin version.
        """
        project = _read_json_object(
            self._project_descriptor,
            context="Unreal project descriptor",
        )
        association = project.get("EngineAssociation")
        if not isinstance(association, str) or not association.strip():
            fail_configuration("Unreal project EngineAssociation must be text")
        descriptor = self._resolve_plugin_descriptor(association.strip())
        plugin = _read_json_object(
            descriptor,
            context="Unreal MCP plugin descriptor",
        )
        version_name = plugin.get("VersionName")
        if not isinstance(version_name, str):
            fail_configuration("Unreal MCP VersionName must be text")
        return normalize_unreal_mcp_version(version_name)

    def _resolve_plugin_descriptor(self, association: str) -> Path:
        candidates: list[Path] = []
        explicit_root = self._environment.get("UNREAL_ENGINE_ROOT")
        if explicit_root:
            candidates.append(Path(explicit_root))
        program_files = self._environment.get("PROGRAMFILES")
        if program_files:
            candidates.append(
                Path(program_files) / "Epic Games" / f"UE_{association}"
            )
        for engine_root in candidates:
            descriptor = engine_root / _PLUGIN_DESCRIPTOR
            if descriptor.is_file():
                return descriptor
        return fail_configuration(f"{_MISSING_DESCRIPTOR}; {_ENGINE_ROOT_HINT}")


def _read_json_object(path: Path, *, context: str) -> JsonObject:
    try:
        parsed = cast(
            "object",
            json.loads(
                path.read_text(encoding="utf-8"),
                object_pairs_hook=reject_duplicate_json_object,
            ),
        )
    except DuplicateJsonKeyError as error:
        fail_configuration(str(error), cause=error)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail_configuration(f"cannot read {context}: {path}", cause=error)
    try:
        return require_json_object(parsed, context=context)
    except ProtocolError as error:
        fail_configuration(str(error), cause=error)
