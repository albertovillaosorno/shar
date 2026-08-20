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
#   - Generated Unreal plan bundle preflight tests.
# - Must-Not:
#   - Contact a live MCP endpoint or mutate Unreal Editor.
# - Allows:
#   - Exercise domain, filesystem, and local CLI intake boundaries.
# - Split-When:
#   - Split when domain and filesystem suites gain separate lifecycles.
# - Merge-When:
#   - Merge when plan preflight has no independent test boundary.
# - Summary:
#   - Unreal plan bundle preflight tests.
# - Description:
#   - Proves exact revisions, inventory, link safety, and local execution.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Invalid evidence fails before any MCP transport is constructed.
#

"""Tests for generated Unreal plan bundle preflight."""

from __future__ import annotations

import json
import os
from pathlib import Path
import re

from mcp.adapter_inbound.arguments import parse_plan_root
from mcp.adapter_inbound.cli import main
from mcp.adapter_outbound.plan_bundle_reader import FilesystemPlanBundleReader
from mcp.domain.errors import ProtocolError
from mcp.domain.plan_bundle import parse_plan_bundle
from mcp.domain.plan_bundle import validate_plan_bundle
from plan_bundle_fixture import build_plan_bundle
from plan_bundle_fixture import write_plan_bundle
import pytest


def _split_bundle(
    *,
    with_import_operation: bool = False,
    with_construction_operation: bool = False,
    construction_source_path: str = "manifest.jsonl",
    construction_source_revision: str | None = None,
    semantic_blocker_count: int = 0,
) -> tuple[str, dict[str, str]]:
    files = build_plan_bundle(
        with_import_operation=with_import_operation,
        with_construction_operation=with_construction_operation,
        construction_source_path=construction_source_path,
        construction_source_revision=construction_source_revision,
        semantic_blocker_count=semantic_blocker_count,
    )
    index = files.pop("index.json")
    return index, files


def _create_file_link(target: Path, link: Path) -> None:
    try:
        link.symlink_to(target)
    except OSError as error:
        if os.name == "nt" and getattr(error, "winerror", None) == 1314:
            pytest.skip("file symlinks require Windows developer mode")
        raise


def test_default_plan_root_uses_generated_cache() -> None:
    assert parse_plan_root(()) == Path(".cache/pipeline/unreal-staging/plans")


def test_domain_accepts_canonical_six_plan_bundle() -> None:
    index, plans = _split_bundle()
    report = validate_plan_bundle(index, plans)
    assert report.operation_count == 0
    assert report.readiness_counts == {}
    assert report.target_engine_version == "5.8.1"
    assert report.target_platform == "editor"
    assert len(report.plans) == 6
    assert report.to_json()["revision"] == report.revision


def test_domain_preserves_semantic_blockers_in_bundle_identity() -> None:
    baseline_index, baseline_plans = _split_bundle()
    blocked_index, blocked_plans = _split_bundle(semantic_blocker_count=3)
    baseline = parse_plan_bundle(baseline_index, baseline_plans)
    blocked = parse_plan_bundle(blocked_index, blocked_plans)
    assert baseline.report.semantic_blocker_count == 0
    assert blocked.report.semantic_blocker_count == 3
    assert blocked.report.to_json()["semanticBlockerCount"] == 3
    assert blocked.report.revision != baseline.report.revision


def test_domain_validates_nonempty_operation_identity_and_readiness() -> None:
    index, plans = _split_bundle(with_import_operation=True)
    bundle = parse_plan_bundle(index, plans)
    report = bundle.report
    assert report.operation_count == 1
    assert report.readiness_counts == {"ready": 1}
    assert len(bundle.operations) == 1
    operation = bundle.operations[0]
    assert operation.plan_id == "asset-import-plan"
    assert operation.source_path == "extracted/dialog/audio.wav"
    assert operation.destination.endswith(".audio_source")
    assert operation.dependencies == ()
    assert operation.readiness == "ready"

    invalid_identity = dict(plans)
    invalid_identity["asset-import-plan.json"] = invalid_identity[
        "asset-import-plan.json"
    ].replace('"operation_id":"operation-', '"operation_id":"operation-0', 1)
    with pytest.raises(ProtocolError, match="identity"):
        validate_plan_bundle(index, invalid_identity)

    invalid_readiness = dict(plans)
    invalid_readiness["asset-import-plan.json"] = invalid_readiness[
        "asset-import-plan.json"
    ].replace('"readiness":"ready"', '"readiness":"requires-conversion"', 1)
    with pytest.raises(ProtocolError, match="source contract"):
        validate_plan_bundle(index, invalid_readiness)


def test_domain_requires_canonical_construction_manifest_evidence() -> None:
    index, plans = _split_bundle(with_construction_operation=True)
    bundle = parse_plan_bundle(index, plans)
    assert len(bundle.operations) == 1
    operation = bundle.operations[0]
    assert operation.plan_id == "asset-construction-plan"
    assert operation.source_path == "manifest.jsonl"
    assert operation.source_revision == bundle.report.source_manifest_revision

    invalid_bundles = (
        _split_bundle(
            with_construction_operation=True,
            construction_source_path="other.jsonl",
        ),
        _split_bundle(
            with_construction_operation=True,
            construction_source_revision="d" * 64,
        ),
    )
    for invalid_index, invalid_plans in invalid_bundles:
        with pytest.raises(ProtocolError, match="source evidence"):
            parse_plan_bundle(invalid_index, invalid_plans)


@pytest.mark.parametrize(
    ("mutation", "message"),
    [
        ("revision", "revision does not match"),
        ("whitespace", "JSON is not canonical"),
        ("engine", "unsupported Unreal environment"),
        ("extra-field", "fields are not canonical"),
        ("missing-plan", "file inventory is not exact"),
    ],
)
def test_domain_rejects_stale_partial_or_noncanonical_bundle(
    mutation: str,
    message: str,
) -> None:
    index, plans = _split_bundle()
    if mutation == "revision":
        match = re.search(r'"revision":"([0-9a-f])', index)
        assert match is not None
        replacement = "0" if match.group(1) != "0" else "1"
        index = f"{index[: match.start(1)]}{replacement}{index[match.end(1) :]}"
    elif mutation == "whitespace":
        index = index.replace('{"schema"', '{ "schema"', 1)
    elif mutation == "engine":
        index = index.replace(
            '"target_engine_version":"5.8.1"', '"target_engine_version":"5.8.0"'
        )
    elif mutation == "extra-field":
        index = index.replace('{"schema"', '{"unexpected":true,"schema"', 1)
    elif mutation == "missing-plan":
        del plans["package-plan.json"]
    with pytest.raises(ProtocolError, match=message):
        validate_plan_bundle(index, plans)


def test_reader_accepts_exact_regular_file_inventory(tmp_path: Path) -> None:
    root = tmp_path / "plans"
    _ = write_plan_bundle(root)
    reader = FilesystemPlanBundleReader(root)
    bundle = reader.read_bundle()
    assert bundle.report.operation_count == 0
    assert bundle.report.revision
    assert bundle.operations == ()
    assert reader.read() == bundle.report


def test_reader_rejects_extra_file_without_exposing_physical_root(
    tmp_path: Path,
) -> None:
    root = tmp_path / "plans"
    _ = write_plan_bundle(root)
    _ = (root / "unexpected.json").write_text("{}\n", encoding="utf-8")
    with pytest.raises(
        ProtocolError, match="inventory is not exact"
    ) as captured:
        FilesystemPlanBundleReader(root).read()
    assert str(root) not in str(captured.value)


def test_reader_rejects_linked_plan_file(tmp_path: Path) -> None:
    root = tmp_path / "plans"
    files = write_plan_bundle(root)
    plan = root / "package-plan.json"
    plan.unlink()
    external = tmp_path / "external-plan.json"
    _ = external.write_text(files["package-plan.json"], encoding="utf-8")
    _create_file_link(external, plan)
    with pytest.raises(ProtocolError, match="link or reparse point"):
        FilesystemPlanBundleReader(root).read()
    assert external.read_text(encoding="utf-8") == files["package-plan.json"]


def test_cli_preflight_is_local_and_opens_no_mcp_transport(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    root = tmp_path / "plans"
    _ = write_plan_bundle(root)
    monkeypatch.chdir(tmp_path)

    failure = "plan preflight opened an MCP transport"

    class _ForbiddenTransport:
        def __init__(self, *_args: object, **_kwargs: object) -> None:
            raise AssertionError(failure)

    monkeypatch.setattr(
        "mcp.adapter_inbound.cli.StreamableHttpTransport",
        _ForbiddenTransport,
    )
    assert main(("plan-preflight", "--root", "plans")) == 0
    captured = capsys.readouterr()
    payload = json.loads(captured.out)
    assert payload["operationCount"] == 0
    assert payload["targetEngineVersion"] == "5.8.1"
    assert not captured.err


def test_cli_execution_preflight_is_local_and_complete_for_empty_bundle(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    root = tmp_path / "plans"
    _ = write_plan_bundle(root)
    monkeypatch.chdir(tmp_path)

    failure = "execution preflight opened an MCP transport"

    class _ForbiddenTransport:
        def __init__(self, *_args: object, **_kwargs: object) -> None:
            raise AssertionError(failure)

    monkeypatch.setattr(
        "mcp.adapter_inbound.cli.StreamableHttpTransport",
        _ForbiddenTransport,
    )
    assert main(("plan-execution-preflight", "--root", "plans")) == 0
    captured = capsys.readouterr()
    payload = json.loads(captured.out)
    assert payload["execution"]["complete"] is True
    assert payload["execution"]["operationCount"] == 0
    assert payload["sources"]["uniqueSourceCount"] == 0
    assert not captured.err


def test_cli_rejects_unsafe_plan_root_before_io(
    capsys: pytest.CaptureFixture[str],
) -> None:
    assert main(("plan-preflight", "--root", "../outside")) == 2
    captured = capsys.readouterr()
    assert "repository-relative child path" in captured.err
    assert not captured.out
