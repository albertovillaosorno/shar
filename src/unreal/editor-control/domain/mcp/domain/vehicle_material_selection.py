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
#   - Exact package-scoped closure over compiled vehicle material requests.
# - Must-Not:
#   - Parse CLI input, read files, contact Unreal Editor, or assign mesh slots.
# - Allows:
#   - Select instances by canonical package id and retain exact dependencies.
# - Split-When:
#   - Another selector gains independent dependency or readiness semantics.
# - Merge-When:
#   - Construction compilation owns identical package-scoping guarantees.
# - Summary:
#   - Vehicle-material package selection policy.
# - Description:
#   - Derives one executable texture/master/instance closure after full compile.
# - Usage:
#   - Called after release-bound material compilation and before source
#   - verification.
# - Defaults:
#   - Unknown package ids and ambiguous source-FBX ownership fail closed.
#

"""Exact package-scoped closure over compiled vehicle material requests."""

from __future__ import annotations

import re
from typing import NamedTuple

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.vehicle_material_construction import (
    CompiledVehicleMaterialConstruction,
)
from mcp.domain.vehicle_material_construction import (
    VehicleMaterialAnyMasterStep,
)
from mcp.domain.vehicle_material_construction import VehicleMaterialInstanceStep
from mcp.domain.vehicle_material_construction import VehicleMaterialTextureStep

_PACKAGE_ID = re.compile(r"^[a-z0-9](?:[a-z0-9]|-(?=[a-z0-9]))*$")


class VehicleMaterialSelectionReport(NamedTuple):
    """Public-safe exact package selection and dependency counts."""

    package_id: str
    texture_count: int
    master_count: int
    instance_count: int
    slot_indices: tuple[int, ...]

    def to_json(self) -> JsonObject:
        """Render package identity and scoped request counts."""
        return {
            "instanceCount": self.instance_count,
            "masterCount": self.master_count,
            "packageId": self.package_id,
            "slotIndices": list(self.slot_indices),
            "textureCount": self.texture_count,
        }


class CompiledVehicleMaterialSelection(NamedTuple):
    """One package's exact native material dependency closure."""

    report: VehicleMaterialSelectionReport
    source_fbx: str
    textures: tuple[VehicleMaterialTextureStep, ...]
    masters: tuple[VehicleMaterialAnyMasterStep, ...]
    instances: tuple[VehicleMaterialInstanceStep, ...]


type VehicleMaterialExecutable = (
    CompiledVehicleMaterialConstruction | CompiledVehicleMaterialSelection
)


def select_vehicle_material_package(
    compiled: CompiledVehicleMaterialConstruction,
    package_id: str,
    slot_indices: tuple[int, ...] = (),
) -> CompiledVehicleMaterialSelection:
    """Select one package and an optional exact construction-ready slot set."""
    if not _canonical_package_id(package_id):
        fail_protocol("vehicle material package id is not canonical")
    package_instances = tuple(
        item for item in compiled.instances if item.package_id == package_id
    )
    if not package_instances:
        fail_protocol(
            "vehicle material package has no construction-ready slots"
        )
    by_slot: dict[int, VehicleMaterialInstanceStep] = {}
    for item in package_instances:
        if item.slot_index in by_slot:
            fail_protocol(
                "vehicle material package slot ownership is ambiguous"
            )
        by_slot[item.slot_index] = item
    if slot_indices:
        if tuple(sorted(set(slot_indices))) != slot_indices:
            fail_protocol("vehicle material slot selection is not canonical")
        missing = tuple(
            index for index in slot_indices if index not in by_slot
        )
        if missing:
            fail_protocol(
                "requested vehicle material slot is not construction-ready"
            )
        instances = tuple(by_slot[index] for index in slot_indices)
    else:
        instances = package_instances
    source_fbx_values = {item.source_fbx for item in instances}
    if len(source_fbx_values) != 1:
        fail_protocol(
            "vehicle material package source FBX ownership is ambiguous"
        )
    recipe_ids = {item.recipe_identity for item in instances}
    texture_ids = {
        item.texture_sha256
        for item in instances
        if item.texture_sha256 is not None
    }
    masters = tuple(
        item for item in compiled.masters if item.recipe_identity in recipe_ids
    )
    textures = tuple(
        item for item in compiled.textures if item.sha256 in texture_ids
    )
    if len(masters) != len(recipe_ids) or len(textures) != len(texture_ids):
        fail_protocol(
            "vehicle material package dependency closure is incomplete"
        )
    return CompiledVehicleMaterialSelection(
        VehicleMaterialSelectionReport(
            package_id=package_id,
            texture_count=len(textures),
            master_count=len(masters),
            instance_count=len(instances),
            slot_indices=tuple(sorted(item.slot_index for item in instances)),
        ),
        next(iter(source_fbx_values)),
        textures,
        masters,
        instances,
    )


def _canonical_package_id(value: str) -> bool:
    return _PACKAGE_ID.fullmatch(value) is not None
