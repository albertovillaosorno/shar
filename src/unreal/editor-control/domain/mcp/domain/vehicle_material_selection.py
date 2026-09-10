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
#   - Called after release-bound v3 compilation and before source verification.
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
from mcp.domain.vehicle_material_construction import VehicleMaterialInstanceStep
from mcp.domain.vehicle_material_construction import VehicleMaterialMasterStep
from mcp.domain.vehicle_material_construction import VehicleMaterialTextureStep

_PACKAGE_ID = re.compile(r"^[a-z0-9](?:[a-z0-9]|-(?=[a-z0-9]))*$")


class VehicleMaterialSelectionReport(NamedTuple):
    """Public-safe exact package selection and dependency counts."""

    package_id: str
    texture_count: int
    master_count: int
    instance_count: int

    def to_json(self) -> JsonObject:
        """Render package identity and scoped request counts."""
        return {
            "instanceCount": self.instance_count,
            "masterCount": self.master_count,
            "packageId": self.package_id,
            "textureCount": self.texture_count,
        }


class CompiledVehicleMaterialSelection(NamedTuple):
    """One package's exact native material dependency closure."""

    report: VehicleMaterialSelectionReport
    source_fbx: str
    textures: tuple[VehicleMaterialTextureStep, ...]
    masters: tuple[VehicleMaterialMasterStep, ...]
    instances: tuple[VehicleMaterialInstanceStep, ...]


type VehicleMaterialExecutable = (
    CompiledVehicleMaterialConstruction | CompiledVehicleMaterialSelection
)


def select_vehicle_material_package(
    compiled: CompiledVehicleMaterialConstruction,
    package_id: str,
) -> CompiledVehicleMaterialSelection:
    """Select one package and every exact native dependency it references."""
    if not _canonical_package_id(package_id):
        fail_protocol("vehicle material package id is not canonical")
    instances = tuple(
        item for item in compiled.instances if item.package_id == package_id
    )
    if not instances:
        fail_protocol(
            "vehicle material package has no construction-ready slots"
        )
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
        ),
        next(iter(source_fbx_values)),
        textures,
        masters,
        instances,
    )


def _canonical_package_id(value: str) -> bool:
    return _PACKAGE_ID.fullmatch(value) is not None
