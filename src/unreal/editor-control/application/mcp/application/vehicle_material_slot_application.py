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
#   - Serialized save/verify/compensate transaction for vehicle material slots.
# - Must-Not:
#   - Create/delete mesh or material assets, parse source files, or infer slots.
# - Allows:
#   - CAS selected null slots to verified MICs, save, read back, and rollback.
# - Split-When:
#   - Runtime light toggling or another presentation mutation gains lifecycle.
# - Merge-When:
#   - Vehicle material application owns slot publication transactions.
# - Summary:
#   - Vehicle material slot native application transaction.
# - Description:
#   - Verifies dependencies and clean state before one reversible slot publish.
# - Usage:
#   - Called after exact binding compilation and live capability audit.
# - Defaults:
#   - First publication requires clean mesh and null selected material slots.
#

"""Serialized native application of vehicle material slot bindings."""

from __future__ import annotations

import json
from typing import NamedTuple
from typing import Protocol

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import DuplicateJsonKeyError
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import normalize_json
from mcp.domain.json_types import reject_duplicate_json_object
from mcp.domain.json_types import require_json_object
from mcp.domain.tool_outcome import ToolCallOutcome
from mcp.domain.vehicle_material_slot_binding import (
    CompiledVehicleMaterialSlotBinding,
)
from mcp.domain.vehicle_material_slot_binding import (
    vehicle_material_slot_binding_revision,
)
from mcp.domain.vehicle_material_slot_capabilities import (
    VehicleMaterialSlotCapabilityReport,
)

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"


class NativeVehicleMaterialSlotClient(Protocol):
    """Native call surface required by vehicle material slot publication."""

    def call_tool(
        self,
        toolset_name: str,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        """Invoke one discovered and schema-validated native tool."""
        ...


class VehicleMaterialSlotApplicationReport(NamedTuple):
    """Public-safe evidence for one successful slot publication."""

    binding_revision: str
    slot_count: int
    saved_count: int
    verified_count: int

    def to_json(self) -> JsonObject:
        """Render outcome counts without asset paths."""
        return {
            "bindingRevision": self.binding_revision,
            "savedCount": self.saved_count,
            "slotCount": self.slot_count,
            "verifiedCount": self.verified_count,
        }


def apply_vehicle_material_slot_binding(
    client: NativeVehicleMaterialSlotClient,
    compiled: CompiledVehicleMaterialSlotBinding,
    capabilities: VehicleMaterialSlotCapabilityReport,
) -> VehicleMaterialSlotApplicationReport:
    """Publish selected null slots or restore them on every later failure."""
    _require_ready(compiled, capabilities)
    _require_dependencies(client, compiled)
    if _is_dirty(client, compiled.skeletal_mesh_package_path):
        fail_protocol("vehicle material slot Skeletal Mesh is already dirty")
    empty = tuple("" for _ in compiled.slot_indices)
    if _read_slots(client, compiled) != empty:
        fail_protocol("vehicle material selected slots are not null")

    try:
        _publish_slots(client, compiled, empty)
    except Exception as error:
        _compensate(client, compiled, empty, error)
        raise
    return VehicleMaterialSlotApplicationReport(
        capabilities.binding_revision,
        len(compiled.slot_indices),
        1,
        len(compiled.slot_indices),
    )


def _publish_slots(
    client: NativeVehicleMaterialSlotClient,
    compiled: CompiledVehicleMaterialSlotBinding,
    empty: tuple[str, ...],
) -> None:
    result = _compare_exchange(client, compiled, empty, compiled.material_paths)
    if result != compiled.material_paths:
        fail_protocol("vehicle material slot CAS returned unexpected paths")
    if _read_slots(client, compiled) != compiled.material_paths:
        fail_protocol("vehicle material slot read-back drifted after CAS")
    if not _is_dirty(client, compiled.skeletal_mesh_package_path):
        fail_protocol("vehicle material slot CAS did not dirty Skeletal Mesh")
    if not _save(client, compiled.skeletal_mesh_package_path):
        fail_protocol("vehicle material slot Skeletal Mesh save returned false")
    if _is_dirty(client, compiled.skeletal_mesh_package_path):
        fail_protocol("vehicle material slot Skeletal Mesh stayed dirty")
    if _read_slots(client, compiled) != compiled.material_paths:
        fail_protocol("vehicle material slot persisted read-back drifted")


def _require_ready(
    compiled: CompiledVehicleMaterialSlotBinding,
    capabilities: VehicleMaterialSlotCapabilityReport,
) -> None:
    if not capabilities.complete:
        fail_protocol("vehicle material slot capability audit is incomplete")
    if capabilities.binding_revision != vehicle_material_slot_binding_revision(
        compiled
    ):
        fail_protocol("vehicle material slot capability audit is stale")
    if capabilities.slot_count != len(compiled.slot_indices):
        fail_protocol("vehicle material slot capability count is stale")


def _require_dependencies(
    client: NativeVehicleMaterialSlotClient,
    compiled: CompiledVehicleMaterialSlotBinding,
) -> None:
    dependencies = (
        (compiled.skeletal_mesh_package_path, "SkeletalMesh"),
        *(
            (package, "MaterialInstanceConstant")
            for package in compiled.material_package_paths
        ),
    )
    for package, expected_class in dependencies:
        if not _exists(client, package):
            fail_protocol("vehicle material slot dependency is missing")
        if _asset_class(client, package) != expected_class:
            fail_protocol("vehicle material slot dependency class drifted")


def _read_slots(
    client: NativeVehicleMaterialSlotClient,
    compiled: CompiledVehicleMaterialSlotBinding,
) -> tuple[str, ...]:
    outcome = client.call_tool(
        compiled.toolset_name,
        compiled.read_tool_name,
        compiled.read_arguments(),
    )
    result = _structured_result(
        outcome, context="vehicle material slot read result"
    )
    return _return_paths(
        result, len(compiled.slot_indices), "vehicle material slot read"
    )


def _compare_exchange(
    client: NativeVehicleMaterialSlotClient,
    compiled: CompiledVehicleMaterialSlotBinding,
    expected: tuple[str, ...],
    replacement: tuple[str, ...],
) -> tuple[str, ...]:
    outcome = client.call_tool(
        compiled.toolset_name,
        compiled.compare_exchange_tool_name,
        compiled.compare_exchange_arguments(expected, replacement),
    )
    result = _structured_result(
        outcome, context="vehicle material slot CAS result"
    )
    return _return_paths(
        result, len(compiled.slot_indices), "vehicle material slot CAS"
    )


def _compensate(
    client: NativeVehicleMaterialSlotClient,
    compiled: CompiledVehicleMaterialSlotBinding,
    empty: tuple[str, ...],
    primary_error: Exception,
) -> None:
    try:
        _restore_slots(client, compiled, empty)
    except Exception as rollback_error:  # noqa: BLE001
        primary_error.add_note(
            "vehicle material slot compensation failed: "
            f"{type(rollback_error).__name__}"
        )


def _restore_slots(
    client: NativeVehicleMaterialSlotClient,
    compiled: CompiledVehicleMaterialSlotBinding,
    empty: tuple[str, ...],
) -> None:
    current = _read_slots(client, compiled)
    if current == empty:
        return
    if current != compiled.material_paths:
        fail_protocol("vehicle material slot compensation found unknown state")
    restored = _compare_exchange(
        client, compiled, compiled.material_paths, empty
    )
    if restored != empty or _read_slots(client, compiled) != empty:
        fail_protocol("vehicle material slot compensation read-back drifted")
    if _is_dirty(client, compiled.skeletal_mesh_package_path):
        if not _save(client, compiled.skeletal_mesh_package_path):
            fail_protocol("vehicle material slot rollback save returned false")
        if _is_dirty(client, compiled.skeletal_mesh_package_path):
            fail_protocol("vehicle material slot rollback stayed dirty")
    if _read_slots(client, compiled) != empty:
        fail_protocol("vehicle material slot rollback persistence drifted")


def _exists(client: NativeVehicleMaterialSlotClient, package: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.exists",
        {"path": package},
    )
    return _return_boolean(
        outcome, context="vehicle material slot existence result"
    )


def _asset_class(client: NativeVehicleMaterialSlotClient, package: str) -> str:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.get_asset_class",
        {"asset_path": package},
    )
    result = _structured_result(
        outcome, context="vehicle material slot class result"
    )
    value = result.get("returnValue")
    if not isinstance(value, str) or not value:
        fail_protocol("vehicle material slot class result is not text")
    return value


def _is_dirty(client: NativeVehicleMaterialSlotClient, package: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.is_dirty",
        {"asset_path": package},
    )
    return _return_boolean(
        outcome, context="vehicle material slot dirty result"
    )


def _save(client: NativeVehicleMaterialSlotClient, package: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.save_assets",
        {"asset_paths": [package]},
    )
    return _return_boolean(outcome, context="vehicle material slot save result")


def _return_boolean(outcome: ToolCallOutcome, *, context: str) -> bool:
    result = _structured_result(outcome, context=context)
    value = result.get("returnValue")
    if not isinstance(value, bool):
        fail_protocol(f"{context} is not boolean")
    return value


def _return_paths(
    result: JsonObject, count: int, context: str
) -> tuple[str, ...]:
    value = normalize_json(
        result.get("returnValue"), context=f"{context}.returnValue"
    )
    if not isinstance(value, list) or len(value) != count:
        fail_protocol(f"{context} path count is inconsistent")
    if any(not isinstance(item, str) for item in value):
        fail_protocol(f"{context} contains a non-text path")
    return tuple(value)


def _structured_result(outcome: ToolCallOutcome, *, context: str) -> JsonObject:
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
                f"{context}: text result is not valid JSON", cause=error
            )
    normalized = normalize_json(value, context=context)
    return require_json_object(normalized, context=context)
