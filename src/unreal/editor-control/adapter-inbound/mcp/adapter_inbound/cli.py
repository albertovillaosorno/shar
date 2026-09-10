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
#   - Cli inbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Cli inbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Cli inbound adapter."""

from __future__ import annotations

from pathlib import Path
import sys
from typing import TYPE_CHECKING

from mcp.adapter_inbound.arguments import UsageError
from mcp.adapter_inbound.arguments import is_help_action
from mcp.adapter_inbound.arguments import parse_catalog_format
from mcp.adapter_inbound.arguments import parse_invocation
from mcp.adapter_inbound.arguments import parse_plan_root
from mcp.adapter_inbound.arguments import parse_raw_call
from mcp.adapter_inbound.arguments import parse_skill_output_path
from mcp.adapter_inbound.arguments import parse_tool_call
from mcp.adapter_inbound.arguments import parse_vehicle_material_options
from mcp.adapter_inbound.arguments import parse_vehicle_physics_options
from mcp.adapter_inbound.arguments import (
    parse_vehicle_physics_prerequisite_options,
)
from mcp.adapter_inbound.arguments import require_operand_count
from mcp.adapter_inbound.arguments import usage_text
from mcp.adapter_outbound.catalog_renderer import render_catalog_json
from mcp.adapter_outbound.catalog_renderer import render_catalog_markdown
from mcp.adapter_outbound.catalog_renderer import render_json
from mcp.adapter_outbound.filesystem_skill_store import FilesystemSkillStore
from mcp.adapter_outbound.plan_bundle_reader import FilesystemPlanBundleReader
from mcp.adapter_outbound.plan_source_verifier import (
    FilesystemPlanSourceVerifier,
)
from mcp.adapter_outbound.skill_markdown_renderer import MarkdownSkillRenderer
from mcp.adapter_outbound.streamable_http import StreamableHttpTransport
from mcp.adapter_outbound.unreal_mcp_version import (
    FilesystemUnrealMcpVersionProvider,
)
from mcp.adapter_outbound.vehicle_material_construction_reader import (
    read_bound_vehicle_material_document,
)
from mcp.adapter_outbound.vehicle_material_source_verifier import (
    verify_vehicle_material_texture_sources,
)
from mcp.adapter_outbound.vehicle_physics_construction_reader import (
    read_bound_vehicle_physics_document,
)
from mcp.adapter_outbound.world_material_construction_reader import (
    read_bound_world_material_document,
)
from mcp.adapter_outbound.world_material_source_verifier import (
    verify_world_material_texture_sources,
)
from mcp.application.plan_application import apply_import_plan
from mcp.application.plan_application import apply_import_steps
from mcp.application.service import UnrealMcpTranslator
from mcp.application.skill_export import UnrealSkillExporter
from mcp.application.vehicle_material_application import (
    apply_vehicle_material_construction,
)
from mcp.application.vehicle_physics_application import (
    apply_vehicle_physics_construction,
)
from mcp.application.world_material_application import (
    apply_world_material_construction,
)
from mcp.domain.errors import UnrealMcpError
from mcp.domain.plan_bundle import ValidatedPlanBundle
from mcp.domain.plan_capabilities import audit_import_capabilities
from mcp.domain.plan_capabilities import audit_plan_capabilities
from mcp.domain.plan_capabilities import required_import_toolsets
from mcp.domain.plan_capabilities import required_toolsets
from mcp.domain.plan_execution import compile_execution_plan
from mcp.domain.vehicle_material_capabilities import (
    audit_vehicle_material_capabilities,
)
from mcp.domain.vehicle_material_capabilities import (
    required_vehicle_material_toolsets,
)
from mcp.domain.vehicle_material_capabilities import (
    vehicle_material_construction_revision,
)
from mcp.domain.vehicle_material_construction import (
    CompiledVehicleMaterialConstruction,
)
from mcp.domain.vehicle_material_construction import (
    compile_vehicle_material_construction,
)
from mcp.domain.vehicle_material_selection import (
    CompiledVehicleMaterialSelection,
)
from mcp.domain.vehicle_material_selection import VehicleMaterialExecutable
from mcp.domain.vehicle_material_selection import (
    select_vehicle_material_package,
)
from mcp.domain.vehicle_physics_capabilities import (
    audit_vehicle_physics_capabilities,
)
from mcp.domain.vehicle_physics_capabilities import (
    required_vehicle_physics_toolsets,
)
from mcp.domain.vehicle_physics_construction import (
    CompiledVehiclePhysicsConstruction,
)
from mcp.domain.vehicle_physics_construction import (
    compile_vehicle_physics_construction,
)
from mcp.domain.vehicle_physics_construction import (
    vehicle_physics_construction_revision,
)
from mcp.domain.vehicle_physics_prerequisites import (
    CompiledVehiclePhysicsPrerequisiteSelection,
)
from mcp.domain.vehicle_physics_prerequisites import (
    CompiledVehiclePhysicsPrerequisites,
)
from mcp.domain.vehicle_physics_prerequisites import (
    VehiclePhysicsPrerequisiteExecutable,
)
from mcp.domain.vehicle_physics_prerequisites import (
    compile_vehicle_physics_prerequisites,
)
from mcp.domain.vehicle_physics_prerequisites import (
    select_vehicle_physics_prerequisite_package,
)
from mcp.domain.vehicle_physics_prerequisites import (
    vehicle_physics_prerequisite_revision,
)
from mcp.domain.vehicle_physics_selection import CompiledVehiclePhysicsSelection
from mcp.domain.vehicle_physics_selection import select_vehicle_physics_package
from mcp.domain.world_material_capabilities import (
    audit_world_material_capabilities,
)
from mcp.domain.world_material_capabilities import (
    required_world_material_toolsets,
)
from mcp.domain.world_material_capabilities import (
    world_material_construction_revision,
)
from mcp.domain.world_material_construction import (
    CompiledWorldMaterialConstruction,
)
from mcp.domain.world_material_construction import (
    compile_world_material_construction,
)

if TYPE_CHECKING:
    from collections.abc import Sequence

    from mcp.adapter_inbound.arguments import CliInvocation

_EXIT_SUCCESS = 0
_EXIT_FAILURE = 1
_EXIT_USAGE = 2
_PROJECT_DESCRIPTOR = (
    Path("src/unreal/project/composition/uproject") / "shar.uproject"
)


def main(argv: Sequence[str] | None = None) -> int:
    """Run the translator CLI and return a stable process exit code.

    Args:
        argv: Optional arguments excluding the executable name.

    Returns:
        Zero on success, one on runtime failure, or two on invalid usage.

    """
    raw_arguments = tuple(sys.argv[1:] if argv is None else argv)
    try:
        invocation = parse_invocation(raw_arguments)
        return _run_invocation(invocation)
    except UsageError as error:
        _write_stderr(f"error: {error}\n\n{usage_text()}")
        return _EXIT_USAGE
    except (UnrealMcpError, OSError, UnicodeError) as error:
        _write_stderr(f"error: {error}\n")
        return _EXIT_FAILURE


def _run_invocation(invocation: CliInvocation) -> int:
    """Dispatch one fully parsed invocation."""
    if is_help_action(invocation.action):
        _write_stdout(usage_text())
        return _EXIT_SUCCESS
    _validate_action_operands(invocation)
    if invocation.action.startswith(("vehicle-material-", "world-material-")):
        return _run_material_invocation(invocation)
    if invocation.action.startswith("vehicle-physics-prerequisites-"):
        return _run_vehicle_physics_prerequisite_invocation(invocation)
    if invocation.action.startswith("vehicle-physics-"):
        return _run_vehicle_physics_invocation(invocation)
    if invocation.action.startswith("plan-"):
        return _run_plan_invocation(invocation)
    return _run(invocation)


def _run_material_invocation(invocation: CliInvocation) -> int:
    if invocation.action.startswith("vehicle-material-"):
        return _run_vehicle_material_invocation(invocation)
    return _run_world_material_invocation(invocation)


def _run_vehicle_material_invocation(invocation: CliInvocation) -> int:
    options = parse_vehicle_material_options(invocation.operands)
    if invocation.action == "vehicle-material-preflight":
        return _run_vehicle_material_preflight(
            options.root, options.package_id
        )
    if invocation.action == "vehicle-material-capabilities":
        return _run_vehicle_material_capabilities(
            invocation, options.root, options.package_id
        )
    return _run_vehicle_material_apply(
        invocation, options.root, options.package_id
    )


def _run_vehicle_physics_prerequisite_invocation(
    invocation: CliInvocation,
) -> int:
    options = parse_vehicle_physics_prerequisite_options(invocation.operands)
    if invocation.action == "vehicle-physics-prerequisites-preflight":
        return _run_vehicle_physics_prerequisite_preflight(
            options.root, options.package_id
        )
    if invocation.action == "vehicle-physics-prerequisites-capabilities":
        return _run_vehicle_physics_prerequisite_capabilities(
            invocation, options.root, options.package_id
        )
    return _run_vehicle_physics_prerequisite_apply(
        invocation, options.root, options.package_id
    )


def _run_vehicle_physics_invocation(invocation: CliInvocation) -> int:
    options = parse_vehicle_physics_options(invocation.operands)
    if invocation.action == "vehicle-physics-preflight":
        return _run_vehicle_physics_preflight(options.root, options.package_id)
    if invocation.action == "vehicle-physics-capabilities":
        return _run_vehicle_physics_capabilities(
            invocation, options.root, options.package_id
        )
    return _run_vehicle_physics_apply(
        invocation, options.root, options.package_id
    )


def _run_world_material_invocation(invocation: CliInvocation) -> int:
    root = parse_plan_root(invocation.operands)
    if invocation.action == "world-material-preflight":
        return _run_world_material_preflight(root)
    if invocation.action == "world-material-capabilities":
        return _run_world_material_capabilities(invocation, root)
    return _run_world_material_apply(invocation, root)


def _run_plan_invocation(invocation: CliInvocation) -> int:
    root = parse_plan_root(invocation.operands)
    if invocation.action == "plan-preflight":
        return _run_plan_preflight(root)
    if invocation.action == "plan-execution-preflight":
        return _run_plan_execution_preflight(root)
    if invocation.action == "plan-capabilities":
        return _run_plan_capabilities(invocation, root)
    return _run_plan_apply(invocation, root)


def _validate_action_operands(invocation: CliInvocation) -> None:
    """Validate one command completely before opening an MCP session."""
    action = invocation.action
    operands = invocation.operands
    if action in {"doctor", "toolsets"}:
        require_operand_count(action, operands, expected=0)
        return
    if action in {
        "vehicle-material-apply",
        "vehicle-material-capabilities",
        "vehicle-material-preflight",
        "vehicle-physics-apply",
        "vehicle-physics-capabilities",
        "vehicle-physics-preflight",
        "vehicle-physics-prerequisites-apply",
        "vehicle-physics-prerequisites-capabilities",
        "vehicle-physics-prerequisites-preflight",
    }:
        _validate_vehicle_scoped_operands(action, operands)
        return
    if action in {
        "plan-apply",
        "plan-capabilities",
        "plan-execution-preflight",
        "plan-preflight",
        "world-material-apply",
        "world-material-capabilities",
        "world-material-preflight",
    }:
        _ = parse_plan_root(operands)
        return
    if action == "describe":
        require_operand_count(action, operands, expected=1)
        return
    if action == "call":
        _ = parse_tool_call(operands)
        return
    if action == "raw-call":
        _ = parse_raw_call(operands)
        return
    if action == "skills":
        _ = parse_skill_output_path(operands)
    else:
        _ = parse_catalog_format(operands)


def _validate_vehicle_scoped_operands(
    action: str,
    operands: tuple[str, ...],
) -> None:
    if action.startswith("vehicle-material-"):
        _ = parse_vehicle_material_options(operands)
    elif action.startswith("vehicle-physics-prerequisites-"):
        _ = parse_vehicle_physics_prerequisite_options(operands)
    else:
        _ = parse_vehicle_physics_options(operands)


def _vehicle_material_context(
    root: Path,
    package_id: str | None,
) -> tuple[
    ValidatedPlanBundle,
    CompiledVehicleMaterialConstruction,
    VehicleMaterialExecutable,
    dict[str, Path],
]:
    bundle = FilesystemPlanBundleReader(root).read_bundle()
    document = read_bound_vehicle_material_document(root.parent, bundle)
    compiled = compile_vehicle_material_construction(document)
    executable: VehicleMaterialExecutable = compiled
    if package_id is not None:
        executable = select_vehicle_material_package(compiled, package_id)
    sources = verify_vehicle_material_texture_sources(
        root.parent.parent,
        executable,
    )
    return bundle, compiled, executable, sources


def _vehicle_material_evidence(
    bundle: ValidatedPlanBundle,
    compiled: CompiledVehicleMaterialConstruction,
    executable: VehicleMaterialExecutable,
    sources: dict[str, Path],
) -> dict[str, object]:
    payload: dict[str, object] = {
        "bundle": bundle.report.to_json(),
        "construction": compiled.report.to_json(),
        "constructionRevision": vehicle_material_construction_revision(
            compiled
        ),
        "verifiedTextureSourceCount": len(sources),
    }
    if isinstance(executable, CompiledVehicleMaterialSelection):
        payload["selection"] = executable.report.to_json()
        payload["selectionRevision"] = vehicle_material_construction_revision(
            executable
        )
    return payload


def _run_vehicle_material_preflight(
    root: Path,
    package_id: str | None,
) -> int:
    bundle, compiled, executable, sources = _vehicle_material_context(
        root, package_id
    )
    payload = _vehicle_material_evidence(
        bundle, compiled, executable, sources
    )
    _write_stdout(render_json(payload))
    return _EXIT_SUCCESS


def _run_vehicle_material_capabilities(
    invocation: CliInvocation,
    root: Path,
    package_id: str | None,
) -> int:
    bundle, compiled, executable, sources = _vehicle_material_context(
        root, package_id
    )
    transport = StreamableHttpTransport(
        invocation.endpoint,
        timeout_seconds=invocation.timeout_seconds,
    )
    with UnrealMcpTranslator(transport) as translator:
        definitions = translator.describe_available_toolsets(
            required_vehicle_material_toolsets(executable)
        )
    capabilities = audit_vehicle_material_capabilities(executable, definitions)
    payload = _vehicle_material_evidence(
        bundle, compiled, executable, sources
    )
    payload["capabilities"] = capabilities.to_json()
    _write_stdout(render_json(payload))
    return _EXIT_SUCCESS if capabilities.complete else _EXIT_FAILURE


def _run_vehicle_material_apply(
    invocation: CliInvocation,
    root: Path,
    package_id: str | None,
) -> int:
    bundle, compiled, executable, sources = _vehicle_material_context(
        root, package_id
    )
    transport = StreamableHttpTransport(
        invocation.endpoint,
        timeout_seconds=invocation.timeout_seconds,
    )
    with UnrealMcpTranslator(transport) as translator:
        definitions = translator.describe_available_toolsets(
            required_vehicle_material_toolsets(executable)
        )
        capabilities = audit_vehicle_material_capabilities(
            executable, definitions
        )
        payload = _vehicle_material_evidence(
            bundle, compiled, executable, sources
        )
        payload["capabilities"] = capabilities.to_json()
        if not capabilities.complete:
            _write_stdout(render_json(payload))
            return _EXIT_FAILURE
        application = apply_vehicle_material_construction(
            translator,
            executable,
            capabilities,
            sources,
        )
    payload["application"] = application.to_json()
    _write_stdout(render_json(payload))
    return _EXIT_SUCCESS


def _vehicle_physics_prerequisite_context(
    root: Path,
    package_id: str | None,
) -> tuple[
    ValidatedPlanBundle,
    CompiledVehiclePhysicsPrerequisites,
    VehiclePhysicsPrerequisiteExecutable,
    dict[str, Path],
    dict[str, object],
]:
    bundle = FilesystemPlanBundleReader(root).read_bundle()
    execution = compile_execution_plan(bundle)
    document = read_bound_vehicle_physics_document(root.parent, bundle)
    construction = compile_vehicle_physics_construction(document, execution)
    prerequisites = compile_vehicle_physics_prerequisites(
        construction,
        execution,
    )
    executable: VehiclePhysicsPrerequisiteExecutable = prerequisites
    if package_id is not None:
        executable = select_vehicle_physics_prerequisite_package(
            construction,
            prerequisites,
            package_id,
        )
    operation_ids = tuple(step.operation_id for step in executable.imports)
    verified = FilesystemPlanSourceVerifier(Path(), root).verify_operation_ids(
        bundle,
        operation_ids,
    )
    return (
        bundle,
        prerequisites,
        executable,
        verified.by_operation,
        verified.report.to_json(),
    )


def _vehicle_physics_prerequisite_evidence(
    bundle: ValidatedPlanBundle,
    prerequisites: CompiledVehiclePhysicsPrerequisites,
    executable: VehiclePhysicsPrerequisiteExecutable,
    source_report: dict[str, object],
) -> dict[str, object]:
    payload: dict[str, object] = {
        "bundle": bundle.report.to_json(),
        "prerequisites": prerequisites.report.to_json(),
        "prerequisiteRevision": vehicle_physics_prerequisite_revision(
            prerequisites
        ),
        "sources": source_report,
    }
    if isinstance(executable, CompiledVehiclePhysicsPrerequisiteSelection):
        payload["selection"] = executable.report.to_json()
        payload["selectionRevision"] = vehicle_physics_prerequisite_revision(
            executable
        )
    return payload


def _run_vehicle_physics_prerequisite_preflight(
    root: Path,
    package_id: str | None,
) -> int:
    bundle, prerequisites, executable, _, source_report = (
        _vehicle_physics_prerequisite_context(root, package_id)
    )
    payload = _vehicle_physics_prerequisite_evidence(
        bundle,
        prerequisites,
        executable,
        source_report,
    )
    _write_stdout(render_json(payload))
    return _EXIT_SUCCESS


def _run_vehicle_physics_prerequisite_capabilities(
    invocation: CliInvocation,
    root: Path,
    package_id: str | None,
) -> int:
    bundle, prerequisites, executable, _, source_report = (
        _vehicle_physics_prerequisite_context(root, package_id)
    )
    revision = vehicle_physics_prerequisite_revision(executable)
    transport = StreamableHttpTransport(
        invocation.endpoint,
        timeout_seconds=invocation.timeout_seconds,
    )
    with UnrealMcpTranslator(transport) as translator:
        definitions = translator.describe_available_toolsets(
            required_import_toolsets(executable.imports)
        )
    capabilities = audit_import_capabilities(
        revision,
        executable.imports,
        definitions,
    )
    payload = _vehicle_physics_prerequisite_evidence(
        bundle,
        prerequisites,
        executable,
        source_report,
    )
    payload["capabilities"] = capabilities.to_json()
    _write_stdout(render_json(payload))
    return _EXIT_SUCCESS if capabilities.complete else _EXIT_FAILURE


def _run_vehicle_physics_prerequisite_apply(
    invocation: CliInvocation,
    root: Path,
    package_id: str | None,
) -> int:
    bundle, prerequisites, executable, sources, source_report = (
        _vehicle_physics_prerequisite_context(root, package_id)
    )
    revision = vehicle_physics_prerequisite_revision(executable)
    transport = StreamableHttpTransport(
        invocation.endpoint,
        timeout_seconds=invocation.timeout_seconds,
    )
    with UnrealMcpTranslator(transport) as translator:
        definitions = translator.describe_available_toolsets(
            required_import_toolsets(executable.imports)
        )
        capabilities = audit_import_capabilities(
            revision,
            executable.imports,
            definitions,
        )
        payload = _vehicle_physics_prerequisite_evidence(
            bundle,
            prerequisites,
            executable,
            source_report,
        )
        payload["capabilities"] = capabilities.to_json()
        if not capabilities.complete:
            _write_stdout(render_json(payload))
            return _EXIT_FAILURE
        application = apply_import_steps(
            translator,
            revision,
            executable.imports,
            capabilities,
            sources,
        )
    payload["application"] = application.to_json()
    _write_stdout(render_json(payload))
    return _EXIT_SUCCESS


def _vehicle_physics_context(
    root: Path,
    package_id: str | None,
) -> tuple[
    ValidatedPlanBundle,
    CompiledVehiclePhysicsConstruction,
    CompiledVehiclePhysicsConstruction,
    CompiledVehiclePhysicsSelection | None,
]:
    bundle = FilesystemPlanBundleReader(root).read_bundle()
    execution = compile_execution_plan(bundle)
    document = read_bound_vehicle_physics_document(root.parent, bundle)
    compiled = compile_vehicle_physics_construction(document, execution)
    executable = compiled
    selection: CompiledVehiclePhysicsSelection | None = None
    if package_id is not None:
        selection = select_vehicle_physics_package(compiled, package_id)
        executable = selection.construction
    return bundle, compiled, executable, selection


def _vehicle_physics_evidence(
    bundle: ValidatedPlanBundle,
    compiled: CompiledVehiclePhysicsConstruction,
    executable: CompiledVehiclePhysicsConstruction,
    selection: CompiledVehiclePhysicsSelection | None,
) -> dict[str, object]:
    payload: dict[str, object] = {
        "bundle": bundle.report.to_json(),
        "construction": compiled.report.to_json(),
        "constructionRevision": vehicle_physics_construction_revision(compiled),
    }
    if selection is not None:
        payload["selection"] = selection.report.to_json()
        payload["selectionRevision"] = vehicle_physics_construction_revision(
            executable
        )
    return payload


def _run_vehicle_physics_preflight(
    root: Path,
    package_id: str | None,
) -> int:
    bundle, compiled, executable, selection = _vehicle_physics_context(
        root, package_id
    )
    _write_stdout(render_json(_vehicle_physics_evidence(
        bundle,
        compiled,
        executable,
        selection,
    )))
    return _EXIT_SUCCESS


def _run_vehicle_physics_capabilities(
    invocation: CliInvocation,
    root: Path,
    package_id: str | None,
) -> int:
    bundle, compiled, executable, selection = _vehicle_physics_context(
        root, package_id
    )
    transport = StreamableHttpTransport(
        invocation.endpoint,
        timeout_seconds=invocation.timeout_seconds,
    )
    with UnrealMcpTranslator(transport) as translator:
        definitions = translator.describe_available_toolsets(
            required_vehicle_physics_toolsets(executable)
        )
    capabilities = audit_vehicle_physics_capabilities(executable, definitions)
    payload = _vehicle_physics_evidence(
        bundle,
        compiled,
        executable,
        selection,
    )
    payload["capabilities"] = capabilities.to_json()
    _write_stdout(render_json(payload))
    return _EXIT_SUCCESS if capabilities.complete else _EXIT_FAILURE


def _run_vehicle_physics_apply(
    invocation: CliInvocation,
    root: Path,
    package_id: str | None,
) -> int:
    bundle, compiled, executable, selection = _vehicle_physics_context(
        root, package_id
    )
    transport = StreamableHttpTransport(
        invocation.endpoint,
        timeout_seconds=invocation.timeout_seconds,
    )
    with UnrealMcpTranslator(transport) as translator:
        definitions = translator.describe_available_toolsets(
            required_vehicle_physics_toolsets(executable)
        )
        capabilities = audit_vehicle_physics_capabilities(
            executable, definitions
        )
        payload = _vehicle_physics_evidence(
            bundle,
            compiled,
            executable,
            selection,
        )
        payload["capabilities"] = capabilities.to_json()
        if not capabilities.complete:
            _write_stdout(render_json(payload))
            return _EXIT_FAILURE
        application = apply_vehicle_physics_construction(
            translator,
            executable,
            capabilities,
        )
    payload["application"] = application.to_json()
    _write_stdout(render_json(payload))
    return _EXIT_SUCCESS


def _world_material_context(
    root: Path,
) -> tuple[
    ValidatedPlanBundle,
    CompiledWorldMaterialConstruction,
    dict[str, Path],
]:
    bundle = FilesystemPlanBundleReader(root).read_bundle()
    document = read_bound_world_material_document(root.parent, bundle)
    compiled = compile_world_material_construction(document)
    sources = verify_world_material_texture_sources(
        root.parent.parent,
        compiled,
    )
    return bundle, compiled, sources


def _world_material_evidence(
    bundle: ValidatedPlanBundle,
    compiled: CompiledWorldMaterialConstruction,
    sources: dict[str, Path],
) -> dict[str, object]:
    return {
        "bundle": bundle.report.to_json(),
        "construction": compiled.report.to_json(),
        "constructionRevision": world_material_construction_revision(compiled),
        "verifiedTextureSourceCount": len(sources),
    }


def _run_world_material_preflight(root: Path) -> int:
    bundle, compiled, sources = _world_material_context(root)
    evidence = _world_material_evidence(bundle, compiled, sources)
    _write_stdout(render_json(evidence))
    return _EXIT_SUCCESS


def _run_world_material_capabilities(
    invocation: CliInvocation,
    root: Path,
) -> int:
    bundle, compiled, sources = _world_material_context(root)
    transport = StreamableHttpTransport(
        invocation.endpoint,
        timeout_seconds=invocation.timeout_seconds,
    )
    with UnrealMcpTranslator(transport) as translator:
        definitions = translator.describe_available_toolsets(
            required_world_material_toolsets(compiled)
        )
    capabilities = audit_world_material_capabilities(compiled, definitions)
    payload = _world_material_evidence(bundle, compiled, sources)
    payload["capabilities"] = capabilities.to_json()
    _write_stdout(render_json(payload))
    return _EXIT_SUCCESS if capabilities.complete else _EXIT_FAILURE


def _run_world_material_apply(
    invocation: CliInvocation,
    root: Path,
) -> int:
    bundle, compiled, sources = _world_material_context(root)
    transport = StreamableHttpTransport(
        invocation.endpoint,
        timeout_seconds=invocation.timeout_seconds,
    )
    with UnrealMcpTranslator(transport) as translator:
        definitions = translator.describe_available_toolsets(
            required_world_material_toolsets(compiled)
        )
        capabilities = audit_world_material_capabilities(compiled, definitions)
        payload = _world_material_evidence(bundle, compiled, sources)
        payload["capabilities"] = capabilities.to_json()
        if not capabilities.complete:
            _write_stdout(render_json(payload))
            return _EXIT_FAILURE
        application = apply_world_material_construction(
            translator,
            compiled,
            capabilities,
            sources,
        )
    payload["application"] = application.to_json()
    _write_stdout(render_json(payload))
    return _EXIT_SUCCESS


def _run_plan_preflight(root: Path) -> int:
    report = FilesystemPlanBundleReader(root).read()
    _write_stdout(render_json(report.to_json()))
    return _EXIT_SUCCESS


def _run_plan_execution_preflight(root: Path) -> int:
    bundle = FilesystemPlanBundleReader(root).read_bundle()
    sources = FilesystemPlanSourceVerifier(Path(), root).verify(bundle)
    execution = compile_execution_plan(bundle)
    _write_stdout(
        render_json(
            {
                "bundle": bundle.report.to_json(),
                "execution": execution.report.to_json(),
                "sources": sources.report.to_json(),
            }
        )
    )
    return _EXIT_SUCCESS if execution.report.complete else _EXIT_FAILURE


def _run_plan_capabilities(
    invocation: CliInvocation,
    root: Path,
) -> int:
    bundle = FilesystemPlanBundleReader(root).read_bundle()
    sources = FilesystemPlanSourceVerifier(Path(), root).verify(bundle)
    execution = compile_execution_plan(bundle)
    transport = StreamableHttpTransport(
        invocation.endpoint,
        timeout_seconds=invocation.timeout_seconds,
    )
    with UnrealMcpTranslator(transport) as translator:
        definitions = translator.describe_available_toolsets(
            required_toolsets(execution)
        )
    capabilities = audit_plan_capabilities(execution, definitions)
    _write_stdout(
        render_json(
            {
                "bundle": bundle.report.to_json(),
                "capabilities": capabilities.to_json(),
                "execution": execution.report.to_json(),
                "sources": sources.report.to_json(),
            }
        )
    )
    return _EXIT_SUCCESS if capabilities.complete else _EXIT_FAILURE


def _run_plan_apply(
    invocation: CliInvocation,
    root: Path,
) -> int:
    bundle = FilesystemPlanBundleReader(root).read_bundle()
    sources = FilesystemPlanSourceVerifier(Path(), root).verify(bundle)
    execution = compile_execution_plan(bundle)
    if not execution.report.complete:
        _write_stdout(
            render_json(
                {
                    "bundle": bundle.report.to_json(),
                    "execution": execution.report.to_json(),
                    "sources": sources.report.to_json(),
                }
            )
        )
        return _EXIT_FAILURE

    transport = StreamableHttpTransport(
        invocation.endpoint,
        timeout_seconds=invocation.timeout_seconds,
    )
    with UnrealMcpTranslator(transport) as translator:
        definitions = translator.describe_available_toolsets(
            required_toolsets(execution)
        )
        capabilities = audit_plan_capabilities(execution, definitions)
        if not capabilities.complete:
            _write_stdout(
                render_json(
                    {
                        "bundle": bundle.report.to_json(),
                        "capabilities": capabilities.to_json(),
                        "execution": execution.report.to_json(),
                        "sources": sources.report.to_json(),
                    }
                )
            )
            return _EXIT_FAILURE
        application = apply_import_plan(
            translator,
            execution,
            capabilities,
            sources.by_operation,
        )
    _write_stdout(
        render_json(
            {
                "application": application.to_json(),
                "bundle": bundle.report.to_json(),
                "capabilities": capabilities.to_json(),
                "execution": execution.report.to_json(),
                "sources": sources.report.to_json(),
            }
        )
    )
    return _EXIT_SUCCESS


def _run(invocation: CliInvocation) -> int:
    skill_output_path = (
        parse_skill_output_path(invocation.operands)
        if invocation.action == "skills"
        else None
    )
    transport = StreamableHttpTransport(
        invocation.endpoint,
        timeout_seconds=invocation.timeout_seconds,
    )
    with UnrealMcpTranslator(transport) as translator:
        if skill_output_path is not None:
            return _run_skills(translator, skill_output_path)
        return _run_connected(
            translator,
            invocation.action,
            invocation.operands,
        )


def _run_connected(
    translator: UnrealMcpTranslator,
    action: str,
    operands: tuple[str, ...],
) -> int:
    if action == "doctor":
        require_operand_count(action, operands, expected=0)
        report = translator.doctor()
        _write_stdout(
            render_json(
                {
                    "missingMetaTools": list(report.missing_meta_tools),
                    "protocolVersion": report.protocol_version,
                    "ready": report.ready,
                    "serverName": report.server_name,
                    "serverVersion": report.server_version,
                    "toolsetCount": report.toolset_count,
                    "topLevelTools": list(report.top_level_tools),
                }
            )
        )
        return _EXIT_SUCCESS if report.ready else _EXIT_FAILURE
    if action == "toolsets":
        require_operand_count(action, operands, expected=0)
        toolsets = translator.list_toolsets()
        _write_stdout(
            render_json(
                {
                    "toolsets": [
                        {
                            "description": item.description,
                            "name": item.name,
                        }
                        for item in toolsets
                    ]
                }
            )
        )
        return _EXIT_SUCCESS
    if action == "describe":
        require_operand_count(action, operands, expected=1)
        definition = translator.describe_toolset(operands[0])
        _write_stdout(render_json(definition.raw_schema))
        return _EXIT_SUCCESS
    if action == "call":
        toolset_name, tool_name, arguments = parse_tool_call(operands)
        outcome = translator.call_tool(
            toolset_name,
            tool_name,
            arguments,
        )
        _write_stdout(render_json(outcome.raw))
        return _EXIT_SUCCESS
    if action == "raw-call":
        tool_name, arguments = parse_raw_call(operands)
        outcome = translator.raw_call(tool_name, arguments)
        _write_stdout(render_json(outcome.raw))
        return _EXIT_SUCCESS
    return _run_catalog(translator, operands)


def _run_skills(
    translator: UnrealMcpTranslator,
    output_path: Path,
) -> int:
    unreal_mcp_version = FilesystemUnrealMcpVersionProvider(
        _PROJECT_DESCRIPTOR
    ).read_version()
    report = UnrealSkillExporter(
        translator,
        MarkdownSkillRenderer(unreal_mcp_version),
        FilesystemSkillStore(output_path),
    ).export()
    _write_stdout(
        render_json(
            {
                "categories": report.category_count,
                "documents": report.document_count,
                "interfaceDigest": report.interface_digest,
                "output": report.output_path,
                "toolsets": report.toolset_count,
                "tools": report.tool_count,
                "unrealMcpVersion": unreal_mcp_version,
            }
        )
    )
    return _EXIT_SUCCESS


def _run_catalog(
    translator: UnrealMcpTranslator,
    operands: tuple[str, ...],
) -> int:
    output_format = parse_catalog_format(operands)
    toolsets = translator.discover_catalog()
    rendered = (
        render_catalog_markdown(toolsets)
        if output_format == "markdown"
        else render_catalog_json(toolsets)
    )
    _write_stdout(rendered)
    return _EXIT_SUCCESS


def _write_stdout(value: str) -> None:
    _ = sys.stdout.write(value)


def _write_stderr(value: str) -> None:
    try:
        _ = sys.stderr.write(value)
    except UnicodeError:
        escaped = value.encode("ascii", errors="backslashreplace").decode(
            "ascii"
        )
        try:
            _ = sys.stderr.write(escaped)
        except (OSError, UnicodeError):
            return
    except OSError:
        return


if __name__ == "__main__":
    raise SystemExit(main())
