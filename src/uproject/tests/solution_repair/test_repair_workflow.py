# File:
#   - test_repair_workflow.py
# Path:
#   - src/uproject/tests/solution_repair/test_repair_workflow.py
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
#   - Patch-only orchestration for existing generated solutions.
# - Must-Not:
#   - Invoke live Unreal processes or depend on network services.
# - Allows:
#   - Deterministic local fixtures and observable assertions.
# - Split-When:
#   - Regeneration orchestration needs separate integration tests.
# - Merge-When:
#   - Another module proves the same patch-only workflow.
# - Summary:
#   - Unit tests for patch-only project-file repair.
# - Description:
#   - Proves missing-solution failure and successful local repair.
# - Usage:
#   - Run by pytest without resolving an Unreal installation.
# - Defaults:
#   - Patch-only mode never removes caches or invokes UBT.
#
# ADRs:
# - docs/adr/unreal/runtime/runtime-parity-test-boundary.md
#
# Large file:
#   - false
#


"""Patch-only project repair workflow contracts."""

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
