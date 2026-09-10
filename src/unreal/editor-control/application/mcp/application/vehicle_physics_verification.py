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
#   - Read-only verification of selected persisted vehicle Physics Assets.
# - Must-Not:
#   - Create, save, delete, repair, or otherwise mutate Unreal assets.
# - Allows:
#   - Check existence, class, dirty state, and exact native recipe equivalence.
# - Split-When:
#   - Verification gains persisted reports or mutation semantics.
# - Merge-When:
#   - Vehicle-physics application owns identical read-only certification.
# - Summary:
#   - Persisted vehicle Physics Asset verification transaction.
# - Description:
#   - Proves selected clean Physics Assets still match release-bound recipes.
# - Usage:
#   - Called after package-scoped compilation and live capability audit.
# - Defaults:
#   - Missing, dirty, wrong-class, or geometry-drifted assets fail closed.
#

"""Read-only verification of persisted vehicle Physics Assets."""

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
from mcp.domain.vehicle_physics_construction import (
    CompiledVehiclePhysicsConstruction,
)
from mcp.domain.vehicle_physics_construction import (
    vehicle_physics_construction_revision,
)
from mcp.domain.vehicle_physics_verification_capabilities import (
    VehiclePhysicsVerificationCapabilityReport,
)
from mcp.domain.vehicle_physics_verification_capabilities import (
    verification_arguments,
)
from mcp.domain.vehicle_physics_verification_capabilities import (
    verification_tool_name,
)

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"


class NativeVehiclePhysicsVerificationClient(Protocol):
    """Native read-only call surface required for persisted verification."""

    def call_tool(
        self,
        toolset_name: str,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        """Invoke one discovered and schema-validated native tool."""
        ...


class VehiclePhysicsVerificationReport(NamedTuple):
    """Public-safe evidence for successful persisted recipe verification."""

    construction_revision: str
    verified_count: int

    def to_json(self) -> JsonObject:
        """Render verification evidence without asset identities."""
        return {
            "constructionRevision": self.construction_revision,
            "verifiedCount": self.verified_count,
        }


def verify_vehicle_physics_assets(
    client: NativeVehiclePhysicsVerificationClient,
    compiled: CompiledVehiclePhysicsConstruction,
    capabilities: VehiclePhysicsVerificationCapabilityReport,
) -> VehiclePhysicsVerificationReport:
    """Verify selected persisted Physics Assets without mutation."""
    revision = vehicle_physics_construction_revision(compiled)
    if not capabilities.complete:
        fail_protocol(
            "vehicle-physics verification capability audit is incomplete"
        )
    if capabilities.construction_revision != revision:
        fail_protocol("vehicle-physics verification capability audit is stale")
    if capabilities.verification_count != len(compiled.requests):
        fail_protocol("vehicle-physics verification capability count is stale")
    for step in compiled.requests:
        if not _exists(client, step.package_path):
            fail_protocol("vehicle-physics verification destination is missing")
        if _asset_class(client, step.package_path) != step.target_class:
            fail_protocol("vehicle-physics verification class is unexpected")
        if _is_dirty(client, step.package_path):
            fail_protocol("vehicle-physics verification asset is dirty")
        outcome = client.call_tool(
            step.toolset_name,
            verification_tool_name(step),
            verification_arguments(step),
        )
        if not _return_boolean(
            outcome,
            context="vehicle-physics native verification result",
        ):
            fail_protocol("vehicle-physics persisted recipe equivalence failed")
    return VehiclePhysicsVerificationReport(revision, len(compiled.requests))


def _exists(
    client: NativeVehiclePhysicsVerificationClient,
    package_path: str,
) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.exists",
        {"path": package_path},
    )
    return _return_boolean(outcome, context="vehicle-physics existence result")


def _asset_class(
    client: NativeVehiclePhysicsVerificationClient,
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


def _is_dirty(
    client: NativeVehiclePhysicsVerificationClient,
    package_path: str,
) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.is_dirty",
        {"asset_path": package_path},
    )
    return _return_boolean(outcome, context="vehicle-physics dirty result")


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
    return require_json_object(value, context=context)
