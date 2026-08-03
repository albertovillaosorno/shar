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
#   - Skill revision domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill revision domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill revision domain module."""

from __future__ import annotations

import re
from typing import NamedTuple

from mcp.domain.errors import fail_configuration

_VERSION_MAJOR_PATTERN = r"^(?P<major>0|[1-9][0-9]*)"
_VERSION_MINOR_PATTERN = r"(?:\.(?P<minor>0|[1-9][0-9]*))?"
_VERSION_PATCH_PATTERN = r"(?:\.(?P<patch>0|[1-9][0-9]*))?$"
_VERSION_PATTERN = re.compile(
    f"{_VERSION_MAJOR_PATTERN}{_VERSION_MINOR_PATTERN}{_VERSION_PATCH_PATTERN}"
)
_DIGEST_PATTERN = re.compile(r"^[0-9a-f]{64}$")


class SkillRevision(NamedTuple):
    """One Unreal MCP plugin version and live interface digest."""

    unreal_mcp_version: str
    interface_digest: str

    @property
    def token(self) -> str:
        """The protected manual-review revision token."""
        return f"{self.unreal_mcp_version}/{self.interface_digest}"


def build_skill_revision(
    unreal_mcp_version: str,
    interface_digest: str,
) -> SkillRevision:
    """Build a validated generated skill revision.

    Returns:
        Normalized Unreal MCP version and validated interface digest.

    """
    normalized_version = normalize_unreal_mcp_version(unreal_mcp_version)
    if _DIGEST_PATTERN.fullmatch(interface_digest) is None:
        fail_configuration(
            "skill interface digest must be 64 lowercase hex digits"
        )
    return SkillRevision(normalized_version, interface_digest)


def normalize_unreal_mcp_version(version_name: str) -> str:
    """Normalize an Unreal plugin version name to three-part SemVer.

    Returns:
        A `major.minor.patch` version string.

    """
    match = _VERSION_PATTERN.fullmatch(version_name)
    if match is None:
        fail_configuration(
            "Unreal MCP VersionName must contain one to three numeric parts"
        )
    major = match.group("major")
    minor = match.group("minor") or "0"
    patch = match.group("patch") or "0"
    return f"{major}.{minor}.{patch}"
