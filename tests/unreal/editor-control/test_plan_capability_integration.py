# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
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

# ruff: noqa: EM101, PLC0415, PLR6301, TRY003
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
    plan_root = tmp_path / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_texture_operation=True,
        texture_source_revision=source_revision,
    )
    source = tmp_path / "extracted" / "texture" / "image.png"
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
    plan_root = tmp_path / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_static_mesh_operation=True,
        static_mesh_source_revision=source_revision,
    )
    source = tmp_path / "fbx-assets" / "static" / "model.fbx"
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
    plan_root = tmp_path / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_skeletal_mesh_operation=True,
        skeletal_mesh_source_revision=source_revision,
    )
    source = tmp_path / "fbx-assets" / "skeletal" / "model.fbx"
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
    plan_root = tmp_path / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_import_operation=True,
        import_source_revision=source_revision,
    )
    source = tmp_path / "extracted" / "dialog" / "audio.wav"
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
    plan_root = tmp_path / "unreal-staging" / "plans"
    _ = write_plan_bundle(
        plan_root,
        with_media_operation=True,
        media_source_revision=source_revision,
    )
    source = tmp_path / "extracted" / "movies" / "intro" / "movie.mov"
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
