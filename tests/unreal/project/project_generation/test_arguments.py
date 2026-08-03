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
#   - Test arguments test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test arguments test module.
# - Description:
#   - Implements the declared test module responsibility for project.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test arguments test module."""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path
from Scripts import repair_unreal_project


def test_workspace_generation_arguments_are_bounded(tmp_path: Path) -> None:
    project_path = tmp_path / "shar.uproject"

    arguments = repair_unreal_project.workspace_generation_arguments(
        project_path
    )

    assert arguments[:3] == (
        "-mode=GenerateProjectFiles",
        "-ProjectFileFormat=VisualStudioWorkspace",
        f"-Project={project_path}",
    )
    assert set(arguments[3:]) == {
        "-Automated",
        "-Engine",
        "-Game",
        "-Platforms=Win64",
        "-Progress",
        "-ProjectNames=shar",
        "-TargetConfigurations=Development",
        "-TargetTypes=Editor",
        "-WaitMutex",
    }
    assert len(arguments) == 12
    assert not any(
        argument.startswith("-XmlConfigCache=") for argument in arguments
    )


def test_solution_generation_arguments_are_bounded(tmp_path: Path) -> None:
    project_path = tmp_path / "shar.uproject"

    arguments = repair_unreal_project.solution_generation_arguments(
        project_path
    )

    assert arguments == (
        "-projectfiles",
        f"-project={project_path}",
        "-game",
        "-rocket",
        "-progress",
    )
