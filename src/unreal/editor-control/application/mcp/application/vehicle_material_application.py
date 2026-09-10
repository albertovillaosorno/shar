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
#   - Serialized native application of representable vehicle-material requests.
# - Must-Not:
#   - Read source bytes, assign mesh slots, or claim blocked presentation
#   - fidelity.
# - Allows:
#   - Create, read back, save, and compensate verified generated assets.
# - Split-When:
#   - Slot assignment or another material family gains a native lifecycle.
# - Merge-When:
#   - General plan application accepts construction transactions.
# - Summary:
#   - Vehicle-material native construction transaction.
# - Description:
#   - Applies texture, master, and instance stages with reverse compensation.
# - Usage:
#   - Called after sidecar, PNG, and live-capability preflight all succeed.
# - Defaults:
#   - Every destination must be absent before the first native mutation.
#

"""Serialized native application of vehicle-material construction."""

from __future__ import annotations

from collections.abc import Mapping
import json
from pathlib import Path
from typing import NamedTuple
from typing import Protocol

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import DuplicateJsonKeyError
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import normalize_json
from mcp.domain.json_types import reject_duplicate_json_object
from mcp.domain.json_types import require_json_object
from mcp.domain.tool_outcome import ToolCallOutcome
from mcp.domain.vehicle_material_capabilities import (
    VehicleMaterialCapabilityReport,
)
from mcp.domain.vehicle_material_capabilities import (
    vehicle_material_construction_revision,
)
from mcp.domain.vehicle_material_selection import VehicleMaterialExecutable

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"


class NativeVehicleMaterialClient(Protocol):
    """Native call surface required by vehicle-material application."""

    def call_tool(
        self,
        toolset_name: str,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        """Invoke one already-discovered and schema-validated native tool."""
        ...


class VehicleMaterialApplicationReport(NamedTuple):
    """Public-safe evidence for one successful construction transaction."""

    construction_revision: str
    created_count: int
    saved_count: int
    verified_count: int

    def to_json(self) -> JsonObject:
        """Render successful counts without paths or source identities."""
        return {
            "constructionRevision": self.construction_revision,
            "createdCount": self.created_count,
            "savedCount": self.saved_count,
            "verifiedCount": self.verified_count,
        }


class _AssetSpec(NamedTuple):
    package_path: str
    object_path: str
    target_class: str


class _NativeCall(NamedTuple):
    spec: _AssetSpec
    toolset_name: str
    tool_name: str
    arguments: JsonObject
    result_kind: str


def apply_vehicle_material_construction(
    client: NativeVehicleMaterialClient,
    compiled: VehicleMaterialExecutable,
    capabilities: VehicleMaterialCapabilityReport,
    texture_sources: Mapping[str, Path],
) -> VehicleMaterialApplicationReport:
    """Apply all representable requests or compensate every created asset."""
    _require_ready(compiled, capabilities, texture_sources)
    for spec in _all_specs(compiled):
        _require_absent(client, spec, changed=False)

    created: list[_AssetSpec] = []
    try:
        _apply_textures(client, compiled, texture_sources, created)
        _apply_masters(client, compiled, created)
        _apply_instances(client, compiled, created)
    except Exception as error:
        _compensate(client, created, error)
        raise
    return VehicleMaterialApplicationReport(
        construction_revision=capabilities.construction_revision,
        created_count=len(created),
        saved_count=len(created),
        verified_count=len(created),
    )


def _apply_textures(
    client: NativeVehicleMaterialClient,
    compiled: VehicleMaterialExecutable,
    texture_sources: Mapping[str, Path],
    created: list[_AssetSpec],
) -> None:
    for step in compiled.textures:
        spec = _AssetSpec(
            step.package_path, step.object_path, step.target_class
        )
        call = _NativeCall(
            spec,
            step.toolset_name,
            step.tool_name,
            step.arguments(str(texture_sources[step.sha256].absolute())),
            "texture",
        )
        _apply_call(client, call, created)


def _apply_masters(
    client: NativeVehicleMaterialClient,
    compiled: VehicleMaterialExecutable,
    created: list[_AssetSpec],
) -> None:
    for step in compiled.masters:
        spec = _AssetSpec(
            step.package_path, step.object_path, step.target_class
        )
        call = _NativeCall(
            spec,
            step.toolset_name,
            step.tool_name,
            step.arguments(),
            "object",
        )
        _apply_call(client, call, created)


def _apply_instances(
    client: NativeVehicleMaterialClient,
    compiled: VehicleMaterialExecutable,
    created: list[_AssetSpec],
) -> None:
    for step in compiled.instances:
        spec = _AssetSpec(
            step.package_path, step.object_path, step.target_class
        )
        call = _NativeCall(
            spec,
            step.toolset_name,
            step.tool_name,
            step.arguments(),
            "object",
        )
        _apply_call(client, call, created)


def _apply_call(
    client: NativeVehicleMaterialClient,
    call: _NativeCall,
    created: list[_AssetSpec],
) -> None:
    _require_absent(client, call.spec, changed=True)
    outcome = _invoke(client, call, created)
    created.append(call.spec)
    if call.result_kind == "texture":
        _require_texture_result(outcome, call.spec)
    elif call.result_kind == "object":
        _require_object_path_result(outcome, call.spec)
    else:
        fail_protocol("vehicle-material native result kind is unsupported")
    _verify_and_save(client, call.spec)


def _require_ready(
    compiled: VehicleMaterialExecutable,
    capabilities: VehicleMaterialCapabilityReport,
    texture_sources: Mapping[str, Path],
) -> None:
    expected_revision = vehicle_material_construction_revision(compiled)
    if not capabilities.complete:
        fail_protocol("vehicle-material capability audit is incomplete")
    if capabilities.construction_revision != expected_revision:
        fail_protocol("vehicle-material capability audit is stale")
    expected_count = sum((
        len(compiled.textures),
        len(compiled.masters),
        len(compiled.instances),
    ))
    if capabilities.construction_count != expected_count:
        fail_protocol("vehicle-material capability count is stale")
    required_sources = {step.sha256 for step in compiled.textures}
    if set(texture_sources) != required_sources:
        fail_protocol("vehicle-material texture source evidence is not exact")


def _all_specs(
    compiled: VehicleMaterialExecutable,
) -> tuple[_AssetSpec, ...]:
    return tuple(
        _AssetSpec(step.package_path, step.object_path, step.target_class)
        for step in (*compiled.textures, *compiled.masters, *compiled.instances)
    )


def _require_absent(
    client: NativeVehicleMaterialClient,
    spec: _AssetSpec,
    *,
    changed: bool,
) -> None:
    if _exists(client, spec.package_path):
        message = (
            "vehicle-material destination changed before construction"
            if changed
            else "vehicle-material destination already exists"
        )
        fail_protocol(message)


def _invoke(
    client: NativeVehicleMaterialClient,
    call: _NativeCall,
    created: list[_AssetSpec],
) -> ToolCallOutcome:
    try:
        return client.call_tool(
            call.toolset_name,
            call.tool_name,
            call.arguments,
        )
    except Exception as construction_error:
        try:
            if _exists(client, call.spec.package_path):
                created.append(call.spec)
        except Exception:  # noqa: BLE001
            construction_error.add_note(
                "vehicle-material outcome and destination state are both "
                "unknown"
            )
        raise


def _require_texture_result(
    outcome: ToolCallOutcome,
    spec: _AssetSpec,
) -> None:
    result = _structured_result(
        outcome, context="vehicle texture import result"
    )
    value = normalize_json(
        result.get("returnValue"),
        context="vehicle texture import result.returnValue",
    )
    if value != [spec.object_path]:
        fail_protocol(
            "vehicle texture import returned an unexpected object inventory"
        )


def _require_object_path_result(
    outcome: ToolCallOutcome,
    spec: _AssetSpec,
) -> None:
    result = _structured_result(
        outcome, context="vehicle material creation result"
    )
    if result.get("returnValue") != spec.object_path:
        fail_protocol("vehicle material creation returned an unexpected object")


def _verify_and_save(
    client: NativeVehicleMaterialClient,
    spec: _AssetSpec,
) -> None:
    if not _exists(client, spec.package_path):
        fail_protocol("vehicle-material construction omitted its destination")
    if _asset_class(client, spec.package_path) != spec.target_class:
        fail_protocol(
            "vehicle-material construction produced an unexpected class"
        )
    if not _is_dirty(client, spec.package_path):
        fail_protocol(
            "vehicle-material construction did not leave its asset dirty"
        )
    if not _save(client, spec.package_path):
        fail_protocol("vehicle-material asset save returned false")
    if _is_dirty(client, spec.package_path):
        fail_protocol("vehicle-material asset remained dirty after save")


def _exists(client: NativeVehicleMaterialClient, package_path: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.exists",
        {"path": package_path},
    )
    return _return_boolean(outcome, context="vehicle-material existence result")


def _asset_class(
    client: NativeVehicleMaterialClient,
    package_path: str,
) -> str:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.get_asset_class",
        {"asset_path": package_path},
    )
    result = _structured_result(
        outcome, context="vehicle-material class result"
    )
    value = result.get("returnValue")
    if not isinstance(value, str) or not value:
        fail_protocol("vehicle-material class result is not non-empty text")
    return value


def _is_dirty(client: NativeVehicleMaterialClient, package_path: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.is_dirty",
        {"asset_path": package_path},
    )
    return _return_boolean(outcome, context="vehicle-material dirty result")


def _save(client: NativeVehicleMaterialClient, package_path: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.save_assets",
        {"asset_paths": [package_path]},
    )
    return _return_boolean(outcome, context="vehicle-material save result")


def _delete(client: NativeVehicleMaterialClient, package_path: str) -> None:
    try:
        _ = client.call_tool(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.delete",
            {"path": package_path},
        )
    except Exception:
        if not _exists(client, package_path):
            return
        raise
    if _exists(client, package_path):
        fail_protocol(
            "vehicle-material compensating delete left an asset present"
        )


def _compensate(
    client: NativeVehicleMaterialClient,
    created: list[_AssetSpec],
    primary_error: Exception,
) -> None:
    failures = 0
    for spec in reversed(created):
        try:
            _delete(client, spec.package_path)
        except Exception:  # noqa: BLE001
            failures += 1
    if failures:
        primary_error.add_note(
            f"vehicle-material compensation failed for {failures} asset(s)"
        )


def _return_boolean(outcome: ToolCallOutcome, *, context: str) -> bool:
    result = _structured_result(outcome, context=context)
    value = result.get("returnValue")
    if not isinstance(value, bool):
        fail_protocol(f"{context} is not boolean")
    return value


def _structured_result(
    outcome: ToolCallOutcome,
    *,
    context: str,
) -> JsonObject:
    _ = outcome.require_success()
    value = outcome.structured_content
    if value is None:
        if not outcome.text:
            fail_protocol(f"{context}: result omitted JSON content")
        try:
            value = json.loads(
                outcome.text,
                object_pairs_hook=reject_duplicate_json_object,
                parse_constant=lambda _: fail_protocol(
                    f"{context}: non-finite JSON number is not supported"
                ),
            )
        except DuplicateJsonKeyError as error:
            fail_protocol(str(error), cause=error)
        except (json.JSONDecodeError, UnicodeError) as error:
            fail_protocol(
                f"{context}: text result is not valid JSON",
                cause=error,
            )
    normalized = normalize_json(value, context=context)
    return require_json_object(normalized, context=context)
