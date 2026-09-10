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
#   - Live schema audit for read-only vehicle Physics Asset verification.
# - Must-Not:
#   - Invoke tools, mutate Unreal Editor, save assets, or accept global scope.
# - Allows:
#   - Validate exact verifier and AssetTools read schemas for selected rigs.
# - Split-When:
#   - Verification needs mutation or independently versioned evidence.
# - Merge-When:
#   - Publication capability audit gains identical read-only semantics.
# - Summary:
#   - Vehicle Physics Asset verification capability auditor.
# - Description:
#   - Binds live read-only schemas to one selected construction revision.
# - Usage:
#   - Called after package selection and before persisted-asset verification.
# - Defaults:
#   - Missing or incompatible verifier/read schemas fail closed.
#

"""Live capability audit for persisted vehicle Physics Asset verification."""

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
    VehiclePhysicsConstructionStep,
)
from mcp.domain.vehicle_physics_construction import (
    vehicle_physics_construction_revision,
)

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"
_VERIFY_LEAF = "VerifyVehiclePhysicsAsset"


class VehiclePhysicsVerificationToolRequirement(NamedTuple):
    """One exact read-only live tool contract."""

    toolset_name: str
    tool_name: str
    input_example: JsonObject
    output_example: JsonObject


class VehiclePhysicsVerificationCapabilityReport(NamedTuple):
    """Public-safe schema evidence for one selected verification revision."""

    construction_revision: str
    verification_count: int
    required_tool_count: int
    available_tool_count: int
    missing_tools: tuple[str, ...]
    incompatible_tools: tuple[str, ...]

    @property
    def complete(self) -> bool:
        """Whether every required read-only schema is compatible."""
        return not self.missing_tools and not self.incompatible_tools

    def to_json(self) -> JsonObject:
        """Render capability evidence without asset identities."""
        return {
            "availableToolCount": self.available_tool_count,
            "complete": self.complete,
            "constructionRevision": self.construction_revision,
            "incompatibleTools": list(self.incompatible_tools),
            "missingTools": list(self.missing_tools),
            "requiredToolCount": self.required_tool_count,
            "verificationCount": self.verification_count,
        }


def required_vehicle_physics_verification_toolsets(
    compiled: CompiledVehiclePhysicsConstruction,
) -> tuple[str, ...]:
    """Return toolsets required to verify selected persisted Physics Assets."""
    requirements = _requirements(compiled)
    return tuple(sorted({item.toolset_name for item in requirements}))


def audit_vehicle_physics_verification_capabilities(
    compiled: CompiledVehiclePhysicsConstruction,
    toolsets: tuple[ToolsetDefinition, ...],
) -> VehiclePhysicsVerificationCapabilityReport:
    """Validate all read-only schemas without invoking a native tool."""
    definitions = {definition.name: definition for definition in toolsets}
    requirements = _requirements(compiled)
    missing: list[str] = []
    incompatible: list[str] = []
    available = 0
    for requirement in requirements:
        definition = definitions.get(requirement.toolset_name)
        tool = None if definition is None else next(
            (
                candidate
                for candidate in definition.tools
                if candidate.name == requirement.tool_name
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
                    "vehicle-physics verification tool has no output schema"
                )
            validate_tool_arguments(
                tool.output_schema,
                requirement.output_example,
                context=f"tool {tool.name} output schema",
            )
        except ProtocolError:
            incompatible.append(requirement.tool_name)
    return VehiclePhysicsVerificationCapabilityReport(
        construction_revision=vehicle_physics_construction_revision(compiled),
        verification_count=len(compiled.requests),
        required_tool_count=len(requirements),
        available_tool_count=available,
        missing_tools=tuple(sorted(missing)),
        incompatible_tools=tuple(sorted(incompatible)),
    )


def verification_arguments(step: VehiclePhysicsConstructionStep) -> JsonObject:
    """Build exact arguments for the native read-only verifier."""
    return {
        "physicsAssetPath": step.object_path,
        "rigIdentity": step.rig_identity,
        "shapes": [shape.to_wire() for shape in step.shapes],
        "skeletalMeshPath": step.skeletal_mesh_path,
        "sourceJointCount": step.source_joint_count,
    }


def verification_tool_name(step: VehiclePhysicsConstructionStep) -> str:
    """Return the native verifier identity in the construction toolset."""
    return f"{step.toolset_name}.{_VERIFY_LEAF}"


def _requirements(
    compiled: CompiledVehiclePhysicsConstruction,
) -> tuple[VehiclePhysicsVerificationToolRequirement, ...]:
    if not compiled.requests:
        return ()
    step = compiled.requests[0]
    boolean_output = {"returnValue": True}
    requirements = (
        VehiclePhysicsVerificationToolRequirement(
            step.toolset_name,
            verification_tool_name(step),
            verification_arguments(step),
            boolean_output,
        ),
        VehiclePhysicsVerificationToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.exists",
            {"path": step.package_path},
            boolean_output,
        ),
        VehiclePhysicsVerificationToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.get_asset_class",
            {"asset_path": step.package_path},
            {"returnValue": step.target_class},
        ),
        VehiclePhysicsVerificationToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.is_dirty",
            {"asset_path": step.package_path},
            {"returnValue": False},
        ),
    )
    return tuple(sorted(requirements, key=lambda item: item.tool_name))
