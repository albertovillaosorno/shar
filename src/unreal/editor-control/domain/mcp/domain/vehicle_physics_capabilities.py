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
#   - Live native capability audit for vehicle Physics Asset publication.
# - Must-Not:
#   - Invoke tools, read files, or mutate Unreal Editor.
# - Allows:
#   - Validate exact construction and AssetTools schemas before mutation.
# - Split-When:
#   - Another vehicle construction family gains an independent native surface.
# - Merge-When:
#   - General construction capability audit accepts this transaction.
# - Summary:
#   - Vehicle-physics native capability auditor.
# - Description:
#   - Binds live schemas to one deterministic construction revision.
# - Usage:
#   - Called after sidecar compilation and before application.
# - Defaults:
#   - Missing or incompatible publication/read-back tools fail closed.
#

"""Live capability audit for vehicle Physics Asset publication."""

from __future__ import annotations

from typing import NamedTuple

from mcp.domain.argument_schema import validate_tool_arguments
from mcp.domain.catalog import ToolsetDefinition
from mcp.domain.errors import ProtocolError
from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.vehicle_physics_construction import (
    CompiledVehiclePhysicsConstruction,
)
from mcp.domain.vehicle_physics_construction import (
    vehicle_physics_construction_revision,
)

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"


class VehiclePhysicsToolRequirement(NamedTuple):
    """One exact live tool contract with representative values."""

    toolset_name: str
    tool_name: str
    input_example: JsonObject
    output_example: JsonObject


class VehiclePhysicsCapabilityReport(NamedTuple):
    """Public-safe live-schema audit for one construction revision."""

    construction_revision: str
    construction_count: int
    required_tool_count: int
    available_tool_count: int
    missing_tools: tuple[str, ...]
    incompatible_tools: tuple[str, ...]

    @property
    def complete(self) -> bool:
        """Whether every required schema is present and compatible."""
        return not self.missing_tools and not self.incompatible_tools

    def to_json(self) -> JsonObject:
        """Render live-schema coverage without asset identities."""
        return {
            "availableToolCount": self.available_tool_count,
            "complete": self.complete,
            "constructionCount": self.construction_count,
            "constructionRevision": self.construction_revision,
            "incompatibleTools": list(self.incompatible_tools),
            "missingTools": list(self.missing_tools),
            "requiredToolCount": self.required_tool_count,
        }


def required_vehicle_physics_toolsets(
    compiled: CompiledVehiclePhysicsConstruction,
) -> tuple[str, ...]:
    """Return exact live toolsets needed by this construction."""
    return tuple(
        sorted({item.toolset_name for item in _requirements(compiled)})
    )


def audit_vehicle_physics_capabilities(
    compiled: CompiledVehiclePhysicsConstruction,
    toolsets: tuple[ToolsetDefinition, ...],
) -> VehiclePhysicsCapabilityReport:
    """Validate every required live schema without invoking a tool."""
    definitions = {definition.name: definition for definition in toolsets}
    requirements = _requirements(compiled)
    missing: list[str] = []
    incompatible: list[str] = []
    available = 0
    for requirement in requirements:
        definition = definitions.get(requirement.toolset_name)
        tool = None if definition is None else next(
            (
                item
                for item in definition.tools
                if item.name == requirement.tool_name
            ),
            None,
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
                fail_protocol(
                    "vehicle-physics native tool has no output schema"
                )
            validate_tool_arguments(
                tool.output_schema,
                requirement.output_example,
                context=f"tool {tool.name} output schema",
            )
        except ProtocolError:
            incompatible.append(requirement.tool_name)
    return VehiclePhysicsCapabilityReport(
        construction_revision=vehicle_physics_construction_revision(compiled),
        construction_count=len(compiled.requests),
        required_tool_count=len(requirements),
        available_tool_count=available,
        missing_tools=tuple(sorted(missing)),
        incompatible_tools=tuple(sorted(incompatible)),
    )


def _requirements(
    compiled: CompiledVehiclePhysicsConstruction,
) -> tuple[VehiclePhysicsToolRequirement, ...]:
    if not compiled.requests:
        return ()
    step = compiled.requests[0]
    requirements = [
        VehiclePhysicsToolRequirement(
            step.toolset_name,
            step.tool_name,
            step.arguments(),
            {"returnValue": step.object_path},
        )
    ]
    boolean_output = {"returnValue": True}
    requirements.extend((
        VehiclePhysicsToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.delete",
            {"path": step.package_path},
            boolean_output,
        ),
        VehiclePhysicsToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.exists",
            {"path": step.package_path},
            boolean_output,
        ),
        VehiclePhysicsToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.get_asset_class",
            {"asset_path": step.package_path},
            {"returnValue": step.target_class},
        ),
        VehiclePhysicsToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.is_dirty",
            {"asset_path": step.package_path},
            {"returnValue": False},
        ),
        VehiclePhysicsToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.save_assets",
            {"asset_paths": [step.package_path]},
            boolean_output,
        ),
    ))
    return tuple(sorted(requirements, key=lambda item: item.tool_name))
