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
from mcp.application.plan_application import apply_import_plan
from mcp.application.service import UnrealMcpTranslator
from mcp.application.skill_export import UnrealSkillExporter
from mcp.domain.errors import UnrealMcpError
from mcp.domain.plan_capabilities import audit_plan_capabilities
from mcp.domain.plan_capabilities import required_toolsets
from mcp.domain.plan_execution import compile_execution_plan

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
        if is_help_action(invocation.action):
            _write_stdout(usage_text())
            return _EXIT_SUCCESS
        _validate_action_operands(invocation)
        if invocation.action == "plan-preflight":
            return _run_plan_preflight(parse_plan_root(invocation.operands))
        if invocation.action == "plan-execution-preflight":
            return _run_plan_execution_preflight(
                parse_plan_root(invocation.operands)
            )
        if invocation.action == "plan-capabilities":
            return _run_plan_capabilities(
                invocation,
                parse_plan_root(invocation.operands),
            )
        if invocation.action == "plan-apply":
            return _run_plan_apply(
                invocation,
                parse_plan_root(invocation.operands),
            )
        return _run(invocation)
    except UsageError as error:
        _write_stderr(f"error: {error}\n\n{usage_text()}")
        return _EXIT_USAGE
    except (UnrealMcpError, OSError, UnicodeError) as error:
        _write_stderr(f"error: {error}\n")
        return _EXIT_FAILURE


def _validate_action_operands(invocation: CliInvocation) -> None:
    """Validate one command completely before opening an MCP session."""
    action = invocation.action
    operands = invocation.operands
    if action in {"doctor", "toolsets"}:
        require_operand_count(action, operands, expected=0)
        return
    if action in {
        "plan-apply",
        "plan-capabilities",
        "plan-execution-preflight",
        "plan-preflight",
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
        return
    _ = parse_catalog_format(operands)


def _run_plan_preflight(root: Path) -> int:
    report = FilesystemPlanBundleReader(root).read()
    _write_stdout(render_json(report.to_json()))
    return _EXIT_SUCCESS


def _run_plan_execution_preflight(root: Path) -> int:
    bundle = FilesystemPlanBundleReader(root).read_bundle()
    sources = FilesystemPlanSourceVerifier(Path("."), root).verify(bundle)
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
    sources = FilesystemPlanSourceVerifier(Path("."), root).verify(bundle)
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
    sources = FilesystemPlanSourceVerifier(Path("."), root).verify(bundle)
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
        except OSError, UnicodeError:
            return
    except OSError:
        return


if __name__ == "__main__":
    raise SystemExit(main())
