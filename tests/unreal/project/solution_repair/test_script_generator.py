# Copyright:
#   - Copyright © 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - Test script generator test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test script generator test module.
# - Description:
#   - Implements the declared test module responsibility for project.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test script generator test module."""

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
