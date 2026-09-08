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
#   - Live native capability audit for compiled world-material construction.
# - Must-Not:
#   - Open MCP sessions, invoke tools, read files, or mutate Unreal Editor.
# - Allows:
#   - Validate exact live input/output schemas with representative plan values.
# - Split-When:
#   - Another construction family gains an independent native surface.
# - Merge-When:
#   - General plan capability audit accepts construction transactions.
# - Summary:
#   - World-material native capability auditor.
# - Description:
#   - Binds one schema audit to a deterministic compiled-construction revision.
# - Usage:
#   - Called after local construction and texture-source verification.
# - Defaults:
#   - Missing or incompatible mutation/read-back tools fail closed.
#

"""Live capability audit for compiled world-material construction."""

from __future__ import annotations

import hashlib
import json
from typing import NamedTuple

from mcp.domain.argument_schema import validate_tool_arguments
from mcp.domain.catalog import ToolsetDefinition
from mcp.domain.errors import ProtocolError
from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.world_material_construction import (
    CompiledWorldMaterialConstruction,
)

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"


class WorldMaterialToolRequirement(NamedTuple):
    """One exact live tool contract with representative wire values."""

    toolset_name: str
    tool_name: str
    input_example: JsonObject
    output_example: JsonObject


class WorldMaterialCapabilityReport(NamedTuple):
    """Public-safe live-schema audit for one construction revision."""

    construction_revision: str
    construction_count: int
    required_tool_count: int
    available_tool_count: int
    missing_tools: tuple[str, ...]
    incompatible_tools: tuple[str, ...]

    @property
    def complete(self) -> bool:
        """Whether every required native schema is present and compatible."""
        return not self.missing_tools and not self.incompatible_tools

    def to_json(self) -> JsonObject:
        """Render construction identity and live-schema coverage."""
        return {
            "availableToolCount": self.available_tool_count,
            "complete": self.complete,
            "constructionCount": self.construction_count,
            "constructionRevision": self.construction_revision,
            "incompatibleTools": list(self.incompatible_tools),
            "missingTools": list(self.missing_tools),
            "requiredToolCount": self.required_tool_count,
        }


def world_material_construction_revision(
    compiled: CompiledWorldMaterialConstruction,
) -> str:
    """Hash the exact typed native construction requests canonically."""
    value = {
        "instances": [item._asdict() for item in compiled.instances],
        "masters": [item._asdict() for item in compiled.masters],
        "report": compiled.report._asdict(),
        "textures": [item._asdict() for item in compiled.textures],
    }
    encoded = json.dumps(
        value,
        ensure_ascii=True,
        separators=(",", ":"),
        sort_keys=True,
    ).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def required_world_material_toolsets(
    compiled: CompiledWorldMaterialConstruction,
) -> tuple[str, ...]:
    """Return exact live toolsets needed by this compiled construction."""
    names = {item.toolset_name for item in _requirements(compiled)}
    return tuple(sorted(names))


def audit_world_material_capabilities(
    compiled: CompiledWorldMaterialConstruction,
    toolsets: tuple[ToolsetDefinition, ...],
) -> WorldMaterialCapabilityReport:
    """Validate every required live tool schema without invoking a tool."""
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
                fail_protocol("world-material native tool has no output schema")
            validate_tool_arguments(
                tool.output_schema,
                requirement.output_example,
                context=f"tool {tool.name} output schema",
            )
        except ProtocolError:
            incompatible.append(requirement.tool_name)
    count = (
        len(compiled.textures)
        + len(compiled.masters)
        + len(compiled.instances)
    )
    return WorldMaterialCapabilityReport(
        construction_revision=world_material_construction_revision(compiled),
        construction_count=count,
        required_tool_count=len(requirements),
        available_tool_count=available,
        missing_tools=tuple(sorted(missing)),
        incompatible_tools=tuple(sorted(incompatible)),
    )


def _requirements(
    compiled: CompiledWorldMaterialConstruction,
) -> tuple[WorldMaterialToolRequirement, ...]:
    requirements: list[WorldMaterialToolRequirement] = []
    if compiled.textures:
        step = compiled.textures[0]
        requirements.append(WorldMaterialToolRequirement(
            step.toolset_name,
            step.tool_name,
            step.arguments("C:/SHAR/verified-world-texture.png"),
            {
                "returnValue": [{
                    "assetClass": step.target_class,
                    "packagePath": step.package_path,
                }]
            },
        ))
    if compiled.masters:
        step = compiled.masters[0]
        requirements.append(WorldMaterialToolRequirement(
            step.toolset_name,
            step.tool_name,
            step.arguments(),
            {"returnValue": step.object_path},
        ))
    if compiled.instances:
        step = compiled.instances[0]
        requirements.append(WorldMaterialToolRequirement(
            step.toolset_name,
            step.tool_name,
            step.arguments(),
            {"returnValue": step.object_path},
        ))
    if requirements:
        requirements.extend(_asset_requirements(compiled))
    return tuple(sorted(requirements, key=lambda item: item.tool_name))


def _asset_requirements(
    compiled: CompiledWorldMaterialConstruction,
) -> tuple[WorldMaterialToolRequirement, ...]:
    if compiled.textures:
        path = compiled.textures[0].package_path
        target_class = compiled.textures[0].target_class
    elif compiled.masters:
        path = compiled.masters[0].package_path
        target_class = compiled.masters[0].target_class
    elif compiled.instances:
        path = compiled.instances[0].package_path
        target_class = compiled.instances[0].target_class
    else:
        return ()
    return (
        WorldMaterialToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.delete",
            {"path": path},
            {"returnValue": True},
        ),
        WorldMaterialToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.exists",
            {"path": path},
            {"returnValue": True},
        ),
        WorldMaterialToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.get_asset_class",
            {"asset_path": path},
            {"returnValue": target_class},
        ),
        WorldMaterialToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.is_dirty",
            {"asset_path": path},
            {"returnValue": False},
        ),
        WorldMaterialToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.save_assets",
            {"asset_paths": [path]},
            {"returnValue": True},
        ),
    )
