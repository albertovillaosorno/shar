# File:
#   - test_xml_config_cache.py
# Path:
#   - src/uproject/tests/project_generation/test_xml_config_cache.py
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
#   - Path and deletion behavior for the project UBT XML cache.
# - Must-Not:
#   - Invoke live Unreal processes or depend on network services.
# - Allows:
#   - Deterministic local fixtures and observable assertions.
# - Split-When:
#   - Another behavior belongs to a different tooling boundary.
# - Merge-When:
#   - Another test module proves the same behavior contract.
# - Summary:
#   - Unit tests for project-scoped UBT cache recovery.
# - Description:
#   - Proves exact selection, no-op, and bounded deletion.
# - Usage:
#   - Run by pytest against isolated temporary directories.
# - Defaults:
#   - Only XmlConfigCache.bin is eligible for deletion.
#
# ADRs:
# - docs/adr/unreal/runtime/runtime-parity-test-boundary.md
#
# Large file:
#   - false
#


"""Project-scoped UnrealBuildTool XML cache contracts."""

from pathlib import Path

from Scripts import repair_unreal_project


def test_cache_path_matches_unreal_build_tool_selection() -> None:
    project_path = Path("C:/workspace/shar/shar.uproject")

    cache_path = repair_unreal_project.project_xml_config_cache_path(
        project_path
    )

    assert cache_path == Path(
        "C:/workspace/shar/Intermediate/Build/XmlConfigCache.bin"
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
