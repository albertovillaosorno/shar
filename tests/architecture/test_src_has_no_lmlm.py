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
#   - Repository-policy evidence isolating legacy LMLM compatibility.
# - Must-Not:
#   - Execute LMLM conversion or create user mod installation directories.
# - Allows:
#   - Inspect tracked source paths, ignore rules, and public README anchors.
# - Split-When:
#   - LMLM isolation policies gain independent validation lifecycles.
# - Merge-When:
#   - Another architecture test owns the same legacy-tool isolation contract.
# - Summary:
#   - Guards LMLM isolation from product source and user mod paths.
# - Description:
#   - Enforces repository boundaries for the legacy compatibility tool.
# - Usage:
#   - Run through the canonical repository pytest gate.
# - Defaults:
#   - Reads tracked repository content without external effects.
#

"""Architecture guard: legacy LMLM compatibility must never enter src/."""

from __future__ import annotations

from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]
_SRC = _ROOT / "src"
_TEXT_SUFFIXES = {
    ".c",
    ".cc",
    ".cpp",
    ".h",
    ".hpp",
    ".json",
    ".md",
    ".py",
    ".rs",
    ".toml",
    ".uplugin",
    ".uproject",
    ".yaml",
    ".yml",
}


def test_src_cannot_reference_lmlm() -> None:
    """Reject any implementation/configuration route back to legacy LMLM."""
    offenders: list[str] = []
    for path in sorted(_SRC.rglob("*")):
        if not path.is_file() or path.suffix.lower() not in _TEXT_SUFFIXES:
            continue
        if b"lmlm" in path.read_bytes().lower():
            offenders.append(path.relative_to(_ROOT).as_posix())
    assert not offenders, (
        "LMLM compatibility is forbidden under src/; keep it isolated in "
        f"tools/lmlm/: {offenders}"
    )


def test_repository_root_has_no_user_mod_drop_directory() -> None:
    """Keep user mod installation paths outside the source repository."""
    assert not (_ROOT / "mods").exists(), (
        "repository-root mods/ is forbidden; native SHAR mods belong only in "
        "the installed application's post-install mods directory"
    )


def test_lmlm_private_io_is_ignored_with_public_readmes() -> None:
    lines = (_ROOT / ".gitignore").read_text(encoding="utf-8").splitlines()
    relevant = tuple(
        line
        for line in lines
        if line.startswith(("/tools/lmlm/import", "!/tools/lmlm/import",
                            "/tools/lmlm/export", "!/tools/lmlm/export"))
    )
    assert relevant == (
        "/tools/lmlm/import/",
        "/tools/lmlm/export/",
    )
