# File:
#   - test_skill_rendering.py
# Path:
#   - src/mcp/tests/test_skill_rendering.py
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
#   - Regression tests for taxonomy coverage and generated skill Markdown.
# - Must-Not:
#   - Access files, connect to Unreal, or test persistence behavior.
# - Allows:
#   - Synthetic complete catalogs and fail-closed drift fixtures.
# - Split-When:
#   - Taxonomy and Markdown rendering need independent fixture families.
# - Merge-When:
#   - Another test module owns the same pure generation behavior.
# - Summary:
#   - Guards complete Unreal MCP capability routing and rendering.
# - Description:
#   - Verifies capability identities appear without native prose.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Uses one synthetic capability per known live toolset.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: generated skill taxonomy and rendering tests
#   - reason: taxonomy coverage and root-index evidence share one pure fixture
#   - split: separate taxonomy tests if another renderer is introduced
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess after taxonomy or Markdown schema changes
#
"""Regression tests for generated Unreal MCP skill Markdown."""

from __future__ import annotations

from typing import TYPE_CHECKING

import pytest
from mcp.src.adapters.driven.catalog_renderer import render_catalog_markdown
from mcp.src.adapters.driven.skill_document_layout import tool_skill_path
from mcp.src.adapters.driven.skill_manual_field_schema import MANUAL_FIELDS
from mcp.src.adapters.driven.skill_manual_review import (
    MANUAL_REVIEW_PLACEHOLDER,
)
from mcp.src.adapters.driven.skill_markdown_policy import (
    render_unbreakable_line,
)
from mcp.src.adapters.driven.skill_markdown_renderer import (
    MarkdownSkillRenderer,
)
from mcp.src.adapters.driven.skill_schema_renderer import render_inputs
from mcp.src.domain.errors import ProtocolError
from mcp.src.domain.skill_taxonomy import (
    CATEGORIES,
    category_for_toolset,
    known_toolset_names,
)

from tests.skill_catalog_fixture import (
    EXPECTED_CATEGORY_COUNT,
    EXPECTED_DOCUMENT_COUNT,
    EXPECTED_TOOLSET_COUNT,
    SENTINEL_NATIVE_PROSE,
    TEST_UNREAL_MCP_VERSION,
    complete_catalog,
    toolset,
)

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject


def test_taxonomy_owns_every_known_toolset_once() -> None:
    """The fixed taxonomy covers all 52 live identities without fallback."""
    names = known_toolset_names()

    assert len(names) == EXPECTED_TOOLSET_COUNT
    assert len(CATEGORIES) == EXPECTED_CATEGORY_COUNT
    category_counts = {
        category.slug: sum(
            category_for_toolset(name) == category for name in names
        )
        for category in CATEGORIES
    }
    assert category_counts == {
        "core-and-governance": 8,
        "assets-and-data": 13,
        "world-and-ui": 4,
        "animation-and-cinematics": 8,
        "gameplay-and-ai": 10,
        "effects-physics-and-procedural": 9,
    }

    with pytest.raises(ProtocolError, match="lacks skill taxonomy ownership"):
        _ = category_for_toolset("FuturePlugin.UnownedToolset")


def test_unbreakable_generated_lines_use_one_exact_narrow_guard() -> None:
    """Only generated lines beyond 80 columns receive the MD013 marker."""
    short_line = "x" * 80
    long_line = "x" * 81

    assert render_unbreakable_line(short_line) == (short_line,)
    assert render_unbreakable_line(long_line) == (
        "<!-- markdownlint-disable-next-line MD013 -->",
        long_line,
    )


def test_renderer_creates_one_explanatory_skill_per_capability() -> None:
    """One root index routes every synthetic per-tool skill."""
    catalog = complete_catalog()
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(catalog)
    by_path = {
        document.relative_path: document.content for document in documents
    }

    assert len(documents) == EXPECTED_DOCUMENT_COUNT
    assert len(by_path) == EXPECTED_DOCUMENT_COUNT
    root_index = by_path["index.md"]
    assert "Toolsets: **52**" in root_index
    assert "Capabilities: **52**" in root_index
    assert "Unreal MCP version: `1.0.0`" in root_index
    assert "Manual review revision: `1.0.0/" in root_index
    assert "Manual guidance current: **0**" in root_index
    assert "Manual guidance review required: **52**" in root_index
    assert not any(
        line.startswith(tuple(f"{number}." for number in range(2, 10)))
        for line in root_index.splitlines()
    )
    for toolset_definition in catalog:
        tool = toolset_definition.tools[0]
        capability_path = tool_skill_path(toolset_definition, tool)
        assert toolset_definition.name in root_index
        assert tool.name in root_index
        assert capability_path in by_path
        skill = by_path[capability_path]
        assert tool.name in skill
        assert "## What this tool does" in skill
        assert "## Human-authored guidance" in skill
        assert "## Invocation example" in skill
        assert "## Verification" in skill
        assert skill.count("[TODO]") == 4
        assert skill.count("[FILL_ME]") == 1
        assert skill.count(MANUAL_REVIEW_PLACEHOLDER) == 1
        assert "- Current revision: `1.0.0/" in skill
        assert "- Manual guidance status: **Review required**" in skill
        for field in MANUAL_FIELDS:
            assert skill.count(f"<!-- BEGIN MANUAL FIELD: {field.key} -->") == 1
            assert skill.count(f"<!-- END MANUAL FIELD: {field.key} -->") == 1
    assert SENTINEL_NATIVE_PROSE in "".join(by_path.values())


def test_schema_prose_cannot_create_accidental_markdown_lists() -> None:
    """Inline schema delimiters remain prose after deterministic wrapping."""
    definition = toolset("AutomationTestToolset.AutomationTestToolset")
    tool = definition.tools[0]
    input_schema: JsonObject = {
        "properties": {
            "value": {
                "description": (
                    "TArray<Item> format: - float: 1.0 - name: MyValue"
                ),
                "type": "string",
            }
        },
        "required": ["value"],
        "type": "object",
    }

    lines = render_inputs(tool._replace(input_schema=input_schema), {})

    assert not any(line.startswith("- float") for line in lines)
    assert "TArray&lt;Item&gt;" in " ".join(lines)
    assert "; float: 1.0; name: MyValue" in " ".join(lines)


def test_schema_prose_rejects_control_characters() -> None:
    """Schema descriptions cannot carry hidden controls into Markdown."""
    definition = toolset("AutomationTestToolset.AutomationTestToolset")
    tool = definition.tools[0]
    input_schema: JsonObject = {
        "properties": {
            "value": {
                "description": "Reads one value.\x00Injected text.",
                "type": "string",
            }
        },
        "type": "object",
    }

    with pytest.raises(ProtocolError, match="schema prose contains controls"):
        _ = render_inputs(tool._replace(input_schema=input_schema), {})


def test_schema_inline_text_rejects_control_characters() -> None:
    """Inline schema labels cannot carry hidden controls into Markdown."""
    definition = toolset("AutomationTestToolset.AutomationTestToolset")
    tool = definition.tools[0]
    schemas: tuple[JsonObject, ...] = (
        {
            "properties": {"value\x00": {"type": "string"}},
            "type": "object",
        },
        {
            "properties": {"value": {"type": "string\x00"}},
            "type": "object",
        },
        {
            "properties": {
                "value": {
                    "pattern": "value\x00",
                    "type": "string",
                }
            },
            "type": "object",
        },
    )

    for input_schema in schemas:
        with pytest.raises(ProtocolError, match=r"schema .* contains controls"):
            _ = render_inputs(tool._replace(input_schema=input_schema), {})


def test_renderer_keeps_generated_skills_technical_only() -> None:
    """General policy prose exposed by a native description is not rendered."""
    catalog = complete_catalog()
    first = catalog[0]
    permission = "permiso"  # cspell:disable-line -- permiso
    explicit = "explícito"  # cspell:disable-line -- explícito
    user = "usuario"  # cspell:disable-line -- usuario
    description = " ".join(
        (
            "Renames one native editor tag with TArray<Item> input.",
            "Only call after explicit permission from the user.",
            "Copyright and trademark questions are handled elsewhere.",
            f"{permission} {explicit} del {user}.",
        )
    )
    tool = first.tools[0]._replace(description=description)
    catalog = (first._replace(tools=(tool,)), *catalog[1:])
    documents = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(catalog)
    skill = next(
        document.content
        for document in documents
        if document.relative_path != "index.md"
    )

    assert (
        "Renames one native editor tag with TArray&lt;Item&gt; input." in skill
    )
    assert "TArray<Item>" not in skill
    assert "permission from the user" not in skill
    assert "Copyright" not in skill
    assert "trademark" not in skill
    assert "permiso" not in skill  # cspell:disable-line -- permiso
    assert "explícito" not in skill  # cspell:disable-line -- explícito
    assert "usuario" not in skill  # cspell:disable-line -- usuario
    assert "## Technical execution posture" in skill
    assert "Safety and approval" not in skill


def test_renderer_rejects_control_characters_in_native_prose() -> None:
    """Corrupt live descriptions must not enter generated Markdown."""
    catalog = complete_catalog()
    first = catalog[0]
    tool = first.tools[0]._replace(
        description="Reads editor state.\x00Injected terminal text."
    )
    catalog = (first._replace(tools=(tool,)), *catalog[1:])

    with pytest.raises(ProtocolError, match="control characters"):
        _ = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(catalog)


def test_catalog_markdown_rejects_control_characters_in_native_prose() -> None:
    """Catalog Markdown cannot contain corrupted native descriptions."""
    first = complete_catalog()[0]
    tool = first.tools[0]._replace(description="Tool details.\x00Injected.")
    malformed = (
        first._replace(description="Toolset details.\x00Injected."),
        first._replace(tools=(tool,)),
    )

    for toolset_definition in malformed:
        with pytest.raises(ProtocolError, match="control characters"):
            _ = render_catalog_markdown((toolset_definition,))


def test_shared_tool_prefixes_become_taxonomy_directories() -> None:
    """Only the sibling-unique suffix remains in each skill filename."""
    definition = toolset("AutomationTestToolset.AutomationTestToolset")
    tools = tuple(
        definition.tools[0]._replace(name=f"{definition.name}.{leaf}")
        for leaf in ("SetYAlpha", "SetYBeta", "DiscoverTests")
    )
    definition = definition._replace(tools=tools)

    assert tool_skill_path(definition, tools[0]) == (
        "capabilities/automation/test/toolset/set/y/alpha.md"
    )
    assert tool_skill_path(definition, tools[1]) == (
        "capabilities/automation/test/toolset/set/y/beta.md"
    )
    assert tool_skill_path(definition, tools[2]) == (
        "capabilities/automation/test/toolset/discover-tests.md"
    )


def test_nested_shared_prefix_uses_the_longest_stable_family() -> None:
    """Nested common names create nested folders without empty filenames."""
    definition = toolset("AutomationTestToolset.AutomationTestToolset")
    tools = tuple(
        definition.tools[0]._replace(name=f"{definition.name}.{leaf}")
        for leaf in (
            "SetSectionRange",
            "SetSectionBlendType",
            "SetPlaybackRange",
        )
    )
    definition = definition._replace(tools=tools)

    assert tool_skill_path(definition, tools[0]) == (
        "capabilities/automation/test/toolset/set/section/range.md"
    )
    assert tool_skill_path(definition, tools[1]) == (
        "capabilities/automation/test/toolset/set/section/blend-type.md"
    )
    assert tool_skill_path(definition, tools[2]) == (
        "capabilities/automation/test/toolset/set/playback-range.md"
    )


def test_name_taxonomy_matches_operator_example() -> None:
    """Automation discovery receives the requested name-derived path."""
    definition = toolset("AutomationTestToolset.AutomationTestToolset")
    synthetic = definition.tools[0]._replace(
        name="AutomationTestToolset.AutomationTestToolset.DiscoverTests"
    )

    assert tool_skill_path(definition, synthetic) == (
        "capabilities/automation/test/toolset/discover-tests.md"
    )


def test_renderer_rejects_missing_or_unowned_toolsets() -> None:
    """Catalog drift must stop generation until taxonomy is reviewed."""
    catalog = complete_catalog()
    with pytest.raises(ProtocolError, match="missing toolsets"):
        _ = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(catalog[:-1])

    unknown = toolset("FuturePlugin.UnownedToolset")
    with pytest.raises(ProtocolError, match="unowned toolsets"):
        _ = MarkdownSkillRenderer(TEST_UNREAL_MCP_VERSION).render(
            (*catalog, unknown)
        )
