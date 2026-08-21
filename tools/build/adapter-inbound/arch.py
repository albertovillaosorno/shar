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
#   - Run tools/build/adapter-inbound/arch.py and choose at least one target.
# - Defaults:
#   - No target is selected automatically.
#

"""Select supported SHAR build targets and persist the decision."""

from __future__ import annotations

import argparse
import importlib
import json
import os
from pathlib import Path
import platform
import stat
import sys
from types import ModuleType
from typing import NamedTuple
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    import tkinter as tk

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
    return Path(__file__).resolve().parents[3]


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


def _validate_canonical_output_root(root: Path, output: Path) -> None:
    """Reject linked or malformed canonical build-data ancestors."""
    canonical = root / _DATA_PATH
    if output != canonical:
        return
    roots = (
        (root / ".cache", "repository cache root"),
        (root / ".cache/build", "build cache root"),
        (root / ".cache/build/data", "build data root"),
    )
    for path, label in roots:
        if not os.path.lexists(path):
            continue
        is_real = (
            path.is_dir()
            and not path.is_symlink()
            and not os.path.isjunction(path)
        )
        if is_real:
            continue
        raise SystemExit(f"arch: {label} must be a real directory: {path}")
    if os.path.lexists(output) and (
        not output.is_file() or output.is_symlink()
    ):
        raise SystemExit(
            f"arch: architecture evidence must be a real file: {output}"
        )


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
    created = False
    try:
        with candidate.open(
            "x",
            encoding="utf-8",
            newline="\n",
        ) as handle:
            created = True
            handle.write(data + "\n")
        Path(candidate).replace(path)
    finally:
        if created:
            candidate.unlink(missing_ok=True)


def _selected_targets(identifiers: list[str]) -> list[Target]:
    """Resolve explicit target identifiers in canonical display order."""
    unknown = sorted(set(identifiers) - _TARGETS_BY_ID.keys())
    if unknown:
        names = ", ".join(unknown)
        raise SystemExit(f"arch: unsupported target identifier: {names}")
    requested = set(identifiers)
    if len(requested) != len(identifiers):
        raise SystemExit("arch: duplicate target identifiers are not allowed")
    return [target for target in _TARGETS if target.identifier in requested]


def _unique_json_object(
    pairs: list[tuple[str, object]],
) -> dict[str, object]:
    """Reject duplicate keys at every JSON object depth."""
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def _selection_identity(metadata: os.stat_result) -> tuple[int, ...]:
    """Return filesystem identity used to bind one saved selection read."""
    return (
        metadata.st_dev,
        metadata.st_ino,
        metadata.st_mtime_ns,
        metadata.st_ctime_ns,
        metadata.st_size,
        metadata.st_nlink,
    )


def _real_selection_identity(path: Path) -> tuple[int, ...]:
    """Require one non-redirected single-link saved selection file."""
    if path.is_symlink() or os.path.isjunction(path):
        raise SystemExit("arch: saved selection must be a real file")
    try:
        metadata = path.stat(follow_symlinks=False)
    except OSError as error:
        message = f"arch: cannot read saved selection: {error}"
        raise SystemExit(message) from error
    if not stat.S_ISREG(metadata.st_mode) or metadata.st_nlink != 1:
        raise SystemExit("arch: saved selection must be a real file")
    return _selection_identity(metadata)


def _selection_snapshot(path: Path) -> bytes:
    """Read one stable architecture-selection byte snapshot."""
    expected = _real_selection_identity(path)
    try:
        with path.open("rb") as handle:
            opened = _selection_identity(os.fstat(handle.fileno()))
            if opened != expected:
                raise SystemExit(
                    "arch: saved selection changed during revalidation"
                )
            snapshot = handle.read()
            finished = _selection_identity(os.fstat(handle.fileno()))
    except OSError as error:
        message = f"arch: cannot read saved selection: {error}"
        raise SystemExit(message) from error
    if (
        finished != expected
        or len(snapshot) != expected[4]
        or _real_selection_identity(path) != expected
    ):
        raise SystemExit("arch: saved selection changed during revalidation")
    return snapshot


def _require_selection_snapshot(path: Path, snapshot: bytes) -> None:
    """Require saved architecture bytes to remain unchanged."""
    if _selection_snapshot(path) != snapshot:
        raise SystemExit("arch: saved selection changed during revalidation")


def _revalidate_selection(path: Path) -> int:
    """Require saved architecture evidence to match canonical current policy."""
    snapshot = _selection_snapshot(path)
    try:
        value = json.loads(
            snapshot.decode("utf-8"),
            object_pairs_hook=_unique_json_object,
        )
    except (UnicodeError, ValueError) as error:
        message = f"arch: cannot read saved selection: {error}"
        raise SystemExit(message) from error
    if not isinstance(value, dict):
        raise SystemExit("arch: saved selection must be a JSON object")
    if set(value) != {"host", "schema", "targets"}:
        raise SystemExit("arch: saved selection has invalid top-level keys")
    if value.get("schema") != _SCHEMA:
        raise SystemExit(f"arch: saved selection schema must be {_SCHEMA}")
    raw_targets = value.get("targets")
    if not isinstance(raw_targets, list) or not raw_targets:
        raise SystemExit("arch: saved selection must contain target records")
    identifiers: list[str] = []
    for raw in raw_targets:
        if not isinstance(raw, dict):
            raise SystemExit("arch: saved target must be a JSON object")
        identifier = raw.get("id")
        if not isinstance(identifier, str) or identifier not in _TARGETS_BY_ID:
            raise SystemExit(f"arch: unsupported saved target: {identifier!r}")
        identifiers.append(identifier)
    selected = _selected_targets(identifiers)
    if len(selected) != len(identifiers):
        raise SystemExit("arch: saved selection contains duplicate targets")
    expected = _payload(selected)
    if value != expected:
        raise SystemExit(
            "arch: saved selection no longer matches this host or target policy"
        )
    _require_selection_snapshot(path, snapshot)
    print(f"arch: revalidated {len(selected)} saved target(s) at {path}")
    return 0


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
    messagebox: ModuleType,
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
        tk = importlib.import_module("tkinter")
        messagebox = importlib.import_module("tkinter.messagebox")
    except ImportError as error:
        print(
            f"arch: checklist GUI support is unavailable: {error}",
            file=sys.stderr,
        )
        return 2
    try:
        window = tk.Tk()
    except tk.TclError as error:
        print(
            f"arch: could not open the checklist window: {error}",
            file=sys.stderr,
        )
        return 2

    window.title("SHAR build targets")
    window.resizable(width=False, height=False)
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
        command=lambda: _save_gui(window, values, output, messagebox),
        text="Save",
        width=12,
    )
    save.pack(anchor="e", pady=(12, 0))

    def invoke_save(event: object) -> None:
        del event
        save.invoke()

    window.bind("<Return>", invoke_save)
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
    parser.add_argument(
        "--revalidate",
        action="store_true",
        help="revalidate the saved selection without opening the checklist",
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

    modes = int(args.list) + int(bool(args.select)) + int(args.revalidate)
    if modes > 1:
        raise SystemExit(
            "arch: --list, --select, and --revalidate cannot be combined"
        )
    if args.list:
        return _print_targets()
    _validate_canonical_output_root(_root(), output)
    if args.select:
        return _save_cli(args.select, output)
    if args.revalidate:
        return _revalidate_selection(output)
    return _show_gui(output)


if __name__ == "__main__":
    raise SystemExit(main())
