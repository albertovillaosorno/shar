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
#   - Physical generated-plan source verification tests.
# - Must-Not:
#   - Read repository-generated assets or contact Unreal Editor.
# - Allows:
#   - Isolated files, digests, links, and deduplicated source fixtures.
# - Split-When:
#   - Split when another physical source root gains independent tests.
# - Merge-When:
#   - Merge when source verification has no independent adapter policy.
# - Summary:
#   - Unreal plan source verifier tests.
# - Description:
#   - Proves bytes, revisions, roots, links, and shared manifest handling.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Diagnostics never disclose physical fixture paths.
#

"""Tests for physical generated Unreal plan source verification."""

from __future__ import annotations

import hashlib
import os
from pathlib import Path

from mcp.adapter_outbound.plan_source_verifier import (
    FilesystemPlanSourceVerifier,
)
from mcp.domain.errors import ProtocolError
from mcp.domain.plan_bundle import PlanBundleReport
from mcp.domain.plan_bundle import PlanOperation
from mcp.domain.plan_bundle import ValidatedPlanBundle
import pytest


def _digest(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


def _operation(
    operation_id: str,
    *,
    plan_id: str,
    source_path: str,
    source_revision: str,
    readiness: str,
) -> PlanOperation:
    source_format = "json" if plan_id == "asset-construction-plan" else "image"
    target_family = "structured-data" if source_format == "json" else "texture"
    target_class = "WidgetBlueprint" if source_format == "json" else "Texture2D"
    importer = (
        "shar-ui-factory" if source_format == "json" else "texture-factory"
    )
    import_profile = (
        "shar-ui-v1" if source_format == "json" else "shar-texture-v1"
    )
    asset_name = operation_id.removeprefix("operation-")
    return PlanOperation(
        plan_id=plan_id,
        operation_id=operation_id,
        package_identity=f"package-{asset_name}",
        source_identity=f"source-{asset_name}",
        source_format=source_format,
        target_family=target_family,
        source_path=source_path,
        source_revision=source_revision,
        destination=f"/Game/Generated/SHAR/test/a_{asset_name}.a_{asset_name}",
        target_class=target_class,
        importer=importer,
        import_profile=import_profile,
        dependencies=(),
        readiness=readiness,
        world_owned=False,
        runtime_bound=True,
    )


def _bundle(*operations: PlanOperation) -> ValidatedPlanBundle:
    manifest_revision = next(
        (
            operation.source_revision
            for operation in operations
            if operation.plan_id == "asset-construction-plan"
        ),
        "e" * 64,
    )
    report = PlanBundleReport(
        revision="f" * 64,
        source_manifest_revision=manifest_revision,
        engine_contract_revision="shar-unreal-porting-contract-v1",
        target_engine_version="5.8.1",
        target_platform="editor",
        semantic_blocker_count=0,
        operation_count=len(operations),
        readiness_counts={},
        plans=(),
    )
    return ValidatedPlanBundle(report, operations)


def _roots(tmp_path: Path) -> tuple[Path, Path]:
    repository = tmp_path / "repository"
    plan_root = repository / "unreal-staging" / "plans"
    plan_root.mkdir(parents=True)
    return repository, plan_root


def _create_file_link(target: Path, link: Path) -> None:
    try:
        link.symlink_to(target)
    except OSError as error:
        if os.name == "nt" and getattr(error, "winerror", None) == 1314:
            pytest.skip("file symlinks require Windows developer mode")
        raise


def test_verifies_unique_sources_and_reuses_shared_manifest(
    tmp_path: Path,
) -> None:
    repository, plan_root = _roots(tmp_path)
    image = repository / "extracted" / "image.png"
    image.parent.mkdir()
    image_bytes = b"verified-image"
    image.write_bytes(image_bytes)
    manifest = plan_root.parent / "manifest.jsonl"
    manifest_bytes = b'{"schema":"fixture"}\n'
    manifest.write_bytes(manifest_bytes)

    operations = (
        _operation(
            "operation-0000000000000001",
            plan_id="asset-import-plan",
            source_path="extracted/image.png",
            source_revision=_digest(image_bytes),
            readiness="ready",
        ),
        _operation(
            "operation-0000000000000002",
            plan_id="asset-construction-plan",
            source_path="manifest.jsonl",
            source_revision=_digest(manifest_bytes),
            readiness="requires-editor-factory",
        ),
        _operation(
            "operation-0000000000000003",
            plan_id="asset-construction-plan",
            source_path="manifest.jsonl",
            source_revision=_digest(manifest_bytes),
            readiness="requires-editor-factory",
        ),
        _operation(
            "operation-0000000000000004",
            plan_id="asset-import-plan",
            source_path="fbx-assets/missing.fbx",
            source_revision="a" * 64,
            readiness="requires-conversion",
        ),
    )
    verified = FilesystemPlanSourceVerifier(repository, plan_root).verify(
        _bundle(*operations)
    )
    assert verified.report.verified_operation_count == 3
    assert verified.report.unique_source_count == 2
    assert verified.report.unique_source_bytes == len(image_bytes) + len(
        manifest_bytes
    )
    assert verified.report.skipped_conversion_count == 1
    assert verified.by_operation[operations[0].operation_id] == image
    assert verified.by_operation[operations[1].operation_id] == manifest
    assert verified.by_operation[operations[2].operation_id] == manifest
    assert operations[3].operation_id not in verified.by_operation
    assert str(repository) not in str(verified.report.to_json())


def test_rejects_digest_mismatch_without_disclosing_physical_path(
    tmp_path: Path,
) -> None:
    repository, plan_root = _roots(tmp_path)
    source = repository / "extracted" / "image.png"
    source.parent.mkdir()
    source.write_bytes(b"unexpected")
    operation = _operation(
        "operation-0000000000000005",
        plan_id="asset-import-plan",
        source_path="extracted/image.png",
        source_revision="0" * 64,
        readiness="ready",
    )
    with pytest.raises(ProtocolError, match="revision") as captured:
        FilesystemPlanSourceVerifier(repository, plan_root).verify(
            _bundle(operation)
        )
    assert str(repository) not in str(captured.value)
    assert "image.png" not in str(captured.value)


def test_rejects_linked_source_without_reading_external_bytes(
    tmp_path: Path,
) -> None:
    repository, plan_root = _roots(tmp_path)
    source = repository / "extracted" / "image.png"
    source.parent.mkdir()
    external = tmp_path / "external.png"
    external_bytes = b"external-source"
    external.write_bytes(external_bytes)
    _create_file_link(external, source)
    operation = _operation(
        "operation-0000000000000006",
        plan_id="asset-import-plan",
        source_path="extracted/image.png",
        source_revision=_digest(external_bytes),
        readiness="ready",
    )
    with pytest.raises(ProtocolError, match="link or reparse point"):
        FilesystemPlanSourceVerifier(repository, plan_root).verify(
            _bundle(operation)
        )
    assert external.read_bytes() == external_bytes


def test_rejects_noncanonical_construction_source(tmp_path: Path) -> None:
    repository, plan_root = _roots(tmp_path)
    operation = _operation(
        "operation-0000000000000007",
        plan_id="asset-construction-plan",
        source_path="other.jsonl",
        source_revision="0" * 64,
        readiness="requires-editor-factory",
    )
    with pytest.raises(ProtocolError, match="construction operation source"):
        FilesystemPlanSourceVerifier(repository, plan_root).verify(
            _bundle(operation)
        )
