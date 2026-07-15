# File:
#   - test_skill_store.py
# Path:
#   - src/mcp/tests/test_skill_store.py
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
#   - Regression tests for generated skill storage and export orchestration.
# - Must-Not:
#   - Connect to Unreal, test taxonomy routing, or modify repository skills.
# - Allows:
#   - Temporary skill roots and a synthetic catalog source.
# - Split-When:
#   - Filesystem and application orchestration need different fixtures.
# - Merge-When:
#   - Another test module owns the same persistence contract.
# - Summary:
#   - Guards safe generated skill replacement and export reporting.
# - Description:
#   - Proves manual workflow files survive complete generated replacement.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Uses the complete synthetic 52-toolset catalog.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: generated Unreal skill persistence regression tests
#   - reason: storage replacement and export reporting share one temp root
#   - split: separate application orchestration if another store is introduced
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess after store ownership or export reporting changes
#
"""Regression tests for generated Unreal MCP skill persistence."""

from __future__ import annotations

from typing import TYPE_CHECKING, cast

import pytest
from mcp.src.adapters.driven.filesystem_skill_store import FilesystemSkillStore
from mcp.src.adapters.driven.skill_manual_review import (
    MANUAL_REVIEW_PLACEHOLDER,
)
from mcp.src.adapters.driven.skill_markdown_renderer import (
    MarkdownSkillRenderer,
)
from mcp.src.application.skill_export import UnrealSkillExporter
from mcp.src.domain.errors import ProtocolError
from mcp.src.domain.skill_documents import interface_digest
from mcp.src.domain.skill_revision import build_skill_revision

from tests.skill_catalog_fixture import (
    EXPECTED_CATEGORY_COUNT,
    EXPECTED_DOCUMENT_COUNT,
    EXPECTED_TOOLSET_COUNT,
    TEST_UNREAL_MCP_VERSION,
    complete_catalog,
)

if TYPE_CHECKING:
    from pathlib import Path

    from mcp.src.application.service import UnrealMcpTranslator
    from mcp.src.domain.catalog import ToolsetDefinition


class _SyntheticTranslator:
    def __init__(self, catalog: tuple[ToolsetDefinition, ...]) -> None:
        self._catalog = catalog

    def discover_catalog(self) -> tuple[ToolsetDefinition, ...]:
        return self._catalog


def test_filesystem_store_preserves_manual_skills_and_removes_stale_files(
    tmp_path: Path,
) -> None:
    """Replacement owns only index.md and capabilities Markdown."""
    output_root = tmp_path / "skills" / "unreal"
    workflow = output_root / "workflows" / "manual.md"
    readme = output_root / "README.md"
    stale = output_root / "capabilities" / "stale" / "old.md"
    workflow.parent.mkdir(parents=True)
    stale.parent.mkdir(parents=True)
    _ = workflow.write_text("manual workflow\n", encoding="utf-8")
    _ = readme.write_text("manual readme\n", encoding="utf-8")
    _ = stale.write_text("stale generated file\n", encoding="utf-8")

    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
        complete_catalog()
    )
    FilesystemSkillStore(output_root).replace(documents)

    assert workflow.read_text(encoding="utf-8") == "manual workflow\n"
    assert readme.read_text(encoding="utf-8") == "manual readme\n"
    assert not stale.exists()
    assert (output_root / "index.md").is_file()
    generated = tuple((output_root / "capabilities").rglob("*.md"))
    assert len(generated) == EXPECTED_DOCUMENT_COUNT - 1
    assert not tuple(output_root.rglob("*.tmp"))
    index = (output_root / "index.md").read_text(encoding="utf-8")
    assert "Manual guidance current: **0**" in index
    assert "Manual guidance review required: **52**" in index


def test_filesystem_store_preserves_manual_fields_during_live_refresh(
    tmp_path: Path,
) -> None:
    """A refreshed generated shell retains exact human-authored field values."""
    output_root = tmp_path / "skills" / "unreal"
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
        complete_catalog()
    )
    store = FilesystemSkillStore(output_root)
    store.replace(documents)
    capability = next(
        document
        for document in documents
        if document.relative_path.startswith("capabilities/")
    )
    target = output_root.joinpath(*capability.relative_path.split("/"))
    existing = target.read_text(encoding="utf-8")
    revision = build_skill_revision(
        TEST_UNREAL_MCP_VERSION,
        interface_digest(complete_catalog()),
    ).token
    existing = existing.replace(
        MANUAL_REVIEW_PLACEHOLDER,
        revision,
        1,
    )
    existing = existing.replace("[TODO]", "Use after deterministic import.", 1)
    existing = existing.replace(
        "[FILL_ME]",
        '```json\n{"asset": "/Game/Test"}\n```',
        1,
    )
    _ = target.write_text(existing, encoding="utf-8", newline="\n")
    refreshed_documents = tuple(
        document._replace(
            content=document.content.replace(
                "Generated from live MCP metadata",
                "Generated from refreshed live MCP metadata",
            )
        )
        for document in documents
    )

    store.replace(refreshed_documents)

    refreshed = target.read_text(encoding="utf-8")
    assert "Generated from refreshed live MCP metadata" in refreshed
    assert "Use after deterministic import." in refreshed
    assert '```json\n{"asset": "/Game/Test"}\n```' in refreshed
    assert "- Manual guidance status: **Current**" in refreshed
    index = (output_root / "index.md").read_text(encoding="utf-8")
    assert "Manual guidance current: **1**" in index
    assert "Manual guidance review required: **51**" in index


def test_filesystem_store_migrates_manual_fields_by_native_identity(
    tmp_path: Path,
) -> None:
    """A taxonomy path move preserves exact protected human guidance."""
    output_root = tmp_path / "skills" / "unreal"
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
        complete_catalog()
    )
    store = FilesystemSkillStore(output_root)
    store.replace(documents)
    capability = next(
        document
        for document in documents
        if document.relative_path.startswith("capabilities/")
    )
    original = output_root.joinpath(*capability.relative_path.split("/"))
    authored = original.read_text(encoding="utf-8").replace(
        "[TODO]",
        "Preserve this project-specific use case.",
        1,
    )
    _ = original.write_text(authored, encoding="utf-8")
    moved_path = "capabilities/migrated/example-tool.md"
    moved_documents = tuple(
        document._replace(relative_path=moved_path)
        if document == capability
        else document
        for document in documents
    )

    store.replace(moved_documents)

    moved = output_root.joinpath(*moved_path.split("/"))
    assert not original.exists()
    assert moved.is_file()
    assert "Preserve this project-specific use case." in moved.read_text(
        encoding="utf-8"
    )


def test_filesystem_store_rejects_non_markdown_capability_path(
    tmp_path: Path,
) -> None:
    """Generated capability documents must remain Markdown files."""
    output_root = tmp_path / "skills" / "unreal"
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
        complete_catalog()
    )
    capability = next(
        document
        for document in documents
        if document.relative_path.startswith("capabilities/")
    )
    invalid_documents = tuple(
        document._replace(relative_path="capabilities/generated/tool.txt")
        if document == capability
        else document
        for document in documents
    )

    with pytest.raises(ProtocolError, match="must be Markdown"):
        FilesystemSkillStore(output_root).replace(invalid_documents)

    assert not output_root.exists()


def test_filesystem_store_rejects_windows_separator_traversal(
    tmp_path: Path,
) -> None:
    """Backslash traversal cannot escape the generated capabilities root."""
    output_root = tmp_path / "skills" / "unreal"
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
        complete_catalog()
    )
    capability = next(
        document
        for document in documents
        if document.relative_path.startswith("capabilities/")
    )
    invalid_documents = tuple(
        document._replace(relative_path=r"capabilities/..\outside.md")
        if document == capability
        else document
        for document in documents
    )

    with pytest.raises(ProtocolError, match="unsafe path separator"):
        FilesystemSkillStore(output_root).replace(invalid_documents)

    assert not output_root.exists()


def test_filesystem_store_rejects_nonportable_windows_segments(
    tmp_path: Path,
) -> None:
    """Windows aliases and device names fail before generated writes."""
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
        complete_catalog()
    )
    capability = next(
        document
        for document in documents
        if document.relative_path.startswith("capabilities/")
    )
    invalid_paths = (
        "capabilities/C:outside.md",
        "capabilities/con.md",
        "capabilities/name:.md",
    )

    for index, invalid_path in enumerate(invalid_paths):
        output_root = tmp_path / f"unreal-{index}"
        invalid_documents = tuple(
            document._replace(relative_path=invalid_path)
            if document == capability
            else document
            for document in documents
        )

        with pytest.raises(ProtocolError, match="not portable"):
            FilesystemSkillStore(output_root).replace(invalid_documents)

        assert not output_root.exists()


def test_filesystem_store_path_error_does_not_reflect_controls(
    tmp_path: Path,
) -> None:
    """Unsafe generated paths cannot add lines to diagnostics."""
    output_root = tmp_path / "skills" / "unreal"
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
        complete_catalog()
    )
    capability = next(
        document
        for document in documents
        if document.relative_path.startswith("capabilities/")
    )
    invalid_documents = tuple(
        document._replace(relative_path="capabilities/bad\ninjected.md")
        if document == capability
        else document
        for document in documents
    )

    with pytest.raises(ProtocolError) as caught:
        FilesystemSkillStore(output_root).replace(invalid_documents)

    assert str(caught.value) == "generated skill path is not portable"
    assert not output_root.exists()


def test_filesystem_store_rejects_nested_index_before_mutation(
    tmp_path: Path,
) -> None:
    """Only the central generated index may use the reserved filename."""
    output_root = tmp_path / "skills" / "unreal"
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
        complete_catalog()
    )
    capability = next(
        document
        for document in documents
        if document.relative_path.startswith("capabilities/")
    )
    invalid_documents = tuple(
        document._replace(relative_path="capabilities/generated/index.md")
        if document == capability
        else document
        for document in documents
    )

    with pytest.raises(ProtocolError, match="reserved central index"):
        FilesystemSkillStore(output_root).replace(invalid_documents)

    assert not output_root.exists()


def test_filesystem_store_rejects_invalid_utf8_before_mutation(
    tmp_path: Path,
) -> None:
    """Unreadable generated skills must not trigger cleanup or replacement."""
    output_root = tmp_path / "skills" / "unreal"
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
        complete_catalog()
    )
    store = FilesystemSkillStore(output_root)
    store.replace(documents)
    capability = next(
        document
        for document in documents
        if document.relative_path.startswith("capabilities/")
    )
    target = output_root.joinpath(*capability.relative_path.split("/"))
    invalid_content = b"\xff"
    _ = target.write_bytes(invalid_content)
    stale = output_root / "capabilities" / "removed" / "stale.md"
    stale.parent.mkdir(parents=True)
    _ = stale.write_text("stale generated file\n", encoding="utf-8")
    original_index = (output_root / "index.md").read_text(encoding="utf-8")

    with pytest.raises(ProtocolError, match="not valid UTF-8"):
        store.replace(documents)

    assert stale.is_file()
    assert (output_root / "index.md").read_text(
        encoding="utf-8"
    ) == original_index
    assert target.read_bytes() == invalid_content


def test_filesystem_store_rejects_malformed_manual_fields_before_mutation(
    tmp_path: Path,
) -> None:
    """Invalid protected markers stop cleanup and all generated writes."""
    output_root = tmp_path / "skills" / "unreal"
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
        complete_catalog()
    )
    store = FilesystemSkillStore(output_root)
    store.replace(documents)
    capability = next(
        document
        for document in documents
        if document.relative_path.startswith("capabilities/")
    )
    target = output_root.joinpath(*capability.relative_path.split("/"))
    malformed = target.read_text(encoding="utf-8").replace(
        "<!-- END MANUAL FIELD: known-caveats -->",
        "",
    )
    _ = target.write_text(malformed, encoding="utf-8", newline="\n")
    stale = output_root / "capabilities" / "removed" / "stale.md"
    stale.parent.mkdir(parents=True)
    _ = stale.write_text("stale generated file\n", encoding="utf-8")
    original_index = (output_root / "index.md").read_text(encoding="utf-8")

    with pytest.raises(ProtocolError, match="markers are out of order"):
        store.replace(documents)

    assert stale.is_file()
    assert (output_root / "index.md").read_text(
        encoding="utf-8"
    ) == original_index
    assert target.read_text(encoding="utf-8") == malformed


def test_exporter_reports_complete_persisted_document_set(
    tmp_path: Path,
) -> None:
    """Application orchestration reports the exact completed export."""
    catalog = complete_catalog()
    translator = cast("UnrealMcpTranslator", _SyntheticTranslator(catalog))
    output_root = tmp_path / "skills" / "unreal"

    report = UnrealSkillExporter(
        translator,
        MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION),
        FilesystemSkillStore(output_root),
    ).export()

    assert report.category_count == EXPECTED_CATEGORY_COUNT
    assert report.document_count == EXPECTED_DOCUMENT_COUNT
    assert report.toolset_count == EXPECTED_TOOLSET_COUNT
    assert report.tool_count == EXPECTED_TOOLSET_COUNT
    assert len(report.interface_digest) == 64
    assert report.output_path == output_root.as_posix()
