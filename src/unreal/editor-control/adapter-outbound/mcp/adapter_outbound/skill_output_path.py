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
#   - Filesystem path safety for generated Unreal skill publication.
# - Must-Not:
#   - Render skills, merge manual fields, or choose generated filenames.
# - Allows:
#   - Validate ancestors, reject links, inspect the owned tree, and create dirs.
# - Split-When:
#   - Split when tree inspection and directory publication diverge.
# - Merge-When:
#   - Merge when generated skill persistence owns all path policy.
# - Summary:
#   - Generated skill output path guard.
# - Description:
#   - Prevents generated cleanup and replacement from crossing filesystem links.
# - Usage:
#   - Called before reads, deletions, directory creation, and atomic writes.
# - Defaults:
#   - Missing roots are allowed; existing unsafe boundaries fail closed.
#

"""Filesystem path safety for generated Unreal skill publication."""

from __future__ import annotations

import os
import stat
from pathlib import Path

from mcp.domain.errors import fail_protocol


def validate_existing_output_surface(output_root: Path) -> None:
    """Validate every existing boundary used by the generated skill store."""
    absolute_root = output_root.absolute()
    _validate_existing_directory_chain(absolute_root)
    root_metadata = _path_metadata(absolute_root)
    if root_metadata is None:
        return
    _validate_regular_directory(
        absolute_root,
        root_metadata,
        "generated skill output root",
    )
    index = absolute_root / "index.md"
    if _path_metadata(index) is not None:
        validate_regular_file(index, "generated skill index")
    capabilities = absolute_root / "capabilities"
    if _path_metadata(capabilities) is not None:
        _validate_regular_tree(capabilities)


def ensure_output_root(output_root: Path) -> None:
    """Create and independently validate one generated skill output root."""
    try:
        output_root.mkdir(parents=True, exist_ok=True)
    except OSError as error:
        fail_protocol("failed to create generated skill output root", cause=error)
    validate_existing_output_surface(output_root)


def ensure_owned_directory(output_root: Path, directory: Path) -> None:
    """Create one descendant directory without crossing a link boundary."""
    try:
        relative = directory.relative_to(output_root)
    except ValueError as error:
        fail_protocol("generated skill target escaped its output root", cause=error)
    current = output_root
    root_metadata = _path_metadata(current)
    if root_metadata is None:
        fail_protocol("generated skill output root disappeared")
    _validate_regular_directory(
        current,
        root_metadata,
        "generated skill output root",
    )
    for part in relative.parts:
        current /= part
        metadata = _path_metadata(current)
        if metadata is None:
            try:
                current.mkdir()
            except OSError as error:
                fail_protocol(
                    "failed to create generated capability directory",
                    cause=error,
                )
            metadata = _path_metadata(current)
            if metadata is None:
                fail_protocol("generated capability directory was not created")
        _validate_regular_directory(
            current,
            metadata,
            "generated capability directory",
        )


def validate_regular_target(path: Path) -> None:
    """Allow a missing target or require one direct regular file."""
    if _path_metadata(path) is not None:
        validate_regular_file(path, "generated skill target")


def validate_temporary_target(path: Path) -> None:
    """Require an atomic temporary path to be unused."""
    if _path_metadata(path) is not None:
        fail_protocol("generated skill temporary path already exists")


def validate_regular_file(path: Path, label: str) -> None:
    """Require one direct regular file without a reparse boundary."""
    metadata = _path_metadata(path)
    if metadata is None:
        fail_protocol(f"{label} is missing")
    _reject_link_or_reparse(path, metadata)
    if not stat.S_ISREG(metadata.st_mode):
        fail_protocol(f"{label} is not a regular file")


def _validate_existing_directory_chain(path: Path) -> None:
    anchor = Path(path.anchor)
    current = anchor
    if path.anchor:
        metadata = _path_metadata(anchor)
        if metadata is None:
            fail_protocol("generated skill output anchor is missing")
        _validate_regular_directory(
            anchor,
            metadata,
            "generated skill output ancestor",
        )
        parts = path.parts[1:]
    else:
        parts = path.parts
    for part in parts:
        current /= part
        metadata = _path_metadata(current)
        if metadata is None:
            return
        _validate_regular_directory(
            current,
            metadata,
            "generated skill output ancestor",
        )


def _validate_regular_tree(directory: Path) -> None:
    metadata = _path_metadata(directory)
    if metadata is None:
        return
    _validate_regular_directory(
        directory,
        metadata,
        "generated capability directory",
    )
    try:
        with os.scandir(directory) as entries:
            for entry in entries:
                path = Path(entry.path)
                try:
                    entry_metadata = entry.stat(follow_symlinks=False)
                except OSError as error:
                    fail_protocol(
                        "failed to inspect generated capability entry",
                        cause=error,
                    )
                _reject_link_or_reparse(path, entry_metadata)
                if stat.S_ISDIR(entry_metadata.st_mode):
                    _validate_regular_tree(path)
                elif not stat.S_ISREG(entry_metadata.st_mode):
                    fail_protocol(
                        "generated capability entry is not a regular file"
                    )
    except OSError as error:
        fail_protocol("failed to list generated capability tree", cause=error)


def _validate_regular_directory(
    path: Path,
    metadata: os.stat_result,
    label: str,
) -> None:
    _reject_link_or_reparse(path, metadata)
    if not stat.S_ISDIR(metadata.st_mode):
        fail_protocol(f"{label} is not a regular directory")


def _reject_link_or_reparse(path: Path, metadata: os.stat_result) -> None:
    attributes = getattr(metadata, "st_file_attributes", 0)
    reparse_attribute = getattr(stat, "FILE_ATTRIBUTE_REPARSE_POINT", 0)
    if (
        stat.S_ISLNK(metadata.st_mode)
        or bool(attributes & reparse_attribute)
        or os.path.isjunction(path)
    ):
        fail_protocol("generated skill output crosses a link or reparse point")


def _path_metadata(path: Path) -> os.stat_result | None:
    try:
        return path.lstat()
    except FileNotFoundError:
        return None
    except OSError as error:
        fail_protocol("failed to inspect generated skill output", cause=error)
