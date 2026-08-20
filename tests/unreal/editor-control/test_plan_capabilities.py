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
#   - Live native capability audit tests for compiled plan execution.
# - Must-Not:
#   - Open MCP sessions or invoke native Unreal tools.
# - Allows:
#   - Synthetic Toolset definitions and representative schema contracts.
# - Split-When:
#   - Split when another execution family needs independent capability fixtures.
# - Merge-When:
#   - Merge when capability audit has no independent policy.
# - Summary:
#   - Unreal plan capability auditor tests.
# - Description:
#   - Proves exact mutation, persistence, and read-back schemas are required.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Missing or incompatible tools prevent a complete report.
#

"""Tests for live native capability audits of compiled Unreal plans."""

from __future__ import annotations

from mcp.domain.catalog import ToolDefinition
from mcp.domain.catalog import ToolsetDefinition
from mcp.domain.json_types import JsonObject
from mcp.domain.plan_bundle import PlanBundleReport
from mcp.domain.plan_bundle import PlanOperation
from mcp.domain.plan_bundle import ValidatedPlanBundle
from mcp.domain.plan_capabilities import audit_plan_capabilities
from mcp.domain.plan_capabilities import required_toolsets
from mcp.domain.plan_execution import CompiledExecutionPlan
from mcp.domain.plan_execution import compile_execution_plan

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"
_TEXTURE_TOOLSET = "editor_toolset.toolsets.texture.TextureTools"
_IMPORT_TOOLSET = "SharImportEditor.SharImportToolset"


def _object_schema(
    properties: JsonObject,
    *required: str,
) -> JsonObject:
    return {
        "type": "object",
        "properties": properties,
        "required": list(required),
        "additionalProperties": False,
    }


def _tool(
    toolset: str,
    leaf: str,
    input_schema: JsonObject,
    output_schema: JsonObject,
) -> ToolDefinition:
    return ToolDefinition(
        name=f"{toolset}.{leaf}",
        description="Synthetic live schema.",
        input_schema=input_schema,
        output_schema=output_schema,
    )


def _toolsets(
    *, omit: str = "", incompatible_import: bool = False
) -> tuple[ToolsetDefinition, ...]:
    text = {"type": "string"}
    boolean = {"type": "boolean"}
    output_boolean = _object_schema({"returnValue": boolean}, "returnValue")
    asset_tools = (
        _tool(
            _ASSET_TOOLSET,
            "delete",
            _object_schema({"path": text}, "path"),
            output_boolean,
        ),
        _tool(
            _ASSET_TOOLSET,
            "exists",
            _object_schema({"path": text}, "path"),
            output_boolean,
        ),
        _tool(
            _ASSET_TOOLSET,
            "get_asset_class",
            _object_schema({"asset_path": text}, "asset_path"),
            _object_schema({"returnValue": text}, "returnValue"),
        ),
        _tool(
            _ASSET_TOOLSET,
            "is_dirty",
            _object_schema({"asset_path": text}, "asset_path"),
            output_boolean,
        ),
        _tool(
            _ASSET_TOOLSET,
            "save_assets",
            _object_schema(
                {
                    "asset_paths": {
                        "type": "array",
                        "items": text,
                        "minItems": 1,
                    }
                },
                "asset_paths",
            ),
            output_boolean,
        ),
    )
    import_output = (
        _object_schema({"returnValue": boolean}, "returnValue")
        if incompatible_import
        else _object_schema(
            {
                "returnValue": {
                    "type": "array",
                    "items": {"type": "object"},
                }
            },
            "returnValue",
        )
    )
    texture_import = _tool(
        _TEXTURE_TOOLSET,
        "import_file",
        _object_schema(
            {
                "asset_name": text,
                "folder_path": text,
                "source_file": text,
            },
            "asset_name",
            "folder_path",
            "source_file",
        ),
        import_output,
    )
    skeletal_import = _tool(
        _IMPORT_TOOLSET,
        "ImportSkeletalMesh",
        _object_schema(
            {
                "assetName": text,
                "folderPath": text,
                "sourceFile": text,
            },
            "assetName",
            "folderPath",
            "sourceFile",
        ),
        _object_schema(
            {
                "returnValue": {
                    "type": "array",
                    "items": text,
                }
            },
            "returnValue",
        ),
    )
    static_import = _tool(
        _IMPORT_TOOLSET,
        "ImportStaticMesh",
        _object_schema(
            {
                "assetName": text,
                "folderPath": text,
                "sourceFile": text,
            },
            "assetName",
            "folderPath",
            "sourceFile",
        ),
        _object_schema(
            {
                "returnValue": {
                    "type": "array",
                    "items": text,
                }
            },
            "returnValue",
        ),
    )
    asset_definition = ToolsetDefinition(
        name=_ASSET_TOOLSET,
        description="Assets.",
        tools=tuple(tool for tool in asset_tools if tool.name != omit),
        raw_schema={},
    )
    texture_definition = ToolsetDefinition(
        name=_TEXTURE_TOOLSET,
        description="Textures.",
        tools=(() if texture_import.name == omit else (texture_import,)),
        raw_schema={},
    )
    import_definition = ToolsetDefinition(
        name=_IMPORT_TOOLSET,
        description="SHAR imports.",
        tools=tuple(
            tool
            for tool in (skeletal_import, static_import)
            if tool.name != omit
        ),
        raw_schema={},
    )
    return asset_definition, texture_definition, import_definition


def _compiled_texture() -> CompiledExecutionPlan:
    operation = PlanOperation(
        plan_id="asset-import-plan",
        operation_id="operation-0000000000000001",
        package_identity="package-texture",
        source_identity="source-texture",
        source_format="image",
        target_family="texture",
        source_path="extracted/image.png",
        source_revision="a" * 64,
        destination="/Game/Generated/SHAR/test/image.image",
        target_class="Texture2D",
        importer="texture-factory",
        import_profile="shar-texture-v1",
        dependencies=(),
        readiness="ready",
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
        readiness_counts={"ready": 1},
        plans=(),
    )
    return compile_execution_plan(ValidatedPlanBundle(report, (operation,)))


def _compiled_static_mesh() -> CompiledExecutionPlan:
    operation = PlanOperation(
        plan_id="asset-import-plan",
        operation_id="operation-0000000000000002",
        package_identity="package-model",
        source_identity="source-model",
        source_format="fbx",
        target_family="model",
        source_path="fbx-assets/static/model.fbx",
        source_revision="d" * 64,
        destination="/Game/Generated/SHAR/models/static/model.model",
        target_class="StaticMesh",
        importer="asset-tools-fbx",
        import_profile="shar-fbx-static-v1",
        dependencies=(),
        readiness="ready",
        world_owned=False,
        runtime_bound=True,
    )
    report = PlanBundleReport(
        revision="e" * 64,
        source_manifest_revision="f" * 64,
        engine_contract_revision="shar-unreal-porting-contract-v1",
        target_engine_version="5.8.1",
        target_platform="editor",
        semantic_blocker_count=0,
        operation_count=1,
        readiness_counts={"ready": 1},
        plans=(),
    )
    return compile_execution_plan(ValidatedPlanBundle(report, (operation,)))


def _compiled_skeletal_mesh() -> CompiledExecutionPlan:
    operation = PlanOperation(
        plan_id="asset-import-plan",
        operation_id="operation-0000000000000003",
        package_identity="package-skeletal-model",
        source_identity="source-skeletal-model",
        source_format="fbx",
        target_family="model",
        source_path="fbx-assets/skeletal/model.fbx",
        source_revision="1" * 64,
        destination="/Game/Generated/SHAR/models/skeletal/model.model",
        target_class="SkeletalMesh",
        importer="asset-tools-fbx",
        import_profile="shar-fbx-skeletal-v1",
        dependencies=(),
        readiness="ready",
        world_owned=False,
        runtime_bound=True,
    )
    report = PlanBundleReport(
        revision="2" * 64,
        source_manifest_revision="3" * 64,
        engine_contract_revision="shar-unreal-porting-contract-v1",
        target_engine_version="5.8.1",
        target_platform="editor",
        semantic_blocker_count=0,
        operation_count=1,
        readiness_counts={"ready": 1},
        plans=(),
    )
    return compile_execution_plan(ValidatedPlanBundle(report, (operation,)))


def test_accepts_complete_live_import_save_and_readback_surface() -> None:
    compiled = _compiled_texture()
    report = audit_plan_capabilities(compiled, _toolsets())
    assert report.complete
    assert report.native_surface_complete
    assert report.required_tool_count == 6
    assert report.available_tool_count == 6
    assert report.missing_tools == ()
    assert report.incompatible_tools == ()
    assert required_toolsets(compiled) == (_ASSET_TOOLSET, _TEXTURE_TOOLSET)
    assert "verified-source" not in str(report.to_json())


def test_static_mesh_requires_owned_native_import_schema() -> None:
    compiled = _compiled_static_mesh()
    report = audit_plan_capabilities(compiled, _toolsets())
    assert report.complete
    assert report.native_surface_complete
    assert report.required_tool_count == 6
    assert required_toolsets(compiled) == (_IMPORT_TOOLSET, _ASSET_TOOLSET)

    missing_identity = f"{_IMPORT_TOOLSET}.ImportStaticMesh"
    missing = audit_plan_capabilities(
        compiled,
        _toolsets(omit=missing_identity),
    )
    assert not missing.complete
    assert missing.missing_tools == (missing_identity,)


def test_skeletal_mesh_requires_owned_companion_aware_import_schema() -> None:
    compiled = _compiled_skeletal_mesh()
    report = audit_plan_capabilities(compiled, _toolsets())
    assert report.complete
    assert report.native_surface_complete
    assert report.required_tool_count == 6
    assert required_toolsets(compiled) == (_IMPORT_TOOLSET, _ASSET_TOOLSET)

    missing_identity = f"{_IMPORT_TOOLSET}.ImportSkeletalMesh"
    missing = audit_plan_capabilities(
        compiled,
        _toolsets(omit=missing_identity),
    )
    assert not missing.complete
    assert missing.missing_tools == (missing_identity,)


def test_reports_missing_and_schema_incompatible_tools() -> None:
    compiled = _compiled_texture()
    missing_identity = f"{_ASSET_TOOLSET}.get_asset_class"
    missing = audit_plan_capabilities(
        compiled,
        _toolsets(omit=missing_identity),
    )
    assert not missing.complete
    assert missing.missing_tools == (missing_identity,)
    assert missing.available_tool_count == 5

    incompatible = audit_plan_capabilities(
        compiled,
        _toolsets(incompatible_import=True),
    )
    assert not incompatible.complete
    assert incompatible.incompatible_tools == (
        f"{_TEXTURE_TOOLSET}.import_file",
    )
