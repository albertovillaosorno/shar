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
#   - Test generated skill tree test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test generated skill tree test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test generated skill tree test module."""

from __future__ import annotations

from pathlib import Path
import re

from mcp.adapter_outbound.skill_manual_field_schema import MANUAL_FIELDS

_REPOSITORY_ROOT = Path(__file__).resolve().parents[3]
_SKILL_ROOT = _REPOSITORY_ROOT / "skills" / "unreal"
_DIGEST_PATTERN = re.compile(r"Interface digest: `([0-9a-f]{64})`")
_REVISION_PATTERN = re.compile(
    r"(?:Manual review revision|Current revision): `([^`]+)`"
)
_CURRENT_COUNT_PATTERN = re.compile(r"Manual guidance current: \*\*(\d+)\*\*")
_REVIEW_COUNT_PATTERN = re.compile(
    r"Manual guidance review required: \*\*(\d+)\*\*"
)
_STATUS_PATTERN = re.compile(
    r"Manual guidance status: \*\*(Current|Review required)\*\*"
)
_LINK_PATTERN = re.compile(r"\[[^]]+\]\(([^)]+)\)")
_EXPECTED_TOOLSETS = 52
_EXPECTED_CAPABILITIES = 830
_EXPECTED_SKILL_FILES = 830
_EXPECTED_GENERATED_DOCUMENTS = 831
_REQUIRED_SKILL_SECTIONS = (
    "## What this tool does",
    "## When to use it",
    "## Technical execution posture",
    "## Human-authored guidance",
    "## Inputs",
    "## Invocation example",
    "## Expected output",
    "## Verification",
    "## Common failure modes",
)
_MAX_SKILL_LINES = 300
_MAX_PROSE_LINE_LENGTH = 80
_MARKDOWNLINT_MD013_NEXT_LINE = "<!-- markdownlint-disable-next-line MD013 -->"
_GENERAL_POLICY_TERMS = (
    "approval",
    "permission from the user",
    "permiso",
    "explícito",
    "usuario",
    " legal ",
    " illegal ",
    " ethical ",
    " moral ",
    "copyright",
    "trademark",
    "intellectual property",
    "propiedad",
    "intelectual",
    "proprietary",
    "confidential",
    "confidencial",
)


def test_generated_root_index_contains_every_live_identity() -> None:
    """The mandatory index lists all toolsets and capability identities."""
    index = (_SKILL_ROOT / "index.md").read_text(encoding="utf-8")

    assert "Toolsets: **52**" in index
    assert "Capabilities: **830**" in index
    assert index.count("#### `") == _EXPECTED_TOOLSETS
    assert index.count("- [`") == _EXPECTED_CAPABILITIES
    assert "Unreal MCP version: `1.0.0`" in index
    assert _CURRENT_COUNT_PATTERN.search(index) is not None
    assert _REVIEW_COUNT_PATTERN.search(index) is not None

    capability_names = _capability_names()
    assert len(capability_names) == _EXPECTED_CAPABILITIES
    assert all(f"`{name}`" in index for name in capability_names)


def test_manual_review_revision_and_index_counts_match_the_tree() -> None:
    """The index summarizes exact version-aware review state for every skill."""
    index = (_SKILL_ROOT / "index.md").read_text(encoding="utf-8")
    revision_match = _REVISION_PATTERN.search(index)
    current_match = _CURRENT_COUNT_PATTERN.search(index)
    review_match = _REVIEW_COUNT_PATTERN.search(index)
    assert revision_match is not None
    assert current_match is not None
    assert review_match is not None
    revision = revision_match.group(1)

    statuses: list[str] = []
    for path in _capability_documents():
        content = path.read_text(encoding="utf-8")
        revisions = _REVISION_PATTERN.findall(content)
        status_match = _STATUS_PATTERN.search(content)
        assert revisions == [revision]
        assert status_match is not None
        statuses.append(status_match.group(1))

    assert int(current_match.group(1)) == statuses.count("Current")
    assert int(review_match.group(1)) == statuses.count("Review required")
    assert len(statuses) == _EXPECTED_CAPABILITIES


def test_generated_tree_counts_and_page_bounds_are_stable() -> None:
    """Generated navigation and capability pages remain SRP bounded."""
    generated = [_SKILL_ROOT / "index.md", *_capability_documents()]
    skill_files = _capability_documents()
    nested_indexes = tuple(
        path for path in skill_files if path.name == "index.md"
    )
    expected_sample = (
        _SKILL_ROOT / "capabilities/automation/test/toolset/discover-tests.md"
    )

    assert len(generated) == _EXPECTED_GENERATED_DOCUMENTS
    assert len(skill_files) == _EXPECTED_SKILL_FILES
    assert expected_sample.is_file()
    assert not nested_indexes
    for path in generated:
        _assert_generated_line_policy(path)
    for path in skill_files:
        _assert_skill_quality(path)


def test_generated_tree_uses_one_digest_and_resolving_local_links() -> None:
    """Every generated document belongs to one snapshot and resolves locally."""
    documents = [_SKILL_ROOT / "index.md", *_capability_documents()]
    digests = {
        match.group(1)
        for path in documents
        for match in _DIGEST_PATTERN.finditer(path.read_text(encoding="utf-8"))
    }
    assert len(digests) == 1

    missing: list[str] = []
    for path in [*_manual_documents(), *documents]:
        text = path.read_text(encoding="utf-8")
        for match in _LINK_PATTERN.finditer(text):
            target = match.group(1)
            if target.startswith(("#", "http://", "https://")):
                continue
            resolved = (path.parent / target).resolve()
            if not resolved.exists():
                missing.append(
                    f"{path.relative_to(_REPOSITORY_ROOT)} -> {target}"
                )
    assert not missing, "\n".join(missing)


def _assert_skill_quality(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()
    assert len(lines) <= _MAX_SKILL_LINES
    assert all(section in text for section in _REQUIRED_SKILL_SECTIONS)
    normalized = f" {text.casefold()} "
    assert all(term not in normalized for term in _GENERAL_POLICY_TERMS)
    for field in MANUAL_FIELDS:
        assert text.count(f"<!-- BEGIN MANUAL FIELD: {field.key} -->") == 1
        assert text.count(f"<!-- END MANUAL FIELD: {field.key} -->") == 1
    assert _REVISION_PATTERN.search(text) is not None
    assert _STATUS_PATTERN.search(text) is not None


def _assert_generated_line_policy(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()
    in_code_block = False
    for index, line in enumerate(lines):
        if line.startswith("```"):
            in_code_block = not in_code_block
            continue
        assert line.rstrip() == line
        if in_code_block or len(line) <= _MAX_PROSE_LINE_LENGTH:
            continue
        assert index > 0
        assert lines[index - 1] == _MARKDOWNLINT_MD013_NEXT_LINE
    assert not in_code_block


def _capability_documents() -> tuple[Path, ...]:
    return tuple(sorted((_SKILL_ROOT / "capabilities").rglob("*.md")))


def _manual_documents() -> tuple[Path, ...]:
    return tuple(sorted((_SKILL_ROOT / "workflows").rglob("*.md")))


def _capability_names() -> tuple[str, ...]:
    names: list[str] = []
    for path in _capability_documents():
        lines = path.read_text(encoding="utf-8").splitlines()
        for index, line in enumerate(lines):
            if line != "Tool:" or index + 3 >= len(lines):
                continue
            if lines[index + 2] != "```text":
                continue
            names.append(lines[index + 3])
            break
    return tuple(names)
