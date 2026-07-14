# File:
#   - endpoint.py
# Path:
#   - src/mcp/src/domain/endpoint.py
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
#   - Validation of the loopback-only Unreal MCP endpoint.
# - Must-Not:
#   - Open sockets, read configuration, or start Unreal Editor.
# - Allows:
#   - Pure URL parsing and canonical endpoint formatting.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Defines the only network destination the client may use.
# - Description:
#   - Rejects remote, credentialed, and ambiguous endpoints.
# - Usage:
#   - Constructed by driving adapters before transport creation.
# - Defaults:
#   - Uses the native server default on loopback port 8000.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: loopback MCP endpoint value
#   - reason: validation and canonical formatting form one immutable value
#   - split: extract host policy if configurable trust boundaries are added
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Loopback-only endpoint value for the native Unreal MCP server."""

from __future__ import annotations

from typing import NamedTuple
from urllib.parse import SplitResult, urlsplit

from mcp.src.domain.errors import fail_endpoint

_ALLOWED_HOSTS = frozenset({"127.0.0.1", "::1", "localhost"})
_DEFAULT_PORT = 8000
_DEFAULT_PATH = "/mcp"
_MAX_PORT = 65_535


class McpEndpoint(NamedTuple):
    """Validated local HTTP endpoint."""

    host: str
    port: int
    path: str

    @classmethod
    def parse(cls, value: str) -> McpEndpoint:
        """Parse a loopback HTTP endpoint.

        Args:
            value: Candidate endpoint URL.

        Returns:
            A validated canonical endpoint.

        """
        parsed = urlsplit(value)
        cls._validate_split(parsed)
        try:
            port = parsed.port or _DEFAULT_PORT
        except ValueError as error:
            fail_endpoint(
                "MCP endpoint contains an invalid port",
                cause=error,
            )
        if not 1 <= port <= _MAX_PORT:
            fail_endpoint("MCP endpoint port must be between 1 and 65535")
        path = parsed.path or _DEFAULT_PATH
        if not path.startswith("/") or path == "/":
            fail_endpoint("MCP endpoint path must be a non-root absolute path")
        hostname = parsed.hostname
        if hostname is None:
            fail_endpoint("MCP endpoint hostname is missing")
        return cls(host=hostname, port=port, path=path)

    @classmethod
    def default(cls) -> McpEndpoint:
        """Return the native Unreal MCP default endpoint.

        Returns:
            The loopback endpoint on port 8000 and path `/mcp`.
        """
        return cls(host="127.0.0.1", port=_DEFAULT_PORT, path=_DEFAULT_PATH)

    @staticmethod
    def _validate_split(parsed: SplitResult) -> None:
        if parsed.scheme != "http":
            fail_endpoint("MCP endpoint must use loopback HTTP")
        if parsed.username is not None or parsed.password is not None:
            fail_endpoint("MCP endpoint must not contain credentials")
        if parsed.query or parsed.fragment:
            fail_endpoint("MCP endpoint must not contain a query or fragment")
        hostname = parsed.hostname
        if hostname is None or hostname.casefold() not in _ALLOWED_HOSTS:
            fail_endpoint("MCP endpoint must use an explicit loopback host")

    @property
    def authority(self) -> str:
        """The host accepted by the HTTP adapter.

        Returns:
            The validated loopback hostname.
        """
        return self.host

    @property
    def origin(self) -> str:
        """The canonical loopback HTTP origin.

        Returns:
            The scheme, host, and explicit port without the MCP path.
        """
        host = f"[{self.host}]" if ":" in self.host else self.host
        return f"http://{host}:{self.port}"

    @property
    def url(self) -> str:
        """The canonical display URL.

        Returns:
            The complete loopback URL with explicit port and path.
        """
        return f"{self.origin}{self.path}"
