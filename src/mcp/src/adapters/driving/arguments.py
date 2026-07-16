# File:
#   - arguments.py
# Path:
#   - src/mcp/src/adapters/driving/arguments.py
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
#   - Terminal grammar, option validation, and JSON argument parsing.
# - Must-Not:
#   - Open MCP sessions, perform HTTP, or invoke native tools.
# - Allows:
#   - Pure parsing into validated CLI values.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Defines the terminal translator command grammar.
# - Description:
#   - Rejects invalid input before any editor connection.
# - Usage:
#   - Called by the driving CLI before adapter composition.
# - Defaults:
#   - Uses the loopback endpoint and a thirty-second timeout.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: terminal command grammar
#   - reason: global options and action operands form one deterministic grammar
#   - split: split command parsers when another command family is introduced
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Terminal argument grammar for the native Unreal MCP translator."""

from __future__ import annotations

import json
import math
import ntpath
from pathlib import Path, PurePosixPath, PureWindowsPath
from typing import NamedTuple, Never, cast

from mcp.src.domain.endpoint import McpEndpoint
from mcp.src.domain.errors import ProtocolError, UnrealMcpError
from mcp.src.domain.json_types import (
    DuplicateJsonKeyError,
    JsonObject,
    reject_duplicate_json_object,
    require_json_object,
)

_DEFAULT_TIMEOUT_SECONDS = 30.0
_TWO_OPTION_PARTS = 2
_KNOWN_ACTIONS = frozenset(
    {"call", "catalog", "describe", "doctor", "raw-call", "skills", "toolsets"}
)
_HELP_ACTIONS = frozenset({"--help", "-h", "help"})
_USAGE = """Usage:
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS] doctor
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS] toolsets
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS] describe TOOLSET
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    call TOOLSET TOOL [--arguments JSON]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    raw-call TOOL [--arguments JSON]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    catalog [--format json|markdown]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    skills [--output RELATIVE_PATH]

The endpoint must be loopback HTTP. The default is
http://127.0.0.1:8000/mcp.
"""


class UsageError(UnrealMcpError):
    """Raised when terminal arguments do not match the public grammar."""


class CliInvocation(NamedTuple):
    """Validated global options and one requested action."""

    endpoint: McpEndpoint
    timeout_seconds: float
    action: str
    operands: tuple[str, ...]


def usage_text() -> str:
    """Return the complete public command grammar.

    Returns:
        Human-readable CLI usage text ending in one newline.
    """
    return _USAGE


def parse_invocation(arguments: tuple[str, ...]) -> CliInvocation:
    """Parse and validate global CLI options and action identity.

    Args:
        arguments: Terminal arguments excluding the executable name.

    Returns:
        One validated invocation that is safe to execute.

    """
    endpoint = McpEndpoint.default()
    timeout_seconds = _DEFAULT_TIMEOUT_SECONDS
    index = 0
    while index < len(arguments):
        argument = arguments[index]
        if argument == "--endpoint":
            endpoint_value, index = _take_option_value(
                arguments,
                index,
                "--endpoint",
            )
            endpoint = McpEndpoint.parse(endpoint_value)
            continue
        if argument == "--timeout":
            timeout_value, index = _take_option_value(
                arguments,
                index,
                "--timeout",
            )
            timeout_seconds = _parse_timeout(timeout_value)
            continue
        break
    action_and_operands = arguments[index:]
    if not action_and_operands:
        _fail_usage("missing command")
    action = action_and_operands[0]
    operands = action_and_operands[1:]
    if action not in _KNOWN_ACTIONS and action not in _HELP_ACTIONS:
        escaped_action = action.encode("unicode_escape").decode("ascii")
        _fail_usage(f"unknown command: {escaped_action}")
    return CliInvocation(
        endpoint=endpoint,
        timeout_seconds=timeout_seconds,
        action=action,
        operands=operands,
    )


def is_help_action(action: str) -> bool:
    """Return whether an action requests local usage help.

    Returns:
        `True` for every supported help spelling.
    """
    return action in _HELP_ACTIONS


def parse_tool_call(
    operands: tuple[str, ...],
) -> tuple[str, str, JsonObject]:
    """Parse a Toolset Registry call action.

    Returns:
        Toolset name, tool name, and strict JSON arguments.

    """
    if len(operands) < _TWO_OPTION_PARTS:
        _fail_usage("call requires TOOLSET and TOOL")
    arguments = _parse_arguments_option(operands[_TWO_OPTION_PARTS:])
    return operands[0], operands[1], arguments


def parse_raw_call(operands: tuple[str, ...]) -> tuple[str, JsonObject]:
    """Parse a top-level native MCP call action.

    Returns:
        Top-level tool name and strict JSON arguments.

    """
    if not operands:
        _fail_usage("raw-call requires TOOL")
    arguments = _parse_arguments_option(operands[1:])
    return operands[0], arguments


def parse_catalog_format(operands: tuple[str, ...]) -> str:
    """Parse the requested catalog output format.

    Returns:
        Either `json` or `markdown`.

    """
    if not operands:
        return "json"
    if len(operands) != _TWO_OPTION_PARTS or operands[0] != "--format":
        _fail_usage("catalog accepts only --format json|markdown")
    output_format = operands[1]
    if output_format not in {"json", "markdown"}:
        _fail_usage("catalog format must be json or markdown")
    return output_format


def parse_skill_output_path(operands: tuple[str, ...]) -> Path:
    """Parse the repository-relative Unreal skill output directory.

    Returns:
        A safe child path for generated Unreal skills.
    """
    if not operands:
        return Path("skills/unreal")
    if len(operands) != _TWO_OPTION_PARTS or operands[0] != "--output":
        _fail_usage("skills accepts only --output RELATIVE_PATH")
    raw_path = operands[1]
    output_path = Path(raw_path)
    posix_path = PurePosixPath(raw_path)
    windows_path = PureWindowsPath(raw_path)
    if (
        posix_path.anchor
        or windows_path.anchor
        or ".." in posix_path.parts
        or ".." in windows_path.parts
    ):
        _fail_usage("skills output must be a repository-relative child path")
    if any(ntpath.isreserved(segment) for segment in windows_path.parts):
        _fail_usage("skills output must use a portable path")
    if output_path == Path() or not output_path.parts:
        _fail_usage("skills output must not be the repository root")
    return output_path


def require_operand_count(
    action: str,
    operands: tuple[str, ...],
    *,
    expected: int,
) -> None:
    """Require one exact operand count for a fixed-arity action."""
    if len(operands) != expected:
        _fail_usage(
            f"{action} expects {expected} operand(s), got {len(operands)}"
        )


def _parse_arguments_option(operands: tuple[str, ...]) -> JsonObject:
    if not operands:
        return {}
    if len(operands) != _TWO_OPTION_PARTS or operands[0] != "--arguments":
        _fail_usage("expected --arguments followed by one JSON object")
    try:
        parsed = cast(
            "object",
            json.loads(
                operands[1],
                object_pairs_hook=reject_duplicate_json_object,
            ),
        )
    except DuplicateJsonKeyError as error:
        _fail_usage(str(error), cause=error)
    except ValueError as error:
        _fail_usage("--arguments is not valid JSON", cause=error)
    if not isinstance(parsed, dict):
        _fail_usage("--arguments must contain one JSON object")
    raw_object = cast("dict[object, object]", parsed)
    try:
        return require_json_object(raw_object, context="--arguments")
    except ProtocolError as error:
        _fail_usage(str(error), cause=error)


def _take_option_value(
    arguments: tuple[str, ...],
    index: int,
    option: str,
) -> tuple[str, int]:
    value_index = index + 1
    if value_index >= len(arguments):
        _fail_usage(f"{option} requires a value")
    return arguments[value_index], value_index + 1


def _parse_timeout(value: str) -> float:
    try:
        timeout = float(value)
    except ValueError as error:
        _fail_usage("--timeout must be a number", cause=error)
    if not math.isfinite(timeout) or timeout <= 0:
        _fail_usage("--timeout must be finite and positive")
    return timeout


def _fail_usage(
    message: str,
    *,
    cause: BaseException | None = None,
) -> Never:
    failure = UsageError(message)
    if cause is None:
        raise failure
    raise failure from cause
