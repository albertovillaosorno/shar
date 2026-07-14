# File:
#   - test_engine_association.py
# Path:
#   - src/uproject/tests/project_descriptor/test_engine_association.py
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
#   - Validation behavior for portable engine associations.
# - Must-Not:
#   - Invoke live Unreal processes or depend on network services.
# - Allows:
#   - Deterministic local fixtures and observable assertions.
# - Split-When:
#   - Another behavior belongs to a different tooling boundary.
# - Merge-When:
#   - Another test module proves the same behavior contract.
# - Summary:
#   - Unit tests for Unreal engine-association validation.
# - Description:
#   - Proves accepted, GUID, version, missing, and shape outcomes.
# - Usage:
#   - Run by pytest as the project descriptor validation slice.
# - Defaults:
#   - Fixtures use isolated JSON project descriptors.
#
# ADRs:
# - docs/adr/unreal/runtime/runtime-parity-test-boundary.md
#
# Large file:
#   - false
#


"""Engine-association contracts for the Unreal project descriptor."""

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
