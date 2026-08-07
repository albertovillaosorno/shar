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
#   - Live native capability requirements for compiled Unreal plan execution.
# - Must-Not:
#   - Open MCP sessions, invoke tools, or weaken incomplete-plan reporting.
# - Allows:
#   - Deduplicate required tools and validate live input and output schemas.
# - Split-When:
#   - Split when world, construction, or packaging capabilities gain lifecycles.
# - Merge-When:
#   - Merge when plan execution owns identical live-schema policy.
# - Summary:
#   - Unreal plan native capability auditor.
# - Description:
#   - Proves compiled imports have mutation, persistence, and read-back tools.
# - Usage:
#   - Called after local source verification with current Toolset definitions.
# - Defaults:
#   - Missing or schema-incompatible tools keep execution incomplete.
#

"""Live native capability requirements for compiled Unreal plan execution."""

from __future__ import annotations

from typing import NamedTuple

from mcp.domain.argument_schema import validate_tool_arguments
from mcp.domain.catalog import ToolsetDefinition
from mcp.domain.errors import ProtocolError
from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.plan_execution import CompiledExecutionPlan
from mcp.domain.plan_execution import NativeImportStep

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"
_IMPORT_TOOLSET = "SharImportEditor.SharImportToolset"


class NativeToolRequirement(NamedTuple):
    """One exact live tool contract and representative wire values."""

    toolset_name: str
    tool_name: str
    input_example: JsonObject
    output_example: JsonObject


class PlanCapabilityReport(NamedTuple):
    """Public-safe result of checking required native schemas."""

    bundle_revision: str
    compiled_count: int
    plan_complete: bool
    required_tool_count: int
    available_tool_count: int
    missing_tools: tuple[str, ...]
    incompatible_tools: tuple[str, ...]

    @property
    def native_surface_complete(self) -> bool:
        """Whether every required live tool has compatible wire schemas."""
        return not self.missing_tools and not self.incompatible_tools

    @property
    def complete(self) -> bool:
        """Whether plan coverage and the native surface are complete."""
        return self.plan_complete and self.native_surface_complete

    def to_json(self) -> JsonObject:
        """Render tool identities and counts without representative values."""
        return {
            "availableToolCount": self.available_tool_count,
            "bundleRevision": self.bundle_revision,
            "compiledCount": self.compiled_count,
            "complete": self.complete,
            "incompatibleTools": list(self.incompatible_tools),
            "missingTools": list(self.missing_tools),
            "nativeSurfaceComplete": self.native_surface_complete,
            "planComplete": self.plan_complete,
            "requiredToolCount": self.required_tool_count,
        }


def required_toolsets(compiled: CompiledExecutionPlan) -> tuple[str, ...]:
    """Return exact live toolset identities needed by compiled imports."""
    return tuple(
        sorted({
            requirement.toolset_name for requirement in _requirements(compiled)
        })
    )


def audit_plan_capabilities(
    compiled: CompiledExecutionPlan,
    toolsets: tuple[ToolsetDefinition, ...],
) -> PlanCapabilityReport:
    """Check required native schemas without invoking tools."""
    definitions = {definition.name: definition for definition in toolsets}
    requirements = _requirements(compiled)
    missing: list[str] = []
    incompatible: list[str] = []
    available = 0
    for requirement in requirements:
        definition = definitions.get(requirement.toolset_name)
        if definition is None:
            missing.append(requirement.tool_name)
            continue
        tool = next(
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
            output_schema = _require_output_schema(tool.output_schema)
            validate_tool_arguments(
                output_schema,
                requirement.output_example,
                context=f"tool {tool.name} output schema",
            )
        except ProtocolError:
            incompatible.append(requirement.tool_name)
    return PlanCapabilityReport(
        bundle_revision=compiled.report.bundle_revision,
        compiled_count=compiled.report.compiled_count,
        plan_complete=compiled.report.complete,
        required_tool_count=len(requirements),
        available_tool_count=available,
        missing_tools=tuple(sorted(missing)),
        incompatible_tools=tuple(sorted(incompatible)),
    )


def _require_output_schema(schema: JsonObject | None) -> JsonObject:
    if schema is None:
        fail_protocol("native tool has no output schema")
    return schema


def _requirements(
    compiled: CompiledExecutionPlan,
) -> tuple[NativeToolRequirement, ...]:
    if not compiled.imports:
        return ()
    first_by_route: dict[str, NativeImportStep] = {}
    for step in compiled.imports:
        first_by_route.setdefault(step.route_id, step)
    requirements = [
        _import_requirement(step)
        for _route, step in sorted(first_by_route.items())
    ]
    media_step = first_by_route.get("file-media-source-hap-v1")
    if media_step is not None:
        requirements.extend(_media_payload_requirements(media_step))
    example_path = compiled.imports[0].package_path
    requirements.extend((
        NativeToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.delete",
            {"path": example_path},
            {"returnValue": True},
        ),
        NativeToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.exists",
            {"path": example_path},
            {"returnValue": True},
        ),
        NativeToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.get_asset_class",
            {"asset_path": example_path},
            {"returnValue": "Texture2D"},
        ),
        NativeToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.is_dirty",
            {"asset_path": example_path},
            {"returnValue": False},
        ),
        NativeToolRequirement(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.save_assets",
            {"asset_paths": [example_path]},
            {"returnValue": True},
        ),
    ))
    return tuple(sorted(requirements, key=lambda item: item.tool_name))


def _import_requirement(step: NativeImportStep) -> NativeToolRequirement:
    extension_by_route = {
        "file-media-source-hap-v1": "mov",
        "sound-wave-wav-v1": "wav",
        "static-mesh-fbx-v1": "fbx",
        "texture-image-v1": "png",
    }
    extension = extension_by_route[step.route_id]
    output: JsonObject = (
        {"returnValue": [step.destination]}
        if step.route_id in {
            "file-media-source-hap-v1",
            "sound-wave-wav-v1",
            "static-mesh-fbx-v1",
        }
        else {"returnValue": []}
    )
    return NativeToolRequirement(
        toolset_name=step.toolset_name,
        tool_name=step.tool_name,
        input_example=step.arguments(f"C:/SHAR/verified-source.{extension}"),
        output_example=output,
    )


def _media_payload_requirements(
    step: NativeImportStep,
) -> tuple[NativeToolRequirement, ...]:
    """Return read-back and cleanup contracts for packaged movie bytes."""
    if step.external_payload_path is None:
        fail_protocol("media import has no external payload identity")
    arguments: JsonObject = {"assetPath": step.destination}
    return (
        NativeToolRequirement(
            _IMPORT_TOOLSET,
            f"{_IMPORT_TOOLSET}.DeleteFileMediaSourcePayload",
            arguments,
            {"returnValue": True},
        ),
        NativeToolRequirement(
            _IMPORT_TOOLSET,
            f"{_IMPORT_TOOLSET}.FileMediaSourcePayloadExists",
            arguments,
            {"returnValue": True},
        ),
        NativeToolRequirement(
            _IMPORT_TOOLSET,
            f"{_IMPORT_TOOLSET}.GetFileMediaSourcePath",
            arguments,
            {"returnValue": step.external_payload_path},
        ),
    )
