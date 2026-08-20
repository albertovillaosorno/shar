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
#   - Test engine resolution test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test engine resolution test module.
# - Description:
#   - Implements the declared test module responsibility for project.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test engine resolution test module."""

from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    from pathlib import Path
from Scripts import repair_unreal_project


def _create_engine_root(tmp_path: Path) -> Path:
    engine_root = tmp_path / "UE_5.8"
    build_script = repair_unreal_project.project_generation_script(engine_root)
    build_script.parent.mkdir(parents=True)
    _ = build_script.write_text("@echo off\n", encoding="utf-8")
    return engine_root


def test_explicit_engine_root_is_preferred(tmp_path: Path) -> None:
    engine_root = _create_engine_root(tmp_path)

    resolved = repair_unreal_project.resolve_engine_root("5.8", engine_root)

    assert resolved == engine_root


def test_environment_engine_root_is_supported(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    engine_root = _create_engine_root(tmp_path)
    monkeypatch.setenv("UNREAL_ENGINE_ROOT", str(engine_root))
    monkeypatch.delenv("PROGRAMFILES", raising=False)

    resolved = repair_unreal_project.resolve_engine_root("5.8", None)

    assert resolved == engine_root


def test_missing_engine_reports_recovery_options(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    monkeypatch.delenv("UNREAL_ENGINE_ROOT", raising=False)
    monkeypatch.delenv("PROGRAMFILES", raising=False)

    with pytest.raises(
        repair_unreal_project.EngineInstallationNotFoundError,
        match="pass --engine-root or set UNREAL_ENGINE_ROOT",
    ):
        _ = repair_unreal_project.resolve_engine_root("5.8", None)
