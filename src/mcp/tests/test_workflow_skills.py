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
#   - Requires fourteen lifecycle runbooks with 120 lines and 600 words.
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
_WORKFLOW_MAP = _WORKFLOW_ROOT / "README.md"
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
    "connection/project-connection-setup.md": (
        "## Goal",
        "## Repository authorities",
        "## Bootstrap sequence",
        "## Drift detection",
        "## Stop conditions",
    ),
    "connection/editor-readiness.md": (
        "## Goal",
        "## Required project configuration",
        "## Phase 1: identify the intended editor",
        "## Connection failure decision tree",
        "## Stop conditions",
    ),
    "connection/server-and-registry-operations.md": (
        "## Goal",
        "## Operational states",
        "## Registry refresh boundary",
        "## Timeout and cancellation",
        "## Stop conditions",
    ),
    "planning/capability-selection.md": (
        "## Goal",
        "## Inputs",
        "## Native discovery and dispatch model",
        "## Selection procedure",
        "## Stop conditions",
    ),
    "planning/schema-and-arguments.md": (
        "## Goal",
        "## Authority order",
        "## Refresh the schema",
        "## Pre-invocation review",
        "## Stop conditions",
    ),
    "execution/read-only-operations.md": (
        "## Goal",
        "## Preparation",
        "## Query procedure",
        "## Evidence quality",
        "## Stop conditions",
    ),
    "execution/safe-mutations.md": (
        "## Goal",
        "## Change boundary",
        "## Pre-state capture",
        "## Mutation procedure",
        "## Stop conditions",
    ),
    "execution/long-running-and-batch-operations.md": (
        "## Goal",
        "## Batch admission checklist",
        "## Timeout handling",
        "## Partial failure",
        "## Stop conditions",
    ),
    "execution/programmatic-tool-scripts.md": (
        "## Goal",
        "## Admission criteria",
        "## Required environment discovery",
        "## Script design",
        "## Stop conditions",
    ),
    "assurance/verification-and-recovery.md": (
        "## Goal",
        "## Verification plan",
        "## Ambiguous outcomes",
        "## Recovery procedure",
        "## Stop conditions",
    ),
    "maintenance/manual-guidance-maintenance.md": (
        "## Goal",
        "## Ownership boundary",
        "## Field responsibilities",
        "## Evidence threshold",
        "## Stop conditions",
    ),
    "maintenance/regeneration-and-taxonomy.md": (
        "## Goal",
        "## Name-derived taxonomy",
        "## Manual workflow taxonomy",
        "## Atomic replacement sequence",
        "## Stop conditions",
    ),
    "extension/toolset-design-and-extension.md": (
        "## Goal",
        "## Discovery before design",
        "## API design principles",
        "## Test design",
        "## Stop conditions",
    ),
    "extension/agent-guidance-authoring.md": (
        "## Goal",
        "## Three distinct guidance surfaces",
        "## Routing decision",
        "## Testing and verification",
        "## Stop conditions",
    ),
}
_EXPECTED_FOLDERS = {
    "assurance",
    "connection",
    "execution",
    "extension",
    "maintenance",
    "planning",
}


def _relative_workflow_path(path: Path) -> str:
    return path.relative_to(_WORKFLOW_ROOT).as_posix()


def _read_workflow(relative_path: str) -> str:
    return (_WORKFLOW_ROOT / relative_path).read_text(encoding="utf-8")


def test_workflow_taxonomy_and_depth_are_complete() -> None:
    """Every workflow remains a detailed SRP operational runbook."""
    documents = tuple(sorted(_WORKFLOW_ROOT.rglob("*.md")))
    relative = {_relative_workflow_path(path) for path in documents}

    assert relative == {"README.md", *_REQUIRED}
    assert {
        path.parent.name for path in documents if path != _WORKFLOW_MAP
    } == _EXPECTED_FOLDERS
    assert not tuple(_WORKFLOW_ROOT.rglob("index.md"))
    assert tuple(_WORKFLOW_ROOT.rglob("README.md")) == (_WORKFLOW_MAP,)

    workflow_map = _WORKFLOW_MAP.read_text(encoding="utf-8")
    assert "# Unreal MCP workflow map" in workflow_map
    assert "## Taxonomy" in workflow_map
    assert "## Default operating route" in workflow_map
    for relative_path, headings in _REQUIRED.items():
        path = _WORKFLOW_ROOT / relative_path
        text = path.read_text(encoding="utf-8")
        assert "(../../index.md)" in text
        assert "(../README.md)" in text
        assert len(text.splitlines()) >= _MINIMUM_LINES
        assert len(text.split()) >= _MINIMUM_WORDS
        assert all(heading in text for heading in headings)
        assert f"({relative_path})" in workflow_map
        normalized = f" {text.casefold()} "
        assert all(term not in normalized for term in _GENERAL_POLICY_TERMS)


def test_workflows_define_the_versioned_manual_review_contract() -> None:
    """Manual review status remains fail-safe across workflow guidance."""
    manual = _read_workflow("maintenance/manual-guidance-maintenance.md")
    regeneration = _read_workflow("maintenance/regeneration-and-taxonomy.md")
    verification = _read_workflow("assurance/verification-and-recovery.md")

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

    assert "(workflows/README.md)" in index
    for relative_path in _REQUIRED:
        assert f"(workflows/{relative_path})" in index
    for group in (
        "### Connection and session",
        "### Planning",
        "### Execution",
        "### Assurance",
        "### Maintenance",
        "### Extension",
    ):
        assert group in index
