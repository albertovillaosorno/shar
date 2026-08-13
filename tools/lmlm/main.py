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
#   - Standalone LMLM inspection and open conversion workspaces.
# - Must-Not:
#   - Participate in the base build, auto-install mods, or overwrite output.
# - Allows:
#   - Explicit local LMLM inputs and inspectable conversion outputs.
# - Split-When:
#   - Final SHAR package adaptation gains a stable independent schema.
# - Merge-When:
#   - LMLM compatibility is retired.
# - Summary:
#   - Converts supported LMLM archives outside the base pipeline.
# - Description:
#   - Creates an open conversion workspace for later SHAR mod adaptation.
# - Usage:
#   - Run `python main.py inspect ...` or `python main.py convert ...`.
# - Defaults:
#   - Decompilable-only, atomic, non-installing, fail-closed conversion.
#

"""Standalone Python compatibility tool for LMLM mod conversion."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import sys
import zipfile
from pathlib import Path

from archive import FileEntry, LmlmError, entry_bytes, parse_archive

_REPORT_SCHEMA = "shar.lmlm-conversion.v1"
_DECOMPILABLE_MODS_ONLY = True


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _load(path: Path) -> tuple[bytes, tuple[FileEntry, ...]]:
    if path.suffix.casefold() != ".lmlm":
        raise LmlmError("input must have the .lmlm extension")
    if not path.is_file():
        raise LmlmError("input LMLM file does not exist")
    data = path.read_bytes()
    return data, parse_archive(data)


def _entry_record(data: bytes, entry: FileEntry) -> dict[str, object]:
    payload = entry_bytes(data, entry)
    return {
        "path": entry.path,
        "sha256": _sha256(payload),
        "size": entry.size,
    }


def _report(
    data: bytes,
    entries: tuple[FileEntry, ...],
) -> dict[str, object]:
    return {
        "decompilable_mods_only": _DECOMPILABLE_MODS_ONLY,
        "entries": [_entry_record(data, entry) for entry in entries],
        "schema": _REPORT_SCHEMA,
        "source": {"sha256": _sha256(data), "size": len(data)},
        "status": "extracted-needs-shar-package-adaptation",
    }


def _write_json(path: Path, payload: object) -> None:
    text = json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True)
    path.write_text(text + "\n", encoding="utf-8", newline="\n")


def _staging_path(output: Path) -> Path:
    return output.with_name(f".{output.name}.lmlm-{os.getpid()}.tmp")


def _materialize(
    data: bytes,
    entries: tuple[FileEntry, ...],
    root: Path,
) -> None:
    content = root / "content"
    content.mkdir(parents=True, exist_ok=False)
    for entry in entries:
        target = content.joinpath(*entry.path.split("/"))
        target.parent.mkdir(parents=True, exist_ok=True)
        with target.open("xb") as handle:
            handle.write(entry_bytes(data, entry))
    _write_json(root / "conversion-report.json", _report(data, entries))


def _zip_workspace(root: Path, destination: Path) -> None:
    if os.path.lexists(destination):
        raise LmlmError("ZIP output already exists")
    candidate = destination.with_name(
        f".{destination.name}.lmlm-{os.getpid()}.tmp"
    )
    if os.path.lexists(candidate):
        raise LmlmError("ZIP staging path already exists")
    try:
        with zipfile.ZipFile(
            candidate,
            mode="x",
            compression=zipfile.ZIP_DEFLATED,
            compresslevel=9,
        ) as archive:
            paths = sorted(root.rglob("*"), key=lambda path: path.as_posix())
            for path in paths:
                if path.is_file():
                    archive.write(path, path.relative_to(root).as_posix())
        os.replace(candidate, destination)
    finally:
        candidate.unlink(missing_ok=True)


def convert(
    input_path: Path,
    output: Path,
    zip_output: Path | None,
) -> dict[str, object]:
    """Create one atomic inspectable conversion workspace."""
    if os.path.lexists(output):
        raise LmlmError("conversion output already exists")
    staging = _staging_path(output)
    if os.path.lexists(staging):
        raise LmlmError("conversion staging path already exists")
    if zip_output is not None:
        if os.path.lexists(zip_output):
            raise LmlmError("ZIP output already exists")
        output_identity = output.resolve(strict=False)
        zip_identity = zip_output.resolve(strict=False)
        if zip_identity == output_identity or zip_identity.is_relative_to(
            output_identity
        ):
            raise LmlmError("ZIP output must be outside the conversion directory")
    data, entries = _load(input_path)
    output.parent.mkdir(parents=True, exist_ok=True)
    if zip_output is not None:
        zip_output.parent.mkdir(parents=True, exist_ok=True)
    try:
        staging.mkdir()
        _materialize(data, entries, staging)
        os.replace(staging, output)
    except BaseException:
        shutil.rmtree(staging, ignore_errors=True)
        raise
    if zip_output is not None:
        _zip_workspace(output, zip_output)
    return _report(data, entries)


def inspect(input_path: Path) -> dict[str, object]:
    """Return deterministic read-only archive evidence."""
    data, entries = _load(input_path)
    return _report(data, entries)


def _parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="shar-lmlm",
        description=(
            "Inspect or decompile a supported legacy LMLM mod without "
            "installing it into SHAR."
        ),
        allow_abbrev=False,
    )
    subcommands = parser.add_subparsers(dest="command", required=True)
    inspect_parser = subcommands.add_parser(
        "inspect",
        help="validate an archive and print deterministic JSON evidence",
        allow_abbrev=False,
    )
    inspect_parser.add_argument("input", type=Path)
    convert_parser = subcommands.add_parser(
        "convert",
        help="create an open conversion workspace",
        allow_abbrev=False,
    )
    convert_parser.add_argument("input", type=Path)
    convert_parser.add_argument("output", type=Path)
    convert_parser.add_argument(
        "--zip",
        dest="zip_output",
        type=Path,
        help="also write a ZIP copy of the open conversion workspace",
    )
    return parser


def main(argv: list[str] | None = None) -> int:
    """Run the standalone converter CLI."""
    args = _parser().parse_args(argv)
    try:
        if args.command == "inspect":
            payload = inspect(args.input)
        else:
            payload = convert(args.input, args.output, args.zip_output)
    except (LmlmError, OSError, zipfile.BadZipFile) as error:
        print(f"shar-lmlm: {error}", file=sys.stderr)
        return 1
    print(json.dumps(payload, ensure_ascii=False, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
