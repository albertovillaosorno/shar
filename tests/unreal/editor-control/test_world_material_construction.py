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
#   - World-material construction compilation and source-verification tests.
# - Must-Not:
#   - Open MCP sessions, mutate Unreal Editor, or depend on generated caches.
# - Allows:
#   - Use synthetic v3 evidence and temporary content-addressed PNG files.
# - Split-When:
#   - Reader, compiler, and filesystem verification gain separate fixtures.
# - Merge-When:
#   - Another test module owns identical world-material construction policy.
# - Summary:
#   - World-material construction boundary tests.
# - Description:
#   - Proves native wire values, dependency joins, binding, and source hashes.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Invalid or stale construction evidence fails closed before mutation.
#

"""Tests for world-material native construction boundaries."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import struct

from mcp.adapter_outbound.world_material_construction_reader import (
    read_bound_world_material_document,
)
from mcp.adapter_outbound.world_material_source_verifier import (
    verify_world_material_texture_sources,
)
from mcp.domain.errors import ProtocolError
from mcp.domain.plan_bundle import PlanBundleReport
from mcp.domain.plan_bundle import SemanticArtifactSummary
from mcp.domain.plan_bundle import ValidatedPlanBundle
from mcp.domain.world_material_construction import (
    compile_world_material_construction,
)
import pytest

_SHA = "1" * 64
_PRESENTATION = "2" * 64
_RECIPE = "simple-unlit-blend-alpha-alpha-test-on-both-faces"


def _document(*, data: bytes = b"png") -> dict[str, object]:
    texture_name = f"T_World_{_SHA}"
    texture_package = f"/Game/Generated/SHAR/Textures/World/{texture_name}"
    texture_object = f"{texture_package}.{texture_name}"
    master_name = "M_SHAR_World_simple_unlit_alpha_test"
    master_package = (
        f"/Game/Generated/SHAR/Materials/World/Masters/{master_name}"
    )
    master_object = f"{master_package}.{master_name}"
    instance_name = f"MI_World_{_PRESENTATION}"
    instance_package = (
        f"/Game/Generated/SHAR/Materials/World/Instances/{instance_name}"
    )
    alpha_reference = 0.375
    alpha_bits = struct.unpack("<I", struct.pack("<f", alpha_reference))[0]
    return {
        "schema": "shar-schoenwald.unreal-world-material-evidence.v3",
        "engine_contract_revision": "shar-unreal-porting-contract-v1",
        "counts": {
            "native_texture_requests": 1,
            "native_master_requests": 1,
            "native_instance_requests": 1,
            "native_master_ready_presentations": 1,
            "native_master_blocked_presentations": 2,
            "presentations": 3,
        },
        "native_construction": {
            "texture_requests": [{
                "asset_name": texture_name,
                "bytes": len(data),
                "folder_path": "/Game/Generated/SHAR/Textures/World",
                "object_path": texture_object,
                "package_path": texture_package,
                "sha256": _SHA,
                "source_path": f"world-assets/textures/texture-{_SHA}.png",
            }],
            "master_requests": [{
                "alpha_test": True,
                "asset_name": master_name,
                "blend_family": "alpha",
                "folder_path": "/Game/Generated/SHAR/Materials/World/Masters",
                "object_path": master_object,
                "package_path": master_package,
                "recipe_identity": _RECIPE,
                "render_both_faces": True,
            }],
            "instance_requests": [{
                "alpha_reference": alpha_reference,
                "alpha_reference_bits": alpha_bits,
                "asset_name": instance_name,
                "base_color_texture_path": texture_object,
                "base_color_tint": [0.25, 0.5, 0.75, 1.0],
                "folder_path": "/Game/Generated/SHAR/Materials/World/Instances",
                "object_path": f"{instance_package}.{instance_name}",
                "package_path": instance_package,
                "parent_material_path": master_object,
                "recipe_identity": _RECIPE,
                "set_alpha_reference": True,
                "slot_presentation_sha256": _PRESENTATION,
                "texture_sha256": _SHA,
            }],
        },
    }


def _bundle(sidecar: bytes) -> ValidatedPlanBundle:
    artifact = SemanticArtifactSummary(
        "world-materials",
        "world-materials.json",
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


def test_compiles_exact_native_world_material_wire() -> None:
    compiled = compile_world_material_construction(_document())
    assert compiled.report._asdict() == {
        "texture_count": 1,
        "master_count": 1,
        "instance_count": 1,
        "blocked_presentation_count": 2,
    }
    texture = compiled.textures[0]
    master = compiled.masters[0]
    instance = compiled.instances[0]
    assert texture.tool_name.endswith("TextureTools.import_file")
    assert texture.arguments("/verified/source.png")["source_file"] == (
        "/verified/source.png"
    )
    assert master.arguments() == {
        "assetName": master.asset_name,
        "bAlphaTest": True,
        "blendFamily": "alpha",
        "folderPath": master.folder_path,
    }
    assert instance.arguments()["baseColorTint"] == {
        "a": 1.0,
        "b": 0.75,
        "g": 0.5,
        "r": 0.25,
    }
    assert instance.arguments()["bSetAlphaReference"] is True
    assert instance.arguments()["alphaReference"] == pytest.approx(0.375)


def test_compile_rejects_broken_parent_and_alpha_bits() -> None:
    document = _document()
    instance = document["native_construction"]["instance_requests"][0]
    instance["parent_material_path"] = "/Game/Generated/SHAR/Missing.Missing"
    with pytest.raises(ProtocolError, match="parent is not planned"):
        compile_world_material_construction(document)
    document = _document()
    instance = document["native_construction"]["instance_requests"][0]
    instance["alpha_reference_bits"] = 0
    with pytest.raises(ProtocolError, match="alpha reference bits disagree"):
        compile_world_material_construction(document)


def test_bound_reader_rechecks_release_index_hash(tmp_path: Path) -> None:
    sidecar = json.dumps(_document(), sort_keys=True).encode()
    path = tmp_path / "world-materials.json"
    path.write_bytes(sidecar)
    bundle = _bundle(sidecar)
    document = read_bound_world_material_document(tmp_path, bundle)
    assert document["schema"].endswith(".v3")
    path.write_bytes(sidecar + b"\n")
    with pytest.raises(ProtocolError, match="byte count is stale"):
        read_bound_world_material_document(tmp_path, bundle)


def test_bound_reader_rejects_redirected_root(tmp_path: Path) -> None:
    sidecar = json.dumps(_document(), sort_keys=True).encode()
    real_root = tmp_path / "real"
    real_root.mkdir()
    (real_root / "world-materials.json").write_bytes(sidecar)
    bundle = _bundle(sidecar)
    linked_root = tmp_path / "linked"
    linked_root.symlink_to(real_root, target_is_directory=True)
    with pytest.raises(ProtocolError, match="staging root is redirected"):
        read_bound_world_material_document(linked_root, bundle)


def test_source_verifier_checks_content_and_rejects_symlink(
    tmp_path: Path,
) -> None:
    data = b"png"
    document = _document(data=data)
    texture = document["native_construction"]["texture_requests"][0]
    actual_sha = hashlib.sha256(data).hexdigest()
    texture["sha256"] = actual_sha
    texture["asset_name"] = f"T_World_{actual_sha}"
    texture["source_path"] = f"world-assets/textures/texture-{actual_sha}.png"
    package = f"/Game/Generated/SHAR/Textures/World/T_World_{actual_sha}"
    texture["package_path"] = package
    texture["object_path"] = f"{package}.T_World_{actual_sha}"
    instance = document["native_construction"]["instance_requests"][0]
    instance["texture_sha256"] = actual_sha
    instance["base_color_texture_path"] = texture["object_path"]
    compiled = compile_world_material_construction(document)
    source_dir = tmp_path / "world-assets" / "textures"
    source_dir.mkdir(parents=True)
    source = source_dir / f"texture-{actual_sha}.png"
    source.write_bytes(data)
    verified = verify_world_material_texture_sources(tmp_path, compiled)
    assert verified == {actual_sha: source}
    source.unlink()
    target = tmp_path / "target.png"
    target.write_bytes(data)
    source.symlink_to(target)
    with pytest.raises(ProtocolError, match="regular unlinked file"):
        verify_world_material_texture_sources(tmp_path, compiled)
