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
#   - Serialized native import-plan application tests.
# - Must-Not:
#   - Contact Unreal Editor or read generated repository assets.
# - Allows:
#   - Synthetic native state, outcomes, failures, and compensation evidence.
# - Split-When:
#   - Split when another transaction family gains independent tests.
# - Merge-When:
#   - Merge when native plan application has no independent lifecycle.
# - Summary:
#   - Unreal native import-plan application tests.
# - Description:
#   - Proves complete execution, preflight refusal, and reverse compensation.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Only assets proven absent before this transaction may be deleted.
#

"""Tests for serialized native Unreal import-plan application."""

from __future__ import annotations

# ruff: noqa: EM101, EM102, FBT003, PLR0911, PLR0912, TRY003
# cspell:ignore FBT
from pathlib import Path

from mcp.application.plan_application import apply_import_plan
from mcp.domain.errors import ProtocolError
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import JsonValue
from mcp.domain.plan_bundle import PlanBundleReport
from mcp.domain.plan_bundle import PlanOperation
from mcp.domain.plan_bundle import ValidatedPlanBundle
from mcp.domain.plan_capabilities import PlanCapabilityReport
from mcp.domain.plan_execution import CompiledExecutionPlan
from mcp.domain.plan_execution import compile_execution_plan
from mcp.domain.tool_outcome import ToolCallOutcome
import pytest

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"


def _outcome(return_value: JsonValue) -> ToolCallOutcome:
    structured: JsonObject = {"returnValue": return_value}
    return ToolCallOutcome(
        raw={"content": [], "structuredContent": structured},
        text="",
        structured_content=structured,
        is_error=False,
    )


class _TextOutcomeClient:
    def __init__(self, text: str) -> None:
        self.text = text
        self.calls: list[tuple[str, JsonObject]] = []

    def call_tool(
        self,
        toolset_name: str,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        del toolset_name
        self.calls.append((tool_name, arguments))
        return ToolCallOutcome(
            raw={
                "content": [{"type": "text", "text": self.text}],
            },
            text=self.text,
            structured_content=None,
            is_error=False,
        )


class _SyntheticClient:
    def __init__(
        self,
        *,
        preexisting: dict[str, str] | None = None,
        wrong_class_on_import: int | None = None,
        raise_after_import: int | None = None,
        preexisting_payloads: dict[str, str] | None = None,
    ) -> None:
        self.assets = dict(preexisting or {})
        self.media_payloads = dict(preexisting_payloads or {})
        self.calls: list[tuple[str, JsonObject]] = []
        self.import_count = 0
        self.wrong_class_on_import = wrong_class_on_import
        self.raise_after_import = raise_after_import

    def call_tool(
        self,
        toolset_name: str,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        del toolset_name
        self.calls.append((tool_name, arguments))
        leaf = tool_name.rsplit(".", 1)[-1]
        if leaf == "exists":
            return _outcome(str(arguments["path"]) in self.assets)
        if leaf in {"ImportFileMediaSource", "ImportStaticMesh", "import_file"}:
            self.import_count += 1
            is_native = leaf != "import_file"
            is_media = leaf == "ImportFileMediaSource"
            asset_name = str(
                arguments["assetName"] if is_native else arguments["asset_name"]
            )
            folder_path = str(
                arguments["folderPath"]
                if is_native
                else arguments["folder_path"]
            )
            package_path = f"{folder_path}/{asset_name}"
            target_class = (
                "UnexpectedClass"
                if self.import_count == self.wrong_class_on_import
                else "FileMediaSource"
                if is_media
                else "StaticMesh"
                if leaf == "ImportStaticMesh"
                else "Texture2D"
            )
            self.assets[package_path] = target_class
            object_path = f"{package_path}.{asset_name}"
            if is_media:
                relative_package = package_path.removeprefix(
                    "/Game/Generated/SHAR/"
                )
                self.media_payloads[object_path] = (
                    f"./Movies/Generated/SHAR/{relative_package}.mov"
                )
            if self.import_count == self.raise_after_import:
                raise TimeoutError("synthetic lost import response")
            if is_native:
                return _outcome([object_path])
            return _outcome([{"packagePath": package_path}])
        if leaf == "FileMediaSourcePayloadExists":
            return _outcome(str(arguments["assetPath"]) in self.media_payloads)
        if leaf == "GetFileMediaSourcePath":
            return _outcome(
                self.media_payloads.get(str(arguments["assetPath"]), "")
            )
        if leaf == "DeleteFileMediaSourcePayload":
            existed = (
                self.media_payloads.pop(str(arguments["assetPath"]), None)
                is not None
            )
            return _outcome(existed)
        if leaf == "get_asset_class":
            return _outcome(self.assets[str(arguments["asset_path"])])
        if leaf == "save_assets":
            return _outcome(True)
        if leaf == "is_dirty":
            return _outcome(False)
        if leaf == "delete":
            self.assets.pop(str(arguments["path"]), None)
            return _outcome(True)
        raise AssertionError(f"unexpected tool: {tool_name}")


def _operation(index: int, *, readiness: str = "ready") -> PlanOperation:
    asset_name = f"image_{index}"
    return PlanOperation(
        plan_id="asset-import-plan",
        operation_id=f"operation-{index:016x}",
        package_identity=f"package-{index}",
        source_identity=f"source-{index}",
        source_format="image",
        target_family="texture",
        source_path=f"extracted/{asset_name}.png",
        source_revision=f"{index:064x}",
        destination=f"/Game/Generated/SHAR/test/{asset_name}.{asset_name}",
        target_class="Texture2D",
        importer="texture-factory",
        import_profile="shar-texture-v1",
        dependencies=(),
        readiness=readiness,
        world_owned=False,
        runtime_bound=True,
    )


def _static_mesh_operation(index: int) -> PlanOperation:
    asset_name = f"model_{index}"
    return PlanOperation(
        plan_id="asset-import-plan",
        operation_id=f"operation-{index:016x}",
        package_identity=f"model-package-{index}",
        source_identity=f"model-source-{index}",
        source_format="fbx",
        target_family="model",
        source_path=f"fbx-assets/{asset_name}.fbx",
        source_revision=f"{index:064x}",
        destination=(
            f"/Game/Generated/SHAR/models/static/{asset_name}.{asset_name}"
        ),
        target_class="StaticMesh",
        importer="asset-tools-fbx",
        import_profile="shar-fbx-static-v1",
        dependencies=(),
        readiness="ready",
        world_owned=False,
        runtime_bound=True,
    )


def _media_operation(index: int) -> PlanOperation:
    asset_name = f"movie_{index}"
    return PlanOperation(
        plan_id="asset-import-plan",
        operation_id=f"operation-{index:016x}",
        package_identity=f"media-package-{index}",
        source_identity=f"media-source-{index}",
        source_format="hap",
        target_family="media",
        source_path=f"extracted/movies/{asset_name}.mov",
        source_revision=f"{index:064x}",
        destination=f"/Game/Generated/SHAR/movies/{asset_name}.{asset_name}",
        target_class="FileMediaSource",
        importer="media-source-movie",
        import_profile="shar-hap-movie-v1",
        dependencies=(),
        readiness="ready",
        world_owned=False,
        runtime_bound=True,
    )


def _compiled(*operations: PlanOperation) -> CompiledExecutionPlan:
    report = PlanBundleReport(
        revision="a" * 64,
        source_manifest_revision="b" * 64,
        engine_contract_revision="shar-unreal-porting-contract-v1",
        target_engine_version="5.8.1",
        target_platform="editor",
        semantic_blocker_count=0,
        operation_count=len(operations),
        readiness_counts={},
        plans=(),
    )
    return compile_execution_plan(ValidatedPlanBundle(report, operations))


def _capabilities(compiled: CompiledExecutionPlan) -> PlanCapabilityReport:
    return PlanCapabilityReport(
        bundle_revision=compiled.report.bundle_revision,
        compiled_count=compiled.report.compiled_count,
        plan_complete=compiled.report.complete,
        required_tool_count=6,
        available_tool_count=6,
        missing_tools=(),
        incompatible_tools=(),
    )


def _sources(
    tmp_path: Path,
    compiled: CompiledExecutionPlan,
) -> dict[str, Path]:
    result: dict[str, Path] = {}
    for step in compiled.imports:
        extension = (
            "mov"
            if step.has_external_payload
            else "fbx"
            if step.route_id == "static-mesh-fbx-v1"
            else "png"
        )
        source = tmp_path / f"{step.operation_id}.{extension}"
        source.write_bytes(b"fixture")
        result[step.operation_id] = source
    return result


def test_applies_complete_plan_with_independent_save_and_readback(
    tmp_path: Path,
) -> None:
    compiled = _compiled(_operation(1), _operation(2))
    client = _SyntheticClient()
    report = apply_import_plan(
        client,
        compiled,
        _capabilities(compiled),
        _sources(tmp_path, compiled),
    )
    assert report.imported_count == 2
    assert report.saved_count == 2
    assert report.verified_count == 2
    assert set(client.assets) == {
        step.package_path for step in compiled.imports
    }
    leaves = tuple(name.rsplit(".", 1)[-1] for name, _args in client.calls)
    assert leaves.count("import_file") == 2
    assert leaves.count("save_assets") == 2
    assert leaves.count("get_asset_class") == 2
    assert leaves.count("is_dirty") == 2
    assert "delete" not in leaves


@pytest.mark.parametrize(
    ("text", "message"),
    (
        ("not-json", "text result is not valid JSON"),
        (
            '{"returnValue":false,"returnValue":true}',
            "duplicate JSON key",
        ),
    ),
)
def test_rejects_invalid_text_wrapped_result_before_import(
    tmp_path: Path,
    text: str,
    message: str,
) -> None:
    compiled = _compiled(_static_mesh_operation(8))
    client = _TextOutcomeClient(text)
    with pytest.raises(ProtocolError, match=message):
        apply_import_plan(
            client,
            compiled,
            _capabilities(compiled),
            _sources(tmp_path, compiled),
        )
    assert tuple(name.rsplit(".", 1)[-1] for name, _ in client.calls) == (
        "exists",
    )


def test_applies_static_mesh_through_owned_native_import(
    tmp_path: Path,
) -> None:
    compiled = _compiled(_static_mesh_operation(9))
    client = _SyntheticClient()
    sources = _sources(tmp_path, compiled)
    report = apply_import_plan(
        client,
        compiled,
        _capabilities(compiled),
        sources,
    )
    step = compiled.imports[0]
    assert report.imported_count == 1
    assert report.saved_count == 1
    assert report.verified_count == 1
    assert client.assets == {step.package_path: "StaticMesh"}
    native_calls = [
        arguments
        for name, arguments in client.calls
        if name.rsplit(".", 1)[-1] == "ImportStaticMesh"
    ]
    assert native_calls == [
        {
            "assetName": step.asset_name,
            "folderPath": step.folder_path,
            "sourceFile": str(sources[step.operation_id]),
        }
    ]


def test_rejects_incomplete_plan_before_any_native_call(tmp_path: Path) -> None:
    compiled = _compiled(_operation(1, readiness="requires-conversion"))
    client = _SyntheticClient()
    with pytest.raises(ProtocolError, match="plan is incomplete"):
        apply_import_plan(
            client,
            compiled,
            _capabilities(compiled),
            _sources(tmp_path, compiled),
        )
    assert client.calls == []


def test_rejects_preexisting_destination_without_deleting_it(
    tmp_path: Path,
) -> None:
    compiled = _compiled(_operation(1), _operation(2))
    first_path = compiled.imports[0].package_path
    client = _SyntheticClient(preexisting={first_path: "Texture2D"})
    with pytest.raises(ProtocolError, match="already exists"):
        apply_import_plan(
            client,
            compiled,
            _capabilities(compiled),
            _sources(tmp_path, compiled),
        )
    assert client.assets == {first_path: "Texture2D"}
    leaves = tuple(name.rsplit(".", 1)[-1] for name, _args in client.calls)
    assert leaves == ("exists",)


def test_compensates_asset_created_before_lost_import_response(
    tmp_path: Path,
) -> None:
    compiled = _compiled(_operation(1))
    client = _SyntheticClient(raise_after_import=1)
    with pytest.raises(TimeoutError, match="lost import response"):
        apply_import_plan(
            client,
            compiled,
            _capabilities(compiled),
            _sources(tmp_path, compiled),
        )
    assert client.assets == {}
    leaves = tuple(name.rsplit(".", 1)[-1] for name, _args in client.calls)
    assert "delete" in leaves


def test_compensates_all_created_assets_in_reverse_order(
    tmp_path: Path,
) -> None:
    compiled = _compiled(_operation(1), _operation(2))
    client = _SyntheticClient(wrong_class_on_import=2)
    with pytest.raises(ProtocolError, match="unexpected asset class"):
        apply_import_plan(
            client,
            compiled,
            _capabilities(compiled),
            _sources(tmp_path, compiled),
        )
    assert client.assets == {}
    deletes = tuple(
        str(arguments["path"])
        for name, arguments in client.calls
        if name.rsplit(".", 1)[-1] == "delete"
    )
    assert deletes == (
        compiled.imports[1].package_path,
        compiled.imports[0].package_path,
    )


def test_applies_media_asset_and_external_payload_as_one_transaction(
    tmp_path: Path,
) -> None:
    compiled = _compiled(_media_operation(11))
    client = _SyntheticClient()
    report = apply_import_plan(
        client,
        compiled,
        _capabilities(compiled),
        _sources(tmp_path, compiled),
    )
    step = compiled.imports[0]
    assert report.imported_count == 1
    assert client.assets == {step.package_path: "FileMediaSource"}
    assert client.media_payloads == {
        step.destination: step.external_payload_path,
    }
    leaves = tuple(name.rsplit(".", 1)[-1] for name, _args in client.calls)
    assert "GetFileMediaSourcePath" in leaves
    assert leaves.count("FileMediaSourcePayloadExists") == 3
    assert "DeleteFileMediaSourcePayload" not in leaves


def test_rejects_preexisting_media_payload_before_import(
    tmp_path: Path,
) -> None:
    compiled = _compiled(_media_operation(12))
    step = compiled.imports[0]
    client = _SyntheticClient(
        preexisting_payloads={step.destination: str(step.external_payload_path)}
    )
    with pytest.raises(ProtocolError, match="external payload already exists"):
        apply_import_plan(
            client,
            compiled,
            _capabilities(compiled),
            _sources(tmp_path, compiled),
        )
    assert client.assets == {}
    assert client.media_payloads == {
        step.destination: step.external_payload_path,
    }


def test_compensates_media_payload_before_asset_after_lost_response(
    tmp_path: Path,
) -> None:
    compiled = _compiled(_media_operation(13))
    client = _SyntheticClient(raise_after_import=1)
    with pytest.raises(TimeoutError, match="lost import response"):
        apply_import_plan(
            client,
            compiled,
            _capabilities(compiled),
            _sources(tmp_path, compiled),
        )
    assert client.assets == {}
    assert client.media_payloads == {}
    leaves = tuple(name.rsplit(".", 1)[-1] for name, _args in client.calls)
    payload_delete = leaves.index("DeleteFileMediaSourcePayload")
    asset_delete = leaves.index("delete")
    assert payload_delete < asset_delete
