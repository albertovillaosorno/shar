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
#   - Physical PNG verification for compiled vehicle material construction.
# - Must-Not:
#   - Follow links, mutate files, contact Unreal Editor, or select identities.
# - Allows:
#   - Check regular files, exact byte counts, and SHA-256 content evidence.
# - Split-When:
#   - Another vehicle construction source family gains filesystem policy.
# - Merge-When:
#   - Generic construction source verification owns these exact guarantees.
# - Summary:
#   - Vehicle-material texture source verifier.
# - Description:
#   - Maps content identities to verified absolute PNG paths under release root.
# - Usage:
#   - Called after vehicle-material compilation and before native mutation.
# - Defaults:
#   - Missing, linked, escaped, stale, or duplicate source evidence fails
#   - closed.
#

"""Physical PNG verification for compiled vehicle material construction."""

from __future__ import annotations

import hashlib
from pathlib import Path
from pathlib import PurePosixPath
import stat
from typing import NamedTuple

from mcp.domain.errors import fail_protocol
from mcp.domain.vehicle_material_selection import VehicleMaterialExecutable


class VerifiedVehicleMaterialTextureSource(NamedTuple):
    """One source path with integrity and Unreal-import provenance digest."""

    path: Path
    md5: str


def verify_vehicle_material_texture_sources(
    release_root: Path,
    compiled: VehicleMaterialExecutable,
) -> dict[str, VerifiedVehicleMaterialTextureSource]:
    """Verify every planned PNG and return SHA-256 keyed provenance."""
    root = release_root.absolute()
    _require_directory(root)
    verified: dict[str, VerifiedVehicleMaterialTextureSource] = {}
    for step in compiled.textures:
        relative = PurePosixPath(step.source_path)
        if (
            relative.is_absolute()
            or relative.parts[:1] != ("vehicle-assets",)
            or any(part in {"", ".", ".."} for part in relative.parts)
        ):
            fail_protocol(
                "vehicle texture source escaped its release namespace"
            )
        path = root.joinpath(*relative.parts)
        _require_unlinked_chain(root, relative)
        try:
            metadata = path.lstat()
        except OSError as error:
            fail_protocol("vehicle texture source is missing", cause=error)
        if not stat.S_ISREG(metadata.st_mode) or stat.S_ISLNK(metadata.st_mode):
            fail_protocol(
                "vehicle texture source is not a regular unlinked file"
            )
        if metadata.st_size != step.byte_count:
            fail_protocol("vehicle texture source byte count is stale")
        try:
            data = path.read_bytes()
        except OSError as error:
            fail_protocol("failed to read vehicle texture source", cause=error)
        if len(data) != metadata.st_size:
            fail_protocol("vehicle texture source size changed during read")
        if hashlib.sha256(data).hexdigest() != step.sha256:
            fail_protocol("vehicle texture source digest is stale")
        if step.sha256 in verified:
            fail_protocol("vehicle texture source digest is duplicated")
        verified[step.sha256] = VerifiedVehicleMaterialTextureSource(
            path=path,
            md5=hashlib.md5(data, usedforsecurity=False).hexdigest(),
        )
    return verified


def _require_directory(path: Path) -> None:
    try:
        metadata = path.lstat()
    except OSError as error:
        fail_protocol("vehicle texture release root is missing", cause=error)
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        fail_protocol(
            "vehicle texture release root is not an unlinked directory"
        )


def _require_unlinked_chain(root: Path, relative: PurePosixPath) -> None:
    current = root
    for part in relative.parts[:-1]:
        current /= part
        try:
            metadata = current.lstat()
        except OSError as error:
            fail_protocol(
                "vehicle texture source ancestor is missing", cause=error
            )
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            fail_protocol(
                "vehicle texture source ancestor is linked or not a directory"
            )
