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
#   - Vehicle-material native construction compiler tests.
# - Must-Not:
#   - Read generated caches, contact Unreal Editor, or assign Skeletal Mesh
#   - slots.
# - Allows:
#   - Synthetic v3 construction requests and fail-closed dependency drift.
# - Split-When:
#   - Filesystem source verification or application tests gain fixtures.
# - Merge-When:
#   - Another suite owns identical vehicle construction compilation coverage.
# - Summary:
#   - Vehicle-material construction compiler tests.
# - Description:
#   - Proves exact SHAR tool wires, generated roots, counts, and dependency
#   - joins.
# - Usage:
#   - Run through repository Python validation.
# - Defaults:
#   - Unplanned parent, texture, or unsupported raster state fails closed.
#

"""Tests for vehicle-material native construction compilation."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import struct

from mcp.adapter_outbound.vehicle_material_construction_reader import (
    read_bound_vehicle_material_document,
)
from mcp.adapter_outbound.vehicle_material_source_verifier import (
    verify_vehicle_material_texture_sources,
)
from mcp.domain.errors import ProtocolError
from mcp.domain.plan_bundle import PlanBundleReport
from mcp.domain.plan_bundle import SemanticArtifactSummary
from mcp.domain.plan_bundle import ValidatedPlanBundle
from mcp.domain.vehicle_material_construction import (
    compile_vehicle_material_construction,
)
import pytest

_SHA = "1" * 64
_REQUEST = "2" * 64
_RECIPE = "simple-unlit-blend-alpha-alpha-test-on-both-faces"


def _document() -> dict[str, object]:
    texture_name = f"T_Vehicle_{_SHA}"
    texture_package = f"/Game/Generated/SHAR/Textures/Vehicles/{texture_name}"
    texture_object = f"{texture_package}.{texture_name}"
    master_name = "M_SHAR_Vehicle_SimpleUnlit_Alpha_AlphaTestOn_TwoSided"
    master_package = (
        f"/Game/Generated/SHAR/Materials/Vehicles/Masters/{master_name}"
    )
    master_object = f"{master_package}.{master_name}"
    instance_name = f"MI_Vehicle_{_REQUEST}"
    instance_package = (
        f"/Game/Generated/SHAR/Materials/Vehicles/Instances/{instance_name}"
    )
    alpha = 0.375
    alpha_bits = struct.unpack("<I", struct.pack("<f", alpha))[0]
    return {
        "schema": "shar-schoenwald.unreal-vehicle-material-evidence.v4",
        "source_schema": "shar.vehicle-catalog.v8",
        "target_policy": {
            "source_projection": "reviewed-pddi-render-state",
            "source_projection_status": "ready",
            "world_material_policy_reuse": "forbidden",
            "native_construction": "simple-unlit-texture-master-instance-ready",
            "mesh_slot_application": "blocked-pending-reviewed-transaction",
            "dynamic_light_binding": (
                "headlight-sidecar-plus-slot-bound-rear-lights"
            ),
            "runtime_shader_mutation": "preserve-separately",
        },
        "counts": {
            "slots": 8,
            "native_graph_ready_slots": 1,
            "native_construction_ready_slots": 1,
            "native_texture_requests": 1,
            "native_master_requests": 1,
            "native_instance_requests": 1,
            "native_ready_slots": 0,
            "native_blocked_slots": 8,
        },
        "native_construction": {
            "texture_requests": [{
                "sha256": _SHA,
                "source_path": "vehicle-assets/sedana/textures/lens.png",
                "bytes": 3,
                "folder_path": "/Game/Generated/SHAR/Textures/Vehicles",
                "asset_name": texture_name,
                "package_path": texture_package,
                "object_path": texture_object,
            }],
            "master_requests": [{
                "recipe_identity": _RECIPE,
                "shader_family": "simple",
                "lit": False,
                "blend_mode": 1,
                "alpha_test": True,
                "alpha_compare": 4,
                "two_sided": True,
                "folder_path": (
                    "/Game/Generated/SHAR/Materials/Vehicles/Masters"
                ),
                "asset_name": master_name,
                "package_path": master_package,
                "object_path": master_object,
            }],
            "instance_requests": [{
                "request_identity": _REQUEST,
                "package_id": "extracted-art-cars-sedana",
                "source_fbx": "vehicle-assets/sedana/sedana.fbx",
                "slot_index": 6,
                "slot_name": "LENS02_m__glass-light-emitter",
                "source_material_name": "LENS02_m",
                "recipe_identity": _RECIPE,
                "texture_sha256": _SHA,
                "folder_path": (
                    "/Game/Generated/SHAR/Materials/Vehicles/Instances"
                ),
                "asset_name": instance_name,
                "package_path": instance_package,
                "object_path": f"{instance_package}.{instance_name}",
                "parent_material_path": master_object,
                "base_color_texture_path": texture_object,
                "base_color_tint": [0.25, 0.5, 0.75, 1.0],
                "set_alpha_reference": True,
                "alpha_reference": alpha,
                "alpha_reference_bits": alpha_bits,
            }],
        },
    }


def test_compiles_exact_vehicle_material_tool_wires() -> None:
    compiled = compile_vehicle_material_construction(_document())
    assert compiled.report.to_json() == {
        "blockedSlotCount": 8,
        "instanceCount": 1,
        "masterCount": 1,
        "textureCount": 1,
    }
    texture = compiled.textures[0]
    master = compiled.masters[0]
    instance = compiled.instances[0]
    assert texture.tool_name.endswith("ImportBaseColorTexture2D")
    assert texture.arguments("/verified/lens.png") == {
        "assetName": texture.asset_name,
        "folderPath": texture.folder_path,
        "sourceFile": "/verified/lens.png",
    }
    assert master.arguments() == {
        "alphaCompare": 4,
        "assetName": master.asset_name,
        "bAlphaTest": True,
        "blendMode": 1,
        "bLit": False,
        "bTwoSided": True,
        "folderPath": master.folder_path,
        "shaderFamily": "simple",
    }
    assert instance.arguments()["baseColorTint"] == {
        "a": 1.0,
        "b": 0.75,
        "g": 0.5,
        "r": 0.25,
    }
    assert instance.arguments()["alphaReference"] == pytest.approx(0.375)


def test_rejects_unplanned_dependencies_and_raster_drift() -> None:
    document = _document()
    instance = document["native_construction"]["instance_requests"][0]
    instance["parent_material_path"] = "/Game/Generated/SHAR/Missing.Missing"
    with pytest.raises(ProtocolError, match="parent is not planned"):
        compile_vehicle_material_construction(document)
    document = _document()
    master = document["native_construction"]["master_requests"][0]
    master["lit"] = True
    with pytest.raises(ProtocolError, match="simple-unlit policy"):
        compile_vehicle_material_construction(document)
    document = _document()
    texture = document["native_construction"]["texture_requests"][0]
    texture["source_path"] = "../outside.png"
    with pytest.raises(ProtocolError, match="source path is not canonical"):
        compile_vehicle_material_construction(document)


def test_rejects_count_and_alpha_bit_drift() -> None:
    document = _document()
    document["counts"]["native_instance_requests"] = 2
    with pytest.raises(ProtocolError, match="counts are inconsistent"):
        compile_vehicle_material_construction(document)
    document = _document()
    instance = document["native_construction"]["instance_requests"][0]
    instance["alpha_reference_bits"] = 0
    with pytest.raises(ProtocolError, match="alpha reference bits disagree"):
        compile_vehicle_material_construction(document)


def test_source_verifier_checks_bytes_digest_and_symlink(
    tmp_path: Path,
) -> None:
    data = b"png"
    document = _document()
    digest = hashlib.sha256(data).hexdigest()
    texture = document["native_construction"]["texture_requests"][0]
    texture["sha256"] = digest
    texture["asset_name"] = f"T_Vehicle_{digest}"
    package = f"/Game/Generated/SHAR/Textures/Vehicles/T_Vehicle_{digest}"
    texture["package_path"] = package
    texture["object_path"] = f"{package}.T_Vehicle_{digest}"
    instance = document["native_construction"]["instance_requests"][0]
    instance["texture_sha256"] = digest
    instance["base_color_texture_path"] = texture["object_path"]
    compiled = compile_vehicle_material_construction(document)
    source = tmp_path / "vehicle-assets" / "sedana" / "textures" / "lens.png"
    source.parent.mkdir(parents=True)
    source.write_bytes(data)
    verified = verify_vehicle_material_texture_sources(tmp_path, compiled)
    assert verified == {digest: source}
    source.unlink()
    target = tmp_path / "target.png"
    target.write_bytes(data)
    source.symlink_to(target)
    with pytest.raises(ProtocolError, match="regular unlinked file"):
        verify_vehicle_material_texture_sources(tmp_path, compiled)


def _bundle(sidecar: bytes) -> ValidatedPlanBundle:
    artifact = SemanticArtifactSummary(
        "vehicle-materials",
        "vehicle-materials.json",
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


def test_bound_reader_rechecks_release_index_hash(tmp_path: Path) -> None:
    sidecar = json.dumps(_document(), sort_keys=True).encode()
    path = tmp_path / "vehicle-materials.json"
    path.write_bytes(sidecar)
    bundle = _bundle(sidecar)
    document = read_bound_vehicle_material_document(tmp_path, bundle)
    assert document["schema"].endswith(".v4")
    path.write_bytes(sidecar + b"\n")
    with pytest.raises(ProtocolError, match="byte count is stale"):
        read_bound_vehicle_material_document(tmp_path, bundle)


def test_bound_reader_rejects_redirected_root(tmp_path: Path) -> None:
    sidecar = json.dumps(_document(), sort_keys=True).encode()
    real_root = tmp_path / "real"
    real_root.mkdir()
    (real_root / "vehicle-materials.json").write_bytes(sidecar)
    linked_root = tmp_path / "linked"
    linked_root.symlink_to(real_root, target_is_directory=True)
    with pytest.raises(ProtocolError, match="staging root is redirected"):
        read_bound_vehicle_material_document(linked_root, _bundle(sidecar))
