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
#   - World-material capability audit and native transaction tests.
# - Must-Not:
#   - Contact Unreal Editor, read generated caches, or mutate repository assets.
# - Allows:
#   - Synthetic live schemas, native state, failures, and rollback evidence.
# - Split-When:
#   - Capability and application fixtures gain independent lifecycles.
# - Merge-When:
#   - Another suite owns identical world-material transaction policy.
# - Summary:
#   - World-material native application tests.
# - Description:
#   - Proves schema gates, exact stage order, read-back, save, and compensation.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Only assets proven absent before mutation may be compensated.
#

"""Tests for world-material native capability and application boundaries."""

from __future__ import annotations

from pathlib import Path
from typing import NamedTuple

from mcp.application.world_material_application import (
    apply_world_material_construction,
)
from mcp.domain.catalog import ToolDefinition
from mcp.domain.catalog import ToolsetDefinition
from mcp.domain.errors import ProtocolError
from mcp.domain.json_types import JsonObject
from mcp.domain.tool_outcome import ToolCallOutcome
from mcp.domain.world_material_capabilities import (
    audit_world_material_capabilities,
)
from mcp.domain.world_material_capabilities import (
    required_world_material_toolsets,
)
from mcp.domain.world_material_construction import (
    CompiledWorldMaterialConstruction,
)
from mcp.domain.world_material_construction import (
    WorldMaterialConstructionReport,
)
from mcp.domain.world_material_construction import WorldMaterialInstanceStep
from mcp.domain.world_material_construction import WorldMaterialMasterStep
from mcp.domain.world_material_construction import WorldMaterialTextureStep
import pytest

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"
_TEXTURE_TOOLSET = "editor_toolset.toolsets.texture.TextureTools"
_MATERIAL_TOOLSET = "SharImportEditor.SharWorldMaterialToolset"
_SHA = "1" * 64
_PRESENTATION = "2" * 64
_RECIPE = "simple-unlit-blend-alpha-alpha-test-on-both-faces"


def _object_schema(
    properties: JsonObject,
    *required: str,
) -> JsonObject:
    return {
        "additionalProperties": False,
        "properties": properties,
        "required": list(required),
        "type": "object",
    }


def _tool(
    toolset: str,
    leaf: str,
    input_schema: JsonObject,
    output_schema: JsonObject,
) -> ToolDefinition:
    return ToolDefinition(
        name=f"{toolset}.{leaf}",
        description="synthetic tool",
        input_schema=input_schema,
        output_schema=output_schema,
    )


def _toolsets(*, omit: str | None = None) -> tuple[ToolsetDefinition, ...]:
    text = {"type": "string"}
    boolean = {"type": "boolean"}
    number = {"type": "number"}
    rgba = _object_schema(
        dict.fromkeys(("a", "b", "g", "r"), number),
        "a",
        "b",
        "g",
        "r",
    )
    boolean_output = _object_schema({"returnValue": boolean}, "returnValue")
    string_output = _object_schema({"returnValue": text}, "returnValue")
    asset_tools = (
        _tool(
            _ASSET_TOOLSET,
            "delete",
            _object_schema({"path": text}, "path"),
            boolean_output,
        ),
        _tool(
            _ASSET_TOOLSET,
            "exists",
            _object_schema({"path": text}, "path"),
            boolean_output,
        ),
        _tool(
            _ASSET_TOOLSET,
            "get_asset_class",
            _object_schema({"asset_path": text}, "asset_path"),
            string_output,
        ),
        _tool(
            _ASSET_TOOLSET,
            "is_dirty",
            _object_schema({"asset_path": text}, "asset_path"),
            boolean_output,
        ),
        _tool(
            _ASSET_TOOLSET,
            "save_assets",
            _object_schema(
                {
                    "asset_paths": {
                        "items": text,
                        "minItems": 1,
                        "type": "array",
                    }
                },
                "asset_paths",
            ),
            boolean_output,
        ),
    )
    texture_tools = (
        _tool(
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
            _object_schema(
                {
                    "returnValue": {
                        "items": {"type": "object"},
                        "type": "array",
                    }
                },
                "returnValue",
            ),
        ),
    )
    master_input = _object_schema(
        {
            "assetName": text,
            "bAlphaTest": boolean,
            "blendFamily": text,
            "folderPath": text,
        },
        "assetName",
        "bAlphaTest",
        "blendFamily",
        "folderPath",
    )
    instance_input = _object_schema(
        {
            "alphaReference": number,
            "assetName": text,
            "bSetAlphaReference": boolean,
            "baseColorTexturePath": text,
            "baseColorTint": rgba,
            "folderPath": text,
            "parentMaterialPath": text,
        },
        "alphaReference",
        "assetName",
        "bSetAlphaReference",
        "baseColorTexturePath",
        "baseColorTint",
        "folderPath",
        "parentMaterialPath",
    )
    material_tools = (
        _tool(
            _MATERIAL_TOOLSET,
            "CreateSimpleUnlitWorldMaster",
            master_input,
            string_output,
        ),
        _tool(
            _MATERIAL_TOOLSET,
            "CreateSimpleUnlitWorldMaterialInstance",
            instance_input,
            string_output,
        ),
    )
    definitions = (
        ToolsetDefinition(_ASSET_TOOLSET, "", asset_tools, {}),
        ToolsetDefinition(_TEXTURE_TOOLSET, "", texture_tools, {}),
        ToolsetDefinition(_MATERIAL_TOOLSET, "", material_tools, {}),
    )
    if omit is None:
        return definitions
    return tuple(
        definition._replace(
            tools=tuple(tool for tool in definition.tools if tool.name != omit)
        )
        for definition in definitions
    )


def _compiled() -> CompiledWorldMaterialConstruction:
    texture_name = f"T_World_{_SHA}"
    texture_package = f"/Game/Generated/SHAR/Textures/World/{texture_name}"
    texture = WorldMaterialTextureStep(
        sha256=_SHA,
        byte_count=3,
        source_path=f"world-assets/textures/texture-{_SHA}.png",
        object_path=f"{texture_package}.{texture_name}",
        package_path=texture_package,
        folder_path="/Game/Generated/SHAR/Textures/World",
        asset_name=texture_name,
    )
    master_name = "M_SHAR_World_simple_unlit_alpha_test"
    master_package = (
        f"/Game/Generated/SHAR/Materials/World/Masters/{master_name}"
    )
    master = WorldMaterialMasterStep(
        recipe_identity=_RECIPE,
        object_path=f"{master_package}.{master_name}",
        package_path=master_package,
        folder_path="/Game/Generated/SHAR/Materials/World/Masters",
        asset_name=master_name,
        blend_family="alpha",
        alpha_test=True,
    )
    instance_name = f"MI_World_{_PRESENTATION}"
    instance_package = (
        f"/Game/Generated/SHAR/Materials/World/Instances/{instance_name}"
    )
    instance = WorldMaterialInstanceStep(
        presentation_sha256=_PRESENTATION,
        recipe_identity=_RECIPE,
        object_path=f"{instance_package}.{instance_name}",
        package_path=instance_package,
        folder_path="/Game/Generated/SHAR/Materials/World/Instances",
        asset_name=instance_name,
        parent_material_path=master.object_path,
        base_color_texture_path=texture.object_path,
        texture_sha256=_SHA,
        base_color_tint=(0.25, 0.5, 0.75, 1.0),
        set_alpha_reference=True,
        alpha_reference=0.375,
    )
    return CompiledWorldMaterialConstruction(
        WorldMaterialConstructionReport(1, 1, 1, 0),
        (texture,),
        (master,),
        (instance,),
    )


def _outcome(value: object) -> ToolCallOutcome:
    content: JsonObject = {"returnValue": value}
    return ToolCallOutcome(
        raw={}, text="", structured_content=content, is_error=False
    )


class _Behavior(NamedTuple):
    raise_after_leaf: str | None = None
    wrong_class_leaf: str | None = None


class _SyntheticClient:
    def __init__(
        self,
        *,
        preexisting: dict[str, str] | None = None,
        behavior: _Behavior | None = None,
    ) -> None:
        self.assets = dict(preexisting or {})
        self.dirty: set[str] = set()
        self.calls: list[tuple[str, JsonObject]] = []
        self.behavior = _Behavior() if behavior is None else behavior

    def call_tool(
        self,
        toolset_name: str,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        del toolset_name
        leaf = tool_name.rsplit(".", 1)[-1]
        self.calls.append((leaf, arguments))
        asset_outcome = self._asset_call(leaf, arguments)
        if asset_outcome is not None:
            return asset_outcome
        if leaf == "import_file":
            package = (
                f'{arguments["folder_path"]}/{arguments["asset_name"]}'
            )
            return self._create(
                leaf,
                package,
                "Texture2D",
                [{"assetClass": "Texture2D", "packagePath": package}],
            )
        if leaf == "CreateSimpleUnlitWorldMaster":
            package = f'{arguments["folderPath"]}/{arguments["assetName"]}'
            object_path = f'{package}.{arguments["assetName"]}'
            return self._create(leaf, package, "Material", object_path)
        if leaf == "CreateSimpleUnlitWorldMaterialInstance":
            package = f'{arguments["folderPath"]}/{arguments["assetName"]}'
            object_path = f'{package}.{arguments["assetName"]}'
            return self._create(
                leaf,
                package,
                "MaterialInstanceConstant",
                object_path,
            )
        raise AssertionError(f"unexpected synthetic tool {leaf}")

    def _asset_call(
        self,
        leaf: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome | None:
        if leaf == "exists":
            return _outcome(str(arguments["path"]) in self.assets)
        if leaf == "get_asset_class":
            return _outcome(self.assets[str(arguments["asset_path"])])
        if leaf == "is_dirty":
            return _outcome(str(arguments["asset_path"]) in self.dirty)
        if leaf == "save_assets":
            raw_paths = arguments["asset_paths"]
            if not isinstance(raw_paths, list):
                raise AssertionError("synthetic save paths are not an array")
            paths = [str(path) for path in raw_paths]
            complete = all(path in self.assets for path in paths)
            if complete:
                self.dirty.difference_update(paths)
            return _outcome(complete)
        if leaf == "delete":
            path = str(arguments["path"])
            existed = self.assets.pop(path, None) is not None
            self.dirty.discard(path)
            return _outcome(existed)
        return None

    def _create(
        self,
        leaf: str,
        package: str,
        target_class: str,
        result: object,
    ) -> ToolCallOutcome:
        actual_class = (
            "Material"
            if self.behavior.wrong_class_leaf == leaf
            else target_class
        )
        self.assets[package] = actual_class
        self.dirty.add(package)
        if self.behavior.raise_after_leaf == leaf:
            raise TimeoutError("synthetic lost construction response")
        return _outcome(result)


def _sources(tmp_path: Path) -> dict[str, Path]:
    source = tmp_path / "texture.png"
    source.write_bytes(b"png")
    return {_SHA: source}


def test_capability_audit_accepts_exact_world_material_surface() -> None:
    compiled = _compiled()
    report = audit_world_material_capabilities(compiled, _toolsets())
    assert report.complete
    assert report.construction_count == 3
    assert report.required_tool_count == 8
    assert report.available_tool_count == 8
    assert report.missing_tools == ()
    assert report.incompatible_tools == ()
    assert required_world_material_toolsets(compiled) == (
        _MATERIAL_TOOLSET,
        _ASSET_TOOLSET,
        _TEXTURE_TOOLSET,
    )


def test_capability_audit_reports_missing_instance_factory() -> None:
    compiled = _compiled()
    identity = (
        f"{_MATERIAL_TOOLSET}.CreateSimpleUnlitWorldMaterialInstance"
    )
    report = audit_world_material_capabilities(
        compiled,
        _toolsets(omit=identity),
    )
    assert not report.complete
    assert report.missing_tools == (identity,)
    assert report.available_tool_count == 7


def test_application_creates_saves_and_verifies_all_stages(
    tmp_path: Path,
) -> None:
    compiled = _compiled()
    capabilities = audit_world_material_capabilities(compiled, _toolsets())
    client = _SyntheticClient()
    report = apply_world_material_construction(
        client,
        compiled,
        capabilities,
        _sources(tmp_path),
    )
    assert report.created_count == 3
    assert report.saved_count == 3
    assert report.verified_count == 3
    assert client.dirty == set()
    assert client.assets == {
        compiled.textures[0].package_path: "Texture2D",
        compiled.masters[0].package_path: "Material",
        compiled.instances[0].package_path: "MaterialInstanceConstant",
    }
    mutations = tuple(
        leaf
        for leaf, _arguments in client.calls
        if leaf in {
            "import_file",
            "CreateSimpleUnlitWorldMaster",
            "CreateSimpleUnlitWorldMaterialInstance",
        }
    )
    assert mutations == (
        "import_file",
        "CreateSimpleUnlitWorldMaster",
        "CreateSimpleUnlitWorldMaterialInstance",
    )
    assert sum(leaf == "save_assets" for leaf, _ in client.calls) == 3


def test_application_refuses_preexisting_destination_before_mutation(
    tmp_path: Path,
) -> None:
    compiled = _compiled()
    capabilities = audit_world_material_capabilities(compiled, _toolsets())
    master = compiled.masters[0]
    client = _SyntheticClient(preexisting={master.package_path: "Material"})
    with pytest.raises(ProtocolError, match="destination already exists"):
        apply_world_material_construction(
            client,
            compiled,
            capabilities,
            _sources(tmp_path),
        )
    assert client.assets == {master.package_path: "Material"}
    leaves = tuple(leaf for leaf, _ in client.calls)
    assert "import_file" not in leaves
    assert "delete" not in leaves


def test_application_rejects_stale_capability_audit_before_native_call(
    tmp_path: Path,
) -> None:
    compiled = _compiled()
    capabilities = audit_world_material_capabilities(compiled, _toolsets())
    stale = capabilities._replace(construction_revision="0" * 64)
    client = _SyntheticClient()
    with pytest.raises(ProtocolError, match="capability audit is stale"):
        apply_world_material_construction(
            client,
            compiled,
            stale,
            _sources(tmp_path),
        )
    assert client.calls == []


def test_lost_master_response_compensates_master_then_texture(
    tmp_path: Path,
) -> None:
    compiled = _compiled()
    capabilities = audit_world_material_capabilities(compiled, _toolsets())
    client = _SyntheticClient(
        behavior=_Behavior(raise_after_leaf="CreateSimpleUnlitWorldMaster")
    )
    with pytest.raises(TimeoutError, match="lost construction response"):
        apply_world_material_construction(
            client,
            compiled,
            capabilities,
            _sources(tmp_path),
        )
    assert client.assets == {}
    deletes = tuple(
        str(arguments["path"])
        for leaf, arguments in client.calls
        if leaf == "delete"
    )
    assert deletes == (
        compiled.masters[0].package_path,
        compiled.textures[0].package_path,
    )


def test_instance_class_drift_compensates_all_assets_in_reverse_order(
    tmp_path: Path,
) -> None:
    compiled = _compiled()
    capabilities = audit_world_material_capabilities(compiled, _toolsets())
    client = _SyntheticClient(
        behavior=_Behavior(
            wrong_class_leaf="CreateSimpleUnlitWorldMaterialInstance"
        )
    )
    with pytest.raises(ProtocolError, match="unexpected class"):
        apply_world_material_construction(
            client,
            compiled,
            capabilities,
            _sources(tmp_path),
        )
    assert client.assets == {}
    deletes = tuple(
        str(arguments["path"])
        for leaf, arguments in client.calls
        if leaf == "delete"
    )
    assert deletes == (
        compiled.instances[0].package_path,
        compiled.masters[0].package_path,
        compiled.textures[0].package_path,
    )
