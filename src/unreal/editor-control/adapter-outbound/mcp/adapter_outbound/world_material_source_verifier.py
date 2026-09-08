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
#   - Physical PNG verification for compiled world Texture2D construction.
# - Must-Not:
#   - Follow links, mutate files, contact Unreal Editor, or select identities.
# - Allows:
#   - Check regular files, exact byte counts, and SHA-256 content evidence.
# - Split-When:
#   - Another construction source family gains independent filesystem policy.
# - Merge-When:
#   - Generic plan-source verification accepts these semantic sidecar sources.
# - Summary:
#   - World-material texture source verifier.
# - Description:
#   - Maps content identities to verified absolute PNG paths under the pipeline
#   - cache.
# - Usage:
#   - Called after world-material construction compilation and before mutation.
# - Defaults:
#   - Missing, linked, escaped, stale, or duplicate source evidence fails
#   - closed.
#

"""Physical PNG verification for compiled world Texture2D construction."""

from __future__ import annotations

import hashlib
from pathlib import Path
from pathlib import PurePosixPath
import stat

from mcp.domain.errors import fail_protocol
from mcp.domain.world_material_construction import (
    CompiledWorldMaterialConstruction,
)


def verify_world_material_texture_sources(
    pipeline_cache_root: Path,
    compiled: CompiledWorldMaterialConstruction,
) -> dict[str, Path]:
    """Verify every planned PNG and return SHA-256 keyed absolute paths."""
    root = pipeline_cache_root.absolute()
    _require_directory_chain(root)
    verified: dict[str, Path] = {}
    for step in compiled.textures:
        relative = PurePosixPath(step.source_path)
        if (
            relative.is_absolute()
            or relative.parts[:2] != ("world-assets", "textures")
            or any(part in {"", ".", ".."} for part in relative.parts)
        ):
            fail_protocol("world texture source escaped its cache namespace")
        path = root.joinpath(*relative.parts)
        _require_unlinked_chain(root, relative)
        try:
            metadata = path.lstat()
        except OSError as error:
            fail_protocol("world texture source is missing", cause=error)
        if not stat.S_ISREG(metadata.st_mode) or stat.S_ISLNK(metadata.st_mode):
            fail_protocol("world texture source is not a regular unlinked file")
        if metadata.st_size != step.byte_count:
            fail_protocol("world texture source byte count is stale")
        try:
            data = path.read_bytes()
        except OSError as error:
            fail_protocol("failed to read world texture source", cause=error)
        if len(data) != metadata.st_size:
            fail_protocol("world texture source size changed during read")
        if hashlib.sha256(data).hexdigest() != step.sha256:
            fail_protocol("world texture source digest is stale")
        if step.sha256 in verified:
            fail_protocol("world texture source digest is duplicated")
        verified[step.sha256] = path
    return verified


def _require_directory_chain(path: Path) -> None:
    try:
        metadata = path.lstat()
    except OSError as error:
        fail_protocol("world texture cache root is missing", cause=error)
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        fail_protocol("world texture cache root is not an unlinked directory")


def _require_unlinked_chain(root: Path, relative: PurePosixPath) -> None:
    current = root
    for part in relative.parts[:-1]:
        current /= part
        try:
            metadata = current.lstat()
        except OSError as error:
            fail_protocol(
                "world texture source ancestor is missing", cause=error
            )
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            fail_protocol(
                "world texture source ancestor is linked or not a directory"
            )
