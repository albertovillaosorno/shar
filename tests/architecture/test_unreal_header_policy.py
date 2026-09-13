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
#   - Repository policy for authored Unreal C++ header naming and guards.
# - Must-Not:
#   - Inspect generated build products or external Unreal Engine sources.
# - Allows:
#   - Read project-owned headers and enforce portable include boundaries.
# - Split-When:
#   - Header naming and include-guard policy gain independent lifecycles.
# - Merge-When:
#   - Another architecture test owns the same authored-header contract.
# - Summary:
#   - Guards SHAR Unreal header extensions and include guards.
# - Description:
#   - Reserves .h for Unreal-reflected headers, uses .hpp otherwise, and forbids
#   - pragma-once in ordinary project-owned C++ headers.
# - Usage:
#   - Run through the canonical repository pytest or Jig gate.
# - Defaults:
#   - Reads only tracked project source.
#

"""Architecture guards for project-owned Unreal C++ headers."""

from __future__ import annotations

from pathlib import Path
import re

_ROOT = Path(__file__).resolve().parents[2]
_UNREAL_SOURCE = (
    _ROOT / "src/unreal/project/composition/uproject/Source"
)
_HEADER_SUFFIXES = frozenset({".h", ".hpp"})
_PRAGMA_ONCE = re.compile(r"^\s*#\s*pragma\s+once\b", re.MULTILINE)


def _headers() -> tuple[Path, ...]:
    """Return authored Unreal headers without generated build products."""
    return tuple(
        sorted(
            path
            for path in _UNREAL_SOURCE.rglob("*")
            if path.is_file() and path.suffix in _HEADER_SUFFIXES
        )
    )


def _guard(path: Path) -> str:
    """Return the deterministic include guard for one authored header."""
    stem = path.stem
    snake = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", stem)
    snake = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", snake).upper()
    return f"SHAR_{snake}_HEADER_INCLUDED"


def test_ordinary_unreal_headers_forbid_pragma_once() -> None:
    """Use portable include guards in ordinary project C++ headers."""
    offenders = [
        path.relative_to(_ROOT).as_posix()
        for path in _headers()
        if path.suffix == ".hpp"
        and _PRAGMA_ONCE.search(path.read_text(encoding="utf-8"))
    ]
    assert not offenders, (
        "#pragma once is forbidden in ordinary SHAR headers: "
        f"{offenders}"
    )


def test_reflected_unreal_headers_use_required_pragma_once() -> None:
    """Keep reflected headers in the form accepted by Unreal Header Tool."""
    offenders = [
        path.relative_to(_ROOT).as_posix()
        for path in _headers()
        if path.suffix == ".h"
        and not _PRAGMA_ONCE.search(path.read_text(encoding="utf-8"))
    ]
    assert not offenders, (
        "reflected Unreal headers require #pragma once: "
        f"{offenders}"
    )


def test_unreal_header_extensions_match_reflection_policy() -> None:
    """Reserve .h for reflected inputs and use .hpp for ordinary C++ headers."""
    offenders: list[str] = []
    for path in _headers():
        text = path.read_text(encoding="utf-8")
        reflected = ".generated.h" in text
        expected = ".h" if reflected else ".hpp"
        if path.suffix != expected:
            relative = path.relative_to(_ROOT).as_posix()
            offenders.append(f"{relative}: expected {expected}")
    assert not offenders, f"noncanonical Unreal header extensions: {offenders}"


def test_ordinary_unreal_headers_use_deterministic_include_guards() -> None:
    """Require one deterministic include guard around each ordinary header."""
    offenders: list[str] = []
    for path in _headers():
        if path.suffix != ".hpp":
            continue
        guard = _guard(path)
        text = path.read_text(encoding="utf-8")
        lines = text.splitlines()
        if (
            f"#ifndef {guard}" not in lines
            or f"#define {guard}" not in lines
            or not lines
            or lines[-1] != f"#endif  // {guard}"
        ):
            offenders.append(path.relative_to(_ROOT).as_posix())
    assert not offenders, f"noncanonical Unreal include guards: {offenders}"
