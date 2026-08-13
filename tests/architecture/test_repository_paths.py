# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT

"""Repository path-integrity guards for tracked architecture and documentation."""

from __future__ import annotations

import re
import tomllib
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]
_MARKDOWN_LINK = re.compile(r"\[[^\]]*\]\(([^)]+)\)")
_RETIRED_PATHS = (
    "mods",
    "src/mods/languages",
    "src/unreal/icon",
    "tools/language-mods",
    "tools/vwj",
)


def test_workspace_members_have_cargo_manifests() -> None:
    """Reject Cargo workspace members that no longer exist."""
    with (_ROOT / "Cargo.toml").open("rb") as stream:
        config = tomllib.load(stream)
    workspace = config.get("workspace")
    assert isinstance(workspace, dict), "root Cargo.toml must declare [workspace]"
    members = workspace.get("members")
    assert isinstance(members, list), "workspace.members must be a list"

    missing = [
        member
        for member in members
        if not isinstance(member, str)
        or not (_ROOT / member / "Cargo.toml").is_file()
    ]
    assert not missing, f"stale Cargo workspace members: {missing}"


def _identity_path(function_file: Path) -> str | None:
    """Read identity.path from the repository's narrow function.yml grammar."""
    in_identity = False
    for line in function_file.read_text(encoding="utf-8").splitlines():
        if line == "identity:":
            in_identity = True
            continue
        if in_identity and line and not line.startswith("  "):
            break
        if in_identity and line.startswith("  path: "):
            return line.split(": ", 1)[1].strip().strip("\"'")
    return None


def test_function_metadata_paths_match_directories() -> None:
    """Keep function identity paths synchronized with their real directories."""
    mismatches: list[str] = []
    for function_file in sorted((_ROOT / "src").glob("**/function.yml")):
        declared = _identity_path(function_file)
        actual = function_file.parent.relative_to(_ROOT).as_posix()
        if declared != actual:
            mismatches.append(f"{function_file.relative_to(_ROOT)}: {declared!r} != {actual!r}")
    assert not mismatches, f"stale function.yml paths: {mismatches}"


def _documentation_files() -> list[Path]:
    files = [_ROOT / "README.md", _ROOT / "TODO.md"]
    files.extend((_ROOT / "docs").rglob("*.md"))
    files.extend((_ROOT / "docs").rglob("*.mdc"))
    return sorted(path for path in files if path.is_file())


def test_repository_relative_markdown_links_exist() -> None:
    """Reject relative documentation links whose repository targets vanished."""
    missing: list[str] = []
    for document in _documentation_files():
        text = document.read_text(encoding="utf-8")
        for match in _MARKDOWN_LINK.finditer(text):
            target = match.group(1).strip()
            if not target or target.startswith(("#", "http:", "https:", "mailto:", "<http:", "<https:")):
                continue
            target = target.strip("<>").split("#", 1)[0]
            if not target:
                continue
            destination = (document.parent / target).resolve()
            try:
                destination.relative_to(_ROOT.resolve())
            except ValueError:
                continue
            if not destination.exists():
                line = text.count("\n", 0, match.start()) + 1
                relative = document.relative_to(_ROOT).as_posix()
                missing.append(f"{relative}:{line}: {target}")
    assert not missing, f"stale repository Markdown links: {missing}"


def test_retired_repository_paths_stay_absent() -> None:
    """Prevent removed compatibility and replacement subsystems from returning."""
    present = [path for path in _RETIRED_PATHS if (_ROOT / path).exists()]
    assert not present, f"retired repository paths returned: {present}"
