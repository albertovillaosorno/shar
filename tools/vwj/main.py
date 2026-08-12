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
#   - Jig-free execution of SHAR's configured external validation gates.
# - Must-Not:
#   - Claim equivalence with Jig-native repository policy validation.
# - Allows:
#   - Inputs: tracked Jig TOML, Git message files, and CLI gate choices.
#   - Outputs: diagnostics and an optional local Git hook.
#   - Side effects: subprocess validation and explicit hook installation.
# - Split-When:
#   - Split when tool discovery or commit policy gains a separate lifecycle.
# - Merge-When:
#   - Merge when Jig becomes the only supported public validation surface.
# - Summary:
#   - Validates without requiring the Jig executable.
# - Description:
#   - Reuses tracked external-tool policy while excluding Jig-native rules.
# - Usage:
#   - Run tools/vwj/main.py check or install-hook from the repository root.
# - Defaults:
#   - Missing tools and invalid commit messages fail explicitly.
#

"""Run SHAR's external validation gates without the Jig executable."""

from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
import tomllib
from collections.abc import Iterable
from pathlib import Path

_CONFIG = Path(".jig/jig.toml")
_HOOK_MARKER = "# SHAR Jig/VWJ commit-message hook."
_SUBJECT = re.compile(
    r"^(?P<type>[a-z][a-z0-9-]*)\((?P<scope>[a-z][a-z0-9-]*)\): "
    r"(?P<text>.+)$"
)


def _root() -> Path:
    root = Path(__file__).resolve().parents[2]
    if not (root / _CONFIG).is_file():
        raise SystemExit("vwj: repository .jig/jig.toml is missing")
    return root


def _config(root: Path) -> dict[str, object]:
    with (root / _CONFIG).open("rb") as handle:
        return tomllib.load(handle)


def _table(config: dict[str, object], name: str) -> dict[str, object]:
    value = config.get(name, {})
    if not isinstance(value, dict):
        raise SystemExit(f"vwj: [{name}] must be a TOML table")
    return value


def _command_name(tool_name: str) -> str:
    aliases = {
        "markdownlint": "markdownlint-cli2",
        "rust_nightly_cargo": "cargo",
        "rust_nightly_cargo_clippy": "cargo-clippy",
        "rust_nightly_cargo_fmt": "cargo-fmt",
        "rust_stable_cargo": "cargo",
    }
    return aliases.get(tool_name, tool_name)


def _configured_command(
    root: Path,
    tool_name: str,
    tools: dict[str, object],
) -> list[str] | None:
    entry = tools.get(tool_name)
    if not isinstance(entry, dict):
        return None
    raw_path = entry.get("path")
    if not isinstance(raw_path, str):
        return None
    path = root / raw_path
    if not path.is_file():
        return None
    if os.name == "nt" and path.suffix.lower() == ".cmd":
        return [os.environ.get("COMSPEC", "cmd.exe"), "/d", "/c", str(path)]
    return [str(path)]


def _path_command(tool_name: str) -> list[str] | None:
    executable = shutil.which(_command_name(tool_name))
    return [executable] if executable else None


def _probe(command: list[str], root: Path) -> bool:
    try:
        result = subprocess.run(
            [*command, "--version"],
            cwd=root,
            check=False,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            timeout=20,
        )
    except (OSError, subprocess.TimeoutExpired):
        return False
    return result.returncode == 0


def _run_gate(
    root: Path,
    name: str,
    gate: dict[str, object],
    tools: dict[str, object],
) -> int:
    tool_name = gate.get("tool")
    raw_args = gate.get("args", [])
    if not isinstance(tool_name, str):
        print(f"vwj: {name}: missing tool name", file=sys.stderr)
        return 2
    if not isinstance(raw_args, list):
        print(f"vwj: {name}: invalid argument vector", file=sys.stderr)
        return 2
    if not all(isinstance(value, str) for value in raw_args):
        print(f"vwj: {name}: invalid argument vector", file=sys.stderr)
        return 2

    command = _configured_command(root, tool_name, tools)
    if command is not None and not _probe(command, root):
        command = None
    if command is None:
        command = _path_command(tool_name)
    if command is None:
        print(
            f"vwj: {name}: tool '{tool_name}' is unavailable",
            file=sys.stderr,
        )
        return 2

    print(f"vwj: {name}: running {_command_name(tool_name)}")
    try:
        result = subprocess.run(
            [*command, *raw_args],
            cwd=root,
            check=False,
        )
    except OSError as error:
        print(f"vwj: {name}: {error}", file=sys.stderr)
        return 2
    if result.returncode:
        print(
            f"vwj: {name}: failed with {result.returncode}",
            file=sys.stderr,
        )
    else:
        print(f"vwj: {name}: clean")
    return result.returncode


def _selected_gates(
    validation: dict[str, object],
    requested: Iterable[str],
) -> list[tuple[str, dict[str, object]]]:
    names = list(requested) or list(validation)
    selected: list[tuple[str, dict[str, object]]] = []
    for name in names:
        gate = validation.get(name)
        if not isinstance(gate, dict):
            raise SystemExit(f"vwj: unknown validation gate: {name}")
        selected.append((name, gate))
    return selected


def _check(args: argparse.Namespace) -> int:
    root = _root()
    config = _config(root)
    validation = _table(config, "validation")
    tools = _table(config, "tool")
    failed: list[str] = []
    for name, gate in _selected_gates(validation, args.gate):
        if _run_gate(root, name, gate, tools):
            failed.append(name)

    if failed:
        print("vwj: failed gates: " + ", ".join(failed), file=sys.stderr)
        return 1
    print("vwj: configured external validation gates are clean")
    print("vwj: Jig-native repository rules were not evaluated")
    return 0


def _message_text(path: Path) -> str:
    lines = path.read_text(encoding="utf-8-sig").splitlines()
    lines = [line for line in lines if not line.startswith("#")]
    while lines and not lines[-1].strip():
        lines.pop()
    return "\n".join(lines)


def _commit_message(args: argparse.Namespace) -> int:
    config = _config(_root())
    policy = _table(config, "commit")
    message = _message_text(Path(args.file))
    if not message:
        print("vwj: commit message is empty", file=sys.stderr)
        return 1

    subject = message.splitlines()[0]
    maximum = policy.get("subject_max_length", 72)
    if isinstance(maximum, int) and len(subject) > maximum:
        print(f"vwj: subject exceeds {maximum} characters", file=sys.stderr)
        return 1
    match = _SUBJECT.fullmatch(subject)
    if match is None:
        print(
            "vwj: subject must be 'type(scope): description'",
            file=sys.stderr,
        )
        return 1

    allowed_types = policy.get("allowed_types", [])
    allowed_scopes = policy.get("allowed_scopes", [])
    if match.group("type") not in allowed_types:
        print(
            f"vwj: unsupported commit type: {match.group('type')}",
            file=sys.stderr,
        )
        return 1
    if match.group("scope") not in allowed_scopes:
        print(
            f"vwj: unsupported commit scope: {match.group('scope')}",
            file=sys.stderr,
        )
        return 1
    if match.group("text").endswith("."):
        print("vwj: subject must not end with a period", file=sys.stderr)
        return 1

    print("vwj commit-message: clean")
    return 0


def _hook_text() -> str:
    return """#!/bin/sh
# SHAR Jig/VWJ commit-message hook.
set -eu

if [ "$#" -ne 1 ]; then
    echo "shar: commit hook requires one message file" >&2
    exit 1
fi

root=$(git rev-parse --show-toplevel 2>/dev/null) || {
    echo "shar: commit hook requires a Git worktree" >&2
    exit 1
}

if command -v jig >/dev/null 2>&1; then
    exec jig commit-message --root "$root" --file "$1"
fi
if command -v jig.cmd >/dev/null 2>&1; then
    exec jig.cmd commit-message --root "$root" --file "$1"
fi

validator="$root/tools/vwj/main.py"
if [ ! -f "$validator" ]; then
    echo "shar: Jig and the VWJ fallback are unavailable" >&2
    exit 1
fi
for python_cmd in python python3 py; do
    if command -v "$python_cmd" >/dev/null 2>&1; then
        exec "$python_cmd" "$validator" commit-message --file "$1"
    fi
done

echo "shar: Python is required for the VWJ hook fallback" >&2
exit 1
"""


def _install_hook(_args: argparse.Namespace) -> int:
    root = _root()
    result = subprocess.run(
        ["git", "rev-parse", "--git-path", "hooks/commit-msg"],
        cwd=root,
        check=True,
        capture_output=True,
        text=True,
    )
    hook = Path(result.stdout.strip())
    if not hook.is_absolute():
        hook = root / hook
    hook.parent.mkdir(parents=True, exist_ok=True)

    if hook.exists():
        existing = hook.read_text(encoding="utf-8")
        known_jig = "Jig PATH commit-message hook" in existing
        if _HOOK_MARKER not in existing and not known_jig:
            raise SystemExit("vwj: refusing to replace an unrelated hook")

    hook.write_text(_hook_text(), encoding="utf-8", newline="\n")
    if os.name != "nt":
        hook.chmod(hook.stat().st_mode | 0o111)
    print(f"vwj: installed Jig-first commit hook at {hook}")
    return 0


def _parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Validate SHAR without requiring the Jig executable.",
    )
    commands = parser.add_subparsers(dest="command", required=True)

    check = commands.add_parser(
        "check",
        help="run external validation gates from .jig/jig.toml",
    )
    check.add_argument(
        "--gate",
        action="append",
        default=[],
        help="run only this gate; repeat to select multiple gates",
    )
    check.set_defaults(handler=_check)

    commit = commands.add_parser(
        "commit-message",
        help="validate the portable subset of commit-message policy",
    )
    commit.add_argument("--file", required=True)
    commit.set_defaults(handler=_commit_message)

    install = commands.add_parser(
        "install-hook",
        help="install a Jig-first commit-msg hook with VWJ fallback",
    )
    install.set_defaults(handler=_install_hook)
    return parser


def main() -> int:
    """Dispatch the requested command."""
    args = _parser().parse_args()
    return args.handler(args)


if __name__ == "__main__":
    raise SystemExit(main())


def _hook_text() -> str:
    return """#!/bin/sh
# SHAR Jig/VWJ commit-message hook.
set -eu

if [ "$#" -ne 1 ]; then
    echo "shar: commit hook requires one message file" >&2
    exit 1
fi

root=$(git rev-parse --show-toplevel 2>/dev/null) || {
    echo "shar: commit hook requires a Git worktree" >&2
    exit 1
}

if command -v jig >/dev/null 2>&1; then
    exec jig commit-message --root "$root" --file "$1"
fi
if command -v jig.cmd >/dev/null 2>&1; then
    exec jig.cmd commit-message --root "$root" --file "$1"
fi

validator="$root/tools/vwj/main.py"
if [ ! -f "$validator" ]; then
    echo "shar: Jig and the VWJ fallback are unavailable" >&2
    exit 1
fi
for python_cmd in python python3 py; do
    if command -v "$python_cmd" >/dev/null 2>&1; then
        exec "$python_cmd" "$validator" commit-message --file "$1"
    fi
done

echo "shar: Python is required for the VWJ hook fallback" >&2
exit 1
"""


def _install_hook(_args: argparse.Namespace) -> int:
    root = _root()
    result = subprocess.run(
        ["git", "rev-parse", "--git-path", "hooks/commit-msg"],
        cwd=root,
        check=True,
        capture_output=True,
        text=True,
    )
    hook = Path(result.stdout.strip())
    if not hook.is_absolute():
        hook = root / hook
    hook.parent.mkdir(parents=True, exist_ok=True)

    if hook.exists():
        existing = hook.read_text(encoding="utf-8")
        known_jig = "Jig PATH commit-message hook" in existing
        if _HOOK_MARKER not in existing and not known_jig:
            raise SystemExit("vwj: refusing to replace an unrelated hook")

    hook.write_text(_hook_text(), encoding="utf-8", newline="\n")
    if os.name != "nt":
        hook.chmod(hook.stat().st_mode | 0o111)
    print(f"vwj: installed Jig-first commit hook at {hook}")
    return 0


def _parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Validate SHAR without requiring the Jig executable.",
    )
    commands = parser.add_subparsers(dest="command", required=True)

    check = commands.add_parser(
        "check",
        help="run external validation gates from .jig/jig.toml",
    )
    check.add_argument(
        "--gate",
        action="append",
        default=[],
        help="run only this gate; repeat to select multiple gates",
    )
    check.set_defaults(handler=_check)

    commit = commands.add_parser(
        "commit-message",
        help="validate the portable subset of commit-message policy",
    )
    commit.add_argument("--file", required=True)
    commit.set_defaults(handler=_commit_message)

    install = commands.add_parser(
        "install-hook",
        help="install a Jig-first commit-msg hook with VWJ fallback",
    )
    install.set_defaults(handler=_install_hook)
    return parser


def main() -> int:
    """Dispatch the requested command."""
    args = _parser().parse_args()
    return args.handler(args)


if __name__ == "__main__":
    raise SystemExit(main())
