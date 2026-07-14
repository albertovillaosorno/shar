# File:
#   - skill_document_layout.py
# Path:
#   - src/mcp/src/adapters/driven/skill_document_layout.py
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
#   - Deterministic name-derived paths for generated Unreal MCP tool skills.
# - Must-Not:
#   - Render Markdown, access files, invoke Unreal, or classify behavior.
# - Allows:
#   - Mapping each native tool identity to exactly one focused skill file.
# - Split-When:
#   - Toolset taxonomy and tool filename rules require separate versioning.
# - Merge-When:
#   - Another adapter owns the same generated skill path contract.
# - Summary:
#   - Defines one name-derived path per native Unreal MCP tool.
# - Description:
#   - Converts registry words into semantic folders and tool names into files.
# - Usage:
#   - Shared by the central index and per-tool skill renderer.
# - Defaults:
#   - Extracts shared sibling prefixes and drops generic path components.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: generated Unreal MCP per-tool skill path layout
#   - reason: tokenization and collision checks form one path identity contract
#   - split: extract tokenization if another generated surface consumes it
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess after native naming or taxonomy conventions change
#
"""Name-derived document layout for native Unreal MCP tool skills."""

from __future__ import annotations

import re
from typing import TYPE_CHECKING

from mcp.src.domain.errors import fail_protocol

if TYPE_CHECKING:
    from mcp.src.domain.catalog import ToolDefinition, ToolsetDefinition

_WORDS = re.compile(r"[A-Z]+(?=[A-Z][a-z]|\d|$)|[A-Z]?[a-z]+|[A-Z]+|\d+")
_GENERIC_DROPPED_WORDS = frozenset({"tools"})
_NORMALIZED_WORDS = {"toolsets": "toolset"}
_MINIMUM_SHARED_TOOLS = 2


def tool_skill_path(
    toolset: ToolsetDefinition,
    tool: ToolDefinition,
) -> str:
    """Return the unique generated Markdown path for one native tool.

    Returns:
        Repository-relative path under `capabilities/`.
    """
    tool_words = _tool_words(tool.name)
    shared_prefix = _shared_tool_prefix(toolset, tool_words)
    directory_parts = (*_toolset_taxonomy(toolset.name), *shared_prefix)
    filename_words = tool_words[len(shared_prefix) :]
    directory = "/".join(directory_parts)
    filename = f"{'-'.join(filename_words)}.md"
    return f"capabilities/{directory}/{filename}"


def validate_unique_tool_paths(
    catalog: tuple[ToolsetDefinition, ...],
) -> None:
    """Fail when two native tools normalize to the same generated path."""
    owners: dict[str, str] = {}
    for toolset in catalog:
        for tool in toolset.tools:
            path = tool_skill_path(toolset, tool)
            previous = owners.get(path)
            if previous is not None:
                message = (
                    f"generated skill path collision: {path}: {previous} "
                    f"and {tool.name}"
                )
                fail_protocol(message)
            owners[path] = tool.name


def _toolset_taxonomy(toolset_name: str) -> tuple[str, ...]:
    words: list[str] = []
    seen: set[str] = set()
    for component in toolset_name.replace("_", ".").split("."):
        for raw_word in _matched_words(component):
            normalized = raw_word.casefold()
            word = _NORMALIZED_WORDS.get(normalized, normalized)
            if word in _GENERIC_DROPPED_WORDS or word in seen:
                continue
            words.append(word)
            seen.add(word)
    if not words:
        fail_protocol(f"toolset cannot produce a taxonomy path: {toolset_name}")
    return tuple(words)


def _shared_tool_prefix(
    toolset: ToolsetDefinition,
    target_words: tuple[str, ...],
) -> tuple[str, ...]:
    longest: tuple[str, ...] = ()
    sibling_words = tuple(_tool_words(tool.name) for tool in toolset.tools)
    for length in range(1, len(target_words)):
        prefix = target_words[:length]
        matching = sum(
            1
            for words in sibling_words
            if len(words) > length and words[:length] == prefix
        )
        if matching >= _MINIMUM_SHARED_TOOLS:
            longest = prefix
    return longest


def _tool_words(tool_name: str) -> tuple[str, ...]:
    leaf = tool_name.rsplit(".", 1)[-1]
    if not leaf:
        fail_protocol(f"tool identity has no leaf name: {tool_name}")
    words = tuple(word.casefold() for word in _matched_words(leaf))
    if not words:
        fail_protocol(f"tool name cannot produce a skill filename: {tool_name}")
    return words


def _matched_words(value: str) -> tuple[str, ...]:
    return tuple(match.group(0) for match in _WORDS.finditer(value))
