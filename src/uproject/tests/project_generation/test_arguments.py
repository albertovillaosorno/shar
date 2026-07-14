# File:
#   - test_arguments.py
# Path:
#   - src/uproject/tests/project_generation/test_arguments.py
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
#   - Argument contracts for both Unreal project generators.
# - Must-Not:
#   - Invoke live Unreal processes or depend on network services.
# - Allows:
#   - Deterministic local fixtures and observable assertions.
# - Split-When:
#   - Another behavior belongs to a different tooling boundary.
# - Merge-When:
#   - Another test module proves the same behavior contract.
# - Summary:
#   - Unit tests for Unreal project-generation arguments.
# - Description:
#   - Locks workspace and solution commands to bounded tokens.
# - Usage:
#   - Run by pytest without launching UnrealBuildTool.
# - Defaults:
#   - Tests use a synthetic portable project path.
#
# ADRs:
# - docs/adr/unreal/runtime/runtime-parity-test-boundary.md
#
# Large file:
#   - false
#


"""Command argument contracts for Unreal project-file generation."""

from pathlib import Path

from Scripts import repair_unreal_project


def test_workspace_generation_arguments_are_bounded() -> None:
    project_path = Path("C:/workspace/shar/shar.uproject")

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


def test_solution_generation_arguments_are_bounded() -> None:
    project_path = Path("C:/workspace/shar/shar.uproject")

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
