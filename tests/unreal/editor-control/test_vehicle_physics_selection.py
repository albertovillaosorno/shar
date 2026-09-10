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
#   - Vehicle Physics Asset package-selection policy tests.
# - Must-Not:
#   - Read repository caches, contact Unreal Editor, or parse source sidecars.
# - Allows:
#   - Synthetic compiled ready-rig requests and exact package identities.
# - Split-When:
#   - Selection gains an independent external boundary.
# - Merge-When:
#   - Construction tests own identical package-selection semantics.
# - Summary:
#   - Vehicle-physics package selection tests.
# - Description:
#   - Proves exact ready-request reduction and fail-closed package identity.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Global blocker counts remain global rather than inferred per package.
#

"""Tests for package-scoped vehicle Physics Asset request selection."""

from __future__ import annotations

from mcp.domain.errors import ProtocolError
from mcp.domain.vehicle_physics_construction import (
    CompiledVehiclePhysicsConstruction,
)
from mcp.domain.vehicle_physics_construction import (
    VehiclePhysicsConstructionReport,
)
from mcp.domain.vehicle_physics_construction import (
    VehiclePhysicsConstructionStep,
)
from mcp.domain.vehicle_physics_construction import VehiclePhysicsShape
from mcp.domain.vehicle_physics_construction import (
    vehicle_physics_construction_revision,
)
from mcp.domain.vehicle_physics_selection import select_vehicle_physics_package
import pytest


def _step(package_id: str, identity: str) -> VehiclePhysicsConstructionStep:
    shape = VehiclePhysicsShape(
        kind="sphere",
        bone_name="w0",
        center=(0.0, 0.0, 0.0),
        radius=0.4,
        axes=(),
        extents=(0.0, 0.0, 0.0),
    )
    asset_name = f"PHYS_{identity}"
    package_path = f"/Game/Generated/SHAR/VehiclePhysics/{asset_name}"
    mesh_package = f"/Game/Generated/SHAR/cars/{package_id}_Skeletal"
    mesh_asset = f"{package_id}_Skeletal"
    return VehiclePhysicsConstructionStep(
        package_id=package_id,
        source_fbx=f"vehicle-assets/{identity}/{identity}.fbx",
        rig_identity=identity,
        source_joint_count=1,
        skeletal_mesh_path=f"{mesh_package}.{mesh_asset}",
        folder_path="/Game/Generated/SHAR/VehiclePhysics",
        asset_name=asset_name,
        package_path=package_path,
        object_path=f"{package_path}.{asset_name}",
        shapes=(shape,),
    )


def _compiled() -> CompiledVehiclePhysicsConstruction:
    return CompiledVehiclePhysicsConstruction(
        VehiclePhysicsConstructionReport(
            request_count=2,
            blocked_rig_count=3,
            shape_count=2,
        ),
        (
            _step("extracted-art-cars-sedana", "sedana"),
            _step("extracted-art-cars-other", "other"),
        ),
    )


def test_selects_one_package_and_preserves_global_blocker_evidence() -> None:
    compiled = _compiled()
    selection = select_vehicle_physics_package(
        compiled,
        "extracted-art-cars-sedana",
    )
    assert selection.report.to_json() == {
        "globalBlockedRigCount": 3,
        "packageId": "extracted-art-cars-sedana",
        "requestCount": 1,
        "shapeCount": 1,
    }
    assert len(selection.construction.requests) == 1
    assert selection.construction.report.blocked_rig_count == 3
    assert vehicle_physics_construction_revision(selection.construction) != (
        vehicle_physics_construction_revision(compiled)
    )


def test_selection_rejects_bad_or_unready_package_identity() -> None:
    compiled = _compiled()
    with pytest.raises(ProtocolError, match="package id is not canonical"):
        select_vehicle_physics_package(compiled, "SedanA")
    with pytest.raises(ProtocolError, match="has no native-ready rigs"):
        select_vehicle_physics_package(
            compiled,
            "extracted-art-cars-missing",
        )
