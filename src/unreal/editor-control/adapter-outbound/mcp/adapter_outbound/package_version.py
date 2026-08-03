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
#   - Package version outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Package version outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Package version outbound adapter."""

from __future__ import annotations

from functools import lru_cache
from importlib.metadata import PackageNotFoundError
from importlib.metadata import version as distribution_version
from pathlib import Path
import tomllib
from typing import cast

from mcp.domain.errors import fail_configuration
from mcp.domain.json_types import require_json_object

_DISTRIBUTION_NAME = "shar-unreal-mcp-cli"
_SOURCE_PYPROJECT = (
    Path(__file__).resolve().parents[3]
    / "composition/mcp/pyproject.toml"
)


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
    if (
        not resolved
        or resolved != resolved.strip()
        or any(not character.isprintable() for character in resolved)
    ):
        fail_configuration("translator package version is invalid")
    return resolved


def _source_project_version() -> str:
    try:
        parsed = tomllib.loads(_SOURCE_PYPROJECT.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, tomllib.TOMLDecodeError) as error:
        fail_configuration(
            f"cannot read translator package metadata: {_SOURCE_PYPROJECT}",
            cause=error,
        )
    project = require_json_object(
        cast("object", parsed.get("project")),
        # jig-ignore-next-line: exact value is indivisible
        context="src/unreal/editor-control/composition/mcp/pyproject.toml.project",
    )
    resolved = project.get("version")
    if not isinstance(resolved, str):
        fail_configuration("translator pyproject version must be text")
    return resolved
