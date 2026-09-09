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
#   - Serialized save/verify/compensate transaction for vehicle Physics Assets.
# - Must-Not:
#   - Delete Skeletal Mesh dependencies, parse source evidence, or infer shapes.
# - Allows:
#   - Publish, read back, save, and compensate only owned Physics Assets.
# - Split-When:
#   - Presentation binding or live Chaos construction gains a lifecycle.
# - Merge-When:
#   - General construction application accepts Physics Asset transactions.
# - Summary:
#   - Vehicle Physics Asset native application transaction.
# - Description:
#   - Verifies dependencies, creates assets, saves, and reverses owned outputs.
# - Usage:
#   - Called after semantic compilation and live capability preflight.
# - Defaults:
#   - Dependencies must exist and destinations must be absent before mutation.
#

"""Serialized native application of vehicle Physics Asset construction."""

from __future__ import annotations

import json
from typing import NamedTuple
from typing import Protocol

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import DuplicateJsonKeyError
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import reject_duplicate_json_object
from mcp.domain.json_types import require_json_object
from mcp.domain.tool_outcome import ToolCallOutcome
from mcp.domain.vehicle_physics_capabilities import (
    VehiclePhysicsCapabilityReport,
)
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


class NativeVehiclePhysicsClient(Protocol):
    """Native call surface required by Physics Asset application."""

    def call_tool(
        self,
        toolset_name: str,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        """Invoke one discovered and schema-validated native tool."""
        ...


class VehiclePhysicsApplicationReport(NamedTuple):
    """Public-safe evidence for successful ready-rig publication."""

    construction_revision: str
    created_count: int
    saved_count: int
    verified_count: int
    blocked_rig_count: int

    def to_json(self) -> JsonObject:
        """Render outcome counts without asset identities."""
        return {
            "blockedRigCount": self.blocked_rig_count,
            "constructionRevision": self.construction_revision,
            "createdCount": self.created_count,
            "savedCount": self.saved_count,
            "verifiedCount": self.verified_count,
        }


def apply_vehicle_physics_construction(
    client: NativeVehiclePhysicsClient,
    compiled: CompiledVehiclePhysicsConstruction,
    capabilities: VehiclePhysicsCapabilityReport,
) -> VehiclePhysicsApplicationReport:
    """Publish every ready rig or compensate every created Physics Asset."""
    _require_ready(compiled, capabilities)
    for step in compiled.requests:
        _require_dependency(client, step)
        _require_absent(client, step.package_path, changed=False)
    created: list[VehiclePhysicsConstructionStep] = []
    try:
        for step in compiled.requests:
            _require_dependency(client, step)
            _require_absent(client, step.package_path, changed=True)
            outcome = _invoke(client, step, created)
            created.append(step)
            result = _structured_result(
                outcome, context="vehicle-physics creation result"
            )
            if result.get("returnValue") != step.object_path:
                fail_protocol(
                    "vehicle-physics creation returned an unexpected object"
                )
            _verify_and_save(client, step)
    except Exception as error:
        _compensate(client, created, error)
        raise
    return VehiclePhysicsApplicationReport(
        construction_revision=capabilities.construction_revision,
        created_count=len(created),
        saved_count=len(created),
        verified_count=len(created),
        blocked_rig_count=compiled.report.blocked_rig_count,
    )


def _require_ready(
    compiled: CompiledVehiclePhysicsConstruction,
    capabilities: VehiclePhysicsCapabilityReport,
) -> None:
    revision = vehicle_physics_construction_revision(compiled)
    if not capabilities.complete:
        fail_protocol("vehicle-physics capability audit is incomplete")
    if capabilities.construction_revision != revision:
        fail_protocol("vehicle-physics capability audit is stale")
    if capabilities.construction_count != len(compiled.requests):
        fail_protocol("vehicle-physics capability count is stale")


def _require_dependency(
    client: NativeVehiclePhysicsClient,
    step: VehiclePhysicsConstructionStep,
) -> None:
    package = step.skeletal_mesh_path.rpartition(".")[0]
    if not package or not _exists(client, package):
        fail_protocol("vehicle-physics Skeletal Mesh dependency is missing")
    if _asset_class(client, package) != "SkeletalMesh":
        fail_protocol("vehicle-physics dependency is not a SkeletalMesh")


def _require_absent(
    client: NativeVehiclePhysicsClient,
    package_path: str,
    *,
    changed: bool,
) -> None:
    if _exists(client, package_path):
        message = (
            "vehicle-physics destination changed before construction"
            if changed
            else "vehicle-physics destination already exists"
        )
        fail_protocol(message)


def _invoke(
    client: NativeVehiclePhysicsClient,
    step: VehiclePhysicsConstructionStep,
    created: list[VehiclePhysicsConstructionStep],
) -> ToolCallOutcome:
    try:
        return client.call_tool(
            step.toolset_name,
            step.tool_name,
            step.arguments(),
        )
    except Exception as construction_error:
        try:
            if _exists(client, step.package_path):
                created.append(step)
        except Exception:  # noqa: BLE001
            construction_error.add_note(
                "vehicle-physics outcome and destination state are both unknown"
            )
        raise


def _verify_and_save(
    client: NativeVehiclePhysicsClient,
    step: VehiclePhysicsConstructionStep,
) -> None:
    if not _exists(client, step.package_path):
        fail_protocol("vehicle-physics construction omitted its destination")
    if _asset_class(client, step.package_path) != step.target_class:
        fail_protocol(
            "vehicle-physics construction produced an unexpected class"
        )
    if not _is_dirty(client, step.package_path):
        fail_protocol("vehicle-physics construction did not leave asset dirty")
    if not _save(client, step.package_path):
        fail_protocol("vehicle-physics asset save returned false")
    if _is_dirty(client, step.package_path):
        fail_protocol("vehicle-physics asset remained dirty after save")


def _exists(client: NativeVehiclePhysicsClient, package_path: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.exists",
        {"path": package_path},
    )
    return _return_boolean(outcome, context="vehicle-physics existence result")


def _asset_class(
    client: NativeVehiclePhysicsClient,
    package_path: str,
) -> str:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.get_asset_class",
        {"asset_path": package_path},
    )
    result = _structured_result(outcome, context="vehicle-physics class result")
    value = result.get("returnValue")
    if not isinstance(value, str) or not value:
        fail_protocol("vehicle-physics class result is not non-empty text")
    return value


def _is_dirty(client: NativeVehiclePhysicsClient, package_path: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.is_dirty",
        {"asset_path": package_path},
    )
    return _return_boolean(outcome, context="vehicle-physics dirty result")


def _save(client: NativeVehiclePhysicsClient, package_path: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.save_assets",
        {"asset_paths": [package_path]},
    )
    return _return_boolean(outcome, context="vehicle-physics save result")


def _delete(client: NativeVehiclePhysicsClient, package_path: str) -> None:
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
        fail_protocol("vehicle-physics compensating delete left asset present")


def _compensate(
    client: NativeVehiclePhysicsClient,
    created: list[VehiclePhysicsConstructionStep],
    primary_error: Exception,
) -> None:
    failures = 0
    for step in reversed(created):
        try:
            _delete(client, step.package_path)
        except Exception:  # noqa: BLE001
            failures += 1
    if failures:
        primary_error.add_note(
            f"vehicle-physics compensation failed for {failures} asset(s)"
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
                f"{context}: text result is not valid JSON", cause=error
            )
    return require_json_object(value, context=context)
