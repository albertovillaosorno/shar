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
#   - Read-only intake of the release-index-bound vehicle-material sidecar.
# - Must-Not:
#   - Follow links, contact Unreal Editor, or accept unbound semantic evidence.
# - Allows:
#   - Recheck exact bytes and SHA-256 immediately before native compilation.
# - Split-When:
#   - Another semantic sidecar gains an independent construction lifecycle.
# - Merge-When:
#   - Plan-bundle intake exposes identical bound semantic-document reads.
# - Summary:
#   - Bound vehicle-material construction sidecar reader.
# - Description:
#   - Uses the already validated release-index summary as its sole authority.
# - Usage:
#   - Called after FilesystemPlanBundleReader and before construction compile.
# - Defaults:
#   - Missing, linked, stale, duplicate-key, or non-object JSON fails closed.
#

"""Read-only intake of the release-index-bound vehicle-material sidecar."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import stat

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import DuplicateJsonKeyError
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import reject_duplicate_json_object
from mcp.domain.json_types import require_json_object
from mcp.domain.plan_bundle import SemanticArtifactSummary
from mcp.domain.plan_bundle import ValidatedPlanBundle

_ARTIFACT_ID = "vehicle-materials"
_FILENAME = "vehicle-materials.json"
_MAX_BYTES = 512 * 1024 * 1024


def read_bound_vehicle_material_document(
    staging_root: Path,
    bundle: ValidatedPlanBundle,
) -> JsonObject:
    """Read the release-index-bound vehicle-material sidecar."""
    summary = _bound_summary(bundle)
    root = staging_root.absolute()
    _require_unlinked_directory(root)
    data = _read_bound_bytes(root / summary.filename, summary)
    return _parse_document(data)


def _bound_summary(bundle: ValidatedPlanBundle) -> SemanticArtifactSummary:
    matches = tuple(
        item
        for item in bundle.report.semantic_artifacts
        if item.artifact_id == _ARTIFACT_ID
    )
    if len(matches) != 1 or matches[0].filename != _FILENAME:
        fail_protocol(
            "plan bundle does not bind one canonical vehicle-material sidecar"
        )
    return matches[0]


def _require_unlinked_directory(path: Path) -> None:
    try:
        metadata = path.lstat()
    except OSError as error:
        fail_protocol("vehicle-material staging root is missing", cause=error)
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        fail_protocol(
            "vehicle-material staging root is redirected or not a directory"
        )


def _read_bound_bytes(
    path: Path,
    summary: SemanticArtifactSummary,
) -> bytes:
    try:
        metadata = path.lstat()
    except OSError as error:
        fail_protocol("vehicle-material sidecar is missing", cause=error)
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail_protocol("vehicle-material sidecar is not a regular unlinked file")
    if metadata.st_size > _MAX_BYTES or metadata.st_size != summary.byte_count:
        fail_protocol("vehicle-material sidecar byte count is stale")
    try:
        data = path.read_bytes()
    except OSError as error:
        fail_protocol("failed to read vehicle-material sidecar", cause=error)
    if len(data) != metadata.st_size:
        fail_protocol("vehicle-material sidecar size changed during read")
    if hashlib.sha256(data).hexdigest() != summary.revision:
        fail_protocol("vehicle-material sidecar revision is stale")
    return data


def _parse_document(data: bytes) -> JsonObject:
    try:
        value = json.loads(
            data.decode("utf-8", errors="strict"),
            object_pairs_hook=reject_duplicate_json_object,
            parse_constant=lambda _: fail_protocol(
                "vehicle-material sidecar has a non-finite number"
            ),
        )
    except (DuplicateJsonKeyError, UnicodeError, json.JSONDecodeError) as error:
        fail_protocol(
            "vehicle-material sidecar is not canonical JSON", cause=error
        )
    return require_json_object(value, context="vehicle-material sidecar")
