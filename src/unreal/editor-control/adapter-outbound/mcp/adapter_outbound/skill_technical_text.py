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
#   - Skill technical text outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill technical text outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill technical text outbound adapter."""

from __future__ import annotations

import re

from mcp.domain.errors import fail_protocol


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
            r"\bpermiso\s+",
            r"(?:explicito|",
            r"explícito)?",
            r"\s*del\s+",
            r"usuario\b",
        ),
        re.IGNORECASE,
    ),
    re.compile(
        _pattern(
            r"\bauthori[sz]",
            r"(?:e|ed|ation)",
            r"\b",
        ),
        re.IGNORECASE,
    ),
    re.compile(
        _pattern(
            r"\bautorizaci",
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
            r"\b(?:derechos?",
            r"\s+de\s+",
            r"autor|",
            r"propiedad",
            r"\s+",
            r"intelectual)",
            r"\b",
        ),
        re.IGNORECASE,
    ),
    re.compile(
        _pattern(
            r"\b(?:proprietary|confidential|",
            r"confidencial)",
            r"\b",
        ),
        re.IGNORECASE,
    ),
)
_SENTENCE = re.compile(r"(?<=[.!?])\s+")


def validated_live_prose(description: str) -> str:
    """Normalize line endings and reject hidden controls in live prose.

    Returns:
        The validated prose with canonical newline characters.

    """
    normalized = description.replace("\r\n", "\n").replace("\r", "\n")
    if any(
        character != "\n" and not character.isprintable()
        for character in normalized
    ):
        fail_protocol("native description contains control characters")
    return normalized


def technical_only_text(description: str) -> str:
    """Return live documentation with general policy sentences removed."""
    normalized = validated_live_prose(description)
    rendered: list[str] = []
    for raw_line in normalized.splitlines():
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
