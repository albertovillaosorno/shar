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
#   - Errors domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Errors domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Errors domain module."""

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
