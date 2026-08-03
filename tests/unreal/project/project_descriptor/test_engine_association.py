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
#   - Test engine association test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test engine association test module.
# - Description:
#   - Implements the declared test module responsibility for project.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test engine association test module."""

from __future__ import annotations

import json
from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    from pathlib import Path
from Scripts import repair_unreal_project


def _write_descriptor(tmp_path: Path, payload: object) -> Path:
    project_path = tmp_path / "shar.uproject"
    _ = project_path.write_text(json.dumps(payload), encoding="utf-8")
    return project_path


def test_portable_engine_association_is_accepted(tmp_path: Path) -> None:
    project_path = _write_descriptor(tmp_path, {"EngineAssociation": "5.8"})

    association = repair_unreal_project.read_engine_association(project_path)

    assert association == "5.8"


def test_machine_specific_engine_guid_is_rejected(tmp_path: Path) -> None:
    project_path = _write_descriptor(
        tmp_path,
        {"EngineAssociation": "{LOCAL-ENGINE-GUID}"},
    )

    with pytest.raises(
        repair_unreal_project.ProjectAssociationError,
        match="must not be a GUID",
    ):
        _ = repair_unreal_project.read_engine_association(project_path)


def test_unsupported_engine_version_is_rejected(tmp_path: Path) -> None:
    project_path = _write_descriptor(tmp_path, {"EngineAssociation": "5.7"})

    with pytest.raises(
        repair_unreal_project.ProjectAssociationError,
        match=r"requires engine 5\.8",
    ):
        _ = repair_unreal_project.read_engine_association(project_path)


def test_missing_engine_association_is_rejected(tmp_path: Path) -> None:
    project_path = _write_descriptor(tmp_path, {})

    with pytest.raises(
        repair_unreal_project.ProjectAssociationError,
        match="has no engine association",
    ):
        _ = repair_unreal_project.read_engine_association(project_path)


def test_non_object_project_descriptor_is_rejected(tmp_path: Path) -> None:
    project_path = _write_descriptor(tmp_path, [])

    with pytest.raises(
        repair_unreal_project.ProjectDescriptorTypeError,
        match="must contain a JSON object",
    ):
        _ = repair_unreal_project.read_engine_association(project_path)
