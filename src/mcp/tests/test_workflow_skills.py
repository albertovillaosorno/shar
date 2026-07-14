# File:
#   - test_workflow_skills.py
# Path:
#   - src/mcp/tests/test_workflow_skills.py
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
#   - Structural and routing guards for manual Unreal MCP workflow skills.
# - Must-Not:
#   - Connect to Unreal, validate generated capability prose, or edit files.
# - Allows:
#   - Checking exact workflow names, required sections, depth, and index links.
# - Split-When:
#   - Workflow prose quality and generated routing require separate fixtures.
# - Merge-When:
#   - Another test module owns the complete workflow-skill contract.
# - Summary:
#   - Prevents Unreal MCP workflows from degrading into basic notes.
# - Description:
#   - Requires detailed SRP runbooks and complete central-index routing.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Requires ten workflows with at least 120 lines and 600 words each.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: manual Unreal MCP workflow regression guards
#   - reason: exact workflow taxonomy and required sections share one contract
#   - split: separate content-depth checks if workflow count grows materially
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess after workflow taxonomy or operator policy changes
#
"""Regression guards for detailed manual Unreal MCP workflow skills."""

from __future__ import annotations

from pathlib import Path

from mcp.src.adapters.driven.skill_markdown_renderer import (
    MarkdownSkillRenderer,
)

from tests.skill_catalog_fixture import (
    TEST_UNREAL_MCP_VERSION,
    complete_catalog,
)

_REPOSITORY_ROOT = Path(__file__).resolve().parents[3]
_WORKFLOW_ROOT = _REPOSITORY_ROOT / "skills" / "unreal" / "workflows"
_MINIMUM_LINES = 120
_MINIMUM_WORDS = 600
_GENERAL_POLICY_TERMS = (
    "approval",
    "permission",
    " legal ",
    " illegal ",
    " ethical ",
    " moral ",
    "copyright",
    "trademark",
    "intellectual property",
    "proprietary",
    "confidential",
)
_REQUIRED = {
    "capability-selection.md": (
        "## Goal",
        "## Inputs",
        "## Selection procedure",
        "## Capability preference order",
        "## Stop conditions",
    ),
    "editor-readiness.md": (
        "## Goal",
        "## Required project configuration",
        "## Phase 1: identify the intended editor",
        "## Connection failure decision tree",
        "## Stop conditions",
    ),
    "long-running-and-batch-operations.md": (
        "## Goal",
        "## Batch admission checklist",
        "## Timeout handling",
        "## Partial failure",
        "## Stop conditions",
    ),
    "manual-guidance-maintenance.md": (
        "## Goal",
        "## Ownership boundary",
        "## Field responsibilities",
        "## Evidence threshold",
        "## Stop conditions",
    ),
    "programmatic-tool-scripts.md": (
        "## Goal",
        "## Admission criteria",
        "## Required environment discovery",
        "## Script design",
        "## Change boundary",
        "## Stop conditions",
    ),
    "read-only-operations.md": (
        "## Goal",
        "## Preparation",
        "## Query procedure",
        "## Evidence quality",
        "## Stop conditions",
    ),
    "regeneration-and-taxonomy.md": (
        "## Goal",
        "## Name-derived taxonomy",
        "## Identity-based manual-field migration",
        "## Atomic replacement sequence",
        "## Stop conditions",
    ),
    "safe-mutations.md": (
        "## Goal",
        "## Change boundary",
        "## Pre-state capture",
        "## Mutation procedure",
        "## Stop conditions",
    ),
    "schema-and-arguments.md": (
        "## Goal",
        "## Authority order",
        "## Refresh the schema",
        "## Pre-invocation review",
        "## Stop conditions",
    ),
    "verification-and-recovery.md": (
        "## Goal",
        "## Verification plan",
        "## Ambiguous outcomes",
        "## Recovery procedure",
        "## Stop conditions",
    ),
}


def test_workflow_taxonomy_and_depth_are_complete() -> None:
    """Every workflow remains a detailed SRP operational runbook."""
    paths = tuple(sorted(_WORKFLOW_ROOT.glob("*.md")))

    assert {path.name for path in paths} == set(_REQUIRED)
    for path in paths:
        text = path.read_text(encoding="utf-8")
        assert "[`../index.md`](../index.md)" in text
        assert len(text.splitlines()) >= _MINIMUM_LINES
        assert len(text.split()) >= _MINIMUM_WORDS
        assert all(heading in text for heading in _REQUIRED[path.name])
        normalized = f" {text.casefold()} "
        assert all(term not in normalized for term in _GENERAL_POLICY_TERMS)


def test_workflows_define_the_versioned_manual_review_contract() -> None:
    """Manual review status remains fail-safe across workflow guidance."""
    manual = (_WORKFLOW_ROOT / "manual-guidance-maintenance.md").read_text(
        encoding="utf-8"
    )
    regeneration = (_WORKFLOW_ROOT / "regeneration-and-taxonomy.md").read_text(
        encoding="utf-8"
    )
    verification = (_WORKFLOW_ROOT / "verification-and-recovery.md").read_text(
        encoding="utf-8"
    )

    for phrase in (
        "manual-review-revision",
        "[REVIEW_REQUIRED]",
        "Current revision",
        "Review required",
    ):
        assert phrase in manual
    for phrase in (
        "<unreal-mcp-version>/<interface-digest>",
        "legacy five-field schema",
        "review counts",
    ):
        assert phrase in regeneration
    assert "update `manual-review-revision` only when every" in verification


def test_generated_index_routes_every_manual_workflow() -> None:
    """The mandatory central index links every manual workflow skill."""
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
        complete_catalog()
    )
    index = next(
        document.content
        for document in documents
        if document.relative_path == "index.md"
    )

    for filename in _REQUIRED:
        assert f"(workflows/{filename})" in index
