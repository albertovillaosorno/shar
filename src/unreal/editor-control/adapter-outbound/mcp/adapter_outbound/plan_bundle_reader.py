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
#   - Read-only filesystem intake for generated Unreal plan bundles.
# - Must-Not:
#   - Mutate files, contact Unreal Editor, or weaken domain validation.
# - Allows:
#   - Verify exact inventory, regular files, size bounds, UTF-8, and links.
# - Split-When:
#   - Split when another generated plan storage format gains a lifecycle.
# - Merge-When:
#   - Merge when plan application owns identical bundle intake.
# - Summary:
#   - Generated Unreal plan bundle filesystem reader.
# - Description:
#   - Reads exactly one index and six plan files without following links.
# - Usage:
#   - Called by local plan preflight before any MCP session is opened.
# - Defaults:
#   - Missing, extra, linked, oversized, or non-UTF-8 evidence fails closed.
#

"""Read-only filesystem intake for generated Unreal plan bundles."""

from __future__ import annotations

import os
import stat
from pathlib import Path

from mcp.domain.errors import fail_protocol
from mcp.domain.plan_bundle import PlanBundleReport
from mcp.domain.plan_bundle import ValidatedPlanBundle
from mcp.domain.plan_bundle import parse_plan_bundle

_INDEX_FILE = "index.json"
_PLAN_FILES = frozenset(
    {
        "asset-import-plan.json",
        "asset-construction-plan.json",
        "world-assembly-plan.json",
        "runtime-binding-plan.json",
        "validation-plan.json",
        "package-plan.json",
    }
)
_EXPECTED_FILES = _PLAN_FILES | {_INDEX_FILE}
_MAX_INDEX_BYTES = 1024 * 1024
_MAX_PLAN_BYTES = 512 * 1024 * 1024
_MAX_BUNDLE_BYTES = 1024 * 1024 * 1024


class FilesystemPlanBundleReader:
    """Read and independently validate one generated plan directory."""

    def __init__(self, root: Path) -> None:
        """Retain one caller-selected plan directory without resolving it."""
        self._root = root

    def read(self) -> PlanBundleReport:
        """Read exactly seven regular files and return the public report."""
        return self.read_bundle().report

    def read_bundle(self) -> ValidatedPlanBundle:
        """Read exactly seven regular files into immutable validated evidence."""
        root = self._root.absolute()
        _validate_existing_directory_chain(root)
        metadata = _metadata(root)
        if metadata is None:
            fail_protocol("generated plan bundle root is missing")
        _require_directory(root, metadata, "generated plan bundle root")
        files = _exact_inventory(root)
        index_text = _read_utf8(
            files[_INDEX_FILE],
            byte_limit=_MAX_INDEX_BYTES,
            label="generated plan bundle index",
        )
        plan_texts: dict[str, str] = {}
        total_bytes = len(index_text.encode("utf-8"))
        for filename in sorted(_PLAN_FILES):
            text = _read_utf8(
                files[filename],
                byte_limit=_MAX_PLAN_BYTES,
                label="generated plan file",
            )
            total_bytes += len(text.encode("utf-8"))
            if total_bytes > _MAX_BUNDLE_BYTES:
                fail_protocol("generated plan bundle exceeds its size limit")
            plan_texts[filename] = text
        return parse_plan_bundle(index_text, plan_texts)


def _exact_inventory(root: Path) -> dict[str, Path]:
    observed: dict[str, Path] = {}
    try:
        with os.scandir(root) as entries:
            for entry in entries:
                path = Path(entry.path)
                try:
                    metadata = entry.stat(follow_symlinks=False)
                except OSError as error:
                    fail_protocol(
                        "failed to inspect generated plan bundle entry",
                        cause=error,
                    )
                _reject_link_or_reparse(path, metadata)
                if not stat.S_ISREG(metadata.st_mode):
                    fail_protocol(
                        "generated plan bundle contains a non-file entry"
                    )
                if entry.name in observed:
                    fail_protocol(
                        "generated plan bundle contains a filename collision"
                    )
                observed[entry.name] = path
    except OSError as error:
        fail_protocol("failed to list generated plan bundle", cause=error)
    if set(observed) != _EXPECTED_FILES:
        fail_protocol("generated plan bundle file inventory is not exact")
    return observed


def _read_utf8(path: Path, *, byte_limit: int, label: str) -> str:
    metadata = _metadata(path)
    if metadata is None:
        fail_protocol(f"{label} is missing")
    _require_regular_file(path, metadata, label)
    if metadata.st_size > byte_limit:
        fail_protocol(f"{label} exceeds its size limit")
    try:
        data = path.read_bytes()
    except OSError as error:
        fail_protocol(f"failed to read {label}", cause=error)
    if len(data) != metadata.st_size:
        fail_protocol(f"{label} size changed during read")
    try:
        return data.decode("utf-8", errors="strict")
    except UnicodeError as error:
        fail_protocol(f"{label} is not canonical UTF-8", cause=error)


def _validate_existing_directory_chain(path: Path) -> None:
    anchor = Path(path.anchor)
    current = anchor
    if path.anchor:
        metadata = _metadata(anchor)
        if metadata is None:
            fail_protocol("generated plan bundle anchor is missing")
        _require_directory(anchor, metadata, "generated plan bundle ancestor")
        parts = path.parts[1:]
    else:
        parts = path.parts
    for part in parts:
        current /= part
        metadata = _metadata(current)
        if metadata is None:
            return
        _require_directory(current, metadata, "generated plan bundle ancestor")


def _require_regular_file(
    path: Path,
    metadata: os.stat_result,
    label: str,
) -> None:
    _reject_link_or_reparse(path, metadata)
    if not stat.S_ISREG(metadata.st_mode):
        fail_protocol(f"{label} is not a regular file")


def _require_directory(
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
        fail_protocol("generated plan bundle crosses a link or reparse point")


def _metadata(path: Path) -> os.stat_result | None:
    try:
        return path.lstat()
    except FileNotFoundError:
        return None
    except OSError as error:
        fail_protocol("failed to inspect generated plan bundle", cause=error)
