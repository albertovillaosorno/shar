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
#   - Vehicle-physics bound-reader and native compilation tests.
# - Must-Not:
#   - Contact Unreal Editor, mutate project assets, or use generated caches.
# - Allows:
#   - Synthetic v1 sidecars and skeletal import-plan dependencies.
# - Split-When:
#   - Capability or application tests gain independent fixtures.
# - Merge-When:
#   - Another suite owns the same vehicle-physics compilation boundary.
# - Summary:
#   - Vehicle-physics editor-control compilation tests.
# - Description:
#   - Proves release binding, exact FBX joins, wire values, and fail-closed
#   - input.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Missing, ambiguous, stale, or malformed physics evidence is rejected.
#

"""Tests for vehicle-physics editor-control compilation."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

from mcp.adapter_outbound.vehicle_physics_construction_reader import (
    read_bound_vehicle_physics_document,
)
from mcp.domain.errors import ProtocolError
from mcp.domain.plan_bundle import PlanBundleReport
from mcp.domain.plan_bundle import SemanticArtifactSummary
from mcp.domain.plan_bundle import ValidatedPlanBundle
from mcp.domain.plan_execution import CompiledExecutionPlan
from mcp.domain.plan_execution import NativeImportStep
from mcp.domain.plan_execution import PlanExecutionReport
from mcp.domain.vehicle_physics_construction import (
    compile_vehicle_physics_construction,
)
import pytest

_SOURCE_FBX = "vehicle-assets/sedana/sedana.fbx"


def _document() -> dict[str, object]:
    return {
        "schema": "shar-schoenwald.unreal-vehicle-physics-evidence.v1",
        "source_schema": "shar.vehicle-catalog.v8",
        "target_policy": {
            "source_coordinate_space": "source-bone-local",
            "source_unit": "meter",
            "skeletal_import_unit_policy": "scene-unit-converted",
            "local_axis_conversion": (
                "source-x-y-z-to-target-z-x-y"
            ),
            "native_dimension_policy": (
                "retain-source-magnitude-under-bone-scale"
            ),
            "rig_binding_policy": "match-imported-render-root",
            "secondary_body_policy": (
                "kinematic-until-joints-translated"
            ),
            "self_collision_policy": (
                "source-empty-disable-all"
            ),
            "box_extent_policy": "source-half-to-native-full",
            "unsupported_shape_policy": "block-rig",
        },
        "counts": {
            "vehicles": 1,
            "rigs": 2,
            "primitives": 3,
            "spheres": 1,
            "oriented_boxes": 1,
            "cylinders": 1,
            "native_ready_rigs": 1,
            "native_blocked_rigs": 1,
            "native_shapes": 2,
        },
        "native_construction": {
            "requests": [{
                "package_id": "extracted-art-cars-sedana",
                "source_fbx": _SOURCE_FBX,
                "subcategory": "cars/road",
                "rig_identity": "sedanA",
                "joint_count": 19,
                "shapes": [
                    {
                        "kind": "box",
                        "bone_name": "sedanA",
                        "center": [1.0, -2.0, 3.0],
                        "axes": [
                            [1.0, 0.0, 0.0],
                            [0.0, -1.0, 0.0],
                            [0.0, 0.0, -1.0],
                        ],
                        "extents": [4.0, 6.0, 8.0],
                    },
                    {
                        "kind": "sphere",
                        "bone_name": "w0",
                        "center": [0.25, 0.5, 0.75],
                        "radius": 0.4,
                    },
                ],
            }],
            "blockers": [{
                "package_id": "extracted-art-cars-sedana",
                "source_fbx": _SOURCE_FBX,
                "rig_identity": "blocked-rig",
                "primitive_count": 1,
                "blockers": [
                    "source-cylinder-has-no-exact-aggregate-geometry"
                ],
            }],
        },
    }


def _skeletal_step() -> NativeImportStep:
    package = "/Game/Generated/SHAR/cars/extracted_art_cars_sedana"
    asset = "extracted_art_cars_sedana"
    return NativeImportStep(
        operation_id="operation-0000000000000001",
        route_id="vehicle-skeletal-mesh-fbx-v1",
        source_path=_SOURCE_FBX,
        source_revision="1" * 64,
        destination=f"{package}.{asset}",
        target_class="SkeletalMesh",
        package_path=package,
        folder_path="/Game/Generated/SHAR/cars",
        asset_name=asset,
        toolset_name="SharImportEditor.SharImportToolset",
        tool_name=(
            "SharImportEditor.SharImportToolset.ImportVehicleSkeletalMesh"
        ),
        external_payload_path=None,
    )


def _execution(*steps: NativeImportStep) -> CompiledExecutionPlan:
    report = PlanExecutionReport(
        bundle_revision="a" * 64,
        operation_count=len(steps),
        compiled_count=len(steps),
        semantic_blocker_count=0,
        blocked_readiness={},
        unsupported_routes={},
        route_counts={"vehicle-skeletal-mesh-fbx-v1": len(steps)},
    )
    return CompiledExecutionPlan(report, steps)


def _bundle(sidecar: bytes) -> ValidatedPlanBundle:
    artifact = SemanticArtifactSummary(
        "vehicle-physics",
        "vehicle-physics.json",
        hashlib.sha256(sidecar).hexdigest(),
        len(sidecar),
    )
    report = PlanBundleReport(
        revision="a" * 64,
        source_manifest_revision="b" * 64,
        engine_contract_revision="shar-unreal-porting-contract-v1",
        target_engine_version="5.8.1",
        target_platform="editor",
        semantic_blocker_count=0,
        operation_count=0,
        readiness_counts={},
        plans=(),
        semantic_artifacts=(artifact,),
    )
    return ValidatedPlanBundle(report, ())


def test_compiles_exact_vehicle_physics_wire_from_skeletal_join() -> None:
    compiled = compile_vehicle_physics_construction(
        _document(), _execution(_skeletal_step())
    )
    assert compiled.report.to_json() == {
        "blockedRigCount": 1,
        "requestCount": 1,
        "shapeCount": 2,
    }
    step = compiled.requests[0]
    assert step.skeletal_mesh_path == _skeletal_step().destination
    assert step.target_class == "PhysicsAsset"
    assert step.folder_path == "/Game/Generated/SHAR/VehiclePhysics"
    assert step.asset_name.startswith("PHYS_")
    arguments = step.arguments()
    assert arguments["rigIdentity"] == "sedanA"
    assert arguments["sourceJointCount"] == 19
    shapes = arguments["shapes"]
    assert isinstance(shapes, list)
    assert shapes[0] == {
        "axisX": {"x": 1.0, "y": 0.0, "z": 0.0},
        "axisY": {"x": 0.0, "y": -1.0, "z": 0.0},
        "axisZ": {"x": 0.0, "y": 0.0, "z": -1.0},
        "boneName": "sedanA",
        "boxExtents": {"x": 4.0, "y": 6.0, "z": 8.0},
        "center": {"x": 1.0, "y": -2.0, "z": 3.0},
        "kind": "Box",
        "radius": 0.0,
    }
    assert shapes[1]["kind"] == "Sphere"
    assert shapes[1]["boneName"] == "w0"
    assert shapes[1]["radius"] == pytest.approx(0.4)


def test_compile_rejects_missing_and_ambiguous_skeletal_join() -> None:
    with pytest.raises(ProtocolError, match="has no skeletal import"):
        compile_vehicle_physics_construction(_document(), _execution())
    first = _skeletal_step()
    second = first._replace(operation_id="operation-0000000000000002")
    with pytest.raises(ProtocolError, match="import is ambiguous"):
        compile_vehicle_physics_construction(
            _document(), _execution(first, second)
        )


def test_rejects_generic_skeletal_route_for_vehicle_physics() -> None:
    generic = _skeletal_step()._replace(
        route_id="skeletal-mesh-fbx-v1",
        tool_name="SharImportEditor.SharImportToolset.ImportSkeletalMesh",
    )
    with pytest.raises(ProtocolError, match="no skeletal import"):
        compile_vehicle_physics_construction(_document(), _execution(generic))


def test_compile_rejects_policy_and_geometry_drift() -> None:
    document = _document()
    document["target_policy"]["local_axis_conversion"] = "none"
    with pytest.raises(ProtocolError, match="target policy drifted"):
        compile_vehicle_physics_construction(
            document, _execution(_skeletal_step())
        )
    document = _document()
    document["target_policy"]["rig_binding_policy"] = "allow-auxiliary-rigs"
    with pytest.raises(ProtocolError, match="target policy drifted"):
        compile_vehicle_physics_construction(
            document, _execution(_skeletal_step())
        )
    document = _document()
    shape = document["native_construction"]["requests"][0]["shapes"][0]
    shape["axes"][2] = [0.0, -1.0, 0.0]
    with pytest.raises(ProtocolError, match="basis is not orthogonal"):
        compile_vehicle_physics_construction(
            document, _execution(_skeletal_step())
        )


def test_compile_rejects_previous_vehicle_catalog_source_schema() -> None:
    document = _document()
    document["source_schema"] = "shar.vehicle-catalog.v7"
    with pytest.raises(ProtocolError, match="source schema is not supported"):
        compile_vehicle_physics_construction(
            document,
            _execution(_skeletal_step()),
        )


def test_bound_reader_rechecks_release_index_hash(tmp_path: Path) -> None:
    sidecar = json.dumps(_document(), sort_keys=True).encode()
    path = tmp_path / "vehicle-physics.json"
    path.write_bytes(sidecar)
    bundle = _bundle(sidecar)
    document = read_bound_vehicle_physics_document(tmp_path, bundle)
    assert document["schema"].endswith(".v1")
    path.write_bytes(sidecar + b"\n")
    with pytest.raises(ProtocolError, match="byte count is stale"):
        read_bound_vehicle_physics_document(tmp_path, bundle)
