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
#   - Vehicle-physics capability and save/compensation transaction tests.
# - Must-Not:
#   - Contact Unreal Editor, read generated caches, or mutate project assets.
# - Allows:
#   - Synthetic live schemas, asset state, failures, and lost responses.
# - Split-When:
#   - Capability and application fixtures gain independent lifecycles.
# - Merge-When:
#   - Another suite owns identical Physics Asset transaction policy.
# - Summary:
#   - Vehicle-physics native application tests.
# - Description:
#   - Proves schema gates, dependency ownership, save, read-back, and rollback.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Existing destinations and missing Skeletal Mesh dependencies fail closed.
#

"""Tests for vehicle-physics native capability and application boundaries."""

from __future__ import annotations

from typing import NamedTuple

from mcp.application.vehicle_physics_application import (
    apply_vehicle_physics_construction,
)
from mcp.domain.catalog import ToolDefinition
from mcp.domain.catalog import ToolsetDefinition
from mcp.domain.errors import ProtocolError
from mcp.domain.json_types import JsonObject
from mcp.domain.tool_outcome import ToolCallOutcome
from mcp.domain.vehicle_physics_capabilities import (
    audit_vehicle_physics_capabilities,
)
from mcp.domain.vehicle_physics_capabilities import (
    required_vehicle_physics_toolsets,
)
from mcp.domain.vehicle_physics_construction import (
    CompiledVehiclePhysicsConstruction,
)
from mcp.domain.vehicle_physics_construction import (
    VehiclePhysicsConstructionReport,
)
from mcp.domain.vehicle_physics_construction import (
    VehiclePhysicsConstructionStep,
)
from mcp.domain.vehicle_physics_construction import VehiclePhysicsShape
import pytest

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"
_PHYSICS_TOOLSET = "SharImportEditor.SharVehiclePhysicsToolset"
_PHYSICS_TOOL = f"{_PHYSICS_TOOLSET}.CreateVehiclePhysicsAsset"
_MESH_PACKAGE = "/Game/Generated/SHAR/cars/sedana"
_MESH_OBJECT = f"{_MESH_PACKAGE}.sedana"


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
    integer = {"type": "integer"}
    number = {"type": "number"}
    vector = _object_schema(
        {"x": number, "y": number, "z": number}, "x", "y", "z"
    )
    shape = _object_schema(
        {
            "axisX": vector,
            "axisY": vector,
            "axisZ": vector,
            "boneName": text,
            "boxExtents": vector,
            "center": vector,
            "kind": {"enum": ["Sphere", "Box"], "type": "string"},
            "radius": number,
        },
        "axisX",
        "axisY",
        "axisZ",
        "boneName",
        "boxExtents",
        "center",
        "kind",
        "radius",
    )
    physics_input = _object_schema(
        {
            "assetName": text,
            "folderPath": text,
            "rigIdentity": text,
            "shapes": {"items": shape, "type": "array"},
            "skeletalMeshPath": text,
            "sourceJointCount": integer,
        },
        "assetName",
        "folderPath",
        "rigIdentity",
        "shapes",
        "skeletalMeshPath",
        "sourceJointCount",
    )
    string_output = _object_schema({"returnValue": text}, "returnValue")
    boolean_output = _object_schema({"returnValue": boolean}, "returnValue")
    physics_tools = (
        _tool(
            _PHYSICS_TOOLSET,
            "CreateVehiclePhysicsAsset",
            physics_input,
            string_output,
        ),
    )
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
                {"asset_paths": {"items": text, "type": "array"}},
                "asset_paths",
            ),
            boolean_output,
        ),
    )
    definitions = (
        ToolsetDefinition(_ASSET_TOOLSET, "", asset_tools, {}),
        ToolsetDefinition(_PHYSICS_TOOLSET, "", physics_tools, {}),
    )
    if omit is None:
        return definitions
    return tuple(
        definition._replace(
            tools=tuple(tool for tool in definition.tools if tool.name != omit)
        )
        for definition in definitions
    )


def _compiled(*, count: int = 1) -> CompiledVehiclePhysicsConstruction:
    requests = []
    for index in range(count):
        asset = f"PHYS_{index:024x}"
        package = f"/Game/Generated/SHAR/VehiclePhysics/{asset}"
        shape = VehiclePhysicsShape(
            kind="sphere",
            bone_name="w0",
            center=(0.0, 0.0, 0.0),
            radius=0.4,
            axes=(),
            extents=(0.0, 0.0, 0.0),
        )
        requests.append(VehiclePhysicsConstructionStep(
            package_id=f"vehicle-{index}",
            source_fbx=f"vehicle-assets/vehicle-{index}/vehicle.fbx",
            rig_identity=f"rig{index}",
            source_joint_count=19,
            skeletal_mesh_path=_MESH_OBJECT,
            folder_path="/Game/Generated/SHAR/VehiclePhysics",
            asset_name=asset,
            package_path=package,
            object_path=f"{package}.{asset}",
            shapes=(shape,),
        ))
    return CompiledVehiclePhysicsConstruction(
        VehiclePhysicsConstructionReport(count, 1, count),
        tuple(requests),
    )


def _outcome(value: object) -> ToolCallOutcome:
    content: JsonObject = {"returnValue": value}
    return ToolCallOutcome(
        raw={}, text="", structured_content=content, is_error=False
    )


class _Behavior(NamedTuple):
    raise_on_create: int | None = None
    wrong_class_on_create: int | None = None


class _SyntheticClient:
    def __init__(
        self,
        *,
        preexisting: dict[str, str] | None = None,
        behavior: _Behavior | None = None,
    ) -> None:
        self.assets = {_MESH_PACKAGE: "SkeletalMesh", **(preexisting or {})}
        self.dirty: set[str] = set()
        self.calls: list[tuple[str, JsonObject]] = []
        self.behavior = _Behavior() if behavior is None else behavior
        self.create_count = 0

    def call_tool(
        self,
        toolset_name: str,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        del toolset_name
        leaf = tool_name.rsplit(".", 1)[-1]
        self.calls.append((leaf, arguments))
        if leaf == "exists":
            return _outcome(str(arguments["path"]) in self.assets)
        if leaf == "get_asset_class":
            return _outcome(self.assets[str(arguments["asset_path"])])
        if leaf == "is_dirty":
            return _outcome(str(arguments["asset_path"]) in self.dirty)
        if leaf == "save_assets":
            raw = arguments["asset_paths"]
            assert isinstance(raw, list)
            paths = [str(path) for path in raw]
            success = all(path in self.assets for path in paths)
            if success:
                self.dirty.difference_update(paths)
            return _outcome(success)
        if leaf == "delete":
            path = str(arguments["path"])
            existed = self.assets.pop(path, None) is not None
            self.dirty.discard(path)
            return _outcome(existed)
        if leaf == "CreateVehiclePhysicsAsset":
            index = self.create_count
            self.create_count += 1
            package = (
                f'{arguments["folderPath"]}/{arguments["assetName"]}'
            )
            object_path = f'{package}.{arguments["assetName"]}'
            target_class = (
                "Material"
                if self.behavior.wrong_class_on_create == index
                else "PhysicsAsset"
            )
            self.assets[package] = target_class
            self.dirty.add(package)
            if self.behavior.raise_on_create == index:
                raise TimeoutError("synthetic lost Physics Asset response")
            return _outcome(object_path)
        raise AssertionError(f"unexpected synthetic tool {leaf}")


def test_capability_audit_accepts_exact_vehicle_physics_surface() -> None:
    compiled = _compiled()
    report = audit_vehicle_physics_capabilities(compiled, _toolsets())
    assert report.complete
    assert report.construction_count == 1
    assert report.required_tool_count == 6
    assert report.available_tool_count == 6
    assert required_vehicle_physics_toolsets(compiled) == (
        _PHYSICS_TOOLSET,
        _ASSET_TOOLSET,
    )


def test_capability_audit_reports_missing_publication_tool() -> None:
    compiled = _compiled()
    report = audit_vehicle_physics_capabilities(
        compiled, _toolsets(omit=_PHYSICS_TOOL)
    )
    assert not report.complete
    assert report.missing_tools == (_PHYSICS_TOOL,)


def test_application_saves_ready_rig_without_mutating_dependency() -> None:
    compiled = _compiled()
    capabilities = audit_vehicle_physics_capabilities(compiled, _toolsets())
    client = _SyntheticClient()
    report = apply_vehicle_physics_construction(
        client, compiled, capabilities
    )
    assert report.created_count == 1
    assert report.saved_count == 1
    assert report.verified_count == 1
    assert report.blocked_rig_count == 1
    assert client.assets[_MESH_PACKAGE] == "SkeletalMesh"
    assert client.assets[compiled.requests[0].package_path] == "PhysicsAsset"
    assert client.dirty == set()
    assert all(
        arguments.get("path") != _MESH_PACKAGE
        for leaf, arguments in client.calls
        if leaf == "delete"
    )


def test_application_refuses_existing_destination_before_mutation() -> None:
    compiled = _compiled()
    capabilities = audit_vehicle_physics_capabilities(compiled, _toolsets())
    destination = compiled.requests[0].package_path
    client = _SyntheticClient(preexisting={destination: "PhysicsAsset"})
    with pytest.raises(ProtocolError, match="destination already exists"):
        apply_vehicle_physics_construction(client, compiled, capabilities)
    assert client.assets[destination] == "PhysicsAsset"
    assert client.create_count == 0


def test_lost_second_response_compensates_only_owned_physics_assets() -> None:
    compiled = _compiled(count=2)
    capabilities = audit_vehicle_physics_capabilities(compiled, _toolsets())
    client = _SyntheticClient(behavior=_Behavior(raise_on_create=1))
    with pytest.raises(TimeoutError, match="lost Physics Asset response"):
        apply_vehicle_physics_construction(client, compiled, capabilities)
    assert client.assets == {_MESH_PACKAGE: "SkeletalMesh"}
    deletes = tuple(
        str(arguments["path"])
        for leaf, arguments in client.calls
        if leaf == "delete"
    )
    assert deletes == (
        compiled.requests[1].package_path,
        compiled.requests[0].package_path,
    )


def test_wrong_physics_class_rolls_back_asset() -> None:
    compiled = _compiled()
    capabilities = audit_vehicle_physics_capabilities(compiled, _toolsets())
    client = _SyntheticClient(behavior=_Behavior(wrong_class_on_create=0))
    with pytest.raises(ProtocolError, match="unexpected class"):
        apply_vehicle_physics_construction(client, compiled, capabilities)
    assert client.assets == {_MESH_PACKAGE: "SkeletalMesh"}
