# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT

"""Tracked policy guards for SHAR's source-linked Jig development install."""

from __future__ import annotations

import tomllib
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]
_CONFIG = _ROOT / ".jig" / "jig.toml"
_SOURCE_PREFIX = ".dependencies/jig/source/"
_SHARED_SOURCE_TOOLS = {
    "cspell",
    "git",
    "markdownlint",
    "rust_nightly_cargo",
    "rust_nightly_cargo_clippy",
    "rust_nightly_cargo_fmt",
    "rust_stable_cargo",
}


def test_shared_jig_tools_resolve_through_source_link() -> None:
    """Prevent tracked validation paths from bypassing Jig's source link."""
    with _CONFIG.open("rb") as stream:
        config = tomllib.load(stream)
    tools = config.get("tool")
    assert isinstance(tools, dict), ".jig/jig.toml must declare [tool] entries"

    offenders: list[str] = []
    for name in sorted(_SHARED_SOURCE_TOOLS):
        entry = tools.get(name)
        if not isinstance(entry, dict):
            offenders.append(f"{name}:missing")
            continue
        path = entry.get("path")
        if not isinstance(path, str) or not path.startswith(_SOURCE_PREFIX):
            offenders.append(f"{name}:{path!r}")

    assert not offenders, (
        "shared Jig validation tools must resolve through "
        f"{_SOURCE_PREFIX}: {offenders}"
    )
