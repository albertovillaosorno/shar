# File:
#   - repair_unreal_project.py
# Path:
#   - src/uproject/Scripts/repair_unreal_project.py
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
#   - Recovery and regeneration of Visual Studio project files.
# - Must-Not:
#   - Modify authored Unreal assets or unrelated generated state.
# - Allows:
#   - Bounded UBT invocation and deterministic SLNX repair.
# - Split-When:
#   - Another generated-file family needs separate recovery.
# - Merge-When:
#   - Another entry point owns this repair workflow.
# - Summary:
#   - Repairs Visual Studio files for the Unreal project.
# - Description:
#   - Clears corrupt UBT state and repairs generated SLNX data.
# - Usage:
#   - Invoked by the batch entry point and imported by unit tests.
# - Defaults:
#   - No engine process runs unless regeneration is requested.
#
# ADRs:
# - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
#
# Large file:
#   - true
#

"""Repair Visual Studio project files for the SHAR Unreal project."""

from __future__ import annotations

import argparse
import codecs
import json
import os
import re
import subprocess  # noqa: S404 -- Runs one resolved local Unreal batch file.
import sys
from pathlib import Path
from typing import Final, cast

ENGINE_ASSOCIATION: Final = "5.8"
_SCRIPT_GENERATOR_PROJECT: Final = "ScriptGeneratorUbtPlugin.ubtplugin.csproj"
_PROJECT_BLOCK_PATTERN = re.compile(
    "".join(
        (
            rf'<Project Path="[^"]*{re.escape(_SCRIPT_GENERATOR_PROJECT)}">',
            r".*?</Project>",
        )
    ),
    flags=re.IGNORECASE | re.DOTALL,
)
_BUILD_TYPE_PATTERN = re.compile(
    r'(<BuildType Project=")([^"]+)("\s*/>)',
    flags=re.IGNORECASE,
)

type Arguments = tuple[Path, Path | None, bool]
type CommandArguments = tuple[str, ...]


class ProjectDescriptorTypeError(TypeError):
    """The project descriptor is not a JSON object."""


class ProjectAssociationError(ValueError):
    """The project engine association is invalid."""


class EngineInstallationNotFoundError(FileNotFoundError):
    """The requested Unreal Engine installation does not exist."""


class GeneratedSolutionNotFoundError(FileNotFoundError):
    """The expected generated Visual Studio solution does not exist."""


def read_engine_association(project_path: Path) -> str:
    """Read the portable Unreal Engine association.

    Returns:
        The validated engine association.

    Raises:
        ProjectAssociationError: The association is missing or unsupported.
        ProjectDescriptorTypeError: The descriptor is not a JSON object.
    """
    payload = cast(
        "object",
        json.loads(project_path.read_text(encoding="utf-8")),
    )
    if not isinstance(payload, dict):
        message = "Unreal project descriptor must contain a JSON object"
        raise ProjectDescriptorTypeError(message)
    descriptor = cast("dict[str, object]", payload)
    association = descriptor.get("EngineAssociation")
    if not isinstance(association, str) or not association:
        message = "Unreal project descriptor has no engine association"
        raise ProjectAssociationError(message)
    if association.startswith("{") or association.endswith("}"):
        message = "Unreal project engine association must not be a GUID"
        raise ProjectAssociationError(message)
    if association != ENGINE_ASSOCIATION:
        message = (
            f"Unreal project requires engine {ENGINE_ASSOCIATION}, "
            f"not {association}"
        )
        raise ProjectAssociationError(message)
    return association


def project_generation_script(engine_root: Path) -> Path:
    """Return the selected engine's Build.bat path.

    Returns:
        The UnrealBuildTool batch-file path.
    """
    return engine_root / "Engine" / "Build" / "BatchFiles" / "Build.bat"


def resolve_engine_root(association: str, explicit_root: Path | None) -> Path:
    """Resolve a verified Unreal Engine installation.

    Returns:
        The matching Unreal Engine root.

    Raises:
        EngineInstallationNotFoundError: No matching installation exists.
    """
    candidates: list[Path] = []
    if explicit_root is not None:
        candidates.append(explicit_root)
    else:
        environment_root = os.environ.get("UNREAL_ENGINE_ROOT")
        if environment_root:
            candidates.append(Path(environment_root))
        program_files = os.environ.get("PROGRAMFILES")
        if program_files:
            candidates.append(
                Path(program_files) / "Epic Games" / f"UE_{association}"
            )

    for candidate in candidates:
        if project_generation_script(candidate).is_file():
            return candidate

    message = (
        f"Unreal Engine {association} was not found; pass --engine-root or "
        "set UNREAL_ENGINE_ROOT"
    )
    raise EngineInstallationNotFoundError(message)


def project_xml_config_cache_path(project_path: Path) -> Path:
    """Return the project-scoped UnrealBuildTool XML cache.

    Returns:
        The cache path selected when UBT receives a project path.
    """
    return project_path.parent / "Intermediate" / "Build" / "XmlConfigCache.bin"


def remove_project_xml_config_cache(project_path: Path) -> bool:
    """Remove only this project's generated XML cache.

    Returns:
        Whether the generated cache existed and was removed.
    """
    cache_path = project_xml_config_cache_path(project_path)
    if not cache_path.exists():
        return False
    cache_path.unlink()
    return True


def workspace_generation_arguments(project_path: Path) -> CommandArguments:
    """Build Visual Studio workspace-generation arguments.

    Returns:
        The bounded arguments matching Visual Studio integration.
    """
    return (
        "-mode=GenerateProjectFiles",
        "-ProjectFileFormat=VisualStudioWorkspace",
        f"-Project={project_path}",
        "-Game",
        "-Engine",
        "-Automated",
        "-Progress",
        "-WaitMutex",
        "-ProjectNames=shar",
        "-TargetTypes=Editor",
        "-TargetConfigurations=Development",
        "-Platforms=Win64",
    )


def solution_generation_arguments(project_path: Path) -> CommandArguments:
    """Build classic Visual Studio solution-generation arguments.

    Returns:
        The bounded arguments used to refresh the generated solution.
    """
    return (
        "-projectfiles",
        f"-project={project_path}",
        "-game",
        "-rocket",
        "-progress",
    )


def run_unreal_build_tool(
    engine_root: Path,
    project_path: Path,
    arguments: CommandArguments,
) -> None:
    """Run one bounded UnrealBuildTool project-generation command."""
    build_script = project_generation_script(engine_root)
    _ = subprocess.run(  # noqa: S603 -- Executable and arguments are bounded.
        (str(build_script), *arguments),
        cwd=project_path.parent,
        check=True,
    )


def repair_solution_configuration(solution_path: Path) -> int:
    """Repair the invalid generated ScriptGenerator project build type.

    Returns:
        The number of invalid project configurations repaired.
    """
    raw_solution = solution_path.read_bytes()
    has_bom = raw_solution.startswith(codecs.BOM_UTF8)
    solution = raw_solution.decode("utf-8-sig")
    repaired_count = 0

    def repair_project(match: re.Match[str]) -> str:
        nonlocal repaired_count
        project_block = match.group(0)

        def repair_build_type(build_match: re.Match[str]) -> str:
            nonlocal repaired_count
            configuration = build_match.group(2)
            if configuration in {"Debug", "Release"}:
                return build_match.group(0)
            repaired_count += 1
            return f"{build_match.group(1)}Release{build_match.group(3)}"

        return _BUILD_TYPE_PATTERN.sub(
            repair_build_type,
            project_block,
            count=1,
        )

    repaired_solution = _PROJECT_BLOCK_PATTERN.sub(repair_project, solution)
    if repaired_count:
        encoded_solution = repaired_solution.encode("utf-8")
        if has_bom:
            encoded_solution = codecs.BOM_UTF8 + encoded_solution
        _ = solution_path.write_bytes(encoded_solution)
    return repaired_count


def repair_project_files(
    project_path: Path,
    explicit_engine_root: Path | None,
    *,
    patch_only: bool,
) -> tuple[bool, int]:
    """Clear corrupt state, regenerate files, and repair the SLNX.

    Returns:
        Whether a cache was removed and how many mappings were repaired.

    Raises:
        GeneratedSolutionNotFoundError: The generated solution does not exist.
    """
    association = read_engine_association(project_path)
    cache_removed = False
    if not patch_only:
        engine_root = resolve_engine_root(association, explicit_engine_root)
        cache_removed = remove_project_xml_config_cache(project_path)
        run_unreal_build_tool(
            engine_root,
            project_path,
            workspace_generation_arguments(project_path),
        )
        run_unreal_build_tool(
            engine_root,
            project_path,
            solution_generation_arguments(project_path),
        )

    solution_path = project_path.with_suffix(".slnx")
    if not solution_path.is_file():
        message = f"generated solution not found: {solution_path}"
        raise GeneratedSolutionNotFoundError(message)
    repaired_count = repair_solution_configuration(solution_path)
    return cache_removed, repaired_count


def parse_arguments() -> Arguments:
    """Parse typed command-line arguments.

    Returns:
        The project path, optional engine root, and patch-only flag.
    """
    project_root = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(
        description=(
            "Clear the project UBT XML cache, regenerate Visual Studio files, "
            "and repair the generated ScriptGenerator SLNX mapping."
        )
    )
    _ = parser.add_argument(
        "--project",
        type=Path,
        default=project_root / "shar.uproject",
    )
    _ = parser.add_argument("--engine-root", type=Path)
    _ = parser.add_argument(
        "--patch-only",
        action="store_true",
        help="Patch an existing generated SLNX without regeneration.",
    )
    parsed = vars(parser.parse_args())
    return (
        cast("Path", parsed["project"]),
        cast("Path | None", parsed["engine_root"]),
        cast("bool", parsed["patch_only"]),
    )


def main() -> int:
    """Repair the local SHAR Unreal project files.

    Returns:
        A successful process exit status.
    """
    project, engine_root, patch_only = parse_arguments()
    cache_removed, repaired_count = repair_project_files(
        project.resolve(),
        engine_root,
        patch_only=patch_only,
    )
    output = (
        f"Project XML configuration cache removed: {cache_removed}\n"
        f"ScriptGenerator configurations repaired: {repaired_count}\n"
    )
    _ = sys.stdout.write(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
