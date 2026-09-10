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
#   - MCP integration tests for generated plan capability audits.
# - Must-Not:
#   - Contact a real Unreal process or invoke native mutation meta-tools.
# - Allows:
#   - Fake Streamable HTTP sessions and synthetic empty plan bundles.
# - Split-When:
#   - Split when execution and discovery integrations gain separate lifecycles.
# - Merge-When:
#   - Merge when plan capability audit has no independent integration boundary.
# - Summary:
#   - Unreal plan capability integration tests.
# - Description:
#   - Proves selective live discovery and mutation-free CLI session handling.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Only required available toolsets are described.
#

"""MCP integration tests for generated plan capability audits."""

from __future__ import annotations

# ruff: noqa: PLC0415
import hashlib
import json
from pathlib import Path

from fake_unreal_server import FakeUnrealServer
from mcp.adapter_inbound.cli import main
from mcp.adapter_outbound.streamable_http import StreamableHttpTransport
from mcp.application.service import UnrealMcpTranslator
from mcp.domain.endpoint import McpEndpoint
from plan_bundle_fixture import write_plan_bundle
import pytest


def _vehicle_physics_document(
    source_fbx: str = "fbx-assets/skeletal/model.fbx",
) -> dict[str, object]:
    return {
        "schema": "shar-schoenwald.unreal-vehicle-physics-evidence.v1",
        "source_schema": "shar.vehicle-catalog.v8",
        "target_policy": {
            "box_extent_policy": "source-half-to-native-full",
            "local_axis_conversion": "reflect-y",
            "native_dimension_policy": (
                "retain-source-magnitude-under-bone-scale"
            ),
            "skeletal_import_unit_policy": "scene-unit-converted",
            "source_coordinate_space": "source-bone-local",
            "source_unit": "meter",
            "unsupported_shape_policy": "block-rig",
        },
        "counts": {
            "vehicles": 1,
            "rigs": 1,
            "primitives": 1,
            "spheres": 1,
            "oriented_boxes": 0,
            "cylinders": 0,
            "native_ready_rigs": 1,
            "native_blocked_rigs": 0,
            "native_shapes": 1,
        },
        "native_construction": {
            "requests": [{
                "package_id": "skeletal-mesh-package",
                "source_fbx": source_fbx,
                "subcategory": "cars/road",
                "rig_identity": "model",
                "joint_count": 1,
                "shapes": [{
                    "kind": "sphere",
                    "bone_name": "model",
                    "center": [0.0, 0.0, 0.0],
                    "radius": 0.5,
                }],
            }],
            "blockers": [],
        },
    }


def _bind_vehicle_physics_sidecar(
    plan_root: Path,
    source_fbx: str = "fbx-assets/skeletal/model.fbx",
) -> None:
    sidecar = plan_root.parent / "vehicle-physics.json"
    payload = (
        json.dumps(
            _vehicle_physics_document(source_fbx),
            ensure_ascii=False,
            separators=(",", ":"),
        )
        + "\n"
    ).encode()
    sidecar.write_bytes(payload)
    index_path = plan_root / "index.json"
    index = json.loads(index_path.read_text(encoding="utf-8"))
    artifacts = index["semantic_artifacts"]
    row = next(
        item for item in artifacts if item["artifact_id"] == "vehicle-physics"
    )
    row["revision"] = hashlib.sha256(payload).hexdigest()
    row["byte_count"] = len(payload)
    index["revision"] = ""
    canonical = json.dumps(
        index, ensure_ascii=False, separators=(",", ":")
    )
    index["revision"] = hashlib.sha256(canonical.encode()).hexdigest()
    index_path.write_text(
        json.dumps(index, ensure_ascii=False, separators=(",", ":")) + "\n",
        encoding="utf-8",
        newline="\n",
    )


def _vehicle_material_document(texture_bytes: bytes) -> dict[str, object]:
    digest = hashlib.sha256(texture_bytes).hexdigest()
    texture_name = f"T_Vehicle_{digest}"
    texture_package = f"/Game/Generated/SHAR/Textures/Vehicles/{texture_name}"
    texture_object = f"{texture_package}.{texture_name}"
    recipe = "simple-unlit-blend-additive-alpha-test-off-one-sided"
    master_name = (
        "M_SHAR_Vehicle_SimpleUnlit_Additive_AlphaTestOff_OneSided"
    )
    master_package = (
        f"/Game/Generated/SHAR/Materials/Vehicles/Masters/{master_name}"
    )
    master_object = f"{master_package}.{master_name}"
    request = "2" * 64
    instance_name = f"MI_Vehicle_{request}"
    instance_package = (
        f"/Game/Generated/SHAR/Materials/Vehicles/Instances/{instance_name}"
    )
    return {
        "schema": "shar-schoenwald.unreal-vehicle-material-evidence.v3",
        "source_schema": "shar.vehicle-catalog.v8",
        "target_policy": {
            "source_projection": "reviewed-pddi-render-state",
            "source_projection_status": "ready",
            "world_material_policy_reuse": "forbidden",
            "native_construction": (
                "simple-unlit-texture-master-instance-ready"
            ),
            "mesh_slot_application": (
                "blocked-pending-reviewed-transaction"
            ),
            "dynamic_light_binding": (
                "verified-source-part-to-material-slots"
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
                "sha256": digest,
                "source_path": "vehicle-assets/sedana/textures/lens.png",
                "bytes": len(texture_bytes),
                "folder_path": "/Game/Generated/SHAR/Textures/Vehicles",
                "asset_name": texture_name,
                "package_path": texture_package,
                "object_path": texture_object,
            }],
            "master_requests": [{
                "recipe_identity": recipe,
                "shader_family": "simple",
                "lit": False,
                "blend_mode": 2,
                "alpha_test": False,
                "alpha_compare": 4,
                "two_sided": False,
                "folder_path": (
                    "/Game/Generated/SHAR/Materials/Vehicles/Masters"
                ),
                "asset_name": master_name,
                "package_path": master_package,
                "object_path": master_object,
            }],
            "instance_requests": [{
                "request_identity": request,
                "package_id": "extracted-art-cars-sedana",
                "source_fbx": "vehicle-assets/sedana/sedana.fbx",
                "slot_index": 6,
                "slot_name": "LENS02_m__glass-light-emitter",
                "source_material_name": "LENS02_m",
                "recipe_identity": recipe,
                "texture_sha256": digest,
                "folder_path": (
                    "/Game/Generated/SHAR/Materials/Vehicles/Instances"
                ),
                "asset_name": instance_name,
                "package_path": instance_package,
                "object_path": f"{instance_package}.{instance_name}",
                "parent_material_path": master_object,
                "base_color_texture_path": texture_object,
                "base_color_tint": [1.0, 1.0, 1.0, 1.0],
                "set_alpha_reference": False,
                "alpha_reference": None,
                "alpha_reference_bits": None,
            }],
        },
    }


def _bind_vehicle_material_sidecar(
    plan_root: Path,
    texture_bytes: bytes,
) -> None:
    _bind_vehicle_material_document(
        plan_root, _vehicle_material_document(texture_bytes)
    )


def _bind_vehicle_material_document(
    plan_root: Path,
    document: dict[str, object],
) -> None:
    sidecar = plan_root.parent / "vehicle-materials.json"
    payload = (
        json.dumps(
            document,
            ensure_ascii=False,
            separators=(",", ":"),
        )
        + "\n"
    ).encode()
    sidecar.write_bytes(payload)
    index_path = plan_root / "index.json"
    index = json.loads(index_path.read_text(encoding="utf-8"))
    artifacts = index["semantic_artifacts"]
    row = next(
        item for item in artifacts if item["artifact_id"] == "vehicle-materials"
    )
    row["revision"] = hashlib.sha256(payload).hexdigest()
    row["byte_count"] = len(payload)
    index["revision"] = ""
    canonical = json.dumps(
        index, ensure_ascii=False, separators=(",", ":")
    )
    index["revision"] = hashlib.sha256(canonical.encode()).hexdigest()
    index_path.write_text(
        json.dumps(index, ensure_ascii=False, separators=(",", ":")) + "\n",
        encoding="utf-8",
        newline="\n",
    )


def _two_package_vehicle_material_document(
    selected_bytes: bytes,
    other_bytes: bytes,
) -> dict[str, object]:
    document = _vehicle_material_document(selected_bytes)
    native = document["native_construction"]
    textures = native["texture_requests"]
    instances = native["instance_requests"]
    selected = instances[0]
    digest = hashlib.sha256(other_bytes).hexdigest()
    texture_name = f"T_Vehicle_{digest}"
    texture_package = f"/Game/Generated/SHAR/Textures/Vehicles/{texture_name}"
    texture_object = f"{texture_package}.{texture_name}"
    textures.append({
        "sha256": digest,
        "source_path": "vehicle-assets/other/textures/other.png",
        "bytes": len(other_bytes),
        "folder_path": "/Game/Generated/SHAR/Textures/Vehicles",
        "asset_name": texture_name,
        "package_path": texture_package,
        "object_path": texture_object,
    })
    request = "3" * 64
    name = f"MI_Vehicle_{request}"
    package = f"/Game/Generated/SHAR/Materials/Vehicles/Instances/{name}"
    instances.append({
        **selected,
        "request_identity": request,
        "package_id": "extracted-art-cars-other",
        "source_fbx": "vehicle-assets/other/other.fbx",
        "slot_index": 1,
        "slot_name": "other_m__transparent-light-emitter",
        "source_material_name": "other_m",
        "texture_sha256": digest,
        "asset_name": name,
        "package_path": package,
        "object_path": f"{package}.{name}",
        "base_color_texture_path": texture_object,
    })
    textures.sort(key=lambda item: str(item["sha256"]))
    instances.sort(key=lambda item: str(item["request_identity"]))
    counts = document["counts"]
    counts["slots"] = 9
    counts["native_graph_ready_slots"] = 2
    counts["native_construction_ready_slots"] = 2
    counts["native_texture_requests"] = 2
    counts["native_instance_requests"] = 2
    counts["native_blocked_slots"] = 9
    return document


def test_cli_vehicle_material_scope_uses_only_selected_dependencies(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    selected_bytes = b"selected-png"
    other_bytes = b"missing-other-png"
    plan_root = tmp_path / ".cache" / "pipeline" / "unreal-staging" / "plans"
    _ = write_plan_bundle(plan_root)
    document = _two_package_vehicle_material_document(
        selected_bytes, other_bytes
    )
    _bind_vehicle_material_document(plan_root, document)
    source = (
        tmp_path
        / ".cache"
        / "pipeline"
        / "vehicle-assets"
        / "sedana"
        / "textures"
        / "lens.png"
    )
    source.parent.mkdir(parents=True)
    source.write_bytes(selected_bytes)
    monkeypatch.chdir(tmp_path)
    scope = ("--package-id", "extracted-art-cars-sedana")

    with FakeUnrealServer(plan_execution=True) as server:
        preflight = _run_json_cli(
            capsys, "vehicle-material-preflight", *scope
        )
        assert preflight["construction"] == {
            "blockedSlotCount": 9,
            "instanceCount": 2,
            "masterCount": 1,
            "textureCount": 2,
        }
        assert preflight["selection"] == {
            "instanceCount": 1,
            "masterCount": 1,
            "packageId": "extracted-art-cars-sedana",
            "textureCount": 1,
        }
        assert preflight["verifiedTextureSourceCount"] == 1
        assert preflight["selectionRevision"] != preflight[
            "constructionRevision"
        ]
        capabilities = _run_json_cli(
            capsys,
            "--endpoint",
            server.endpoint,
            "vehicle-material-capabilities",
            *scope,
        )
        assert capabilities["capabilities"]["constructionCount"] == 3
        applied = _run_json_cli(
            capsys,
            "--endpoint",
            server.endpoint,
            "vehicle-material-apply",
            *scope,
        )

    assert applied["application"]["createdCount"] == 3
    assert applied["application"]["constructionRevision"] == applied[
        "selectionRevision"
    ]
    assert len(server.assets) == 3
    assert all("other" not in path.lower() for path in server.assets)


def test_cli_vehicle_material_runs_three_gates_and_applies_assets(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    texture_bytes = b"synthetic-png"
    plan_root = tmp_path / ".cache" / "pipeline" / "unreal-staging" / "plans"
    _ = write_plan_bundle(plan_root)
    _bind_vehicle_material_sidecar(plan_root, texture_bytes)
    source = (
        tmp_path
        / ".cache"
        / "pipeline"
        / "vehicle-assets"
        / "sedana"
        / "textures"
        / "lens.png"
    )
    source.parent.mkdir(parents=True)
    source.write_bytes(texture_bytes)
    monkeypatch.chdir(tmp_path)

    with FakeUnrealServer(plan_execution=True) as server:
        preflight = _run_json_cli(capsys, "vehicle-material-preflight")
        assert preflight["construction"] == {
            "blockedSlotCount": 8,
            "instanceCount": 1,
            "masterCount": 1,
            "textureCount": 1,
        }
        assert preflight["verifiedTextureSourceCount"] == 1
        capability_payload = _run_json_cli(
            capsys,
            "--endpoint",
            server.endpoint,
            "vehicle-material-capabilities",
        )
        capabilities = capability_payload["capabilities"]
        assert isinstance(capabilities, dict)
        assert capabilities["complete"] is True
        assert capabilities["requiredToolCount"] == 8
        assert capabilities["availableToolCount"] == 8
        applied = _run_json_cli(
            capsys,
            "--endpoint",
            server.endpoint,
            "vehicle-material-apply",
        )

    assert applied["application"] == {
        "constructionRevision": applied["constructionRevision"],
        "createdCount": 3,
        "savedCount": 3,
        "verifiedCount": 3,
    }
    document = _vehicle_material_document(texture_bytes)
    native = document["native_construction"]
    expected = {
        native["texture_requests"][0]["package_path"]: "Texture2D",
        native["master_requests"][0]["package_path"]: "Material",
        native["instance_requests"][0]["package_path"]: (
            "MaterialInstanceConstant"
        ),
    }
    assert server.assets == expected
    assert server.dirty_assets == frozenset()
    assert server.session_closed
    native_leaves = tuple(
        request["params"]["arguments"].get("tool_name")
        for request in server.requests
        if request.get("method") == "tools/call"
        and isinstance(request.get("params"), dict)
        and request["params"].get("name") == "call_tool"
    )
    assert "ImportBaseColorTexture2D" in native_leaves
    assert "CreateSimpleUnlitVehicleMaster" in native_leaves
    assert "CreateSimpleUnlitVehicleMaterialInstance" in native_leaves
    assert native_leaves.count("save_assets") == 3
    assert "delete" not in native_leaves


def test_translator_describes_only_available_required_toolsets() -> None:
    with FakeUnrealServer() as server:
        transport = StreamableHttpTransport(
            McpEndpoint.parse(server.endpoint),
            timeout_seconds=2.0,
        )
        with UnrealMcpTranslator(transport) as translator:
            definitions = translator.describe_available_toolsets((
                "EditorToolset.EditorToolset",
                "MissingToolset.MissingToolset",
            ))
    assert tuple(item.name for item in definitions) == (
        "EditorToolset.EditorToolset",
    )
    meta_tools = tuple(
        request["params"]["name"]
        for request in server.requests
        if request.get("method") == "tools/call"
        and isinstance(request.get("params"), dict)
    )
    assert meta_tools == ("list_toolsets", "describe_toolset")
    assert server.session_closed


def test_cli_capability_audit_opens_no_mutation_for_empty_bundle(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    _ = write_plan_bundle(tmp_path / "plans")
    monkeypatch.chdir(tmp_path)
    with FakeUnrealServer() as server:
        code = main((
            "--endpoint",
            server.endpoint,
            "plan-capabilities",
            "--root",
            "plans",
        ))
    captured = capsys.readouterr()
    assert code == 0
    payload = json.loads(captured.out)
    assert payload["capabilities"]["complete"] is True
    assert payload["capabilities"]["requiredToolCount"] == 0
    assert payload["sources"]["verifiedOperationCount"] == 0
    assert not captured.err
    assert all(
        request.get("method") != "tools/call" for request in server.requests
    )
    assert server.session_closed


def test_cli_plan_apply_rejects_incomplete_plan_before_transport(
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    from mcp.adapter_outbound.plan_source_verifier import (
        PlanSourceVerificationReport,
    )
    from mcp.adapter_outbound.plan_source_verifier import VerifiedPlanSources
    from mcp.domain.plan_bundle import PlanBundleReport
    from mcp.domain.plan_bundle import PlanOperation
    from mcp.domain.plan_bundle import ValidatedPlanBundle

    operation = PlanOperation(
        plan_id="asset-import-plan",
        operation_id="operation-0000000000000001",
        package_identity="package-model",
        source_identity="source-model",
        source_format="fbx",
        target_family="model",
        source_path="fbx-assets/model.fbx",
        source_revision="a" * 64,
        destination="/Game/Generated/SHAR/test/model.model",
        target_class="StaticMesh",
        importer="asset-tools-fbx",
        import_profile="shar-fbx-static-v1",
        dependencies=(),
        readiness="requires-conversion",
        world_owned=False,
        runtime_bound=True,
    )
    report = PlanBundleReport(
        revision="b" * 64,
        source_manifest_revision="c" * 64,
        engine_contract_revision="shar-unreal-porting-contract-v1",
        target_engine_version="5.8.1",
        target_platform="editor",
        semantic_blocker_count=0,
        operation_count=1,
        readiness_counts={"requires-conversion": 1},
        plans=(),
    )
    bundle = ValidatedPlanBundle(report, (operation,))
    sources = VerifiedPlanSources(
        PlanSourceVerificationReport(
            bundle_revision=report.revision,
            verified_operation_count=0,
            unique_source_count=0,
            unique_source_bytes=0,
            skipped_conversion_count=1,
        ),
        {},
    )

    class _Reader:
        def __init__(self, _root: Path) -> None:
            pass

        def read_bundle(self) -> ValidatedPlanBundle:
            return bundle

    class _Verifier:
        def __init__(self, _repository: Path, _root: Path) -> None:
            pass

        def verify(self, _bundle: ValidatedPlanBundle) -> VerifiedPlanSources:
            return sources

    class _ForbiddenTransport:
        def __init__(self, *_args: object, **_kwargs: object) -> None:
            raise AssertionError("incomplete plan constructed MCP transport")

    monkeypatch.setattr(
        "mcp.adapter_inbound.cli.FilesystemPlanBundleReader",
        _Reader,
    )
    monkeypatch.setattr(
        "mcp.adapter_inbound.cli.FilesystemPlanSourceVerifier",
        _Verifier,
    )
    monkeypatch.setattr(
        "mcp.adapter_inbound.cli.StreamableHttpTransport",
        _ForbiddenTransport,
    )
    code = main((
        "--endpoint",
        "http://127.0.0.1:65534/mcp",
        "plan-apply",
        "--root",
        "plans",
    ))
    captured = capsys.readouterr()
    assert code == 1
    payload = json.loads(captured.out)
    assert payload["execution"]["complete"] is False
    assert payload["execution"]["blockedReadiness"] == {
        "requires-conversion": 1
    }
    assert not captured.err


def test_cli_plan_apply_completes_one_texture_over_streamable_http(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    source_bytes = b"synthetic-texture-source"
    source_revision = hashlib.sha256(source_bytes).hexdigest()
    plan_root = tmp_path / ".cache" / "pipeline" / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_texture_operation=True,
        texture_source_revision=source_revision,
    )
    source = (
        tmp_path / ".cache" / "pipeline" / "extracted" / "texture" / "image.png"
    )
    source.parent.mkdir(parents=True)
    source.write_bytes(source_bytes)
    monkeypatch.chdir(tmp_path)

    with FakeUnrealServer(plan_execution=True) as server:
        code = main((
            "--endpoint",
            server.endpoint,
            "plan-apply",
        ))

    captured = capsys.readouterr()
    assert code == 0
    payload = json.loads(captured.out)
    assert payload["application"] == {
        "bundleRevision": payload["bundle"]["revision"],
        "importedCount": 1,
        "savedCount": 1,
        "verifiedCount": 1,
    }
    package_path = "/Game/Generated/SHAR/test/texture_image"
    assert server.assets == {package_path: "Texture2D"}
    assert server.dirty_assets == frozenset()
    assert not captured.err
    assert server.session_closed

    native_leaves = tuple(
        request["params"]["arguments"].get("tool_name")
        for request in server.requests
        if request.get("method") == "tools/call"
        and isinstance(request.get("params"), dict)
        and request["params"].get("name") == "call_tool"
    )
    assert native_leaves == (
        "exists",
        "exists",
        "import_file",
        "exists",
        "get_asset_class",
        "is_dirty",
        "save_assets",
        "is_dirty",
    )
    assert "delete" not in native_leaves


def test_cli_plan_apply_completes_one_static_mesh_over_streamable_http(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    source_bytes = b"Kaydara FBX Binary synthetic-static-source"
    source_revision = hashlib.sha256(source_bytes).hexdigest()
    plan_root = tmp_path / ".cache" / "pipeline" / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_static_mesh_operation=True,
        static_mesh_source_revision=source_revision,
    )
    source = (
        tmp_path / ".cache" / "pipeline" / "fbx-assets" / "static" / "model.fbx"
    )
    source.parent.mkdir(parents=True)
    source.write_bytes(source_bytes)
    monkeypatch.chdir(tmp_path)

    with FakeUnrealServer(plan_execution=True) as server:
        code = main((
            "--endpoint",
            server.endpoint,
            "plan-apply",
        ))

    captured = capsys.readouterr()
    assert code == 0, captured.err or captured.out
    payload = json.loads(captured.out)
    assert payload["application"]["importedCount"] == 1
    assert payload["application"]["savedCount"] == 1
    assert payload["application"]["verifiedCount"] == 1
    package_path = "/Game/Generated/SHAR/models/static/model"
    assert server.assets == {package_path: "StaticMesh"}
    assert server.dirty_assets == frozenset()
    assert not captured.err
    assert server.session_closed

    native_leaves = tuple(
        request["params"]["arguments"].get("tool_name")
        for request in server.requests
        if request.get("method") == "tools/call"
        and isinstance(request.get("params"), dict)
        and request["params"].get("name") == "call_tool"
    )
    assert native_leaves == (
        "exists",
        "exists",
        "ImportStaticMesh",
        "exists",
        "get_asset_class",
        "is_dirty",
        "save_assets",
        "is_dirty",
    )
    assert "delete" not in native_leaves


def test_cli_plan_apply_completes_one_skeletal_mesh_with_companion_over_http(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    source_bytes = b"Kaydara FBX Binary synthetic-skeletal-source"
    source_revision = hashlib.sha256(source_bytes).hexdigest()
    plan_root = tmp_path / ".cache" / "pipeline" / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_skeletal_mesh_operation=True,
        skeletal_mesh_source_revision=source_revision,
    )
    source = (
        tmp_path
        / ".cache"
        / "pipeline"
        / "fbx-assets"
        / "skeletal"
        / "model.fbx"
    )
    source.parent.mkdir(parents=True)
    source.write_bytes(source_bytes)
    monkeypatch.chdir(tmp_path)

    with FakeUnrealServer(plan_execution=True) as server:
        code = main((
            "--endpoint",
            server.endpoint,
            "plan-apply",
        ))

    captured = capsys.readouterr()
    assert code == 0, captured.err or captured.out
    payload = json.loads(captured.out)
    assert payload["application"]["importedCount"] == 1
    assert payload["application"]["savedCount"] == 1
    assert payload["application"]["verifiedCount"] == 1
    mesh = "/Game/Generated/SHAR/models/skeletal/model"
    skeleton = f"{mesh}_Skeleton"
    assert server.assets == {
        mesh: "SkeletalMesh",
        skeleton: "Skeleton",
    }
    assert server.dirty_assets == frozenset()
    assert not captured.err
    assert server.session_closed

    native_leaves = tuple(
        request["params"]["arguments"].get("tool_name")
        for request in server.requests
        if request.get("method") == "tools/call"
        and isinstance(request.get("params"), dict)
        and request["params"].get("name") == "call_tool"
    )
    assert native_leaves == (
        "exists",
        "exists",
        "exists",
        "exists",
        "ImportSkeletalMesh",
        "exists",
        "get_asset_class",
        "is_dirty",
        "exists",
        "get_asset_class",
        "is_dirty",
        "save_assets",
        "is_dirty",
        "is_dirty",
    )
    assert "delete" not in native_leaves


def test_cli_plan_apply_completes_one_sound_wave_over_streamable_http(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    source_bytes = b"RIFF-synthetic-wave-source"
    source_revision = hashlib.sha256(source_bytes).hexdigest()
    plan_root = tmp_path / ".cache" / "pipeline" / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_import_operation=True,
        import_source_revision=source_revision,
    )
    source = (
        tmp_path / ".cache" / "pipeline" / "extracted" / "dialog" / "audio.wav"
    )
    source.parent.mkdir(parents=True)
    source.write_bytes(source_bytes)
    monkeypatch.chdir(tmp_path)

    with FakeUnrealServer(plan_execution=True) as server:
        code = main((
            "--endpoint",
            server.endpoint,
            "plan-apply",
        ))

    captured = capsys.readouterr()
    assert code == 0, captured.err or captured.out
    payload = json.loads(captured.out)
    assert payload["application"]["importedCount"] == 1
    assert payload["application"]["savedCount"] == 1
    assert payload["application"]["verifiedCount"] == 1
    package_path = "/Game/Generated/SHAR/dialog/dialog/audio_source"
    assert server.assets == {package_path: "SoundWave"}
    assert server.dirty_assets == frozenset()
    assert not captured.err
    assert server.session_closed

    native_leaves = tuple(
        request["params"]["arguments"].get("tool_name")
        for request in server.requests
        if request.get("method") == "tools/call"
        and isinstance(request.get("params"), dict)
        and request["params"].get("name") == "call_tool"
    )
    assert native_leaves == (
        "exists",
        "exists",
        "ImportSoundWave",
        "exists",
        "get_asset_class",
        "is_dirty",
        "save_assets",
        "is_dirty",
    )


def test_cli_plan_apply_completes_one_file_media_source_over_http(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    source_bytes = b"synthetic-hap-mov-source"
    source_revision = hashlib.sha256(source_bytes).hexdigest()
    plan_root = tmp_path / ".cache" / "pipeline" / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_media_operation=True,
        media_source_revision=source_revision,
    )
    source = (
        tmp_path
        / ".cache"
        / "pipeline"
        / "extracted"
        / "movies"
        / "intro"
        / "movie.mov"
    )
    source.parent.mkdir(parents=True)
    source.write_bytes(source_bytes)
    monkeypatch.chdir(tmp_path)

    with FakeUnrealServer(plan_execution=True) as server:
        code = main((
            "--endpoint",
            server.endpoint,
            "plan-apply",
        ))

    captured = capsys.readouterr()
    assert code == 0, captured.err or captured.out
    payload = json.loads(captured.out)
    assert payload["application"]["importedCount"] == 1
    assert payload["application"]["savedCount"] == 1
    assert payload["application"]["verifiedCount"] == 1
    package_path = "/Game/Generated/SHAR/movies/intro/intro_movie"
    object_path = f"{package_path}.intro_movie"
    assert server.assets == {package_path: "FileMediaSource"}
    assert server.media_payloads == {
        object_path: "./Movies/Generated/SHAR/movies/intro/intro_movie.mov"
    }
    assert server.dirty_assets == frozenset()
    assert not captured.err
    assert server.session_closed

    native_leaves = tuple(
        request["params"]["arguments"].get("tool_name")
        for request in server.requests
        if request.get("method") == "tools/call"
        and isinstance(request.get("params"), dict)
        and request["params"].get("name") == "call_tool"
    )
    assert native_leaves == (
        "exists",
        "FileMediaSourcePayloadExists",
        "exists",
        "FileMediaSourcePayloadExists",
        "ImportFileMediaSource",
        "exists",
        "get_asset_class",
        "is_dirty",
        "GetFileMediaSourcePath",
        "FileMediaSourcePayloadExists",
        "save_assets",
        "is_dirty",
    )


def _run_json_cli(
    capsys: pytest.CaptureFixture[str],
    *arguments: str,
) -> dict[str, object]:
    code = main(arguments)
    captured = capsys.readouterr()
    assert code == 0, captured.err or captured.out
    assert not captured.err
    payload = json.loads(captured.out)
    assert isinstance(payload, dict)
    return payload


def test_cli_vehicle_physics_applies_bound_release_after_skeletal_import(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    source_bytes = b"Kaydara FBX Binary synthetic-vehicle-physics-source"
    source_revision = hashlib.sha256(source_bytes).hexdigest()
    plan_root = tmp_path / ".cache" / "pipeline" / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_skeletal_mesh_operation=True,
        skeletal_mesh_source_revision=source_revision,
    )
    _bind_vehicle_physics_sidecar(plan_root)
    source = (
        tmp_path
        / ".cache"
        / "pipeline"
        / "fbx-assets"
        / "skeletal"
        / "model.fbx"
    )
    source.parent.mkdir(parents=True)
    source.write_bytes(source_bytes)
    monkeypatch.chdir(tmp_path)

    with FakeUnrealServer(plan_execution=True) as server:
        _run_json_cli(capsys, "--endpoint", server.endpoint, "plan-apply")
        preflight = _run_json_cli(
            capsys,
            "vehicle-physics-preflight",
            "--package-id",
            "skeletal-mesh-package",
        )
        assert preflight["construction"] == {
            "blockedRigCount": 0,
            "requestCount": 1,
            "shapeCount": 1,
        }
        assert preflight["selection"] == {
            "globalBlockedRigCount": 0,
            "packageId": "skeletal-mesh-package",
            "requestCount": 1,
            "shapeCount": 1,
        }
        capability_payload = _run_json_cli(
            capsys,
            "--endpoint",
            server.endpoint,
            "vehicle-physics-capabilities",
            "--package-id",
            "skeletal-mesh-package",
        )
        capabilities = capability_payload["capabilities"]
        assert isinstance(capabilities, dict)
        assert capabilities["complete"] is True
        assert capabilities["requiredToolCount"] == 6
        assert capabilities["availableToolCount"] == 6
        applied = _run_json_cli(
            capsys,
            "--endpoint",
            server.endpoint,
            "vehicle-physics-apply",
            "--package-id",
            "skeletal-mesh-package",
        )
        verified = _run_json_cli(
            capsys,
            "--endpoint",
            server.endpoint,
            "vehicle-physics-verify",
            "--package-id",
            "skeletal-mesh-package",
        )

    assert isinstance(verified["capabilities"], dict)
    assert verified["capabilities"]["complete"] is True
    assert verified["capabilities"]["requiredToolCount"] == 4
    assert verified["capabilities"]["availableToolCount"] == 4
    assert verified["verification"] == {
        "constructionRevision": verified["selectionRevision"],
        "verifiedCount": 1,
    }
    assert applied["application"] == {
        "blockedRigCount": 0,
        "constructionRevision": applied["constructionRevision"],
        "createdCount": 1,
        "savedCount": 1,
        "verifiedCount": 1,
    }
    digest = hashlib.sha256(b"skeletal-mesh-package\0model").hexdigest()[:24]
    physics = f"/Game/Generated/SHAR/VehiclePhysics/PHYS_{digest}"
    mesh = "/Game/Generated/SHAR/models/skeletal/model"
    skeleton = f"{mesh}_Skeleton"
    assert server.assets == {
        mesh: "SkeletalMesh",
        skeleton: "Skeleton",
        physics: "PhysicsAsset",
    }
    assert server.dirty_assets == frozenset()
    assert server.session_closed
    native_leaves = tuple(
        request["params"]["arguments"].get("tool_name")
        for request in server.requests
        if request.get("method") == "tools/call"
        and isinstance(request.get("params"), dict)
        and request["params"].get("name") == "call_tool"
    )
    assert "CreateVehiclePhysicsAsset" in native_leaves
    assert native_leaves.count("VerifyVehiclePhysicsAsset") == 1
    assert native_leaves.count("save_assets") == 2
    assert "delete" not in native_leaves


def test_cli_vehicle_prerequisites_apply_with_semantic_blocker(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    source_bytes = b"Kaydara FBX Binary vehicle-prerequisite-source"
    source_revision = hashlib.sha256(source_bytes).hexdigest()
    source_path = "vehicle-assets/model/model.fbx"
    plan_root = tmp_path / ".cache" / "pipeline" / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_skeletal_mesh_operation=True,
        skeletal_mesh_source_revision=source_revision,
        skeletal_mesh_source_path=source_path,
        semantic_blocker_count=1,
    )
    _bind_vehicle_physics_sidecar(plan_root, source_path)
    source = tmp_path / ".cache" / "pipeline" / source_path
    source.parent.mkdir(parents=True)
    source.write_bytes(source_bytes)
    monkeypatch.chdir(tmp_path)

    with FakeUnrealServer(plan_execution=True) as server:
        preflight = _run_json_cli(
            capsys,
            "vehicle-physics-prerequisites-preflight",
            "--package-id",
            "skeletal-mesh-package",
        )
        assert preflight["bundle"]["semanticBlockerCount"] == 1
        assert preflight["prerequisites"] == {
            "importCount": 1,
            "readyRigCount": 1,
            "sourceCount": 1,
        }
        assert preflight["sources"]["verifiedOperationCount"] == 1
        assert preflight["selection"] == {
            "importCount": 1,
            "packageId": "skeletal-mesh-package",
            "readyRigCount": 1,
            "sourceCount": 1,
        }
        assert preflight["selectionRevision"] != (
            preflight["prerequisiteRevision"]
        )
        capabilities = _run_json_cli(
            capsys,
            "--endpoint",
            server.endpoint,
            "vehicle-physics-prerequisites-capabilities",
            "--package-id",
            "skeletal-mesh-package",
        )
        assert capabilities["capabilities"]["complete"] is True
        assert capabilities["capabilities"]["requiredToolCount"] == 6
        applied = _run_json_cli(
            capsys,
            "--endpoint",
            server.endpoint,
            "vehicle-physics-prerequisites-apply",
            "--package-id",
            "skeletal-mesh-package",
        )
        physics = _run_json_cli(
            capsys,
            "--endpoint",
            server.endpoint,
            "vehicle-physics-apply",
        )

    assert applied["application"]["importedCount"] == 1
    assert applied["application"]["savedCount"] == 1
    assert applied["application"]["verifiedCount"] == 1
    assert applied["bundle"]["semanticBlockerCount"] == 1
    assert physics["application"]["createdCount"] == 1
    mesh = "/Game/Generated/SHAR/models/skeletal/model"
    skeleton = f"{mesh}_Skeleton"
    digest = hashlib.sha256(b"skeletal-mesh-package\0model").hexdigest()[:24]
    physics_asset = f"/Game/Generated/SHAR/VehiclePhysics/PHYS_{digest}"
    assert server.assets == {
        mesh: "SkeletalMesh",
        skeleton: "Skeleton",
        physics_asset: "PhysicsAsset",
    }
    assert server.dirty_assets == frozenset()
    assert server.session_closed
