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
#   - Exact package-scoped vehicle material slot binding compilation.
# - Must-Not:
#   - Read files, contact Unreal Editor, mutate assets, or create materials.
# - Allows:
#   - Join a selected material package to its exact planned Skeletal Mesh.
# - Split-When:
#   - Another presentation binding family gains independent semantics.
# - Merge-When:
#   - Material selection owns identical Skeletal Mesh join guarantees.
# - Summary:
#   - Vehicle material slot binding compiler.
# - Description:
#   - Produces one deterministic CAS request from selected slots and imports.
# - Usage:
#   - Called after global material and plan compilation and package selection.
# - Defaults:
#   - Missing, ambiguous, duplicate, or non-skeletal joins fail closed.
#

"""Compile exact vehicle material slot bindings to a planned Skeletal Mesh."""

from __future__ import annotations

import hashlib
import json
from typing import NamedTuple

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.plan_execution import CompiledExecutionPlan
from mcp.domain.plan_execution import NativeImportStep
from mcp.domain.vehicle_material_selection import (
    CompiledVehicleMaterialSelection,
)

_MATERIAL_TOOLSET = "SharImportEditor.SharVehicleMaterialToolset"
_READ_TOOL = f"{_MATERIAL_TOOLSET}.ReadVehicleMaterialSlots"
_CAS_TOOL = f"{_MATERIAL_TOOLSET}.CompareExchangeVehicleMaterialSlots"


class VehicleMaterialSlotBindingReport(NamedTuple):
    """Public-safe coverage for one exact vehicle material binding."""

    package_id: str
    slot_count: int
    slot_indices: tuple[int, ...]

    def to_json(self) -> JsonObject:
        """Render binding coverage without asset paths."""
        return {
            "packageId": self.package_id,
            "slotCount": self.slot_count,
            "slotIndices": list(self.slot_indices),
        }


class CompiledVehicleMaterialSlotBinding(NamedTuple):
    """One package's deterministic Skeletal Mesh material slot CAS contract."""

    report: VehicleMaterialSlotBindingReport
    source_fbx: str
    skeletal_mesh_path: str
    skeletal_mesh_package_path: str
    slot_indices: tuple[int, ...]
    slot_names: tuple[str, ...]
    material_paths: tuple[str, ...]
    material_package_paths: tuple[str, ...]
    toolset_name: str = _MATERIAL_TOOLSET
    read_tool_name: str = _READ_TOOL
    compare_exchange_tool_name: str = _CAS_TOOL

    def read_arguments(self) -> JsonObject:
        """Build exact native arguments for selected slot read-back."""
        return {
            "expectedSlotNames": list(self.slot_names),
            "skeletalMeshPath": self.skeletal_mesh_path,
            "slotIndices": list(self.slot_indices),
        }

    def compare_exchange_arguments(
        self,
        expected_paths: tuple[str, ...],
        replacement_paths: tuple[str, ...],
    ) -> JsonObject:
        """Build one exact selected-slot compare-and-exchange request."""
        if len(expected_paths) != len(self.slot_indices):
            fail_protocol("vehicle material slot expected path count drifted")
        if len(replacement_paths) != len(self.slot_indices):
            fail_protocol("vehicle material slot replacement count drifted")
        return {
            "expectedMaterialPaths": list(expected_paths),
            "expectedSlotNames": list(self.slot_names),
            "replacementMaterialPaths": list(replacement_paths),
            "skeletalMeshPath": self.skeletal_mesh_path,
            "slotIndices": list(self.slot_indices),
        }


def compile_vehicle_material_slot_binding(
    selection: CompiledVehicleMaterialSelection,
    execution: CompiledExecutionPlan,
) -> CompiledVehicleMaterialSlotBinding:
    """Join one selected material package to its exact skeletal import."""
    skeletal = _resolve_skeletal_import(selection.source_fbx, execution)
    instances = tuple(
        sorted(selection.instances, key=lambda item: item.slot_index)
    )
    if not instances:
        fail_protocol("vehicle material slot binding has no selected slots")
    indices = tuple(item.slot_index for item in instances)
    names = tuple(item.slot_name for item in instances)
    material_paths = tuple(item.object_path for item in instances)
    material_packages = tuple(item.package_path for item in instances)
    if any(index < 0 for index in indices) or len(indices) != len(set(indices)):
        fail_protocol("vehicle material slot indices are not unique and valid")
    if any(not name for name in names) or len(names) != len(set(names)):
        fail_protocol("vehicle material slot names are not unique and nonempty")
    if len(material_paths) != len(set(material_paths)):
        fail_protocol("vehicle material slot instance paths are duplicated")
    if any(item.source_fbx != selection.source_fbx for item in instances):
        fail_protocol("vehicle material slot source ownership drifted")
    return CompiledVehicleMaterialSlotBinding(
        VehicleMaterialSlotBindingReport(
            selection.report.package_id,
            len(indices),
            indices,
        ),
        selection.source_fbx,
        skeletal.destination,
        skeletal.package_path,
        indices,
        names,
        material_paths,
        material_packages,
    )


def _resolve_skeletal_import(
    source_fbx: str,
    execution: CompiledExecutionPlan,
) -> NativeImportStep:
    matches = tuple(
        step
        for step in execution.imports
        if step.source_path == source_fbx
        and step.route_id == "vehicle-skeletal-mesh-fbx-v1"
    )
    if len(matches) != 1:
        fail_protocol("vehicle material Skeletal Mesh import join is not exact")
    step = matches[0]
    if step.target_class != "SkeletalMesh":
        fail_protocol("vehicle material slot target is not a SkeletalMesh")
    if not step.destination.startswith("/Game/Generated/SHAR/cars/"):
        fail_protocol("vehicle material slot target is outside generated cars")
    return step


def vehicle_material_slot_binding_revision(
    compiled: CompiledVehicleMaterialSlotBinding,
) -> str:
    """Hash the exact slot binding contract canonically."""
    encoded = json.dumps(
        {
            "material_paths": compiled.material_paths,
            "report": compiled.report._asdict(),
            "skeletal_mesh_path": compiled.skeletal_mesh_path,
            "slot_indices": compiled.slot_indices,
            "slot_names": compiled.slot_names,
            "source_fbx": compiled.source_fbx,
        },
        ensure_ascii=True,
        separators=(",", ":"),
        sort_keys=True,
    ).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()
