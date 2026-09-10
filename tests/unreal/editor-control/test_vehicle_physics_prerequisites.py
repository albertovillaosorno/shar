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
#   - Vehicle-physics skeletal prerequisite compiler tests.
# - Must-Not:
#   - Read repository caches or contact Unreal Editor.
# - Allows:
#   - Synthetic ready rigs and skeletal import contracts.
# - Split-When:
#   - Prerequisite capability/application tests gain separate lifecycles.
# - Merge-When:
#   - Construction compiler tests own identical prerequisite joins.
# - Summary:
#   - Vehicle-physics skeletal prerequisite tests.
# - Description:
#   - Proves deduplication, exact joins, rejection, and revision binding.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Non-vehicle, ambiguous, or mismatched dependencies fail closed.
#

"""Tests for exact vehicle-physics Skeletal Mesh prerequisite compilation."""

from __future__ import annotations

from mcp.domain.errors import ProtocolError
from mcp.domain.plan_execution import CompiledExecutionPlan
from mcp.domain.plan_execution import NativeImportStep
from mcp.domain.plan_execution import PlanExecutionReport
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
from mcp.domain.vehicle_physics_prerequisites import (
    compile_vehicle_physics_prerequisites,
)
from mcp.domain.vehicle_physics_prerequisites import (
    select_vehicle_physics_prerequisite_package,
)
from mcp.domain.vehicle_physics_prerequisites import (
    vehicle_physics_prerequisite_revision,
)
import pytest

_SOURCE = "vehicle-assets/sedana/sedana.fbx"
_PACKAGE = "/Game/Generated/SHAR/cars/sedana_Skeletal"
_OBJECT = f"{_PACKAGE}.sedana_Skeletal"


def _import(
    *,
    source: str = _SOURCE,
    revision: str = "1" * 64,
    operation: str = "operation-0000000000000001",
    destination: str = _OBJECT,
) -> NativeImportStep:
    package, _, asset = destination.rpartition(".")
    folder, _, _ = package.rpartition("/")
    return NativeImportStep(
        operation_id=operation,
        route_id="skeletal-mesh-fbx-v1",
        source_path=source,
        source_revision=revision,
        destination=destination,
        target_class="SkeletalMesh",
        package_path=package,
        folder_path=folder,
        asset_name=asset,
        toolset_name="SharImportEditor.SharImportToolset",
        tool_name="SharImportEditor.SharImportToolset.ImportSkeletalMesh",
        external_payload_path=None,
    )


def _execution(*imports: NativeImportStep) -> CompiledExecutionPlan:
    return CompiledExecutionPlan(
        PlanExecutionReport(
            bundle_revision="a" * 64,
            operation_count=len(imports),
            compiled_count=len(imports),
            semantic_blocker_count=1,
            blocked_readiness={},
            unsupported_routes={},
            route_counts={"skeletal-mesh-fbx-v1": len(imports)},
        ),
        imports,
    )


def _request(
    identity: str,
    *,
    source: str = _SOURCE,
    package_id: str = "sedana",
    skeletal_mesh_path: str = _OBJECT,
) -> VehiclePhysicsConstructionStep:
    shape = VehiclePhysicsShape(
        kind="sphere",
        bone_name="w0",
        center=(0.0, 0.0, 0.0),
        radius=0.4,
        axes=(),
        extents=(0.0, 0.0, 0.0),
    )
    return VehiclePhysicsConstructionStep(
        package_id=package_id,
        source_fbx=source,
        rig_identity=identity,
        source_joint_count=19,
        skeletal_mesh_path=skeletal_mesh_path,
        folder_path="/Game/Generated/SHAR/VehiclePhysics",
        asset_name=f"PHYS_{identity}",
        package_path=f"/Game/Generated/SHAR/VehiclePhysics/PHYS_{identity}",
        object_path=(
            "/Game/Generated/SHAR/VehiclePhysics/"
            f"PHYS_{identity}.PHYS_{identity}"
        ),
        shapes=(shape,),
    )


def _construction(
    *requests: VehiclePhysicsConstructionStep,
) -> CompiledVehiclePhysicsConstruction:
    return CompiledVehiclePhysicsConstruction(
        VehiclePhysicsConstructionReport(
            request_count=len(requests),
            blocked_rig_count=1,
            shape_count=len(requests),
        ),
        requests,
    )


def test_deduplicates_two_ready_rigs_using_one_vehicle_fbx() -> None:
    compiled = compile_vehicle_physics_prerequisites(
        _construction(_request("sedana"), _request("sedanaBV")),
        _execution(_import()),
    )
    assert compiled.report.import_count == 1
    assert compiled.report.source_count == 1
    assert compiled.report.ready_rig_count == 2
    assert compiled.imports[0].source_path == _SOURCE


def test_rejects_non_vehicle_source_and_destination_mismatch() -> None:
    non_vehicle = "fbx-assets/sedana/sedana.fbx"
    with pytest.raises(ProtocolError, match="not a vehicle FBX"):
        compile_vehicle_physics_prerequisites(
            _construction(_request("sedana", source=non_vehicle)),
            _execution(_import(source=non_vehicle)),
        )
    with pytest.raises(ProtocolError, match="destination does not match"):
        compile_vehicle_physics_prerequisites(
            _construction(_request("sedana")),
            _execution(_import(destination="/Game/Other/model.model")),
        )


def test_rejects_ambiguous_skeletal_import_source() -> None:
    with pytest.raises(ProtocolError, match="ambiguous"):
        compile_vehicle_physics_prerequisites(
            _construction(_request("sedana")),
            _execution(
                _import(),
                _import(operation="operation-0000000000000002"),
            ),
        )


def test_revision_binds_source_revision() -> None:
    construction = _construction(_request("sedana"))
    first = compile_vehicle_physics_prerequisites(
        construction,
        _execution(_import(revision="1" * 64)),
    )
    second = compile_vehicle_physics_prerequisites(
        construction,
        _execution(_import(revision="2" * 64)),
    )
    assert vehicle_physics_prerequisite_revision(first) != (
        vehicle_physics_prerequisite_revision(second)
    )


def test_selects_exact_package_after_global_prerequisites_compile() -> None:
    other_source = "vehicle-assets/other/other.fbx"
    other_object = "/Game/Generated/SHAR/cars/other_Skeletal.other_Skeletal"
    construction = _construction(
        _request("sedana", package_id="extracted-art-cars-sedana"),
        _request(
            "other",
            source=other_source,
            package_id="extracted-art-cars-other",
            skeletal_mesh_path=other_object,
        ),
    )
    prerequisites = compile_vehicle_physics_prerequisites(
        construction,
        _execution(
            _import(),
            _import(
                source=other_source,
                operation="operation-0000000000000002",
                destination=other_object,
            ),
        ),
    )
    selected = select_vehicle_physics_prerequisite_package(
        construction,
        prerequisites,
        "extracted-art-cars-sedana",
    )
    assert prerequisites.report.import_count == 2
    assert prerequisites.report.ready_rig_count == 2
    assert selected.report.to_json() == {
        "importCount": 1,
        "packageId": "extracted-art-cars-sedana",
        "readyRigCount": 1,
        "sourceCount": 1,
    }
    assert selected.imports == (_import(),)
    assert vehicle_physics_prerequisite_revision(selected) != (
        vehicle_physics_prerequisite_revision(prerequisites)
    )


def test_package_selection_rejects_package_without_ready_rig() -> None:
    construction = _construction(_request("sedana"))
    prerequisites = compile_vehicle_physics_prerequisites(
        construction,
        _execution(_import()),
    )
    with pytest.raises(ProtocolError, match="has no native-ready rigs"):
        select_vehicle_physics_prerequisite_package(
            construction,
            prerequisites,
            "extracted-art-cars-missing",
        )
