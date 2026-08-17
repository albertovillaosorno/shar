# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT

"""Repository guards for the local algorithm workspace publication boundary."""

from __future__ import annotations

from pathlib import Path
import subprocess

_ROOT = Path(__file__).resolve().parents[2]


def _is_ignored(relative_path: str) -> bool:
    """Ask Git whether one repository-relative path is ignored."""
    result = subprocess.run(
        ["git", "check-ignore", "--quiet", "--no-index", "--", relative_path],
        cwd=_ROOT,
        check=False,
    )
    assert result.returncode in (0, 1), (
        f"git check-ignore failed for {relative_path!r}: {result.returncode}"
    )
    return result.returncode == 0


def test_algorithm_workspace_blocks_private_and_generated_payloads() -> None:
    """Keep lawful local evidence and generated outputs outside publication."""
    private_paths = (
        "algorithms/game/in/example.exe",
        "algorithms/game/in/example.rcf",
        "algorithms/game/master/content/example.bin",
        "algorithms/lang/french/in/dialogf.rcf",
        "algorithms/lang/french/master/content/example.bin",
        "algorithms/muckluck/in/example.lmlm",
        "algorithms/muckluck/master/content/example.bin",
        "algorithms/out/The Simpsons Hit & Run/example.bin",
        "algorithms/out/mods/example/mod.json",
    )

    assert all(_is_ignored(path) for path in private_paths)


def test_algorithm_workspace_admits_only_public_contract_files() -> None:
    """Allow public README metadata and serialized algorithm plans."""
    public_paths = (
        "algorithms/README.md",
        "algorithms/game/in/README.md",
        "algorithms/game/master/README.md",
        "algorithms/game/algorithm/windows-x64.txt",
        "algorithms/lang/french/README.md",
        "algorithms/lang/french/algorithm/windows-x64.txt",
        "algorithms/muckluck/README.md",
        "algorithms/muckluck/algorithm/windows-x64.txt",
        "algorithms/out/README.md",
        "algorithms/out/mods/README.md",
    )

    assert not any(_is_ignored(path) for path in public_paths)


def test_algorithm_workspace_tracks_semantic_directory_anchors() -> None:
    """Require README anchors for semantic directories that may be locally empty."""
    required = {
        "algorithms/game/README.md",
        "algorithms/game/in/README.md",
        "algorithms/game/master/README.md",
        "algorithms/muckluck/README.md",
        "algorithms/muckluck/in/README.md",
        "algorithms/muckluck/master/README.md",
        "algorithms/muckluck/algorithm/README.md",
        "algorithms/out/README.md",
        "algorithms/out/mods/README.md",
    }
    for locale in ("french", "german", "spanish-latam", "spanish-spain"):
        required.update(
            {
                f"algorithms/lang/{locale}/README.md",
                f"algorithms/lang/{locale}/in/README.md",
                f"algorithms/lang/{locale}/master/README.md",
                f"algorithms/lang/{locale}/algorithm/README.md",
            }
        )

    missing = sorted(path for path in required if not (_ROOT / path).is_file())
    assert not missing, f"algorithm workspace is missing README anchors: {missing}"
