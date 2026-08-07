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
#   - Deterministic compilation of validated plans into native import steps.
# - Must-Not:
#   - Read files, contact Unreal Editor, or claim unsupported routes are ready.
# - Allows:
#   - Classify readiness and map exact source contracts to native tool calls.
# - Split-When:
#   - Split when construction, world, or package execution gains a lifecycle.
# - Merge-When:
#   - Merge when native plan application owns identical compilation policy.
# - Summary:
#   - Unreal native plan execution compiler.
# - Description:
#   - Produces typed import steps and an honest unsupported-route report.
# - Usage:
#   - Called after canonical bundle preflight and before source or tool checks.
# - Defaults:
#   - Only exact reviewed native import routes compile.
#

"""Deterministic compilation of validated plans into native import steps."""

from __future__ import annotations

from collections import Counter
from typing import NamedTuple

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.plan_bundle import PlanOperation
from mcp.domain.plan_bundle import ValidatedPlanBundle

_TEXTURE_ROUTE = (
    "image",
    "Texture2D",
    "texture-factory",
    "shar-texture-v1",
)
_STATIC_MESH_ROUTE = (
    "fbx",
    "StaticMesh",
    "asset-tools-fbx",
    "shar-fbx-static-v1",
)
_AUDIO_ROUTE = (
    "wav",
    "SoundWave",
    "sound-wave-factory",
    "shar-audio-v1",
)
_MEDIA_ROUTE = (
    "hap",
    "FileMediaSource",
    "media-source-movie",
    "shar-hap-movie-v1",
)
_IMPORT_TOOLSET = "SharImportEditor.SharImportToolset"


class NativeImportStep(NamedTuple):
    """One reviewed direct-import call awaiting a verified absolute source."""

    operation_id: str
    route_id: str
    source_path: str
    source_revision: str
    destination: str
    target_class: str
    package_path: str
    folder_path: str
    asset_name: str
    toolset_name: str
    tool_name: str
    external_payload_path: str | None

    @property
    def has_external_payload(self) -> bool:
        """Whether this import publishes one file outside its asset package."""
        return self.external_payload_path is not None

    def arguments(self, source_file: str) -> JsonObject:
        """Build exact native arguments for one verified physical source."""
        if not source_file:
            fail_protocol("native import source file must not be empty")
        if self.route_id in {
            "file-media-source-hap-v1",
            "sound-wave-wav-v1",
            "static-mesh-fbx-v1",
        }:
            return {
                "assetName": self.asset_name,
                "folderPath": self.folder_path,
                "sourceFile": source_file,
            }
        arguments: JsonObject = {
            "asset_name": self.asset_name,
            "folder_path": self.folder_path,
            "source_file": source_file,
        }
        return arguments


class PlanExecutionReport(NamedTuple):
    """Public-safe compilation summary for one validated plan bundle."""

    bundle_revision: str
    operation_count: int
    compiled_count: int
    semantic_blocker_count: int
    blocked_readiness: dict[str, int]
    unsupported_routes: dict[str, int]
    route_counts: dict[str, int]

    @property
    def complete(self) -> bool:
        """Whether every operation has one reviewed executable route."""
        return (
            self.compiled_count == self.operation_count
            and self.semantic_blocker_count == 0
            and not self.blocked_readiness
            and not self.unsupported_routes
        )

    def to_json(self) -> JsonObject:
        """Render counts without source paths or local filesystem evidence."""
        return {
            "blockedReadiness": dict(sorted(self.blocked_readiness.items())),
            "bundleRevision": self.bundle_revision,
            "compiledCount": self.compiled_count,
            "complete": self.complete,
            "operationCount": self.operation_count,
            "routeCounts": dict(sorted(self.route_counts.items())),
            "semanticBlockerCount": self.semantic_blocker_count,
            "unsupportedRoutes": dict(sorted(self.unsupported_routes.items())),
        }


class CompiledExecutionPlan(NamedTuple):
    """Public report and ordered native import steps."""

    report: PlanExecutionReport
    imports: tuple[NativeImportStep, ...]


def compile_execution_plan(
    bundle: ValidatedPlanBundle,
) -> CompiledExecutionPlan:
    """Compile every operation or classify its exact blocking reason."""
    imports: list[NativeImportStep] = []
    blocked = Counter[str]()
    unsupported = Counter[str]()
    routes = Counter[str]()
    for operation in bundle.operations:
        if operation.readiness != "ready":
            blocked[operation.readiness] += 1
            continue
        step = _compile_ready_import(operation)
        if step is None:
            unsupported[_route_key(operation)] += 1
            continue
        imports.append(step)
        routes[step.route_id] += 1
    report = PlanExecutionReport(
        bundle_revision=bundle.report.revision,
        operation_count=len(bundle.operations),
        compiled_count=len(imports),
        semantic_blocker_count=bundle.report.semantic_blocker_count,
        blocked_readiness=dict(blocked),
        unsupported_routes=dict(unsupported),
        route_counts=dict(routes),
    )
    return CompiledExecutionPlan(report, tuple(imports))


def _compile_ready_import(operation: PlanOperation) -> NativeImportStep | None:
    route = (
        operation.source_format,
        operation.target_class,
        operation.importer,
        operation.import_profile,
    )
    external_payload_path: str | None = None
    if route == _TEXTURE_ROUTE:
        route_id = "texture-image-v1"
        toolset = "editor_toolset.toolsets.texture.TextureTools"
        tool = f"{toolset}.import_file"
    elif route == _AUDIO_ROUTE:
        route_id = "sound-wave-wav-v1"
        toolset = _IMPORT_TOOLSET
        tool = f"{toolset}.ImportSoundWave"
    elif route == _MEDIA_ROUTE:
        route_id = "file-media-source-hap-v1"
        toolset = _IMPORT_TOOLSET
        tool = f"{toolset}.ImportFileMediaSource"
    elif route == _STATIC_MESH_ROUTE:
        route_id = "static-mesh-fbx-v1"
        toolset = _IMPORT_TOOLSET
        tool = f"{toolset}.ImportStaticMesh"
    else:
        return None
    package_path, separator, object_name = operation.destination.rpartition(".")
    if not separator:
        fail_protocol("compiled import destination has no object name")
    folder_path, slash, asset_name = package_path.rpartition("/")
    if not slash or object_name != asset_name:
        fail_protocol("compiled import destination is not canonical")
    if route == _MEDIA_ROUTE:
        relative_package = package_path.removeprefix("/Game/Generated/SHAR/")
        if relative_package == package_path or not relative_package:
            fail_protocol("compiled media destination escaped generated root")
        external_payload_path = (
            f"./Movies/Generated/SHAR/{relative_package}.mov"
        )
    return NativeImportStep(
        operation_id=operation.operation_id,
        route_id=route_id,
        source_path=operation.source_path,
        source_revision=operation.source_revision,
        destination=operation.destination,
        target_class=operation.target_class,
        package_path=package_path,
        folder_path=folder_path,
        asset_name=asset_name,
        toolset_name=toolset,
        tool_name=tool,
        external_payload_path=external_payload_path,
    )


def _route_key(operation: PlanOperation) -> str:
    return (
        f"{operation.source_format}/{operation.target_class}/"
        f"{operation.importer}/{operation.import_profile}"
    )
