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
#   - Vehicle-material package selection and dependency-closure tests.
# - Must-Not:
#   - Read generated caches, contact Unreal Editor, or assign mesh slots.
# - Allows:
#   - Synthetic shared masters/textures and exact package identities.
# - Split-When:
#   - CLI option parsing or native application gains selection-specific policy.
# - Merge-When:
#   - Construction tests own exact package closure semantics.
# - Summary:
#   - Vehicle-material package selection tests.
# - Description:
#   - Proves exact instance selection and dependency closure after full compile.
# - Usage:
#   - Run through repository Python validation.
# - Defaults:
#   - Unknown or source-ambiguous package selections fail closed.
#

"""Tests for exact vehicle-material package selection."""

from __future__ import annotations

import hashlib

from mcp.domain.errors import ProtocolError
from mcp.domain.vehicle_material_construction import (
    CompiledVehicleMaterialConstruction,
)
from mcp.domain.vehicle_material_construction import (
    VehicleMaterialConstructionReport,
)
from mcp.domain.vehicle_material_construction import VehicleMaterialInstanceStep
from mcp.domain.vehicle_material_construction import VehicleMaterialMasterStep
from mcp.domain.vehicle_material_construction import VehicleMaterialTextureStep
from mcp.domain.vehicle_material_selection import (
    select_vehicle_material_package,
)
import pytest

_TEXTURE_ROOT = "/Game/Generated/SHAR/Textures/Vehicles"
_MASTER_ROOT = "/Game/Generated/SHAR/Materials/Vehicles/Masters"
_INSTANCE_ROOT = "/Game/Generated/SHAR/Materials/Vehicles/Instances"
_RECIPE_SHARED = "simple-unlit-blend-additive-alpha-test-off-one-sided"
_RECIPE_OTHER = "simple-unlit-blend-additive-alpha-test-off-both-faces"


def _texture(digit: str) -> VehicleMaterialTextureStep:
    digest = digit * 64
    name = f"T_Vehicle_{digest}"
    package = f"{_TEXTURE_ROOT}/{name}"
    return VehicleMaterialTextureStep(
        sha256=digest,
        byte_count=3,
        source_path=f"vehicle-assets/car{digit}/textures/base.png",
        object_path=f"{package}.{name}",
        package_path=package,
        folder_path=_TEXTURE_ROOT,
        asset_name=name,
    )


def _master(
    recipe: str,
    name: str,
    *,
    two_sided: bool,
) -> VehicleMaterialMasterStep:
    package = f"{_MASTER_ROOT}/{name}"
    return VehicleMaterialMasterStep(
        recipe_identity=recipe,
        object_path=f"{package}.{name}",
        package_path=package,
        folder_path=_MASTER_ROOT,
        asset_name=name,
        blend_mode=2,
        alpha_test=False,
        two_sided=two_sided,
    )


def _instance(
    package_id: str,
    *,
    source_fbx: str,
    slot_index: int,
    recipe: str,
    texture: VehicleMaterialTextureStep,
    master: VehicleMaterialMasterStep,
) -> VehicleMaterialInstanceStep:
    request = hashlib.sha256(
        f"{package_id}\0{slot_index}".encode()
    ).hexdigest()
    name = f"MI_Vehicle_{request}"
    package = f"{_INSTANCE_ROOT}/{name}"
    return VehicleMaterialInstanceStep(
        request_identity=request,
        package_id=package_id,
        source_fbx=source_fbx,
        slot_index=slot_index,
        slot_name=f"slot-{slot_index}",
        source_material_name=f"material-{slot_index}",
        recipe_identity=recipe,
        texture_sha256=texture.sha256,
        object_path=f"{package}.{name}",
        package_path=package,
        folder_path=_INSTANCE_ROOT,
        asset_name=name,
        parent_material_path=master.object_path,
        base_color_texture_path=texture.object_path,
        base_color_tint=(1.0, 1.0, 1.0, 1.0),
        set_alpha_reference=False,
        alpha_reference=None,
    )


def _compiled() -> CompiledVehicleMaterialConstruction:
    shared_texture = _texture("1")
    other_texture = _texture("2")
    shared_master = _master(
        _RECIPE_SHARED,
        "M_SHAR_Vehicle_SimpleUnlit_Additive_AlphaTestOff_OneSided",
        two_sided=False,
    )
    other_master = _master(
        _RECIPE_OTHER,
        "M_SHAR_Vehicle_SimpleUnlit_Additive_AlphaTestOff_TwoSided",
        two_sided=True,
    )
    instances = (
        _instance(
            "extracted-art-cars-sedana",
            source_fbx="vehicle-assets/sedana/sedana.fbx",
            slot_index=0,
            recipe=_RECIPE_SHARED,
            texture=shared_texture,
            master=shared_master,
        ),
        _instance(
            "extracted-art-cars-sedana",
            source_fbx="vehicle-assets/sedana/sedana.fbx",
            slot_index=7,
            recipe=_RECIPE_OTHER,
            texture=other_texture,
            master=other_master,
        ),
        _instance(
            "extracted-art-cars-other",
            source_fbx="vehicle-assets/other/other.fbx",
            slot_index=1,
            recipe=_RECIPE_SHARED,
            texture=shared_texture,
            master=shared_master,
        ),
    )
    return CompiledVehicleMaterialConstruction(
        VehicleMaterialConstructionReport(2, 2, 3, 16),
        (shared_texture, other_texture),
        (shared_master, other_master),
        instances,
    )


def test_selects_exact_package_and_dependency_closure() -> None:
    selected = select_vehicle_material_package(
        _compiled(), "extracted-art-cars-sedana"
    )
    assert selected.report.to_json() == {
        "instanceCount": 2,
        "masterCount": 2,
        "packageId": "extracted-art-cars-sedana",
        "textureCount": 2,
    }
    assert selected.source_fbx == "vehicle-assets/sedana/sedana.fbx"
    assert tuple(item.slot_index for item in selected.instances) == (0, 7)
    assert tuple(item.sha256 for item in selected.textures) == (
        "1" * 64,
        "2" * 64,
    )


def test_shared_dependency_is_retained_once_for_other_package() -> None:
    selected = select_vehicle_material_package(
        _compiled(), "extracted-art-cars-other"
    )
    assert selected.report.texture_count == 1
    assert selected.report.master_count == 1
    assert selected.report.instance_count == 1
    assert selected.textures[0].sha256 == "1" * 64
    assert selected.masters[0].recipe_identity == _RECIPE_SHARED


def test_rejects_unknown_and_noncanonical_package_ids() -> None:
    compiled = _compiled()
    with pytest.raises(ProtocolError, match="has no construction-ready slots"):
        select_vehicle_material_package(compiled, "extracted-art-cars-missing")
    for package_id in ("Sedana", "extracted--cars", "extracted-cars-"):
        with pytest.raises(ProtocolError, match="package id is not canonical"):
            select_vehicle_material_package(compiled, package_id)


def test_rejects_ambiguous_source_fbx_for_one_package() -> None:
    compiled = _compiled()
    first = compiled.instances[0]
    second = compiled.instances[1]._replace(
        source_fbx="vehicle-assets/bad/bad.fbx"
    )
    drifted = compiled._replace(
        instances=(first, second, compiled.instances[2])
    )
    with pytest.raises(
        ProtocolError, match="source FBX ownership is ambiguous"
    ):
        select_vehicle_material_package(drifted, "extracted-art-cars-sedana")
