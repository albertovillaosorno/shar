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
#   - Live schema audit for vehicle material slot publication.
# - Must-Not:
#   - Invoke tools, read files, or mutate Unreal Editor.
# - Allows:
#   - Validate exact slot read/CAS and AssetTools contracts before mutation.
# - Split-When:
#   - Another binding lifecycle needs an independent native surface.
# - Merge-When:
#   - Vehicle material capability audit owns slot publication too.
# - Summary:
#   - Vehicle material slot capability auditor.
# - Description:
#   - Binds live native schemas to one deterministic slot binding revision.
# - Usage:
#   - Called after slot binding compilation and before application.
# - Defaults:
#   - Any missing or incompatible required tool blocks publication.
#

"""Live capability audit for vehicle material slot publication."""

from __future__ import annotations

from typing import NamedTuple

from mcp.domain.argument_schema import validate_tool_arguments
from mcp.domain.catalog import ToolsetDefinition
from mcp.domain.errors import ProtocolError
from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.vehicle_material_slot_binding import (
    CompiledVehicleMaterialSlotBinding,
)
from mcp.domain.vehicle_material_slot_binding import (
    vehicle_material_slot_binding_revision,
)

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"


class VehicleMaterialSlotToolRequirement(NamedTuple):
    """One exact live tool contract with representative values."""

    toolset_name: str
    tool_name: str
    input_example: JsonObject
    output_example: JsonObject


class VehicleMaterialSlotCapabilityReport(NamedTuple):
    """Public-safe schema audit for one slot binding revision."""

    binding_revision: str
    slot_count: int
    required_tool_count: int
    available_tool_count: int
    missing_tools: tuple[str, ...]
    incompatible_tools: tuple[str, ...]

    @property
    def complete(self) -> bool:
        """Whether every required schema is present and compatible."""
        return not self.missing_tools and not self.incompatible_tools

    def to_json(self) -> JsonObject:
        """Render schema coverage without asset identities."""
        return {
            "availableToolCount": self.available_tool_count,
            "bindingRevision": self.binding_revision,
            "complete": self.complete,
            "incompatibleTools": list(self.incompatible_tools),
            "missingTools": list(self.missing_tools),
            "requiredToolCount": self.required_tool_count,
            "slotCount": self.slot_count,
        }


def required_vehicle_material_slot_toolsets(
    compiled: CompiledVehicleMaterialSlotBinding,
) -> tuple[str, ...]:
    """Return exact toolsets needed by slot publication."""
    return tuple(
        sorted({item.toolset_name for item in _requirements(compiled)})
    )


def audit_vehicle_material_slot_capabilities(
    compiled: CompiledVehicleMaterialSlotBinding,
    toolsets: tuple[ToolsetDefinition, ...],
) -> VehicleMaterialSlotCapabilityReport:
    """Validate every required live schema without invoking a tool."""
    definitions = {definition.name: definition for definition in toolsets}
    requirements = _requirements(compiled)
    missing: list[str] = []
    incompatible: list[str] = []
    available = 0
    for requirement in requirements:
        definition = definitions.get(requirement.toolset_name)
        tool = (
            None
            if definition is None
            else next(
                (
                    item
                    for item in definition.tools
                    if item.name == requirement.tool_name
                ),
                None,
            )
        )
        if tool is None:
            missing.append(requirement.tool_name)
            continue
        available += 1
        try:
            validate_tool_arguments(
                tool.input_schema,
                requirement.input_example,
                context=f"tool {tool.name} input schema",
            )
            if tool.output_schema is None:
                fail_protocol("vehicle material slot tool has no output schema")
            validate_tool_arguments(
                tool.output_schema,
                requirement.output_example,
                context=f"tool {tool.name} output schema",
            )
        except ProtocolError:
            incompatible.append(requirement.tool_name)
    return VehicleMaterialSlotCapabilityReport(
        vehicle_material_slot_binding_revision(compiled),
        len(compiled.slot_indices),
        len(requirements),
        available,
        tuple(sorted(missing)),
        tuple(sorted(incompatible)),
    )


def _requirements(
    compiled: CompiledVehicleMaterialSlotBinding,
) -> tuple[VehicleMaterialSlotToolRequirement, ...]:
    empty = tuple("" for _ in compiled.slot_indices)
    paths_output = {"returnValue": list(compiled.material_paths)}
    boolean_output = {"returnValue": True}
    requirements = [
        VehicleMaterialSlotToolRequirement(
            compiled.toolset_name,
            compiled.read_tool_name,
            compiled.read_arguments(),
            {"returnValue": list(empty)},
        ),
        VehicleMaterialSlotToolRequirement(
            compiled.toolset_name,
            compiled.compare_exchange_tool_name,
            compiled.compare_exchange_arguments(empty, compiled.material_paths),
            paths_output,
        ),
        VehicleMaterialSlotToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.exists",
            {"path": compiled.skeletal_mesh_package_path},
            boolean_output,
        ),
        VehicleMaterialSlotToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.get_asset_class",
            {"asset_path": compiled.skeletal_mesh_package_path},
            {"returnValue": "SkeletalMesh"},
        ),
        VehicleMaterialSlotToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.is_dirty",
            {"asset_path": compiled.skeletal_mesh_package_path},
            {"returnValue": False},
        ),
        VehicleMaterialSlotToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.save_assets",
            {"asset_paths": [compiled.skeletal_mesh_package_path]},
            boolean_output,
        ),
    ]
    return tuple(sorted(requirements, key=lambda item: item.tool_name))
