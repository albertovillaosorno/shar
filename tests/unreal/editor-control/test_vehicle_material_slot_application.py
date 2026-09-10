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
#   - Vehicle material slot compilation, capability, and application tests.
# - Must-Not:
#   - Contact Unreal Editor, read generated caches, or create real assets.
# - Allows:
#   - Synthetic schemas, dependencies, slot state, save, and rollback evidence.
# - Split-When:
#   - Compiler and application fixtures gain independent complex lifecycles.
# - Merge-When:
#   - Another suite owns identical selected-slot publication guarantees.
# - Summary:
#   - Vehicle material slot publication tests.
# - Description:
#   - Proves exact skeletal join, live schema audit, CAS save, and compensation.
# - Usage:
#   - Run through repository Python validation.
# - Defaults:
#   - Publication begins only from clean mesh state with selected null slots.
#

"""Tests for vehicle material slot publication boundaries."""

from __future__ import annotations

from dataclasses import dataclass

from mcp.application.vehicle_material_slot_application import (
    apply_vehicle_material_slot_binding,
)
from mcp.domain.catalog import ToolDefinition
from mcp.domain.catalog import ToolsetDefinition
from mcp.domain.errors import ProtocolError
from mcp.domain.json_types import JsonObject
from mcp.domain.plan_execution import CompiledExecutionPlan
from mcp.domain.plan_execution import NativeImportStep
from mcp.domain.plan_execution import PlanExecutionReport
from mcp.domain.tool_outcome import ToolCallOutcome
from mcp.domain.vehicle_material_construction import VehicleMaterialInstanceStep
from mcp.domain.vehicle_material_selection import (
    CompiledVehicleMaterialSelection,
)
from mcp.domain.vehicle_material_selection import VehicleMaterialSelectionReport
from mcp.domain.vehicle_material_slot_binding import (
    CompiledVehicleMaterialSlotBinding,
)
from mcp.domain.vehicle_material_slot_binding import (
    compile_vehicle_material_slot_binding,
)
from mcp.domain.vehicle_material_slot_capabilities import (
    audit_vehicle_material_slot_capabilities,
)
from mcp.domain.vehicle_material_slot_capabilities import (
    required_vehicle_material_slot_toolsets,
)
import pytest

_ASSET = "editor_toolset.toolsets.asset.AssetTools"
_MATERIAL = "SharImportEditor.SharVehicleMaterialToolset"
_MESH_PACKAGE = (
    "/Game/Generated/SHAR/cars/extracted_art_cars_sedana_Skeletal/"
    "extracted_art_cars_sedana_Skeletal"
)
_MESH_OBJECT = f"{_MESH_PACKAGE}.extracted_art_cars_sedana_Skeletal"
_INSTANCE_ROOT = "/Game/Generated/SHAR/Materials/Vehicles/Instances"


def _instance(slot: int, digit: str) -> VehicleMaterialInstanceStep:
    name = f"MI_Vehicle_{digit * 64}"
    package = f"{_INSTANCE_ROOT}/{name}"
    return VehicleMaterialInstanceStep(
        request_identity=digit * 64,
        package_id="extracted-art-cars-sedana",
        source_fbx="vehicle-assets/sedana/sedana.fbx",
        slot_index=slot,
        slot_name=f"slot_{slot}",
        source_material_name=f"material_{slot}",
        recipe_identity="simple-unlit-blend-alpha-alpha-test-off-one-sided",
        texture_sha256=None,
        object_path=f"{package}.{name}",
        package_path=package,
        folder_path=_INSTANCE_ROOT,
        asset_name=name,
        parent_material_path=(
            "/Game/Generated/SHAR/Materials/Vehicles/Masters/M.M"
        ),
        base_color_texture_path=None,
        base_color_tint=(1.0, 1.0, 1.0, 1.0),
        set_alpha_reference=False,
        alpha_reference=None,
    )


def _selection() -> CompiledVehicleMaterialSelection:
    return CompiledVehicleMaterialSelection(
        VehicleMaterialSelectionReport("extracted-art-cars-sedana", 0, 0, 2),
        "vehicle-assets/sedana/sedana.fbx",
        (),
        (),
        (_instance(7, "7"), _instance(0, "1")),
    )


def _execution() -> CompiledExecutionPlan:
    step = NativeImportStep(
        operation_id="operation-sedana",
        route_id="skeletal-mesh-fbx-v1",
        source_path="vehicle-assets/sedana/sedana.fbx",
        source_revision="a" * 64,
        destination=_MESH_OBJECT,
        target_class="SkeletalMesh",
        package_path=_MESH_PACKAGE,
        folder_path=_MESH_PACKAGE.rpartition("/")[0],
        asset_name="extracted_art_cars_sedana_Skeletal",
        toolset_name="SharImportEditor.SharImportToolset",
        tool_name="SharImportEditor.SharImportToolset.ImportSkeletalMesh",
        external_payload_path=None,
    )
    report = PlanExecutionReport("b" * 64, 1, 1, 0, {}, {}, {step.route_id: 1})
    return CompiledExecutionPlan(report, (step,))


def _compiled() -> CompiledVehicleMaterialSlotBinding:
    return compile_vehicle_material_slot_binding(_selection(), _execution())


def _object_schema(properties: JsonObject, *required: str) -> JsonObject:
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
    integer = {"type": "integer"}
    boolean = {"type": "boolean"}
    strings = {"items": text, "type": "array"}
    integers = {"items": integer, "type": "array"}
    paths_output = _object_schema({"returnValue": strings}, "returnValue")
    read_input = _object_schema(
        {
            "expectedSlotNames": strings,
            "skeletalMeshPath": text,
            "slotIndices": integers,
        },
        "expectedSlotNames",
        "skeletalMeshPath",
        "slotIndices",
    )
    compare_exchange_input = _object_schema(
        {
            "expectedMaterialPaths": strings,
            "expectedSlotNames": strings,
            "replacementMaterialPaths": strings,
            "skeletalMeshPath": text,
            "slotIndices": integers,
        },
        "expectedMaterialPaths",
        "expectedSlotNames",
        "replacementMaterialPaths",
        "skeletalMeshPath",
        "slotIndices",
    )
    material_tools = (
        _tool(_MATERIAL, "ReadVehicleMaterialSlots", read_input, paths_output),
        _tool(
            _MATERIAL,
            "CompareExchangeVehicleMaterialSlots",
            compare_exchange_input,
            paths_output,
        ),
    )
    bool_output = _object_schema({"returnValue": boolean}, "returnValue")
    asset_tools = (
        _tool(
            _ASSET,
            "exists",
            _object_schema({"path": text}, "path"),
            bool_output,
        ),
        _tool(
            _ASSET,
            "get_asset_class",
            _object_schema({"asset_path": text}, "asset_path"),
            _object_schema({"returnValue": text}, "returnValue"),
        ),
        _tool(
            _ASSET,
            "is_dirty",
            _object_schema({"asset_path": text}, "asset_path"),
            bool_output,
        ),
        _tool(
            _ASSET,
            "save_assets",
            _object_schema({"asset_paths": strings}, "asset_paths"),
            bool_output,
        ),
    )
    definitions = (
        ToolsetDefinition(_ASSET, "", asset_tools, {}),
        ToolsetDefinition(_MATERIAL, "", material_tools, {}),
    )
    if omit is None:
        return definitions
    return tuple(
        definition._replace(
            tools=tuple(tool for tool in definition.tools if tool.name != omit)
        )
        for definition in definitions
    )


def _outcome(value: object) -> ToolCallOutcome:
    return ToolCallOutcome(
        raw={},
        text="",
        structured_content={"returnValue": value},
        is_error=False,
    )


@dataclass
class _Behavior:
    fail_save_once: bool = False
    lose_compare_exchange_response_once: bool = False


class _Client:
    def __init__(self, compiled: CompiledVehicleMaterialSlotBinding) -> None:
        self.compiled = compiled
        self.assets = {compiled.skeletal_mesh_package_path: "SkeletalMesh"}
        self.assets.update(
            dict.fromkeys(
                compiled.material_package_paths, "MaterialInstanceConstant"
            )
        )
        self.slots = tuple("" for _ in compiled.slot_indices)
        self.dirty = False
        self.behavior = _Behavior()
        self.calls: list[str] = []

    def call_tool(
        self,
        toolset_name: str,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        del toolset_name
        leaf = tool_name.rsplit(".", 1)[-1]
        self.calls.append(leaf)
        asset_outcome = self._asset_call(leaf, arguments)
        if asset_outcome is not None:
            return asset_outcome
        if leaf == "ReadVehicleMaterialSlots":
            return _outcome(list(self.slots))
        if leaf == "CompareExchangeVehicleMaterialSlots":
            expected = tuple(
                str(value) for value in arguments["expectedMaterialPaths"]
            )
            replacement = tuple(
                str(value) for value in arguments["replacementMaterialPaths"]
            )
            if self.slots != expected:
                raise RuntimeError("synthetic CAS expectation drifted")
            self.slots = replacement
            self.dirty = True
            if self.behavior.lose_compare_exchange_response_once:
                self.behavior.lose_compare_exchange_response_once = False
                raise TimeoutError("synthetic lost CAS response")
            return _outcome(list(self.slots))
        raise AssertionError(f"unexpected tool {leaf}")

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
            return _outcome(self.dirty)
        if leaf == "save_assets":
            if self.behavior.fail_save_once:
                self.behavior.fail_save_once = False
                return _outcome(value=False)
            self.dirty = False
            return _outcome(value=True)
        return None


def test_compiler_joins_skeletal_import_and_sorts_slots() -> None:
    compiled = _compiled()
    assert compiled.report.to_json() == {
        "packageId": "extracted-art-cars-sedana",
        "slotCount": 2,
    }
    assert compiled.skeletal_mesh_path == _MESH_OBJECT
    assert compiled.slot_indices == (0, 7)
    assert compiled.slot_names == ("slot_0", "slot_7")
    assert compiled.material_paths[0].endswith(
        f"{"1" * 64}.MI_Vehicle_{"1" * 64}"
    )


def test_compiler_rejects_missing_or_ambiguous_skeletal_join() -> None:
    selection = _selection()
    empty = _execution()._replace(imports=())
    with pytest.raises(ProtocolError, match="import join is not exact"):
        compile_vehicle_material_slot_binding(selection, empty)

    execution = _execution()
    duplicate = execution.imports[0]._replace(
        operation_id="operation-duplicate"
    )
    ambiguous = execution._replace(imports=(*execution.imports, duplicate))
    with pytest.raises(ProtocolError, match="import join is not exact"):
        compile_vehicle_material_slot_binding(selection, ambiguous)


def test_capability_audit_accepts_exact_slot_surface() -> None:
    compiled = _compiled()
    report = audit_vehicle_material_slot_capabilities(compiled, _toolsets())
    assert report.complete
    assert report.slot_count == 2
    assert report.required_tool_count == 6
    assert report.available_tool_count == 6
    assert required_vehicle_material_slot_toolsets(compiled) == (
        _MATERIAL,
        _ASSET,
    )


def test_capability_audit_reports_missing_compare_exchange() -> None:
    compiled = _compiled()
    identity = f"{_MATERIAL}.CompareExchangeVehicleMaterialSlots"
    report = audit_vehicle_material_slot_capabilities(
        compiled, _toolsets(omit=identity)
    )
    assert not report.complete
    assert report.missing_tools == (identity,)


def test_application_rejects_stale_capability_revision() -> None:
    compiled = _compiled()
    capabilities = audit_vehicle_material_slot_capabilities(
        compiled, _toolsets()
    )._replace(binding_revision="0" * 64)
    client = _Client(compiled)
    with pytest.raises(ProtocolError, match="capability audit is stale"):
        apply_vehicle_material_slot_binding(client, compiled, capabilities)
    assert client.calls == []


def test_application_saves_and_verifies_selected_slots() -> None:
    compiled = _compiled()
    capabilities = audit_vehicle_material_slot_capabilities(
        compiled, _toolsets()
    )
    client = _Client(compiled)
    report = apply_vehicle_material_slot_binding(client, compiled, capabilities)
    assert report.saved_count == 1
    assert report.verified_count == 2
    assert client.slots == compiled.material_paths
    assert not client.dirty
    assert client.calls.count("CompareExchangeVehicleMaterialSlots") == 1
    assert client.calls.count("save_assets") == 1


def test_application_refuses_dirty_mesh_before_compare_exchange() -> None:
    compiled = _compiled()
    capabilities = audit_vehicle_material_slot_capabilities(
        compiled, _toolsets()
    )
    client = _Client(compiled)
    client.dirty = True
    with pytest.raises(ProtocolError, match="already dirty"):
        apply_vehicle_material_slot_binding(client, compiled, capabilities)
    assert "CompareExchangeVehicleMaterialSlots" not in client.calls


def test_application_refuses_already_published_slots_before_exchange() -> None:
    compiled = _compiled()
    capabilities = audit_vehicle_material_slot_capabilities(
        compiled, _toolsets()
    )
    client = _Client(compiled)
    client.slots = compiled.material_paths
    with pytest.raises(ProtocolError, match="selected slots are not null"):
        apply_vehicle_material_slot_binding(client, compiled, capabilities)
    assert "CompareExchangeVehicleMaterialSlots" not in client.calls


def test_save_failure_restores_null_slots_and_saves_rollback() -> None:
    compiled = _compiled()
    capabilities = audit_vehicle_material_slot_capabilities(
        compiled, _toolsets()
    )
    client = _Client(compiled)
    client.behavior.fail_save_once = True
    with pytest.raises(ProtocolError, match="save returned false"):
        apply_vehicle_material_slot_binding(client, compiled, capabilities)
    assert client.slots == ("", "")
    assert not client.dirty
    assert client.calls.count("CompareExchangeVehicleMaterialSlots") == 2
    assert client.calls.count("save_assets") == 2


def test_lost_compare_exchange_response_reconciles_and_rolls_back() -> None:
    compiled = _compiled()
    capabilities = audit_vehicle_material_slot_capabilities(
        compiled, _toolsets()
    )
    client = _Client(compiled)
    client.behavior.lose_compare_exchange_response_once = True
    with pytest.raises(TimeoutError, match="lost CAS response"):
        apply_vehicle_material_slot_binding(client, compiled, capabilities)
    assert client.slots == ("", "")
    assert not client.dirty
    assert client.calls.count("CompareExchangeVehicleMaterialSlots") == 2
    assert client.calls.count("save_assets") == 1
