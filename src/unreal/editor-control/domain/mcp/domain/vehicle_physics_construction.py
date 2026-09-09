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
#   - Typed compilation of verified vehicle-physics construction evidence.
# - Must-Not:
#   - Read files, contact Unreal Editor, guess Skeletal Mesh paths, or unblock
#   - unsupported source geometry.
# - Allows:
#   - Join exact source FBX identity to reviewed skeletal import steps.
# - Split-When:
#   - Physics publication or presentation binding gains another lifecycle.
# - Merge-When:
#   - General construction compilation owns the same semantic join.
# - Summary:
#   - Vehicle-physics native construction compiler.
# - Description:
#   - Resolves source-bound rigs to exact generated Skeletal Mesh dependencies.
# - Usage:
#   - Called after plan execution and bound sidecar validation.
# - Defaults:
#   - Missing, ambiguous, malformed, or unsupported joins fail closed.
#

"""Typed compilation of verified vehicle-physics construction evidence."""

from __future__ import annotations

import hashlib
import json
import math
from typing import NamedTuple

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import JsonValue
from mcp.domain.json_types import require_json_object
from mcp.domain.plan_execution import CompiledExecutionPlan
from mcp.domain.plan_execution import NativeImportStep

_SCHEMA = "shar-schoenwald.unreal-vehicle-physics-evidence.v1"
_SOURCE_SCHEMA = "shar.vehicle-catalog.v7"
_TOOLSET = "SharImportEditor.SharVehiclePhysicsToolset"
_TOOL = f"{_TOOLSET}.CreateVehiclePhysicsAsset"
_OUTPUT_ROOT = "/Game/Generated/SHAR/VehiclePhysics"
_EXPECTED_POLICY = {
    "box_extent_policy": "source-half-to-native-full",
    "local_axis_conversion": "reflect-y",
    "native_dimension_policy": "retain-source-magnitude-under-bone-scale",
    "skeletal_import_unit_policy": "scene-unit-converted",
    "source_coordinate_space": "source-bone-local",
    "source_unit": "meter",
    "unsupported_shape_policy": "block-rig",
}


class VehiclePhysicsShape(NamedTuple):
    """One already projected native analytic shape."""

    kind: str
    bone_name: str
    center: tuple[float, float, float]
    radius: float
    axes: tuple[tuple[float, float, float], ...]
    extents: tuple[float, float, float]

    def to_wire(self) -> JsonObject:
        """Render one reflected native-struct input for the toolset."""
        axis_x, axis_y, axis_z = self.axes or (
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
        )
        return {
            "axisX": _vector(axis_x),
            "axisY": _vector(axis_y),
            "axisZ": _vector(axis_z),
            "boneName": self.bone_name,
            "boxExtents": _vector(self.extents),
            "center": _vector(self.center),
            "kind": "Box" if self.kind == "box" else "Sphere",
            "radius": self.radius,
        }


class VehiclePhysicsConstructionStep(NamedTuple):
    """One source rig joined to its exact generated Skeletal Mesh."""

    package_id: str
    source_fbx: str
    rig_identity: str
    source_joint_count: int
    skeletal_mesh_path: str
    folder_path: str
    asset_name: str
    package_path: str
    object_path: str
    shapes: tuple[VehiclePhysicsShape, ...]

    @property
    def target_class(self) -> str:
        """Expected published Unreal class."""
        return "PhysicsAsset"

    @property
    def toolset_name(self) -> str:
        """Native publication toolset identity."""
        return _TOOLSET

    @property
    def tool_name(self) -> str:
        """Native publication tool identity."""
        return _TOOL

    def arguments(self) -> JsonObject:
        """Build exact native publication arguments."""
        return {
            "assetName": self.asset_name,
            "folderPath": self.folder_path,
            "rigIdentity": self.rig_identity,
            "shapes": [shape.to_wire() for shape in self.shapes],
            "skeletalMeshPath": self.skeletal_mesh_path,
            "sourceJointCount": self.source_joint_count,
        }


class VehiclePhysicsConstructionReport(NamedTuple):
    """Public-safe compilation counts for vehicle Physics Assets."""

    request_count: int
    blocked_rig_count: int
    shape_count: int

    def to_json(self) -> JsonObject:
        """Render construction readiness counts."""
        return {
            "blockedRigCount": self.blocked_rig_count,
            "requestCount": self.request_count,
            "shapeCount": self.shape_count,
        }


class CompiledVehiclePhysicsConstruction(NamedTuple):
    """Typed source-bound Physics Asset publication requests."""

    report: VehiclePhysicsConstructionReport
    requests: tuple[VehiclePhysicsConstructionStep, ...]


def compile_vehicle_physics_construction(
    document: JsonObject,
    execution: CompiledExecutionPlan,
) -> CompiledVehiclePhysicsConstruction:
    """Validate v1 evidence and join each ready rig to one skeletal import."""
    if document.get("schema") != _SCHEMA:
        fail_protocol("vehicle-physics construction schema is not supported")
    if document.get("source_schema") != _SOURCE_SCHEMA:
        fail_protocol("vehicle-physics source schema is not supported")
    policy = require_json_object(
        document.get("target_policy"), context="vehicle-physics target policy"
    )
    if policy != _EXPECTED_POLICY:
        fail_protocol("vehicle-physics target policy drifted")
    counts = require_json_object(
        document.get("counts"), context="vehicle-physics counts"
    )
    native = require_json_object(
        document.get("native_construction"),
        context="vehicle-physics native construction",
    )
    raw_requests = _array(native, "requests")
    raw_blockers = _array(native, "blockers")
    imports = _skeletal_imports_by_source(execution)
    requests = tuple(_request(value, imports) for value in raw_requests)
    _require_unique_requests(requests)
    shape_count = sum(len(item.shapes) for item in requests)
    expected_requests = _integer(counts, "native_ready_rigs")
    expected_blockers = _integer(counts, "native_blocked_rigs")
    expected_shapes = _integer(counts, "native_shapes")
    if (
        expected_requests != len(requests)
        or expected_blockers != len(raw_blockers)
        or expected_shapes != shape_count
        or _integer(counts, "rigs") != expected_requests + expected_blockers
    ):
        fail_protocol(
            "vehicle-physics native construction counts are inconsistent"
        )
    return CompiledVehiclePhysicsConstruction(
        VehiclePhysicsConstructionReport(
            len(requests), len(raw_blockers), shape_count
        ),
        requests,
    )


def vehicle_physics_construction_revision(
    compiled: CompiledVehiclePhysicsConstruction,
) -> str:
    """Hash the exact typed requests canonically."""
    encoded = json.dumps(
        {
            "report": compiled.report._asdict(),
            "requests": [item._asdict() for item in compiled.requests],
        },
        ensure_ascii=True,
        separators=(",", ":"),
        sort_keys=True,
    ).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def _skeletal_imports_by_source(
    execution: CompiledExecutionPlan,
) -> dict[str, NativeImportStep]:
    matches: dict[str, NativeImportStep] = {}
    for step in execution.imports:
        if step.route_id != "skeletal-mesh-fbx-v1":
            continue
        if step.source_path in matches:
            fail_protocol("vehicle-physics source FBX import is ambiguous")
        matches[step.source_path] = step
    return matches


def _request(
    value: JsonValue,
    imports: dict[str, NativeImportStep],
) -> VehiclePhysicsConstructionStep:
    row = require_json_object(value, context="vehicle-physics request")
    package_id = _text(row, "package_id")
    source_fbx = _text(row, "source_fbx")
    rig_identity = _text(row, "rig_identity")
    source_joint_count = _integer(row, "joint_count")
    if source_joint_count <= 0:
        fail_protocol("vehicle-physics source joint count is not positive")
    skeletal = imports.get(source_fbx)
    if skeletal is None:
        fail_protocol("vehicle-physics source FBX has no skeletal import")
    if skeletal.target_class != "SkeletalMesh":
        fail_protocol(
            "vehicle-physics source FBX did not resolve to SkeletalMesh"
        )
    shapes = tuple(_shape(item) for item in _array(row, "shapes"))
    if not shapes:
        fail_protocol("vehicle-physics request has no native shapes")
    digest = hashlib.sha256(
        f"{package_id}\0{rig_identity}".encode()
    ).hexdigest()[:24]
    asset_name = f"PHYS_{digest}"
    package_path = f"{_OUTPUT_ROOT}/{asset_name}"
    return VehiclePhysicsConstructionStep(
        package_id=package_id,
        source_fbx=source_fbx,
        rig_identity=rig_identity,
        source_joint_count=source_joint_count,
        skeletal_mesh_path=skeletal.destination,
        folder_path=_OUTPUT_ROOT,
        asset_name=asset_name,
        package_path=package_path,
        object_path=f"{package_path}.{asset_name}",
        shapes=shapes,
    )


def _shape(value: JsonValue) -> VehiclePhysicsShape:
    row = require_json_object(value, context="vehicle-physics shape")
    kind = _text(row, "kind")
    bone = _text(row, "bone_name")
    center = _vec3(row.get("center"), "vehicle-physics center")
    if kind == "sphere":
        radius = _positive_float(row.get("radius"), "vehicle-physics radius")
        return VehiclePhysicsShape(
            kind, bone, center, radius, (), (0.0, 0.0, 0.0)
        )
    if kind == "box":
        axes_value = row.get("axes")
        if not isinstance(axes_value, list) or len(axes_value) != 3:
            fail_protocol("vehicle-physics box axes are not a triad")
        axes = tuple(
            _vec3(axis, "vehicle-physics box axis") for axis in axes_value
        )
        _require_basis(axes)
        extents = _vec3(row.get("extents"), "vehicle-physics box extents")
        if any(value <= 0.0 for value in extents):
            fail_protocol("vehicle-physics box extents are not positive")
        return VehiclePhysicsShape(kind, bone, center, 0.0, axes, extents)
    return fail_protocol("vehicle-physics shape kind is unsupported")


def _require_basis(axes: tuple[tuple[float, float, float], ...]) -> None:
    tolerance = 1.0e-5

    def dot(left: tuple[float, ...], right: tuple[float, ...]) -> float:
        return sum(a * b for a, b in zip(left, right, strict=True))

    for axis in axes:
        if abs(dot(axis, axis) - 1.0) > tolerance:
            fail_protocol("vehicle-physics box basis is not normalized")
    pairs = ((axes[0], axes[1]), (axes[0], axes[2]), (axes[1], axes[2]))
    for left, right in pairs:
        if abs(dot(left, right)) > tolerance:
            fail_protocol("vehicle-physics box basis is not orthogonal")


def _require_unique_requests(
    requests: tuple[VehiclePhysicsConstructionStep, ...],
) -> None:
    identities = [(item.package_id, item.rig_identity) for item in requests]
    paths = [item.object_path for item in requests]
    if len(identities) != len(set(identities)):
        fail_protocol("vehicle-physics native rig identities are not unique")
    if len(paths) != len(set(paths)):
        fail_protocol("vehicle-physics native destinations are not unique")


def _array(row: JsonObject, field: str) -> list[JsonValue]:
    value = row.get(field)
    if not isinstance(value, list):
        fail_protocol(f"vehicle-physics {field} is not an array")
    return value


def _text(row: JsonObject, field: str) -> str:
    value = row.get(field)
    if not isinstance(value, str) or not value or value.strip() != value:
        fail_protocol(f"vehicle-physics {field} is not canonical text")
    return value


def _integer(row: JsonObject, field: str) -> int:
    value = row.get(field)
    if isinstance(value, bool) or not isinstance(value, int) or value < 0:
        fail_protocol(f"vehicle-physics {field} is not a nonnegative integer")
    return value


def _positive_float(value: JsonValue | None, context: str) -> float:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        fail_protocol(f"{context} is not numeric")
    result = float(value)
    if not math.isfinite(result) or result <= 0.0:
        fail_protocol(f"{context} is not finite and positive")
    return result


def _vec3(value: JsonValue | None, context: str) -> tuple[float, float, float]:
    if not isinstance(value, list) or len(value) != 3:
        fail_protocol(f"{context} is not a vector3")
    result: list[float] = []
    for component in value:
        if (
            isinstance(component, bool)
            or not isinstance(component, (int, float))
        ):
            fail_protocol(f"{context} has a nonnumeric component")
        number = float(component)
        if not math.isfinite(number):
            fail_protocol(f"{context} has a non-finite component")
        result.append(number)
    return result[0], result[1], result[2]


def _vector(value: tuple[float, float, float]) -> JsonObject:
    return {"x": value[0], "y": value[1], "z": value[2]}
