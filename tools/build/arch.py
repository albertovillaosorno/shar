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
#   - User selection and persistence of supported SHAR build targets.
# - Must-Not:
#   - Build targets, infer unsupported targets, or silently select a host.
# - Allows:
#   - Inputs: checklist choices or explicit CLI target identifiers.
#   - Outputs: versioned architecture-selection JSON.
#   - Side effects: atomically writes the requested selection file.
# - Split-When:
#   - Split when target policy gains an independent lifecycle.
# - Merge-When:
#   - Merge when the build runner owns identical selection behavior.
# - Summary:
#   - Selects one or more supported build architectures.
# - Description:
#   - Shows a minimal checklist and annotates the exact current host target.
# - Usage:
#   - Run tools/build/arch.py and choose at least one target.
# - Defaults:
#   - No target is selected automatically.
#

"""Select supported SHAR build targets and persist the decision."""

from __future__ import annotations

import argparse
import json
import os
import platform
import sys
import tkinter as tk
from pathlib import Path
from tkinter import messagebox
from typing import NamedTuple

_SCHEMA = "shar.build.arch.v1"
_DATA_PATH = Path(".cache/build/data/arch.json")
_HOST_MARKER = "← this is your system"


class Target(NamedTuple):
    """One canonical selectable build target."""

    identifier: str
    system: str
    architecture: str
    label: str
    artifact: str


_TARGETS = (
    Target("android-arm64", "android", "arm64", "Android ARM64", "apk"),
    Target("ios-arm64", "ios", "arm64", "iOS ARM64", "ipa"),
    Target("linux-arm64", "linux", "arm64", "Linux ARM64", "native"),
    Target(
        "linux-x64",
        "linux",
        "amd64",
        "Linux AMD64 / x86-64 (64-bit PC)",
        "native",
    ),
    Target("macos-arm64", "macos", "arm64", "macOS ARM64", "native"),
    Target("windows-arm64", "windows", "arm64", "Windows ARM64", "native"),
    Target(
        "windows-x64",
        "windows",
        "amd64",
        "Windows AMD64 / x86-64 (64-bit PC)",
        "native",
    ),
)
_TARGETS_BY_ID = {target.identifier: target for target in _TARGETS}


def _root() -> Path:
    """Return the repository root from this script's tracked location."""
    return Path(__file__).resolve().parents[2]


def _host_system() -> str | None:
    """Normalize the current desktop host operating system."""
    names = {
        "darwin": "macos",
        "linux": "linux",
        "windows": "windows",
    }
    return names.get(platform.system().casefold())


def _host_architecture() -> str | None:
    """Normalize common host architecture aliases."""
    names = {
        "aarch64": "arm64",
        "amd64": "amd64",
        "arm64": "arm64",
        "x86_64": "amd64",
    }
    return names.get(platform.machine().casefold())


def _host_target() -> Target | None:
    """Return the selectable target matching the exact current host."""
    system = _host_system()
    architecture = _host_architecture()
    for target in _TARGETS:
        if target.system == system and target.architecture == architecture:
            return target
    return None


def _display_label(target: Target, host: Target | None) -> str:
    """Add a visible host marker without changing target identity."""
    if host is not None and target.identifier == host.identifier:
        return f"{target.label}  {_HOST_MARKER}"
    return target.label


def _payload(selected: list[Target]) -> dict[str, object]:
    """Build deterministic versioned decision data."""
    host = _host_target()
    host_data: dict[str, str | None] = {
        "architecture": _host_architecture(),
        "system": _host_system(),
        "target": host.identifier if host is not None else None,
    }
    targets = [
        {
            "architecture": target.architecture,
            "artifact": target.artifact,
            "id": target.identifier,
            "system": target.system,
        }
        for target in selected
    ]
    return {
        "host": host_data,
        "schema": _SCHEMA,
        "targets": targets,
    }


def _write_selection(path: Path, selected: list[Target]) -> None:
    """Atomically persist a non-empty canonical target selection."""
    if not selected:
        raise ValueError("at least one build target must be selected")
    if len({target.identifier for target in selected}) != len(selected):
        raise ValueError("duplicate build targets are not allowed")

    path.parent.mkdir(parents=True, exist_ok=True)
    candidate = path.with_name(f".{path.name}.{os.getpid()}.tmp")
    data = json.dumps(
        _payload(selected),
        ensure_ascii=False,
        indent=2,
        sort_keys=True,
    )
    try:
        candidate.write_text(data + "\n", encoding="utf-8", newline="\n")
        os.replace(candidate, path)
    finally:
        candidate.unlink(missing_ok=True)


def _selected_targets(identifiers: list[str]) -> list[Target]:
    """Resolve explicit target identifiers in canonical display order."""
    unknown = sorted(set(identifiers) - _TARGETS_BY_ID.keys())
    if unknown:
        names = ", ".join(unknown)
        raise SystemExit(f"arch: unsupported target identifier: {names}")
    requested = set(identifiers)
    return [target for target in _TARGETS if target.identifier in requested]


def _print_targets() -> int:
    """Print the canonical target inventory with the exact host annotation."""
    host = _host_target()
    for target in _TARGETS:
        print(f"{target.identifier}: {_display_label(target, host)}")
    return 0


def _save_cli(identifiers: list[str], output: Path) -> int:
    """Persist an explicit non-GUI selection for automation and testing."""
    if not identifiers:
        raise SystemExit("arch: --select requires at least one target")
    selected = _selected_targets(identifiers)
    _write_selection(output, selected)
    print(f"arch: saved {len(selected)} target(s) to {output}")
    return 0


def _save_gui(
    window: tk.Tk,
    values: dict[str, tk.BooleanVar],
    output: Path,
) -> None:
    """Validate the checklist, persist it, and close the successful window."""
    identifiers = [
        target.identifier
        for target in _TARGETS
        if values[target.identifier].get()
    ]
    if not identifiers:
        messagebox.showerror(
            "SHAR build targets",
            "Select at least one build target.",
            parent=window,
        )
        return
    selected = _selected_targets(identifiers)
    try:
        _write_selection(output, selected)
    except OSError as error:
        messagebox.showerror(
            "SHAR build targets",
            f"Could not save the selection:\n{error}",
            parent=window,
        )
        return
    window.destroy()


def _show_gui(output: Path) -> int:
    """Show the minimal supported build-target checklist."""
    try:
        window = tk.Tk()
    except tk.TclError as error:
        print(
            f"arch: could not open the checklist window: {error}",
            file=sys.stderr,
        )
        return 2

    window.title("SHAR build targets")
    window.resizable(False, False)
    frame = tk.Frame(window, padx=18, pady=16)
    frame.pack(fill="both", expand=True)

    heading = tk.Label(
        frame,
        anchor="w",
        justify="left",
        text="Choose one or more build targets:",
    )
    heading.pack(fill="x", pady=(0, 8))

    host = _host_target()
    values: dict[str, tk.BooleanVar] = {}
    for target in _TARGETS:
        value = tk.BooleanVar(value=False)
        values[target.identifier] = value
        check = tk.Checkbutton(
            frame,
            anchor="w",
            justify="left",
            text=_display_label(target, host),
            variable=value,
        )
        check.pack(fill="x", pady=2)

    save = tk.Button(
        frame,
        command=lambda: _save_gui(window, values, output),
        text="Save",
        width=12,
    )
    save.pack(anchor="e", pady=(12, 0))
    window.bind("<Return>", lambda _event: save.invoke())
    window.mainloop()
    return 0


def _parser() -> argparse.ArgumentParser:
    """Build the user and automation command-line surface."""
    parser = argparse.ArgumentParser(
        description="Choose the SHAR build target architectures.",
    )
    parser.add_argument(
        "--list",
        action="store_true",
        help="print supported targets and the exact current host match",
    )
    parser.add_argument(
        "--select",
        action="append",
        default=[],
        metavar="TARGET",
        help="save TARGET without the GUI; repeat to select multiple targets",
    )
    parser.add_argument(
        "--output",
        type=Path,
        help="override the decision path for testing or tooling",
    )
    return parser


def main() -> int:
    """Dispatch listing, explicit selection, or the default checklist UI."""
    args = _parser().parse_args()
    output = args.output
    if output is None:
        output = _root() / _DATA_PATH
    elif not output.is_absolute():
        output = _root() / output

    if args.list:
        if args.select:
            raise SystemExit("arch: --list and --select cannot be combined")
        return _print_targets()
    if args.select:
        return _save_cli(args.select, output)
    return _show_gui(output)


if __name__ == "__main__":
    raise SystemExit(main())
