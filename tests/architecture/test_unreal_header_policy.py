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
#   - Repository policy for authored Unreal C++ headers and UHT boundaries.
# - Must-Not:
#   - Inspect generated build products or external Unreal Engine sources.
# - Allows:
#   - Read project-owned headers and enforce Unreal reflection conventions.
# - Split-When:
#   - Reflection and general header policy gain independent lifecycles.
# - Merge-When:
#   - Another architecture test owns the same authored-header contract.
# - Summary:
#   - Guards SHAR Unreal header naming and generated-header consistency.
# - Description:
#   - Requires .h and pragma-once everywhere while matching UHT generated
#   - includes exactly to headers that contain Unreal reflection declarations.
# - Usage:
#   - Run through the canonical repository pytest or Jig gate.
# - Defaults:
#   - Reads only project-owned Unreal source headers.
#

"""Architecture guards for project-owned Unreal C++ headers."""

from __future__ import annotations

from pathlib import Path
import re

_ROOT = Path(__file__).resolve().parents[2]
_UNREAL_SOURCE = _ROOT / "src/unreal/project/composition/uproject/Source"
_PRAGMA_ONCE = re.compile(r"^\s*#\s*pragma\s+once\b", re.MULTILINE)
_REFLECTION_DECLARATION = re.compile(
    r"^\s*U(?:CLASS|STRUCT|ENUM|INTERFACE)\s*\(",
    re.MULTILINE,
)
_GENERATED_INCLUDE = re.compile(
    r'^\s*#\s*include\s+"([A-Za-z0-9_]+)\.generated\.h"\s*$',
    re.MULTILINE,
)


def _headers() -> tuple[Path, ...]:
    """Return project-owned Unreal headers."""
    return tuple(sorted(_UNREAL_SOURCE.rglob("*.h")))


def test_unreal_headers_use_h_extension_only() -> None:
    """Keep project headers aligned with Unreal's conventional .h extension."""
    hpp_files = [
        path.relative_to(_ROOT).as_posix()
        for path in _UNREAL_SOURCE.rglob("*.hpp")
    ]
    assert not hpp_files, f"Unreal project headers must use .h: {hpp_files}"


def test_unreal_headers_use_pragma_once() -> None:
    """Require the same header guard convention across project Unreal code."""
    offenders = [
        path.relative_to(_ROOT).as_posix()
        for path in _headers()
        if not _PRAGMA_ONCE.search(path.read_text(encoding="utf-8"))
    ]
    assert not offenders, f"SHAR headers require #pragma once: {offenders}"


def test_reflection_headers_include_matching_generated_header() -> None:
    """Require reflected declarations to include their matching UHT output."""
    offenders: list[str] = []
    for path in _headers():
        text = path.read_text(encoding="utf-8")
        if not _REFLECTION_DECLARATION.search(text):
            continue
        includes = _GENERATED_INCLUDE.findall(text)
        if includes != [path.stem]:
            relative = path.relative_to(_ROOT).as_posix()
            offenders.append(f"{relative}: generated includes={includes}")
    assert not offenders, f"invalid reflected header UHT includes: {offenders}"


def test_non_reflection_headers_have_no_generated_header() -> None:
    """Reject generated-header coupling from ordinary C++ headers."""
    offenders = [
        path.relative_to(_ROOT).as_posix()
        for path in _headers()
        if not _REFLECTION_DECLARATION.search(path.read_text(encoding="utf-8"))
        and _GENERATED_INCLUDE.search(path.read_text(encoding="utf-8"))
    ]
    assert not offenders, f"ordinary headers include UHT output: {offenders}"
