# File:
#   - skill_description.py
# Path:
#   - src/mcp/src/adapters/driven/skill_description.py
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
#   - Interpretation of live MCP documentation text for generated tool skills.
# - Must-Not:
#   - Render complete Markdown, inspect schemas, access files, or call tools.
# - Allows:
#   - Extracting purpose, argument notes, returns, failures, and risk posture.
# - Split-When:
#   - Docstring parsing and operational posture require separate policies.
# - Merge-When:
#   - Another adapter owns the same live-description interpretation contract.
# - Summary:
#   - Converts native interface prose into focused skill guidance.
# - Description:
#   - Parses exposed MCP documentation only; it does not inspect engine source.
# - Usage:
#   - Consumed by the per-tool skill renderer.
# - Defaults:
#   - Preserves useful prose and fails back to explicit schema-only guidance.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: live Unreal MCP description interpretation
#   - reason: purpose, sections, and posture share one technical contract
#   - split: extract posture policy if native annotations become available
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess when native tool documentation format changes
#
"""Interpret live Unreal MCP descriptions for generated skills."""

from __future__ import annotations

import re
import textwrap
from typing import NamedTuple

from mcp.src.adapters.driven.skill_technical_text import technical_only_text

_SECTION = re.compile(
    r"^(?P<name>Args|Arguments|Parameters|Returns|Raises|Examples?|Notes?):\s*$",
    re.IGNORECASE,
)
_ARGUMENT = re.compile(r"^\s*(?P<name>[A-Za-z_][A-Za-z0-9_]*):\s*(?P<text>.*)$")
_MUTATION_PREFIXES = (
    "add",
    "apply",
    "assign",
    "bind",
    "clear",
    "connect",
    "create",
    "delete",
    "destroy",
    "disable",
    "duplicate",
    "enable",
    "import",
    "insert",
    "move",
    "remove",
    "rename",
    "replace",
    "reset",
    "retarget",
    "save",
    "set",
    "spawn",
    "unbind",
    "update",
)
_EXECUTION_PREFIXES = (
    "activate",
    "close",
    "compile",
    "deactivate",
    "execute",
    "export",
    "load",
    "open",
    "run",
    "start",
    "stop",
)
_READ_PREFIXES = (
    "can",
    "check",
    "describe",
    "discover",
    "find",
    "get",
    "has",
    "inspect",
    "is",
    "list",
    "preview",
    "query",
    "search",
    "validate",
)


class DescriptionSections(NamedTuple):
    """Parsed live tool documentation sections."""

    purpose: str
    arguments: dict[str, str]
    returns: str
    raises: str
    notes: str


class OperationalPosture(NamedTuple):
    """Conservative operational guidance inferred from the native identity."""

    label: str
    guidance: str
    requires_state_verification: bool


def parse_description(description: str) -> DescriptionSections:
    """Parse exposed native tool documentation into focused sections.

    Returns:
        Purpose, argument notes, return guidance, failures, and notes.
    """
    technical_description = technical_only_text(description)
    lines = textwrap.dedent(technical_description).strip().splitlines()
    sections: dict[str, list[str]] = {"purpose": []}
    current = "purpose"
    for raw_line in lines:
        line = raw_line.rstrip()
        section = _SECTION.fullmatch(line.strip())
        if section is not None:
            section_name = section.group("name")
            if section_name is None:
                continue
            current = _normalize_section(section_name)
            _ = sections.setdefault(current, [])
            continue
        sections.setdefault(current, []).append(line)
    arguments = _parse_arguments(sections.get("arguments", []))
    return DescriptionSections(
        purpose=_paragraphs(sections.get("purpose", [])),
        arguments=arguments,
        returns=_paragraphs(sections.get("returns", [])),
        raises=_paragraphs(sections.get("raises", [])),
        notes=_paragraphs(sections.get("notes", [])),
    )


def operational_posture(tool_name: str) -> OperationalPosture:
    """Return conservative side-effect guidance from the tool leaf identity."""
    leaf = tool_name.rsplit(".", 1)[-1].casefold()
    if leaf.startswith(_MUTATION_PREFIXES):
        return OperationalPosture(
            label="Persistent mutation likely",
            guidance=(
                "Capture pre-state, bound the target set, and verify the "
                "resulting editor or asset state through an independent read."
            ),
            requires_state_verification=True,
        )
    if leaf.startswith(_EXECUTION_PREFIXES):
        return OperationalPosture(
            label="Execution or transient mutation likely",
            guidance=(
                "Confirm execution scope, cancellation behavior, and expected "
                "side effects before invocation."
            ),
            requires_state_verification=True,
        )
    if leaf.startswith(_READ_PREFIXES):
        return OperationalPosture(
            label="Expected read-only",
            guidance=(
                "Use the returned structured evidence directly, but still "
                "confirm the live schema because names do not prove "
                "side effects."
            ),
            requires_state_verification=False,
        )
    return OperationalPosture(
        label="Review required",
        guidance=(
            "The native identity does not establish side effects. Review "
            "the live schema and editor context before invocation."
        ),
        requires_state_verification=True,
    )


def markdown_paragraphs(text: str) -> list[str]:
    """Return wrapped Markdown paragraphs from normalized documentation text."""
    if not text:
        return [
            "The live interface exposes no additional purpose text.",
            "Use the schema and native identity as the authoritative contract.",
        ]
    paragraphs = [part.strip() for part in text.split("\n\n") if part.strip()]
    rendered: list[str] = []
    for paragraph in paragraphs:
        markdown_safe = paragraph.replace("<", "&lt;").replace(">", "&gt;")
        rendered.extend(textwrap.wrap(markdown_safe, width=79) or [""])
        rendered.append("")
    if rendered:
        _ = rendered.pop()
    return rendered


def _normalize_section(name: str) -> str:
    normalized = name.casefold()
    if normalized in {"args", "arguments", "parameters"}:
        return "arguments"
    if normalized.startswith("return"):
        return "returns"
    if normalized.startswith("raise"):
        return "raises"
    if normalized.startswith("note"):
        return "notes"
    return "notes"


def _parse_arguments(lines: list[str]) -> dict[str, str]:
    arguments: dict[str, list[str]] = {}
    current: str | None = None
    for line in lines:
        match = _ARGUMENT.match(line)
        if match is not None:
            name = match.group("name")
            value = match.group("text")
            if name is None or value is None:
                continue
            arguments[name] = [value.strip()]
            current = name
            continue
        if current is not None and line.strip():
            arguments[current].append(line.strip())
    return {
        name: " ".join(part for part in parts if part).strip()
        for name, parts in arguments.items()
    }


def _paragraphs(lines: list[str]) -> str:
    normalized: list[str] = []
    paragraph: list[str] = []
    for line in lines:
        stripped = line.strip()
        if stripped:
            paragraph.append(stripped)
            continue
        if paragraph:
            normalized.append(" ".join(paragraph))
            paragraph = []
    if paragraph:
        normalized.append(" ".join(paragraph))
    return "\n\n".join(normalized)
