# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - Optional Windows desktop shortcut creation for a built SHAR executable.
# - Must-Not:
#   - Modify startup, registry, shell policy, or non-Windows hosts.
# - Allows:
#   - Inputs: an explicit or discovered packaged Windows executable.
#   - Outputs: one user-approved desktop .lnk file.
#   - Side effects: creates or replaces that approved shortcut only.
# - Split-When:
#   - Split when another platform gains a native shortcut adapter.
# - Merge-When:
#   - Merge when the build runner owns identical optional shortcut behavior.
# - Summary:
#   - Creates an optional Windows desktop shortcut after packaging.
# - Description:
#   - Uses Windows' WScript.Shell COM interface through PowerShell.
# - Usage:
#   - Run after a Windows build, optionally passing --target.
# - Defaults:
#   - No shortcut is created unless the user explicitly approves it.
#

"""Optionally create a Windows desktop shortcut for a packaged SHAR build."""

from __future__ import annotations

import argparse
import os
from pathlib import Path
import subprocess
import sys

_PREFERRED_NAMES = ("SHAR.exe", "Simpsons.exe")
_IGNORED_NAMES = {
    "CrashReportClient.exe",
    "UnrealPrereqSetup_x64.exe",
    "UnrealPrereqSetup_x86.exe",
}


def _root() -> Path:
    return Path(__file__).resolve().parents[2]


def _discover_target(root: Path) -> Path:
    dist = root / "dist"
    if not dist.is_dir():
        raise SystemExit(
            "shortcut: dist/ does not exist; build Windows first or "
            "use --target"
        )

    executables = sorted(
        path.resolve()
        for path in dist.rglob("*.exe")
        if path.name not in _IGNORED_NAMES
    )
    for preferred in _PREFERRED_NAMES:
        matches = [path for path in executables if path.name == preferred]
        if len(matches) == 1:
            return matches[0]
    if len(executables) == 1:
        return executables[0]
    if not executables:
        raise SystemExit(
            "shortcut: no packaged Windows executable found in dist/"
        )
    raise SystemExit(
        "shortcut: multiple Windows executables found; choose one with --target"
    )


def _target(value: str | None, root: Path) -> Path:
    path = Path(value).expanduser() if value else _discover_target(root)
    if not path.is_absolute():
        path = root / path
    path = path.resolve()
    if not path.is_file():
        raise SystemExit(f"shortcut: target does not exist: {path}")
    if path.suffix.lower() != ".exe":
        raise SystemExit("shortcut: target must be a Windows .exe")
    return path


def _approved(target: Path, *, assume_yes: bool) -> bool:
    if assume_yes:
        return True
    answer = input(
        f"Create a desktop shortcut for {target.name}? [y/N] "
    ).strip()
    return answer.casefold() in {"y", "yes"}


def _create_shortcut(target: Path, name: str) -> Path:
    if not name.strip():
        raise SystemExit("shortcut: shortcut name cannot be empty")
    invalid = set(r'<>:"/\|?*')
    if any(character in invalid for character in name):
        raise SystemExit("shortcut: shortcut name contains invalid characters")

    env = os.environ.copy()
    env["SHAR_SHORTCUT_TARGET"] = str(target)
    env["SHAR_SHORTCUT_NAME"] = name
    script = r"""
$desktop = [Environment]::GetFolderPath('Desktop')
$link = Join-Path $desktop ($env:SHAR_SHORTCUT_NAME + '.lnk')
$shell = New-Object -ComObject WScript.Shell
$shortcut = $shell.CreateShortcut($link)
$shortcut.TargetPath = $env:SHAR_SHORTCUT_TARGET
$shortcut.WorkingDirectory = Split-Path $env:SHAR_SHORTCUT_TARGET
$shortcut.IconLocation = $env:SHAR_SHORTCUT_TARGET + ',0'
$shortcut.Save()
Write-Output $link
"""
    result = subprocess.run(
        [
            "powershell.exe",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            script,
        ],
        check=True,
        capture_output=True,
        env=env,
        text=True,
    )
    output = result.stdout.strip()
    if not output:
        raise SystemExit("shortcut: PowerShell did not report a shortcut path")
    return Path(output)


def _parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Optionally create a Windows desktop shortcut for SHAR.",
    )
    parser.add_argument(
        "--target",
        help="packaged .exe; otherwise discover a single executable in dist/",
    )
    parser.add_argument(
        "--name",
        default="SHAR",
        help="desktop shortcut name; default: SHAR",
    )
    parser.add_argument(
        "--yes",
        action="store_true",
        help="create without the interactive confirmation prompt",
    )
    return parser


def main() -> int:
    """Create the shortcut only on Windows and only after approval."""
    if os.name != "nt":
        print(
            "shortcut: this helper is available only on Windows",
            file=sys.stderr,
        )
        return 2
    args = _parser().parse_args()
    target = _target(args.target, _root())
    if not _approved(target, assume_yes=args.yes):
        print("shortcut: cancelled; no shortcut was created")
        return 0
    link = _create_shortcut(target, args.name)
    print(f"shortcut: created {link}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
