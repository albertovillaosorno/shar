# File:
#   - cli.py
# Path:
#   - src/mcp/src/adapters/driving/cli.py
#
# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE
# Path-Rule:
#   - All paths in this header are repository-root relative.
#
# Boundary-Contract:
# - Owns:
#   - Terminal composition, execution dispatch, and result presentation.
# - Must-Not:
#   - Implement MCP wire rules or native Unreal tool behavior.
# - Allows:
#   - Compose application services with driven adapters.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Exposes the native Unreal MCP translator as a CLI.
# - Description:
#   - Maps validated terminal actions to application use cases.
# - Usage:
#   - Installed as the shar-unreal-mcp console command.
# - Defaults:
#   - Writes machine-readable JSON and stable exit codes.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: terminal driving adapter
#   - reason: dispatch, output, and exit codes form one operator interface
#   - split: extract command handlers if another presentation surface is added
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Driving terminal adapter for the native Unreal MCP translator."""

from __future__ import annotations

import sys
from pathlib import Path
from typing import TYPE_CHECKING

from mcp.src.adapters.driven.catalog_renderer import (
    render_catalog_json,
    render_catalog_markdown,
    render_json,
)
from mcp.src.adapters.driven.filesystem_skill_store import (
    FilesystemSkillStore,
)
from mcp.src.adapters.driven.skill_markdown_renderer import (
    MarkdownSkillRenderer,
)
from mcp.src.adapters.driven.streamable_http import (
    StreamableHttpTransport,
)
from mcp.src.adapters.driven.unreal_mcp_version import (
    FilesystemUnrealMcpVersionProvider,
)
from mcp.src.adapters.driving.arguments import (
    UsageError,
    is_help_action,
    parse_catalog_format,
    parse_invocation,
    parse_raw_call,
    parse_skill_output_path,
    parse_tool_call,
    require_operand_count,
    usage_text,
)
from mcp.src.application.service import UnrealMcpTranslator
from mcp.src.application.skill_export import UnrealSkillExporter
from mcp.src.domain.errors import UnrealMcpError

if TYPE_CHECKING:
    from collections.abc import Sequence

    from mcp.src.adapters.driving.arguments import CliInvocation

_EXIT_SUCCESS = 0
_EXIT_FAILURE = 1
_EXIT_USAGE = 2
_PROJECT_DESCRIPTOR = Path("src/uproject/shar.uproject")


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
        return _run(invocation)
    except UsageError as error:
        _write_stderr(f"error: {error}\n\n{usage_text()}")
        return _EXIT_USAGE
    except (UnrealMcpError, OSError) as error:
        _write_stderr(f"error: {error}\n")
        return _EXIT_FAILURE


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
    _ = sys.stderr.write(value)


if __name__ == "__main__":
    raise SystemExit(main())
