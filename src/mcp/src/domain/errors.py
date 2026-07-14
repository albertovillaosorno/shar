# File:
#   - errors.py
# Path:
#   - src/mcp/src/domain/errors.py
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
#   - Typed translator failures and centralized failure construction.
# - Must-Not:
#   - Import transport, CLI, filesystem, or Unreal APIs.
# - Allows:
#   - Stable exceptions and cause-preserving fail helpers.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Defines fail-closed translator error categories.
# - Description:
#   - Keeps messages and exception chaining in one domain module.
# - Usage:
#   - Called by domain, application, and adapter boundaries.
# - Defaults:
#   - Unknown failures are never converted to success.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: translator domain failure taxonomy
#   - reason: error types and fail helpers share one failure contract
#   - split: extract construction helpers if error categories expand
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Domain-specific failures for the Unreal MCP terminal translator."""

from __future__ import annotations

from typing import Never


class UnrealMcpError(RuntimeError):
    """Base failure exposed by the translator."""


class ConfigurationError(UnrealMcpError):
    """Raised when translator configuration is invalid."""


class EndpointValidationError(UnrealMcpError):
    """Raised when an endpoint violates the loopback-only policy."""


class ProtocolError(UnrealMcpError):
    """Raised when the server returns an invalid MCP response."""


class TransportError(UnrealMcpError):
    """Raised when Streamable HTTP cannot complete a request."""


class RequestTimeoutError(TransportError):
    """Raised when one bounded MCP exchange exceeds its timeout."""


class ToolCallError(UnrealMcpError):
    """Raised when a native Unreal tool reports an application error."""


def fail_configuration(
    message: str,
    *,
    cause: BaseException | None = None,
) -> Never:
    """Raise a translator configuration failure."""
    _raise(ConfigurationError, message, cause=cause)


def fail_endpoint(
    message: str,
    *,
    cause: BaseException | None = None,
) -> Never:
    """Raise an endpoint validation failure."""
    _raise(EndpointValidationError, message, cause=cause)


def fail_protocol(
    message: str,
    *,
    cause: BaseException | None = None,
) -> Never:
    """Raise an MCP protocol failure."""
    _raise(ProtocolError, message, cause=cause)


def fail_transport(
    message: str,
    *,
    cause: BaseException | None = None,
) -> Never:
    """Raise a transport failure."""
    _raise(TransportError, message, cause=cause)


def fail_timeout(
    message: str,
    *,
    cause: BaseException | None = None,
) -> Never:
    """Raise a bounded MCP exchange timeout."""
    _raise(RequestTimeoutError, message, cause=cause)


def fail_tool_call(message: str) -> Never:
    """Raise a native Unreal tool-call failure."""
    _raise(ToolCallError, message)


def _raise(
    error_type: type[UnrealMcpError],
    message: str,
    *,
    cause: BaseException | None = None,
) -> Never:
    """Raise one typed failure while preserving an optional cause."""
    failure = error_type(message)
    if cause is None:
        raise failure
    raise failure from cause
