# File:
#   - test_script_generator.py
# Path:
#   - src/uproject/tests/solution_repair/test_script_generator.py
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
#   - Repair behavior for generated ScriptGenerator SLNX mappings.
# - Must-Not:
#   - Invoke live Unreal processes or depend on network services.
# - Allows:
#   - Deterministic local fixtures and observable assertions.
# - Split-When:
#   - Another solution family needs independent repair coverage.
# - Merge-When:
#   - Another module proves the same SLNX repair contract.
# - Summary:
#   - Unit tests for ScriptGenerator solution repair.
# - Description:
#   - Proves repair, preservation, BOM, and idempotence behavior.
# - Usage:
#   - Run by pytest against local synthetic SLNX fixtures.
# - Defaults:
#   - Debug and Release mappings remain unchanged.
#
# ADRs:
# - docs/adr/unreal/runtime/runtime-parity-test-boundary.md
#
# Large file:
#   - false
#


"""ScriptGenerator SLNX configuration repair contracts."""

from __future__ import annotations

import codecs
from typing import TYPE_CHECKING

from Scripts import repair_unreal_project

if TYPE_CHECKING:
    from pathlib import Path


def test_invalid_script_generator_configuration_is_repaired(
    tmp_path: Path,
) -> None:
    solution = """<Solution>
  <Project Path="Engine/ScriptGeneratorUbtPlugin.ubtplugin.csproj">
    <BuildType Project="Development" />
  </Project>
  <Project Path="Engine/Other.csproj">
    <BuildType Project="Development" />
  </Project>
</Solution>
"""
    solution_path = tmp_path / "shar.slnx"
    _ = solution_path.write_text(solution, encoding="utf-8")

    repaired = repair_unreal_project.repair_solution_configuration(
        solution_path
    )
    updated = solution_path.read_text(encoding="utf-8")

    assert repaired == 1
    assert (
        'ScriptGeneratorUbtPlugin.ubtplugin.csproj">\n'
        '    <BuildType Project="Release" />'
    ) in updated
    assert (
        'Other.csproj">\n    <BuildType Project="Development" />'
    ) in updated


def test_supported_script_generator_configuration_is_unchanged(
    tmp_path: Path,
) -> None:
    solution = """<Solution>
  <Project Path="ScriptGeneratorUbtPlugin.ubtplugin.csproj">
    <BuildType Project="Debug" />
  </Project>
</Solution>
"""
    solution_path = tmp_path / "shar.slnx"
    _ = solution_path.write_text(solution, encoding="utf-8")

    repaired = repair_unreal_project.repair_solution_configuration(
        solution_path
    )

    assert repaired == 0
    assert solution_path.read_text(encoding="utf-8") == solution


def test_solution_repair_preserves_utf8_bom(tmp_path: Path) -> None:
    solution = """<Solution>
  <Project Path="ScriptGeneratorUbtPlugin.ubtplugin.csproj">
    <BuildType Project="Development" />
  </Project>
</Solution>
"""
    solution_path = tmp_path / "shar.slnx"
    _ = solution_path.write_bytes(codecs.BOM_UTF8 + solution.encode("utf-8"))

    repaired = repair_unreal_project.repair_solution_configuration(
        solution_path
    )

    assert repaired == 1
    assert solution_path.read_bytes().startswith(codecs.BOM_UTF8)


def test_solution_repair_is_idempotent(tmp_path: Path) -> None:
    solution = """<Solution>
  <Project Path="ScriptGeneratorUbtPlugin.ubtplugin.csproj">
    <BuildType Project="Release" />
  </Project>
</Solution>
"""
    solution_path = tmp_path / "shar.slnx"
    _ = solution_path.write_text(solution, encoding="utf-8")

    first_repair = repair_unreal_project.repair_solution_configuration(
        solution_path
    )
    first_bytes = solution_path.read_bytes()
    second_repair = repair_unreal_project.repair_solution_configuration(
        solution_path
    )

    assert first_repair == 0
    assert second_repair == 0
    assert solution_path.read_bytes() == first_bytes
