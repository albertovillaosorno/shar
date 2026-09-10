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
#   - Exact Skeletal Mesh import prerequisites for ready vehicle physics rigs.
# - Must-Not:
#   - Read files, contact Unreal Editor, import blocked-only vehicles, or weaken
#   - global plan completeness.
# - Allows:
#   - Select reviewed vehicle skeletal imports already required by ready rigs.
# - Split-When:
#   - Another construction family needs independent prerequisite compilation.
# - Merge-When:
#   - General construction orchestration owns identical prerequisite selection.
# - Summary:
#   - Vehicle-physics skeletal prerequisite compiler.
# - Description:
#   - Deduplicates exact vehicle FBX dependencies of native-ready physics rigs.
# - Usage:
#   - Called after vehicle physics construction compilation and plan execution.
# - Defaults:
#   - Missing, ambiguous, mismatched, or non-vehicle imports fail closed.
#

"""Compile exact Skeletal Mesh prerequisites for ready vehicle physics rigs."""

from __future__ import annotations

import hashlib
import json
from typing import NamedTuple

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.plan_execution import CompiledExecutionPlan
from mcp.domain.plan_execution import NativeImportStep
from mcp.domain.vehicle_physics_construction import (
    CompiledVehiclePhysicsConstruction,
)


class VehiclePhysicsPrerequisiteReport(NamedTuple):
    """Public-safe counts for exact vehicle skeletal prerequisite imports."""

    import_count: int
    ready_rig_count: int
    source_count: int

    def to_json(self) -> JsonObject:
        """Render prerequisite coverage counts."""
        return {
            "importCount": self.import_count,
            "readyRigCount": self.ready_rig_count,
            "sourceCount": self.source_count,
        }


class CompiledVehiclePhysicsPrerequisites(NamedTuple):
    """Selected vehicle Skeletal Mesh imports required by ready physics rigs."""

    report: VehiclePhysicsPrerequisiteReport
    imports: tuple[NativeImportStep, ...]


class VehiclePhysicsPrerequisiteSelectionReport(NamedTuple):
    """Public-safe coverage for one exact vehicle package prerequisite scope."""

    package_id: str
    import_count: int
    ready_rig_count: int
    source_count: int

    def to_json(self) -> JsonObject:
        """Render one scoped prerequisite selection."""
        return {
            "importCount": self.import_count,
            "packageId": self.package_id,
            "readyRigCount": self.ready_rig_count,
            "sourceCount": self.source_count,
        }


class CompiledVehiclePhysicsPrerequisiteSelection(NamedTuple):
    """Executable prerequisite imports selected for one vehicle package."""

    report: VehiclePhysicsPrerequisiteSelectionReport
    imports: tuple[NativeImportStep, ...]


VehiclePhysicsPrerequisiteExecutable = (
    CompiledVehiclePhysicsPrerequisites
    | CompiledVehiclePhysicsPrerequisiteSelection
)


def compile_vehicle_physics_prerequisites(
    construction: CompiledVehiclePhysicsConstruction,
    execution: CompiledExecutionPlan,
) -> CompiledVehiclePhysicsPrerequisites:
    """Select deduplicated skeletal imports needed by ready physics rigs."""
    required_sources = {request.source_fbx for request in construction.requests}
    _require_vehicle_sources(required_sources)
    skeletal_by_source = _skeletal_imports(required_sources, execution)
    _require_request_joins(construction, skeletal_by_source)
    imports = tuple(
        step
        for step in execution.imports
        if step.source_path in required_sources
        and step.route_id == "skeletal-mesh-fbx-v1"
    )
    _require_unique_imports(imports, required_sources)
    return CompiledVehiclePhysicsPrerequisites(
        VehiclePhysicsPrerequisiteReport(
            import_count=len(imports),
            ready_rig_count=len(construction.requests),
            source_count=len(required_sources),
        ),
        imports,
    )


def select_vehicle_physics_prerequisite_package(
    construction: CompiledVehiclePhysicsConstruction,
    prerequisites: CompiledVehiclePhysicsPrerequisites,
    package_id: str,
) -> CompiledVehiclePhysicsPrerequisiteSelection:
    """Select one package after global prerequisite compilation succeeds."""
    requests = tuple(
        request
        for request in construction.requests
        if request.package_id == package_id
    )
    if not requests:
        fail_protocol(
            "vehicle-physics prerequisite package has no native-ready rigs"
        )
    required_sources = {request.source_fbx for request in requests}
    imports = tuple(
        step
        for step in prerequisites.imports
        if step.source_path in required_sources
    )
    _require_unique_imports(imports, required_sources)
    by_source = {step.source_path: step for step in imports}
    for request in requests:
        destination = by_source[request.source_fbx].destination
        if destination != request.skeletal_mesh_path:
            fail_protocol(
                "vehicle-physics prerequisite scoped destination "
                "does not match rig"
            )
    return CompiledVehiclePhysicsPrerequisiteSelection(
        VehiclePhysicsPrerequisiteSelectionReport(
            package_id=package_id,
            import_count=len(imports),
            ready_rig_count=len(requests),
            source_count=len(required_sources),
        ),
        imports,
    )


def _require_vehicle_sources(required_sources: set[str]) -> None:
    if not required_sources:
        fail_protocol("vehicle-physics prerequisite source set is empty")
    if any(
        not source.startswith("vehicle-assets/")
        for source in required_sources
    ):
        fail_protocol(
            "vehicle-physics prerequisite source is not a vehicle FBX"
        )


def _skeletal_imports(
    required_sources: set[str],
    execution: CompiledExecutionPlan,
) -> dict[str, NativeImportStep]:
    matches: dict[str, NativeImportStep] = {}
    for step in execution.imports:
        if step.route_id != "skeletal-mesh-fbx-v1":
            continue
        if step.source_path not in required_sources:
            continue
        if step.source_path in matches:
            fail_protocol(
                "vehicle-physics prerequisite skeletal import is ambiguous"
            )
        matches[step.source_path] = step
    if set(matches) != required_sources:
        fail_protocol("vehicle-physics prerequisite skeletal import is missing")
    return matches


def _require_request_joins(
    construction: CompiledVehiclePhysicsConstruction,
    skeletal_by_source: dict[str, NativeImportStep],
) -> None:
    for request in construction.requests:
        step = skeletal_by_source[request.source_fbx]
        if step.target_class != "SkeletalMesh":
            fail_protocol(
                "vehicle-physics prerequisite target is not SkeletalMesh"
            )
        if step.destination != request.skeletal_mesh_path:
            fail_protocol(
                "vehicle-physics prerequisite destination does not match rig"
            )


def _require_unique_imports(
    imports: tuple[NativeImportStep, ...],
    required_sources: set[str],
) -> None:
    operation_ids = [step.operation_id for step in imports]
    destinations = [step.destination for step in imports]
    if len(operation_ids) != len(set(operation_ids)):
        fail_protocol(
            "vehicle-physics prerequisite operation identity is duplicated"
        )
    if len(destinations) != len(set(destinations)):
        fail_protocol("vehicle-physics prerequisite destination is duplicated")
    if len(imports) != len(required_sources):
        fail_protocol(
            "vehicle-physics prerequisite import cardinality is inconsistent"
        )


def vehicle_physics_prerequisite_revision(
    compiled: VehiclePhysicsPrerequisiteExecutable,
) -> str:
    """Hash the exact selected prerequisite import contract canonically."""
    encoded = json.dumps(
        {
            "report": compiled.report._asdict(),
            "imports": [
                {
                    "destination": step.destination,
                    "expected_object_paths": step.expected_object_paths,
                    "operation_id": step.operation_id,
                    "route_id": step.route_id,
                    "source_path": step.source_path,
                    "source_revision": step.source_revision,
                }
                for step in compiled.imports
            ],
        },
        ensure_ascii=True,
        separators=(",", ":"),
        sort_keys=True,
    ).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()
