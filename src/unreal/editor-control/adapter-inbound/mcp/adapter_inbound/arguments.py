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
#   - Arguments inbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Arguments inbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Arguments inbound adapter."""

from __future__ import annotations

import json
import math
import ntpath
from pathlib import Path
from pathlib import PurePosixPath
from pathlib import PureWindowsPath
import re
from typing import NamedTuple
from typing import Never
from typing import cast

from mcp.domain.endpoint import McpEndpoint
from mcp.domain.errors import ProtocolError
from mcp.domain.errors import UnrealMcpError
from mcp.domain.json_types import DuplicateJsonKeyError
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import reject_duplicate_json_object
from mcp.domain.json_types import require_json_object

_DEFAULT_TIMEOUT_SECONDS = 30.0
_TWO_OPTION_PARTS = 2
_PACKAGE_ID = re.compile(r"^[a-z0-9](?:[a-z0-9]|-(?=[a-z0-9]))*$")
_KNOWN_ACTIONS = frozenset(
    {
        "call",
        "catalog",
        "describe",
        "doctor",
        "plan-apply",
        "plan-capabilities",
        "plan-execution-preflight",
        "plan-preflight",
        "raw-call",
        "skills",
        "toolsets",
        "vehicle-material-apply",
        "vehicle-material-capabilities",
        "vehicle-material-preflight",
        "vehicle-material-slots-apply",
        "vehicle-material-slots-capabilities",
        "vehicle-material-slots-preflight",
        "vehicle-physics-apply",
        "vehicle-physics-capabilities",
        "vehicle-physics-preflight",
        "vehicle-physics-prerequisites-apply",
        "vehicle-physics-prerequisites-capabilities",
        "vehicle-physics-prerequisites-preflight",
        "world-material-apply",
        "world-material-capabilities",
        "world-material-preflight",
    }
)
_HELP_ACTIONS = frozenset({"--help", "-h", "help"})
_USAGE = """Usage:
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS] doctor
  shar-unreal-mcp plan-preflight [--root RELATIVE_PATH]
  shar-unreal-mcp plan-execution-preflight [--root RELATIVE_PATH]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    plan-capabilities [--root RELATIVE_PATH]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    plan-apply [--root RELATIVE_PATH]
  shar-unreal-mcp vehicle-material-preflight [--root RELATIVE_PATH]
    [--package-id PACKAGE_ID]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    vehicle-material-capabilities [--root RELATIVE_PATH]
    [--package-id PACKAGE_ID]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    vehicle-material-apply [--root RELATIVE_PATH] [--package-id PACKAGE_ID]
  shar-unreal-mcp vehicle-material-slots-preflight [--root RELATIVE_PATH]
    --package-id PACKAGE_ID
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    vehicle-material-slots-capabilities [--root RELATIVE_PATH]
    --package-id PACKAGE_ID
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    vehicle-material-slots-apply [--root RELATIVE_PATH] --package-id PACKAGE_ID
  shar-unreal-mcp vehicle-physics-prerequisites-preflight
    [--root RELATIVE_PATH] [--package-id PACKAGE_ID]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    vehicle-physics-prerequisites-capabilities [--root RELATIVE_PATH]
    [--package-id PACKAGE_ID]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    vehicle-physics-prerequisites-apply [--root RELATIVE_PATH]
    [--package-id PACKAGE_ID]
  shar-unreal-mcp vehicle-physics-preflight [--root RELATIVE_PATH]
    [--package-id PACKAGE_ID]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    vehicle-physics-capabilities [--root RELATIVE_PATH]
    [--package-id PACKAGE_ID]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    vehicle-physics-apply [--root RELATIVE_PATH] [--package-id PACKAGE_ID]
  shar-unreal-mcp world-material-preflight [--root RELATIVE_PATH]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    world-material-capabilities [--root RELATIVE_PATH]
  shar-unreal-mcp [--endpoint URL] [--timeout SECONDS]
    world-material-apply [--root RELATIVE_PATH]
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


class VehicleMaterialOptions(NamedTuple):
    """Validated local options for vehicle-material three-gate commands."""

    root: Path
    package_id: str | None


class VehiclePhysicsPrerequisiteOptions(NamedTuple):
    """Validated options for vehicle-physics prerequisite commands."""

    root: Path
    package_id: str | None


class VehiclePhysicsOptions(NamedTuple):
    """Validated options for vehicle Physics Asset commands."""

    root: Path
    package_id: str | None


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
    if operands[0].strip() == "call_tool":
        _fail_usage("raw-call cannot invoke call_tool; use call")
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


def parse_vehicle_material_options(
    operands: tuple[str, ...],
) -> VehicleMaterialOptions:
    """Parse optional plan root and exact vehicle package identity."""
    root, package_id = _parse_vehicle_package_options(
        operands, command="vehicle-material"
    )
    return VehicleMaterialOptions(root=root, package_id=package_id)


def parse_vehicle_physics_prerequisite_options(
    operands: tuple[str, ...],
) -> VehiclePhysicsPrerequisiteOptions:
    """Parse optional root and vehicle package for physics prerequisites."""
    root, package_id = _parse_vehicle_package_options(
        operands, command="vehicle-physics-prerequisites"
    )
    return VehiclePhysicsPrerequisiteOptions(root=root, package_id=package_id)


def parse_vehicle_physics_options(
    operands: tuple[str, ...],
) -> VehiclePhysicsOptions:
    """Parse optional root and exact vehicle package for Physics Assets."""
    root, package_id = _parse_vehicle_package_options(
        operands, command="vehicle-physics"
    )
    return VehiclePhysicsOptions(root=root, package_id=package_id)


def _parse_vehicle_package_options(
    operands: tuple[str, ...],
    *,
    command: str,
) -> tuple[Path, str | None]:
    if len(operands) % _TWO_OPTION_PARTS != 0:
        _fail_usage(f"{command} options require option/value pairs")
    root = Path(".cache/pipeline/unreal-staging/plans")
    package_id: str | None = None
    seen: set[str] = set()
    for index in range(0, len(operands), _TWO_OPTION_PARTS):
        option = operands[index]
        value = operands[index + 1]
        if option in seen:
            _fail_usage(f"{command} option is duplicated: {option}")
        seen.add(option)
        if option == "--root":
            root = _portable_relative_child(value, label="plan root")
            continue
        if option == "--package-id":
            if _PACKAGE_ID.fullmatch(value) is None:
                _fail_usage(f"{command} package id is not canonical")
            package_id = value
            continue
        _fail_usage(f"{command} accepts only --root and --package-id options")
    return root, package_id


def parse_plan_root(operands: tuple[str, ...]) -> Path:
    """Parse one repository-relative generated plan directory.

    Returns:
        A safe child path used only for read-only bundle preflight.

    """
    if not operands:
        return Path(".cache/pipeline/unreal-staging/plans")
    if len(operands) != _TWO_OPTION_PARTS or operands[0] != "--root":
        _fail_usage("plan-preflight accepts only --root RELATIVE_PATH")
    return _portable_relative_child(operands[1], label="plan root")


def parse_skill_output_path(operands: tuple[str, ...]) -> Path:
    """Parse the repository-relative Unreal skill output directory.

    Returns:
        A safe child path for generated Unreal skills.

    """
    if not operands:
        return Path("skills/unreal")
    if len(operands) != _TWO_OPTION_PARTS or operands[0] != "--output":
        _fail_usage("skills accepts only --output RELATIVE_PATH")
    return _portable_relative_child(operands[1], label="skills output")


def _portable_relative_child(raw_path: str, *, label: str) -> Path:
    output_path = Path(raw_path)
    posix_path = PurePosixPath(raw_path)
    windows_path = PureWindowsPath(raw_path)
    if (
        posix_path.anchor
        or windows_path.anchor
        or ".." in posix_path.parts
        or ".." in windows_path.parts
    ):
        _fail_usage(f"{label} must be a repository-relative child path")
    if any(ntpath.isreserved(segment) for segment in windows_path.parts):
        _fail_usage(f"{label} must use a portable path")
    if output_path == Path() or not output_path.parts:
        _fail_usage(f"{label} must not be the repository root")
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
