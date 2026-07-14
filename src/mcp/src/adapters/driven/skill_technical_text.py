# File:
#   - skill_technical_text.py
# Path:
#   - src/mcp/src/adapters/driven/skill_technical_text.py
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
#   - Technical-only projection of live MCP documentation prose.
# - Must-Not:
#   - Parse schemas, render complete Markdown, access files, or invoke Unreal.
# - Allows:
#   - Removing general policy sentences before generated skill rendering.
# - Split-When:
#   - Language-specific filters require independently versioned policies.
# - Merge-When:
#   - Another module owns the same technical documentation projection.
# - Summary:
#   - Keeps generated Unreal skills limited to technical interface guidance.
# - Description:
#   - Filters general policy prose while preserving exposed technical sentences.
# - Usage:
#   - Called before structured native-description parsing.
# - Defaults:
#   - Supports known English and Spanish general-policy phrases.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Technical-only projection for live Unreal MCP documentation text."""

from __future__ import annotations

import re


def _pattern(*parts: str) -> str:
    """Join reviewed regular-expression fragments.

    Returns:
        One complete regular-expression source string.
    """
    return "".join(parts)


_GENERAL_POLICY_PATTERNS = (
    re.compile(r"\b(?:explicit\s+)?approval\b", re.IGNORECASE),
    re.compile(r"\bpermission\s+from\s+the\s+user\b", re.IGNORECASE),
    re.compile(
        _pattern(
            r"\bpermiso\s+",  # cspell:disable-line -- bpermiso
            r"(?:explicito|",  # cspell:disable-line -- explicito
            r"explícito)?",  # cspell:disable-line -- explícito
            r"\s*del\s+",
            r"usuario\b",  # cspell:disable-line -- usuario
        ),
        re.IGNORECASE,
    ),
    re.compile(
        _pattern(
            r"\bauthori[sz]",  # cspell:disable-line -- bauthori
            r"(?:e|ed|ation)",  # cspell:disable-line -- ation
            r"\b",
        ),
        re.IGNORECASE,
    ),
    re.compile(
        _pattern(
            r"\bautorizaci",  # cspell:disable-line -- bautorizaci
            r"[oó]n\b",
        ),
        re.IGNORECASE,
    ),
    re.compile(r"\b(?:legal|illegal|ethical|moral)\b", re.IGNORECASE),
    re.compile(
        r"\b(?:copyright|trademark|intellectual\s+property)\b",
        re.IGNORECASE,
    ),
    re.compile(
        _pattern(
            r"\b(?:derechos?",  # cspell:disable-line -- derechos
            r"\s+de\s+",
            r"autor|",  # cspell:disable-line -- autor
            r"propiedad",  # cspell:disable-line -- propiedad
            r"\s+",
            r"intelectual)",  # cspell:disable-line -- intelectual
            r"\b",
        ),
        re.IGNORECASE,
    ),
    re.compile(
        _pattern(
            r"\b(?:proprietary|confidential|",
            r"confidencial)",  # cspell:disable-line -- confidencial
            r"\b",
        ),
        re.IGNORECASE,
    ),
)
_SENTENCE = re.compile(r"(?<=[.!?])\s+")


def technical_only_text(description: str) -> str:
    """Return live documentation with general policy sentences removed."""
    rendered: list[str] = []
    for raw_line in description.splitlines():
        stripped = raw_line.strip()
        if not stripped or stripped.endswith(":"):
            rendered.append(raw_line)
            continue
        sentences = tuple(
            sentence.strip()
            for sentence in _SENTENCE.split(stripped)
            if sentence.strip()
        )
        technical = tuple(
            sentence
            for sentence in sentences
            if not any(
                pattern.search(sentence) for pattern in _GENERAL_POLICY_PATTERNS
            )
        )
        if technical:
            rendered.append(" ".join(technical))
    return "\n".join(rendered)
