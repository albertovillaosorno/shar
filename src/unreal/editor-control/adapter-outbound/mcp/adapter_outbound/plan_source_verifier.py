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
#   - Read-only physical verification of generated plan source evidence.
# - Must-Not:
#   - Mutate files, contact Unreal Editor, or publish physical source paths.
# - Allows:
#   - Resolve confined sources, reject links, stream SHA-256, and detect races.
# - Split-When:
#   - Split when another source root or storage backend gains a lifecycle.
# - Merge-When:
#   - Merge when native execution owns identical physical evidence checks.
# - Summary:
#   - Generated Unreal plan source verifier.
# - Description:
#   - Verifies every non-conversion source before native execution compilation.
# - Usage:
#   - Called after canonical bundle intake and before live capability checks.
# - Defaults:
#   - Missing, linked, changed, or digest-mismatched sources fail closed.
#

"""Read-only physical verification of generated plan source evidence."""

# cspell:ignore RDONLY fstat

from __future__ import annotations

import hashlib
import os
import stat
from pathlib import Path
from typing import NamedTuple

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.plan_bundle import PlanOperation
from mcp.domain.plan_bundle import ValidatedPlanBundle

_CHUNK_BYTES = 1024 * 1024


class PlanSourceVerificationReport(NamedTuple):
    """Public-safe evidence counts for physically verified sources."""

    bundle_revision: str
    verified_operation_count: int
    unique_source_count: int
    unique_source_bytes: int
    skipped_conversion_count: int

    def to_json(self) -> JsonObject:
        """Render counts without repository or source path disclosure."""
        return {
            "bundleRevision": self.bundle_revision,
            "skippedConversionCount": self.skipped_conversion_count,
            "uniqueSourceBytes": self.unique_source_bytes,
            "uniqueSourceCount": self.unique_source_count,
            "verifiedOperationCount": self.verified_operation_count,
        }


class VerifiedPlanSources(NamedTuple):
    """Verification report and private operation-to-physical-path mapping."""

    report: PlanSourceVerificationReport
    by_operation: dict[str, Path]


class FilesystemPlanSourceVerifier:
    """Verify plan source bytes beneath repository-owned generated roots."""

    def __init__(self, repository_root: Path, plan_root: Path) -> None:
        """Retain unresolved roots so link checks observe the authored chain."""
        self._repository_root = repository_root
        self._plan_root = plan_root

    def verify(self, bundle: ValidatedPlanBundle) -> VerifiedPlanSources:
        """Verify every source not explicitly awaiting upstream conversion."""
        repository_root = self._repository_root.absolute()
        plan_root = self._plan_root.absolute()
        staging_root = plan_root.parent
        _require_root(repository_root, "repository source root")
        _require_root(staging_root, "generated staging root")

        verified_paths: dict[str, Path] = {}
        cached: dict[tuple[str, str], tuple[Path, int]] = {}
        revisions_by_key: dict[str, str] = {}
        skipped = 0
        for operation in bundle.operations:
            if operation.readiness == "requires-conversion":
                skipped += 1
                continue
            if (
                operation.plan_id == "asset-construction-plan"
                and operation.source_revision
                != bundle.report.source_manifest_revision
            ):
                fail_protocol("construction source revision is stale")
            base, namespace, physical_source_path = _source_location(
                operation,
                repository_root=repository_root,
            )
            cache_key = (namespace, operation.source_path)
            revision_key = f"{namespace}/{operation.source_path}"
            prior_revision = revisions_by_key.setdefault(
                revision_key,
                operation.source_revision,
            )
            if prior_revision != operation.source_revision:
                fail_protocol("plan source path has conflicting revisions")
            cached_source = cached.get(cache_key)
            if cached_source is None:
                source = base.joinpath(*physical_source_path.split("/"))
                byte_count = _verify_source(
                    source,
                    base=base,
                    expected_revision=operation.source_revision,
                )
                cached_source = (source, byte_count)
                cached[cache_key] = cached_source
            verified_paths[operation.operation_id] = cached_source[0]

        report = PlanSourceVerificationReport(
            bundle_revision=bundle.report.revision,
            verified_operation_count=len(verified_paths),
            unique_source_count=len(cached),
            unique_source_bytes=sum(item[1] for item in cached.values()),
            skipped_conversion_count=skipped,
        )
        return VerifiedPlanSources(report, verified_paths)


def _source_location(
    operation: PlanOperation,
    *,
    repository_root: Path,
) -> tuple[Path, str, str]:
    if operation.plan_id == "asset-construction-plan":
        if operation.source_path != "manifest.jsonl":
            fail_protocol("construction operation source is not canonical")
        return (
            repository_root,
            "unreal-manifest",
            "game/manifest/unreal.jsonl",
        )
    if operation.source_path.startswith("extracted/"):
        return (
            repository_root,
            "extracted",
            f".cache/pipeline/{operation.source_path}",
        )
    if operation.source_path.startswith("fbx-assets/"):
        return (
            repository_root,
            "fbx-assets",
            f".cache/pipeline/{operation.source_path}",
        )
    return repository_root, "repository", operation.source_path


def _verify_source(
    path: Path,
    *,
    base: Path,
    expected_revision: str,
) -> int:
    _validate_existing_chain(base, path.parent)
    metadata = _metadata(path)
    if metadata is None:
        fail_protocol("plan source file is missing")
    _require_regular_file(path, metadata)
    flags = os.O_RDONLY | getattr(os, "O_BINARY", 0)
    descriptor: int | None = None
    try:
        descriptor = os.open(path, flags)
        opened = os.fstat(descriptor)
        _require_same_file(metadata, opened)
        digest = hashlib.sha256()
        byte_count = 0
        while True:
            chunk = os.read(descriptor, _CHUNK_BYTES)
            if not chunk:
                break
            digest.update(chunk)
            byte_count += len(chunk)
        final_opened = os.fstat(descriptor)
    except OSError as error:
        fail_protocol("failed to read plan source file", cause=error)
    finally:
        if descriptor is not None:
            try:
                os.close(descriptor)
            except OSError:
                pass
    final_metadata = _metadata(path)
    if final_metadata is None:
        fail_protocol("plan source file disappeared during verification")
    _require_regular_file(path, final_metadata)
    _require_same_file(opened, final_opened)
    _require_same_file(metadata, final_metadata)
    if byte_count != metadata.st_size:
        fail_protocol("plan source file size changed during verification")
    if digest.hexdigest() != expected_revision:
        fail_protocol("plan source revision does not match physical bytes")
    _validate_existing_chain(base, path.parent)
    return byte_count


def _require_root(path: Path, label: str) -> None:
    _validate_existing_chain(Path(path.anchor), path)
    metadata = _metadata(path)
    if metadata is None:
        fail_protocol(f"{label} is missing")
    _reject_link_or_reparse(path, metadata)
    if not stat.S_ISDIR(metadata.st_mode):
        fail_protocol(f"{label} is not a regular directory")


def _validate_existing_chain(base: Path, target: Path) -> None:
    try:
        relative = target.relative_to(base)
    except ValueError as error:
        fail_protocol("plan source escaped its declared root", cause=error)
    current = base
    base_metadata = _metadata(current)
    if base_metadata is None:
        fail_protocol("plan source root is missing")
    _reject_link_or_reparse(current, base_metadata)
    if not stat.S_ISDIR(base_metadata.st_mode):
        fail_protocol("plan source root is not a regular directory")
    for part in relative.parts:
        current /= part
        metadata = _metadata(current)
        if metadata is None:
            fail_protocol("plan source ancestor is missing")
        _reject_link_or_reparse(current, metadata)
        if not stat.S_ISDIR(metadata.st_mode):
            fail_protocol("plan source ancestor is not a regular directory")


def _require_regular_file(path: Path, metadata: os.stat_result) -> None:
    _reject_link_or_reparse(path, metadata)
    if not stat.S_ISREG(metadata.st_mode):
        fail_protocol("plan source is not a regular file")


def _require_same_file(first: os.stat_result, second: os.stat_result) -> None:
    identity = ("st_dev", "st_ino", "st_size", "st_mtime_ns")
    if any(
        getattr(first, field, None) != getattr(second, field, None)
        for field in identity
    ):
        fail_protocol("plan source changed during verification")


def _reject_link_or_reparse(path: Path, metadata: os.stat_result) -> None:
    attributes = getattr(metadata, "st_file_attributes", 0)
    reparse_attribute = getattr(stat, "FILE_ATTRIBUTE_REPARSE_POINT", 0)
    if (
        stat.S_ISLNK(metadata.st_mode)
        or bool(attributes & reparse_attribute)
        or os.path.isjunction(path)
    ):
        fail_protocol("plan source crosses a link or reparse point")


def _metadata(path: Path) -> os.stat_result | None:
    try:
        return path.lstat()
    except FileNotFoundError:
        return None
    except OSError as error:
        fail_protocol("failed to inspect plan source", cause=error)
