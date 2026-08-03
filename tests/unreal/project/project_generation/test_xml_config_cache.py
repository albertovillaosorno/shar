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
#   - Test xml config cache test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test xml config cache test module.
# - Description:
#   - Implements the declared test module responsibility for project.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test xml config cache test module."""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path
from Scripts import repair_unreal_project


def test_cache_path_matches_unreal_build_tool_selection(
    tmp_path: Path,
) -> None:
    project_path = tmp_path / "shar.uproject"

    cache_path = repair_unreal_project.project_xml_config_cache_path(
        project_path
    )

    assert cache_path == (
        tmp_path / "Intermediate" / "Build" / "XmlConfigCache.bin"
    )


def test_missing_cache_is_a_no_op(tmp_path: Path) -> None:
    project_path = tmp_path / "shar.uproject"

    removed = repair_unreal_project.remove_project_xml_config_cache(
        project_path
    )

    assert not removed


def test_cache_removal_is_limited_to_selected_project(tmp_path: Path) -> None:
    project_path = tmp_path / "shar.uproject"
    cache_path = repair_unreal_project.project_xml_config_cache_path(
        project_path
    )
    unrelated_path = tmp_path / "Intermediate" / "Build" / "OtherCache.bin"
    cache_path.parent.mkdir(parents=True)
    _ = cache_path.write_bytes(b"truncated")
    _ = unrelated_path.write_bytes(b"valid")

    removed = repair_unreal_project.remove_project_xml_config_cache(
        project_path
    )

    assert removed
    assert not cache_path.exists()
    assert unrelated_path.read_bytes() == b"valid"
