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
#   - Exact package-scoped selection over compiled vehicle Physics Asset
#   - requests.
# - Must-Not:
#   - Parse CLI input, read files, contact Unreal Editor, or invent blocker
#   - scope.
# - Allows:
#   - Retain ready Physics Asset requests for one canonical vehicle package.
# - Split-When:
#   - Another selector gains independent dependency or readiness semantics.
# - Merge-When:
#   - Vehicle-physics compilation owns identical package-scoping guarantees.
# - Summary:
#   - Vehicle-physics package selection policy.
# - Description:
#   - Derives one executable ready-rig subset after complete global compilation.
# - Usage:
#   - Called after release-bound vehicle-physics construction compilation.
# - Defaults:
#   - Unknown package ids and packages with no native-ready rigs fail closed.
#

"""Select one package from compiled vehicle Physics Asset requests."""

from __future__ import annotations

import re
from typing import NamedTuple

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.vehicle_physics_construction import (
    CompiledVehiclePhysicsConstruction,
)
from mcp.domain.vehicle_physics_construction import (
    VehiclePhysicsConstructionReport,
)

_PACKAGE_ID = re.compile(r"^[a-z0-9](?:[a-z0-9]|-(?=[a-z0-9]))*$")


class VehiclePhysicsSelectionReport(NamedTuple):
    """Public-safe counts for one package's ready Physics Asset requests."""

    package_id: str
    request_count: int
    shape_count: int
    global_blocked_rig_count: int

    def to_json(self) -> JsonObject:
        """Render exact scope without pretending blockers are package-local."""
        return {
            "globalBlockedRigCount": self.global_blocked_rig_count,
            "packageId": self.package_id,
            "requestCount": self.request_count,
            "shapeCount": self.shape_count,
        }


class CompiledVehiclePhysicsSelection(NamedTuple):
    """One package's executable Physics Asset request subset."""

    report: VehiclePhysicsSelectionReport
    construction: CompiledVehiclePhysicsConstruction


def select_vehicle_physics_package(
    compiled: CompiledVehiclePhysicsConstruction,
    package_id: str,
) -> CompiledVehiclePhysicsSelection:
    """Select one package only after complete construction compilation."""
    if _PACKAGE_ID.fullmatch(package_id) is None:
        fail_protocol("vehicle-physics package id is not canonical")
    requests = tuple(
        request
        for request in compiled.requests
        if request.package_id == package_id
    )
    if not requests:
        fail_protocol("vehicle-physics package has no native-ready rigs")
    shape_count = sum(len(request.shapes) for request in requests)
    selected = CompiledVehiclePhysicsConstruction(
        VehiclePhysicsConstructionReport(
            request_count=len(requests),
            blocked_rig_count=compiled.report.blocked_rig_count,
            shape_count=shape_count,
        ),
        requests,
    )
    return CompiledVehiclePhysicsSelection(
        VehiclePhysicsSelectionReport(
            package_id=package_id,
            request_count=len(requests),
            shape_count=shape_count,
            global_blocked_rig_count=compiled.report.blocked_rig_count,
        ),
        selected,
    )
