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
#   - Serialized native application of one complete reviewed import plan.
# - Must-Not:
#   - Read source bytes, discover schemas, or execute incomplete plan subsets.
# - Allows:
#   - Import, independent read-back, explicit save, and compensating deletion.
# - Split-When:
#   - Split when construction, world, or package transactions gain lifecycles.
# - Merge-When:
#   - Merge when another service owns identical native import transactions.
# - Summary:
#   - Unreal native import-plan application service.
# - Description:
#   - Applies complete imports and compensates newly created assets on failure.
# - Usage:
#   - Called only after bundle, source, route, and live-capability gates pass.
# - Defaults:
#   - Preexisting destinations and incomplete evidence fail before mutation.
#

"""Serialized native application of one complete reviewed Unreal import plan."""

from __future__ import annotations

from collections.abc import Mapping
from pathlib import Path
from typing import NamedTuple
from typing import Protocol

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import normalize_json
from mcp.domain.json_types import require_json_object
from mcp.domain.plan_capabilities import PlanCapabilityReport
from mcp.domain.plan_execution import CompiledExecutionPlan
from mcp.domain.plan_execution import NativeImportStep
from mcp.domain.tool_outcome import ToolCallOutcome

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"


class NativePlanClient(Protocol):
    """Native call surface required by import-plan application."""

    def call_tool(
        self,
        toolset_name: str,
        tool_name: str,
        arguments: JsonObject,
    ) -> ToolCallOutcome:
        """Invoke one already-discovered and schema-validated native tool."""
        ...


class PlanApplicationReport(NamedTuple):
    """Public-safe successful import transaction evidence."""

    bundle_revision: str
    imported_count: int
    saved_count: int
    verified_count: int

    def to_json(self) -> JsonObject:
        """Render successful counts without physical or Unreal asset paths."""
        return {
            "bundleRevision": self.bundle_revision,
            "importedCount": self.imported_count,
            "savedCount": self.saved_count,
            "verifiedCount": self.verified_count,
        }


def apply_import_plan(
    client: NativePlanClient,
    compiled: CompiledExecutionPlan,
    capabilities: PlanCapabilityReport,
    source_paths: Mapping[str, Path],
) -> PlanApplicationReport:
    """Apply one complete import plan or compensate every created asset."""
    _require_application_ready(compiled, capabilities, source_paths)
    for step in compiled.imports:
        if _exists(client, step.package_path):
            fail_protocol("native import destination already exists")

    created: list[NativeImportStep] = []
    saved_count = 0
    verified_count = 0
    try:
        for step in compiled.imports:
            if _exists(client, step.package_path):
                fail_protocol("native import destination changed before import")
            source = source_paths[step.operation_id].absolute()
            try:
                outcome = client.call_tool(
                    step.toolset_name,
                    step.tool_name,
                    step.arguments(str(source)),
                )
            except BaseException as import_error:
                try:
                    if _exists(client, step.package_path):
                        created.append(step)
                except BaseException:
                    import_error.add_note(
                        "native import outcome and destination state are both unknown"
                    )
                raise
            created.append(step)
            _require_nonempty_import_result(outcome, step)
            if not _exists(client, step.package_path):
                fail_protocol("native import did not publish its destination")
            actual_class = _asset_class(client, step.package_path)
            if actual_class != step.target_class:
                fail_protocol("native import produced an unexpected asset class")
            verified_count += 1
            if not _save(client, step.package_path):
                fail_protocol("native import asset save returned false")
            saved_count += 1
            if _is_dirty(client, step.package_path):
                fail_protocol("native import asset remained dirty after save")
    except BaseException as error:
        _compensate(client, created, error)
        raise

    return PlanApplicationReport(
        bundle_revision=compiled.report.bundle_revision,
        imported_count=len(created),
        saved_count=saved_count,
        verified_count=verified_count,
    )


def _require_application_ready(
    compiled: CompiledExecutionPlan,
    capabilities: PlanCapabilityReport,
    source_paths: Mapping[str, Path],
) -> None:
    if not compiled.report.complete:
        fail_protocol("native import plan is incomplete")
    if not capabilities.complete:
        fail_protocol("native import capability audit is incomplete")
    if capabilities.bundle_revision != compiled.report.bundle_revision:
        fail_protocol("native import capability revision is stale")
    required_ids = {step.operation_id for step in compiled.imports}
    if not required_ids or not required_ids.issubset(source_paths):
        fail_protocol("native import source evidence is incomplete")
    if len(required_ids) != len(compiled.imports):
        fail_protocol("native import plan contains duplicate operation identities")


def _exists(client: NativePlanClient, package_path: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.exists",
        {"path": package_path},
    )
    return _return_boolean(outcome, context="asset existence result")


def _asset_class(client: NativePlanClient, package_path: str) -> str:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.get_asset_class",
        {"asset_path": package_path},
    )
    result = _structured_result(outcome, context="asset class result")
    value = result.get("returnValue")
    if not isinstance(value, str) or not value:
        fail_protocol("asset class result is not non-empty text")
    return value


def _save(client: NativePlanClient, package_path: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.save_assets",
        {"asset_paths": [package_path]},
    )
    return _return_boolean(outcome, context="asset save result")


def _is_dirty(client: NativePlanClient, package_path: str) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.is_dirty",
        {"asset_path": package_path},
    )
    return _return_boolean(outcome, context="asset dirty-state result")


def _delete(client: NativePlanClient, package_path: str) -> None:
    try:
        _ = client.call_tool(
            _ASSET_TOOLSET,
            f"{_ASSET_TOOLSET}.delete",
            {"path": package_path},
        )
    except BaseException:
        if not _exists(client, package_path):
            return
        raise
    if _exists(client, package_path):
        fail_protocol("compensating delete left the imported asset present")


def _compensate(
    client: NativePlanClient,
    created: list[NativeImportStep],
    primary_error: BaseException,
) -> None:
    failures = 0
    for step in reversed(created):
        try:
            _delete(client, step.package_path)
        except BaseException:
            failures += 1
    if failures:
        primary_error.add_note(
            f"native import compensation failed for {failures} created asset(s)"
        )


def _require_nonempty_import_result(
    outcome: ToolCallOutcome,
    step: NativeImportStep,
) -> None:
    result = _structured_result(outcome, context="native import result")
    value = normalize_json(
        result.get("returnValue"),
        context="native import result.returnValue",
    )
    if not isinstance(value, list) or not value:
        fail_protocol("native import result contains no created assets")
    if step.route_id == "sound-wave-wav-v1":
        if step.destination not in value:
            fail_protocol("native audio import omitted its planned destination")
        if any(not isinstance(item, str) or not item for item in value):
            fail_protocol("native audio import returned an invalid object path")
        return
    for item in value:
        if not isinstance(item, dict):
            fail_protocol("native import result contains a non-object asset")


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
    return require_json_object(outcome.structured_content, context=context)
