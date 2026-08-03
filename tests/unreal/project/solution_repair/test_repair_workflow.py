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
#   - Test repair workflow test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test repair workflow test module.
# - Description:
#   - Implements the declared test module responsibility for project.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test repair workflow test module."""

from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    from pathlib import Path
from Scripts import repair_unreal_project


def test_patch_only_requires_generated_solution(tmp_path: Path) -> None:
    project_path = tmp_path / "shar.uproject"
    _ = project_path.write_text(
        '{"EngineAssociation": "5.8"}',
        encoding="utf-8",
    )

    with pytest.raises(
        repair_unreal_project.GeneratedSolutionNotFoundError,
        match="generated solution not found",
    ):
        _ = repair_unreal_project.repair_project_files(
            project_path,
            None,
            patch_only=True,
        )


def test_patch_only_repairs_existing_solution(tmp_path: Path) -> None:
    project_path = tmp_path / "shar.uproject"
    solution_path = tmp_path / "shar.slnx"
    _ = project_path.write_text(
        '{"EngineAssociation": "5.8"}',
        encoding="utf-8",
    )
    _ = solution_path.write_text(
        """<Solution>
  <Project Path="ScriptGeneratorUbtPlugin.ubtplugin.csproj">
    <BuildType Project="Development" />
  </Project>
</Solution>
""",
        encoding="utf-8",
    )

    cache_removed, repaired = repair_unreal_project.repair_project_files(
        project_path,
        None,
        patch_only=True,
    )

    assert not cache_removed
    assert repaired == 1
    assert 'BuildType Project="Release"' in solution_path.read_text(
        encoding="utf-8"
    )
