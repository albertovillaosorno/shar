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
#   - Native Unreal plan execution compilation tests.
# - Must-Not:
#   - Read generated files, open MCP sessions, or mutate Unreal Editor.
# - Allows:
#   - Exercise exact route mappings, readiness, and public reports.
# - Split-When:
#   - Split when another execution family gains independent fixtures.
# - Merge-When:
#   - Merge when execution compilation has no independent policy.
# - Summary:
#   - Unreal plan execution compiler tests.
# - Description:
#   - Proves reviewed imports compile and unsupported routes remain visible.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - No partial plan is reported complete.
#

"""Tests for deterministic native Unreal plan execution compilation."""

from __future__ import annotations

# ruff: noqa: PLR0913
from mcp.domain.plan_bundle import PlanBundleReport
from mcp.domain.plan_bundle import PlanOperation
from mcp.domain.plan_bundle import ValidatedPlanBundle
from mcp.domain.plan_execution import compile_execution_plan


def _operation(
    *,
    operation_id: str,
    source_format: str,
    target_family: str,
    target_class: str,
    importer: str,
    import_profile: str,
    readiness: str = "ready",
) -> PlanOperation:
    asset_name = operation_id.replace("operation-", "asset_")
    return PlanOperation(
        plan_id=(
            "asset-construction-plan"
            if readiness == "requires-editor-factory"
            else "asset-import-plan"
        ),
        operation_id=operation_id,
        package_identity=f"package-{asset_name}",
        source_identity=f"source-{asset_name}",
        source_format=source_format,
        target_family=target_family,
        source_path=f"extracted/{asset_name}.{source_format}",
        source_revision="a" * 64,
        destination=f"/Game/Generated/SHAR/test/{asset_name}.{asset_name}",
        target_class=target_class,
        importer=importer,
        import_profile=import_profile,
        dependencies=(),
        readiness=readiness,
        world_owned=False,
        runtime_bound=True,
    )


def _bundle(
    *operations: PlanOperation,
    semantic_blocker_count: int = 0,
) -> ValidatedPlanBundle:
    report = PlanBundleReport(
        revision="b" * 64,
        source_manifest_revision="c" * 64,
        engine_contract_revision="shar-unreal-porting-contract-v1",
        target_engine_version="5.8.1",
        target_platform="editor",
        semantic_blocker_count=semantic_blocker_count,
        operation_count=len(operations),
        readiness_counts={},
        plans=(),
    )
    return ValidatedPlanBundle(report, operations)


def test_compiles_exact_texture_and_static_mesh_routes() -> None:
    texture = _operation(
        operation_id="operation-0000000000000001",
        source_format="image",
        target_family="texture",
        target_class="Texture2D",
        importer="texture-factory",
        import_profile="shar-texture-v1",
    )
    mesh = _operation(
        operation_id="operation-0000000000000002",
        source_format="fbx",
        target_family="model",
        target_class="StaticMesh",
        importer="asset-tools-fbx",
        import_profile="shar-fbx-static-v1",
    )
    compiled = compile_execution_plan(_bundle(texture, mesh))
    assert compiled.report.complete
    assert compiled.report.route_counts == {
        "static-mesh-fbx-v1": 1,
        "texture-image-v1": 1,
    }
    texture_step, mesh_step = compiled.imports
    assert texture_step.arguments("C:/verified/source.png") == {
        "asset_name": "asset_0000000000000001",
        "folder_path": "/Game/Generated/SHAR/test",
        "source_file": "C:/verified/source.png",
    }
    assert mesh_step.toolset_name == "SharImportEditor.SharImportToolset"
    assert mesh_step.tool_name == (
        "SharImportEditor.SharImportToolset.ImportStaticMesh"
    )
    assert mesh_step.arguments("C:/verified/source.fbx") == {
        "assetName": "asset_0000000000000002",
        "folderPath": "/Game/Generated/SHAR/test",
        "sourceFile": "C:/verified/source.fbx",
    }


def test_reports_blockers_without_partial_success() -> None:
    blocked_fbx = _operation(
        operation_id="operation-0000000000000003",
        source_format="fbx",
        target_family="model",
        target_class="StaticMesh",
        importer="asset-tools-fbx",
        import_profile="shar-fbx-static-v1",
        readiness="requires-conversion",
    )
    audio = _operation(
        operation_id="operation-0000000000000004",
        source_format="wav",
        target_family="audio",
        target_class="SoundWave",
        importer="sound-wave-factory",
        import_profile="shar-audio-v1",
    )
    media = _operation(
        operation_id="operation-0000000000000005",
        source_format="hap",
        target_family="media",
        target_class="FileMediaSource",
        importer="media-source-movie",
        import_profile="shar-hap-movie-v1",
    )
    construction = _operation(
        operation_id="operation-0000000000000006",
        source_format="json",
        target_family="structured-data",
        target_class="WidgetBlueprint",
        importer="shar-ui-factory",
        import_profile="shar-ui-v1",
        readiness="requires-editor-factory",
    )
    compiled = compile_execution_plan(
        _bundle(blocked_fbx, audio, media, construction)
    )
    assert not compiled.report.complete
    assert len(compiled.imports) == 2
    audio_step, media_step = compiled.imports
    assert audio_step.route_id == "sound-wave-wav-v1"
    assert audio_step.toolset_name == "SharImportEditor.SharImportToolset"
    assert audio_step.tool_name.endswith(".ImportSoundWave")
    assert audio_step.target_class == "SoundWave"
    assert audio_step.external_payload_path is None
    assert audio_step.arguments("C:/verified/source.wav") == {
        "assetName": "asset_0000000000000004",
        "folderPath": "/Game/Generated/SHAR/test",
        "sourceFile": "C:/verified/source.wav",
    }
    assert media_step.route_id == "file-media-source-hap-v1"
    assert media_step.tool_name.endswith(".ImportFileMediaSource")
    assert media_step.target_class == "FileMediaSource"
    assert media_step.external_payload_path == (
        "./Movies/Generated/SHAR/test/asset_0000000000000005.mov"
    )
    assert media_step.arguments("C:/verified/source.mov") == {
        "assetName": "asset_0000000000000005",
        "folderPath": "/Game/Generated/SHAR/test",
        "sourceFile": "C:/verified/source.mov",
    }
    assert compiled.report.route_counts == {
        "file-media-source-hap-v1": 1,
        "sound-wave-wav-v1": 1,
    }
    assert compiled.report.blocked_readiness == {
        "requires-conversion": 1,
        "requires-editor-factory": 1,
    }
    assert compiled.report.unsupported_routes == {}
    public = compiled.report.to_json()
    rendered = str(public)
    assert "C:/verified" not in rendered
    assert "extracted/" not in rendered
    assert public["complete"] is False


def test_semantic_blockers_prevent_false_complete_empty_bundle() -> None:
    compiled = compile_execution_plan(_bundle(semantic_blocker_count=1))
    assert compiled.imports == ()
    assert compiled.report.operation_count == 0
    assert compiled.report.compiled_count == 0
    assert compiled.report.semantic_blocker_count == 1
    assert compiled.report.complete is False
    assert compiled.report.to_json()["semanticBlockerCount"] == 1


def test_compiles_skeletal_fbx_with_explicit_skeleton_companion() -> None:
    operation = _operation(
        operation_id="operation-0000000000000007",
        source_format="fbx",
        target_family="model",
        target_class="SkeletalMesh",
        importer="asset-tools-fbx",
        import_profile="shar-fbx-skeletal-v1",
    )
    compiled = compile_execution_plan(_bundle(operation))
    assert compiled.report.complete
    assert compiled.report.route_counts == {"skeletal-mesh-fbx-v1": 1}
    step = compiled.imports[0]
    assert step.tool_name == (
        "SharImportEditor.SharImportToolset.ImportSkeletalMesh"
    )
    assert step.arguments("C:/verified/source.fbx") == {
        "assetName": "asset_0000000000000007",
        "folderPath": "/Game/Generated/SHAR/test",
        "sourceFile": "C:/verified/source.fbx",
    }
    assert tuple(output.role for output in step.outputs) == (
        "primary",
        "skeleton",
    )
    assert tuple(output.target_class for output in step.outputs) == (
        "SkeletalMesh",
        "Skeleton",
    )
    skeleton_package = (
        "/Game/Generated/SHAR/test/asset_0000000000000007_Skeleton"
    )
    assert step.expected_object_paths == (
        (
            "/Game/Generated/SHAR/test/asset_0000000000000007."
            "asset_0000000000000007"
        ),
        f"{skeleton_package}.asset_0000000000000007_Skeleton",
    )
    rollback_orders = tuple(
        output.rollback_order for output in step.rollback_outputs
    )
    assert rollback_orders == (0, 1)
