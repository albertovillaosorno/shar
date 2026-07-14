# File:
#   - package_version.py
# Path:
#   - src/mcp/src/adapters/driven/package_version.py
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
#   - Reading the Python translator package version from one metadata authority.
# - Must-Not:
#   - Define a version literal or influence generated skill review revisions.
# - Allows:
#   - Falling back to source pyproject metadata for an uninstalled checkout.
# - Split-When:
#   - Distribution and source metadata require independent adapters.
# - Merge-When:
#   - Another adapter owns the same package metadata lookup.
# - Summary:
#   - Resolves the terminal client's package version without duplication.
# - Description:
#   - Installed metadata wins; source pyproject is the development fallback.
# - Usage:
#   - Supplies MCP clientInfo during transport initialization.
# - Defaults:
#   - Fails closed when neither metadata source is valid.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
#
# Large file:
#   - false
#
"""Single-source package version resolution for the Python translator."""

from __future__ import annotations

import tomllib
from functools import lru_cache
from importlib.metadata import PackageNotFoundError
from importlib.metadata import version as distribution_version
from pathlib import Path
from typing import cast

from mcp.src.domain.errors import fail_configuration
from mcp.src.domain.json_types import require_json_object

_DISTRIBUTION_NAME = "shar-unreal-mcp-cli"
_SOURCE_PYPROJECT = Path(__file__).resolve().parents[3] / "pyproject.toml"


@lru_cache(maxsize=1)
def package_version() -> str:
    """Return the Python client version from its canonical metadata.

    Returns:
        Installed distribution version or source-project version.
    """
    try:
        resolved = distribution_version(_DISTRIBUTION_NAME)
    except PackageNotFoundError:
        resolved = _source_project_version()
    if not resolved.strip():
        fail_configuration("translator package version must not be empty")
    return resolved


def _source_project_version() -> str:
    try:
        parsed = tomllib.loads(_SOURCE_PYPROJECT.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        fail_configuration(
            f"cannot read translator package metadata: {_SOURCE_PYPROJECT}",
            cause=error,
        )
    project = require_json_object(
        cast("object", parsed.get("project")),
        context="src/mcp/pyproject.toml.project",
    )
    resolved = project.get("version")
    if not isinstance(resolved, str):
        fail_configuration("translator pyproject version must be text")
    return resolved
