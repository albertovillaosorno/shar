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
#   - Optional sequential orchestration of the supported user build commands.
# - Must-Not:
#   - Duplicate validation, architecture, packaging, or dependency policy.
# - Allows:
#   - Inputs: explicit request to reselect targets.
#   - Outputs: child command diagnostics from each canonical build step.
#   - Side effects: only those owned by the invoked canonical build scripts.
# - Split-When:
#   - Split when orchestration gains independent scheduling or resume policy.
# - Merge-When:
#   - Merge when another user entry point owns the identical sequence.
# - Summary:
#   - Runs the supported SHAR build flow in canonical order.
# - Description:
#   - Reuses persisted JSON evidence rather than hidden process state.
# - Usage:
#   - Run tools/build/auto.py from any location.
# - Defaults:
#   - Revalidates an existing architecture decision instead of reopening it.
#

"""Run the supported SHAR dependency, check, target, and build sequence."""

from __future__ import annotations

import argparse
from pathlib import Path
import subprocess
import sys

_ARCH_PATH = Path(".cache/build/data/arch.json")


class AutoFailure(RuntimeError):
    """One child build step failed and orchestration must stop."""


def _root() -> Path:
    """Return the repository root from this tracked script location."""
    return Path(__file__).resolve().parents[2]


def _run_step(root: Path, name: str, arguments: list[str]) -> None:
    """Run one canonical build script using the current exact Python."""
    script = root / "tools" / "build" / name
    command = [sys.executable, str(script), *arguments]
    print(f"auto: running {name}")
    result = subprocess.run(command, cwd=root, check=False)
    if result.returncode:
        raise AutoFailure(f"{name} failed with exit code {result.returncode}")


def _architecture_arguments(root: Path, reselect: bool) -> list[str]:
    """Choose interactive selection or explicit saved-decision validation."""
    if reselect or not (root / _ARCH_PATH).is_file():
        return []
    return ["--revalidate"]


def _parser() -> argparse.ArgumentParser:
    """Build the intentionally small one-command user interface."""
    parser = argparse.ArgumentParser(
        description="Run the complete supported SHAR build flow.",
    )
    parser.add_argument(
        "--reselect",
        action="store_true",
        help="open the architecture checklist even when arch.json exists",
    )
    parser.add_argument(
        "--validate-only",
        action="store_true",
        help="finish with runner SDK validation instead of packaging",
    )
    return parser


def main() -> int:
    """Run every canonical build step in declared order and stop on failure."""
    args = _parser().parse_args()
    root = _root()
    architecture_args = _architecture_arguments(root, args.reselect)
    runner_args = ["--validate-only"] if args.validate_only else []
    try:
        _run_step(root, "dependencies.py", [])
        _run_step(root, "check.py", [])
        _run_step(root, "arch.py", architecture_args)
        _run_step(root, "run.py", runner_args)
    except (AutoFailure, OSError) as error:
        print(f"auto: {error}", file=sys.stderr)
        return 1
    print("auto: complete")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
