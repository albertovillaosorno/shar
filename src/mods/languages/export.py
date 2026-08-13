# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT

"""Emit canonical non-English SHAR language-mod source content."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import sys
from dataclasses import dataclass
from pathlib import Path

_TEXT = Path("art/frontend/scrooby2/resource/txtbible/srr2.txt")
_SCHEMA = "shar.language-mod-source.v1"


@dataclass(frozen=True, slots=True)
class Language:
    name: str
    code: str
    column: int
    dialogue: str
    readme: str | None


LANGUAGES = {
    "french": Language("french", "F", 4, "dialogf.rcf", "Lisez-moi.rtf"),
    "german": Language("german", "G", 5, "dialogg.rcf", "Liesmich.rtf"),
    "italian": Language("italian", "I", 6, "dialogi.rcf", None),
    "spanish": Language("spanish", "S", 7, "dialogs.rcf", "Léeme.rtf"),
}


class ExportError(ValueError):
    """A deterministic language-export failure."""


def _digest(path: Path) -> dict[str, object]:
    digest = hashlib.sha256()
    size = 0
    with path.open("rb") as source:
        while block := source.read(1024 * 1024):
            digest.update(block)
            size += len(block)
    return {"path": path.name, "sha256": digest.hexdigest(), "size": size}


def _parse_table(path: Path, language: Language) -> list[dict[str, str]]:
    try:
        lines = path.read_bytes().decode("cp1252").splitlines()
    except UnicodeDecodeError as error:
        raise ExportError("text table is not valid Windows-1252") from error
    if len(lines) < 5:
        raise ExportError("text table is truncated")
    if lines[0].split("	")[:2] != ["Languages", "EFGIS"]:
        raise ExportError("text table language declaration is not EFGIS")
    columns = lines[2].split("\t")
    names = lines[3].split("\t")
    expected_codes = ["E", "F", "G", "I", "S"]
    if columns[3:8] != expected_codes:
        raise ExportError("text table language columns are not E/F/G/I/S")
    if names[3:8] != ["ENGLISH", "FRENCH", "GERMAN", "ITALIAN", "SPANISH"]:
        raise ExportError("text table language names are not canonical")
    records = []
    for ordinal, line in enumerate(lines[5:], start=6):
        fields = line.split("\t")
        if len(fields) != 9:
            raise ExportError(f"text table row {ordinal} does not have 9 fields")
        records.append(
            {
                "english": fields[3],
                "key": fields[1],
                "notes": fields[8],
                "screen": fields[0],
                "value": fields[language.column],
            }
        )
    if records and all(record["value"] == "???" for record in records):
        raise ExportError(
            f"{language.name} column contains no translated text in this source"
        )
    return records


def _write_jsonl(path: Path, records: list[dict[str, str]]) -> None:
    with path.open("x", encoding="utf-8", newline="\n") as output:
        for record in records:
            output.write(json.dumps(record, ensure_ascii=False, sort_keys=True))
            output.write("\n")


def _copy(source: Path, destination: Path) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    with source.open("rb") as reader, destination.open("xb") as writer:
        shutil.copyfileobj(reader, writer, length=1024 * 1024)


def export_language(game: Path, output: Path, language: Language) -> dict[str, object]:
    """Atomically export one inspectable non-English localization workspace."""
    if os.path.lexists(output):
        raise ExportError("output already exists")
    game_identity = game.resolve(strict=True)
    output_identity = output.resolve(strict=False)
    if output_identity == game_identity or output_identity.is_relative_to(
        game_identity
    ):
        raise ExportError("output must be outside the source game directory")
    table = game / _TEXT
    if not table.is_file():
        raise ExportError("canonical srr2.txt source table is missing")
    records = _parse_table(table, language)
    staging = output.with_name(f".{output.name}.language-{os.getpid()}.tmp")
    if os.path.lexists(staging):
        raise ExportError("staging output already exists")
    output.parent.mkdir(parents=True, exist_ok=True)
    sources = [(_TEXT, "srr2.txt")]
    sidecar = _TEXT.with_name(f"srr2.{language.code}")
    sources.append((sidecar, sidecar.name))
    sources.append((Path(language.dialogue), language.dialogue))
    if language.readme is not None:
        sources.append((Path(language.readme), language.readme))
    try:
        staging.mkdir()
        source_dir = staging / "source"
        included = []
        missing = []
        for relative, name in sources:
            source = game / relative
            if source.is_symlink():
                raise ExportError(f"source must not be a symbolic link: {relative}")
            if source.is_file():
                target = source_dir / name
                _copy(source, target)
                included.append(_digest(target))
            else:
                missing.append(relative.as_posix())
        _write_jsonl(staging / "text.jsonl", records)
        manifest = {
            "base_language": "english",
            "included_sources": included,
            "language": language.name,
            "language_code": language.code,
            "missing_optional_sources": missing,
            "records": len(records),
            "schema": _SCHEMA,
            "untranslated_placeholders": sum(
                1 for record in records if record["value"] == "???"
            ),
            "status": "source-bundle-needs-final-mod-package-adaptation",
        }
        (staging / "manifest.json").write_text(
            json.dumps(manifest, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
            newline="\n",
        )
        os.replace(staging, output)
        return manifest
    except BaseException:
        shutil.rmtree(staging, ignore_errors=True)
        raise


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        prog="shar-language-mod",
        description="Emit one canonical official-language SHAR mod source bundle.",
        allow_abbrev=False,
    )
    parser.add_argument("language", choices=sorted(LANGUAGES))
    parser.add_argument("game", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args(argv)
    try:
        manifest = export_language(args.game, args.output, LANGUAGES[args.language])
    except (ExportError, OSError) as error:
        print(f"shar-language-mod: {error}", file=sys.stderr)
        return 1
    print(json.dumps(manifest, ensure_ascii=False, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
