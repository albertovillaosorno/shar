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
from mcp.domain.plan_capabilities import PlanCapabilityReport
from mcp.domain.plan_execution import CompiledExecutionPlan
from mcp.domain.plan_execution import NativeAssetOutput
from mcp.domain.plan_execution import NativeImportStep
from mcp.domain.tool_outcome import ToolCallOutcome

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"
_IMPORT_TOOLSET = "SharImportEditor.SharImportToolset"


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
        _require_step_absent(client, step, changed=False)

    created: list[NativeImportStep] = []
    saved_count = 0
    verified_count = 0
    try:
        for step in compiled.imports:
            _require_step_absent(client, step, changed=True)
            source = source_paths[step.operation_id].absolute()
            outcome = _invoke_import(client, step, source, created)
            created.append(step)
            _verify_and_save_import(client, outcome, step)
            verified_count += 1
            saved_count += 1
    except Exception as error:
        _compensate(client, created, error)
        raise

    return PlanApplicationReport(
        bundle_revision=compiled.report.bundle_revision,
        imported_count=len(created),
        saved_count=saved_count,
        verified_count=verified_count,
    )


def _require_step_absent(
    client: NativePlanClient,
    step: NativeImportStep,
    *,
    changed: bool,
) -> None:
    for output in step.outputs:
        if _exists(client, output.package_path):
            message = (
                "native import output changed before import"
                if changed
                else "native import output already exists"
            )
            fail_protocol(message)
    if step.has_external_payload and _payload_exists(client, step):
        message = (
            "native import external payload changed before import"
            if changed
            else "native import external payload already exists"
        )
        fail_protocol(message)


def _invoke_import(
    client: NativePlanClient,
    step: NativeImportStep,
    source: Path,
    created: list[NativeImportStep],
) -> ToolCallOutcome:
    try:
        return client.call_tool(
            step.toolset_name,
            step.tool_name,
            step.arguments(str(source)),
        )
    except Exception as import_error:
        try:
            if _step_effect_exists(client, step):
                created.append(step)
        except Exception:  # noqa: BLE001
            import_error.add_note(
                "native import outcome and destination state are both unknown"
            )
        raise


def _step_effect_exists(
    client: NativePlanClient,
    step: NativeImportStep,
) -> bool:
    if any(_exists(client, output.package_path) for output in step.outputs):
        return True
    return step.has_external_payload and _payload_exists(client, step)


def _verify_and_save_import(
    client: NativePlanClient,
    outcome: ToolCallOutcome,
    step: NativeImportStep,
) -> None:
    _require_exact_import_result(outcome, step)
    for output in step.outputs:
        _verify_imported_output(client, output)
    if step.has_external_payload:
        _verify_media_payload(client, step)
    package_paths = tuple(output.package_path for output in step.outputs)
    if not _save(client, package_paths):
        fail_protocol("native import asset save returned false")
    for output in step.outputs:
        if _is_dirty(client, output.package_path):
            fail_protocol("native import asset remained dirty after save")


def _verify_imported_output(
    client: NativePlanClient,
    output: NativeAssetOutput,
) -> None:
    """Read back one declared output before saving the transaction."""
    if not _exists(client, output.package_path):
        fail_protocol("native import did not publish a declared output")
    actual_class = _asset_class(client, output.package_path)
    if actual_class != output.target_class:
        fail_protocol("native import produced an unexpected asset class")
    is_dirty = _is_dirty(client, output.package_path)
    if is_dirty != output.expected_dirty_after_import:
        fail_protocol("native import produced an unexpected dirty state")


def _verify_media_payload(
    client: NativePlanClient,
    step: NativeImportStep,
) -> None:
    if _payload_path(client, step) != step.external_payload_path:
        fail_protocol(
            "native media import published an unexpected payload path"
        )
    if not _payload_exists(client, step):
        fail_protocol("native media import omitted its external payload")


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
        fail_protocol(
            "native import plan contains duplicate operation identities"
        )


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


def _save(
    client: NativePlanClient,
    package_paths: tuple[str, ...],
) -> bool:
    outcome = client.call_tool(
        _ASSET_TOOLSET,
        f"{_ASSET_TOOLSET}.save_assets",
        {"asset_paths": list(package_paths)},
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
    except Exception:
        if not _exists(client, package_path):
            return
        raise
    if _exists(client, package_path):
        fail_protocol("compensating delete left the imported asset present")


def _compensate(
    client: NativePlanClient,
    created: list[NativeImportStep],
    primary_error: Exception,
) -> None:
    failures = 0
    for step in reversed(created):
        if step.has_external_payload:
            try:
                _delete_payload(client, step)
            except Exception:  # noqa: BLE001
                failures += 1
        for output in step.rollback_outputs:
            try:
                _delete(client, output.package_path)
            except Exception:  # noqa: BLE001
                failures += 1
    if failures:
        primary_error.add_note(
            f"native import compensation failed for {failures} created asset(s)"
        )


def _require_exact_import_result(
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
    if step.route_id in {
        "file-media-source-hap-v1",
        "sound-wave-wav-v1",
        "skeletal-mesh-fbx-v1",
        "static-mesh-fbx-v1",
    }:
        if any(not isinstance(item, str) or not item for item in value):
            fail_protocol("native SHAR import returned an invalid object path")
        if tuple(value) != step.expected_object_paths:
            fail_protocol(
                "native SHAR import output inventory does not match plan"
            )
        return
    package_paths: list[str] = []
    for item in value:
        if not isinstance(item, dict):
            fail_protocol("native import result contains a non-object asset")
        package_path = item.get("packagePath")
        if not isinstance(package_path, str) or not package_path:
            fail_protocol("native import result omitted its package path")
        package_paths.append(package_path)
    if tuple(package_paths) != tuple(
        output.package_path for output in step.outputs
    ):
        fail_protocol("native import output inventory does not match plan")


def _payload_arguments(step: NativeImportStep) -> JsonObject:
    if not step.has_external_payload:
        fail_protocol("native import has no external payload")
    return {"assetPath": step.destination}


def _payload_exists(client: NativePlanClient, step: NativeImportStep) -> bool:
    outcome = client.call_tool(
        _IMPORT_TOOLSET,
        f"{_IMPORT_TOOLSET}.FileMediaSourcePayloadExists",
        _payload_arguments(step),
    )
    return _return_boolean(outcome, context="media payload existence result")


def _payload_path(client: NativePlanClient, step: NativeImportStep) -> str:
    outcome = client.call_tool(
        _IMPORT_TOOLSET,
        f"{_IMPORT_TOOLSET}.GetFileMediaSourcePath",
        _payload_arguments(step),
    )
    result = _structured_result(outcome, context="media payload path result")
    value = result.get("returnValue")
    if not isinstance(value, str) or not value:
        fail_protocol("media payload path result is not non-empty text")
    return value


def _delete_payload(client: NativePlanClient, step: NativeImportStep) -> None:
    try:
        outcome = client.call_tool(
            _IMPORT_TOOLSET,
            f"{_IMPORT_TOOLSET}.DeleteFileMediaSourcePayload",
            _payload_arguments(step),
        )
        _ = _return_boolean(outcome, context="media payload delete result")
    except Exception:
        if not _payload_exists(client, step):
            return
        raise
    if _payload_exists(client, step):
        fail_protocol("compensating delete left the media payload present")


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
