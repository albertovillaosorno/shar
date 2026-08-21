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
#   - Content-free calibration metrics for source-manifest count vectors.
# - Must-Not:
#   - Admit installations, read game payloads, use hashes as source, or encode
#     proprietary file names/bytes into reconstruction procedures.
# - Allows:
#   - JSONL manifest coordinates containing directory aliases, extensions,
#     and counts.
# - Split-When:
#   - A calibrated production admission policy gains an independent lifecycle.
# - Merge-When:
#   - Source validation owns the same evidence-only metric.
# - Summary:
#   - Measures candidate structural coverage without becoming an admission gate.
# - Description:
#   - Computes count-vector coverage and weighted-Jaccard similarity only.
# - Usage:
#   - Feed lawful reference/candidate count ledgers for empirical calibration.
# - Defaults:
#   - No threshold exists and no pass/fail decision is returned.
#

"""Content-free source-installation similarity calibration."""

# CSpell:ignore Liesmich Lisez Léeme textbible uninst

from __future__ import annotations

from collections.abc import Mapping
from collections.abc import Sequence
from dataclasses import dataclass
from fractions import Fraction
import json
import os
from pathlib import Path
import stat
import sys
from typing import Any

Coordinate = tuple[str, str]
_MANIFEST_SCHEMA = "shar-schoenwald.game-manifest-ledger.v2"
_MAX_COUNT = (1 << 64) - 1
_MAX_COUNT_TEXT = str(_MAX_COUNT)
_MANIFEST_KIND_TAXONOMY = (
    "error",
    "language_textbible",
    "movie",
    "p3d_container",
    "rcf_container",
    "audio",
    "music_arrangement",
    "script",
    "image",
    "generated_artifact",
    "character_outfit",
    "build-log",
    "document",
    "ui-resource",
    "sound-type",
    "metadata",
    "json-ledger",
)
_MANIFEST_KINDS = frozenset(_MANIFEST_KIND_TAXONOMY)
_MANIFEST_REQUIRED_FILE_METADATA = (
    ("README.rtf", 1),
    ("Simpsons.exe", 1),
    ("Simpsons.ico", 1),
    ("art/frontend/scrooby2/resource/txtbible/srr2.E", 1),
    ("art/frontend/scrooby2/resource/txtbible/srr2.F", 0),
    ("art/frontend/scrooby2/resource/txtbible/srr2.G", 0),
    ("art/frontend/scrooby2/resource/txtbible/srr2.I", 0),
    ("art/frontend/scrooby2/resource/txtbible/srr2.S", 0),
    ("art/frontend/scrooby2/resource/txtbible/srr2.txt", 1),
    ("dialog.rcf", 1),
    ("dialogf.rcf", 0),
    ("dialogg.rcf", 0),
    ("dialogi.rcf", 0),
    ("dialogs.rcf", 0),
    ("Liesmich.rtf", 0),
    ("Lisez-moi.rtf", 0),
    ("Léeme.rtf", 0),
    ("uninst.ico", 0),
)
_MANIFEST_REQUIRED_FILES = frozenset(_MANIFEST_REQUIRED_FILE_METADATA)
_MANIFEST_KIND_BY_EXTENSION = {
    "bik": "movie",
    "bk2": "movie",
    "bmp": "image",
    "cho": "character_outfit",
    "con": "script",
    "e": "language_textbible",
    "err": "build-log",
    "f": "language_textbible",
    "g": "language_textbible",
    "i": "language_textbible",
    "ico": "image",
    "jpeg": "image",
    "jpg": "image",
    "json": "metadata",
    "jsonl": "json-ledger",
    "lua": "script",
    "mfk": "script",
    "p3d": "p3d_container",
    "pag": "ui-resource",
    "png": "image",
    "prj": "ui-resource",
    "rcf": "rcf_container",
    "rms": "music_arrangement",
    "rmv": "movie",
    "rsd": "audio",
    "rsm": "music_arrangement",
    "s": "language_textbible",
    "scr": "ui-resource",
    "tga": "image",
    "txt": "language_textbible",
    "typ": "sound-type",
    "wav": "audio",
    "x": "language_textbible",
}


class SimilarityInputError(ValueError):
    """Base fail-closed source-similarity input error."""


class EmptyReferenceError(SimilarityInputError):
    """The reference count vector contains no units."""

    def __init__(self) -> None:
        """Initialize the canonical empty-reference failure."""
        super().__init__("reference count vector must not be empty")


class EmptyUnionError(SimilarityInputError):
    """Reference and candidate produce no count-vector union."""

    def __init__(self) -> None:
        """Initialize the canonical empty-union failure."""
        super().__init__("count-vector union must not be empty")


class InvalidCoordinateError(SimilarityInputError):
    """A count-vector coordinate is not a pair of strings."""

    def __init__(self) -> None:
        """Initialize the canonical coordinate-shape failure."""
        super().__init__(
            "coordinate must be a pair of strings with normalized directory "
            "and extension"
        )


class InvalidCountError(SimilarityInputError):
    """A coordinate count is not a nonnegative integer."""

    def __init__(self) -> None:
        """Initialize the canonical coordinate-count failure."""
        super().__init__(
            "coordinate count must be a nonnegative integer within 64-bit range"
        )


class LedgerInputError(SimilarityInputError):
    """A public JSONL count ledger is malformed or unreadable."""


def _parse_json_integer(lexeme: str) -> int:
    """Decode one JSON integer while rejecting the producer-impossible -0."""
    if lexeme == "-0":
        raise ValueError("signed zero is not canonical")
    return int(lexeme)


def _object_without_duplicates(
    pairs: list[tuple[str, Any]],
) -> dict[str, Any]:
    """Build one JSON object while rejecting duplicate member names."""
    record: dict[str, Any] = {}
    for key, value in pairs:
        if key in record:
            raise LedgerInputError("count ledger repeats a JSON object key")
        record[key] = value
    return record


@dataclass(frozen=True, slots=True)
class _LedgerIdentity:
    """Stable local identity captured around one calibration-ledger read."""

    device: int
    inode: int
    modified_ns: int
    ctime_ns: int
    size: int


def _ledger_identity(metadata: os.stat_result) -> _LedgerIdentity:
    """Project path-free filesystem evidence needed to detect ledger drift."""
    return _LedgerIdentity(
        device=metadata.st_dev,
        inode=metadata.st_ino,
        modified_ns=metadata.st_mtime_ns,
        ctime_ns=metadata.st_ctime_ns,
        size=metadata.st_size,
    )


def _regular_ledger_identity(path: Path) -> _LedgerIdentity:
    """Return one non-redirected regular-file identity for a count ledger."""
    try:
        redirected_parent = any(
            parent.is_symlink() or os.path.isjunction(parent)
            for parent in (path.parent, *path.parent.parents)
        )
        if redirected_parent:
            raise LedgerInputError(
                "count ledger parent must be a real directory"
            )
        metadata = path.lstat()
    except OSError as error:
        raise LedgerInputError(
            "count ledger could not be read as UTF-8"
        ) from error
    if not stat.S_ISREG(metadata.st_mode):
        raise LedgerInputError("count ledger must be a regular file")
    return _ledger_identity(metadata)


def _current_ledger_identity(path: Path) -> _LedgerIdentity | None:
    """Return the final regular-file identity, or `None` after path drift."""
    try:
        return _regular_ledger_identity(path)
    except LedgerInputError:
        return None


@dataclass(frozen=True, slots=True)
class SimilarityEvidence:
    """Pure calibration evidence with no admission decision."""

    reference_units: int
    candidate_units: int
    shared_units: int
    union_units: int
    reference_coverage: Fraction
    weighted_jaccard: Fraction


def _has_escaped_json_member_name(line: str) -> bool:
    """Return whether one JSON object key uses a backslash escape."""
    index = 0
    while index < len(line):
        if line[index] != '"':
            index += 1
            continue
        index += 1
        escaped = False
        used_escape = False
        while index < len(line):
            character = line[index]
            if escaped:
                escaped = False
                index += 1
                continue
            if character == "\\":
                escaped = True
                used_escape = True
                index += 1
                continue
            if character == '"':
                break
            index += 1
        if index + 1 < len(line) and line[index + 1] == ":" and used_escape:
            return True
        index += 1
    return False


def _has_json_token_whitespace(line: str) -> bool:
    """Return whether JSON tokens are separated by spaces or tabs."""
    in_string = False
    escaped = False
    for character in line:
        if in_string:
            if escaped:
                escaped = False
            elif character == "\\":
                escaped = True
            elif character == '"':
                in_string = False
            continue
        if character == '"':
            in_string = True
        elif character in {" ", "\t", "\r", "\n"}:
            return True
    return False


def _parse_jsonl_record(line: str, line_number: int) -> dict[str, Any]:
    """Parse one JSONL object while preserving a path-free line diagnostic."""
    if not line.strip():
        raise LedgerInputError(
            f"count ledger has empty JSONL record at line {line_number}"
        )
    if line.strip() != line:
        raise LedgerInputError(
            f"count ledger record has outer whitespace at line {line_number}"
        )
    if _has_json_token_whitespace(line):
        raise LedgerInputError(
            f"count ledger JSON tokens have whitespace at line {line_number}"
        )
    if _has_escaped_json_member_name(line):
        raise LedgerInputError(
            "count ledger JSON member name uses an escape "
            f"at line {line_number}"
        )
    try:
        record: Any = json.loads(
            line,
            object_pairs_hook=_object_without_duplicates,
            parse_int=_parse_json_integer,
        )
    except (ValueError, RecursionError) as error:
        raise LedgerInputError(
            f"count ledger contains invalid JSONL at line {line_number}"
        ) from error
    if not isinstance(record, dict):
        raise LedgerInputError(
            f"count ledger record must be an object at line {line_number}"
        )
    return record


def _valid_utf8_text(value: object) -> bool:
    """Return whether one JSON value is text made of Unicode scalar values."""
    if not isinstance(value, str):
        return False
    try:
        value.encode("utf-8")
    except UnicodeEncodeError:
        return False
    return True


def _valid_manifest_kind(value: object) -> bool:
    """Return whether one value is a public manifest kind token."""
    return _valid_utf8_text(value) and value in _MANIFEST_KINDS


def _expected_manifest_kind(directory: str, extension: str) -> str:
    """Mirror the public manifest producer's stable bucket classification."""
    if extension == "png" and not directory:
        return "generated_artifact"
    if extension == "p3d" and directory in {
        "at/fd/sy/re/te",
        "at/fd/s2/re/te",
    }:
        return "language_textbible"
    return _MANIFEST_KIND_BY_EXTENSION.get(extension, "error")


_LOWERCASE_EXPANSION = "i\u0307"
_RUST17_LOWERCASE_PLUS_ONE = frozenset({0xA7CE, 0xA7D2, 0xA7D4})
_RUST17_BERIA_ERFE_UPPER_START = 0x16EA0
_RUST17_BERIA_ERFE_UPPER_END = 0x16EB8
_RUST17_BERIA_ERFE_CASE_OFFSET = 0x1B


def _rust_lower_character(character: str) -> str:
    """Mirror Rust 1.97 Unicode 17 lowercase beyond Python Unicode 16."""
    code = ord(character)
    if code in _RUST17_LOWERCASE_PLUS_ONE:
        return chr(code + 1)
    if _RUST17_BERIA_ERFE_UPPER_START <= code <= _RUST17_BERIA_ERFE_UPPER_END:
        return chr(code + _RUST17_BERIA_ERFE_CASE_OFFSET)
    return character.lower()


def _rust_lower(value: str) -> str:
    """Lower text with the producer's character-by-character Rust mapping."""
    return "".join(_rust_lower_character(character) for character in value)


def _valid_obfuscated_component(component: str) -> bool:
    """Return whether one component can be two lowercased endpoint chars."""
    size = len(component)
    if size == 2:
        return True
    if size == 3:
        return component.startswith(_LOWERCASE_EXPANSION) or component.endswith(
            _LOWERCASE_EXPANSION
        )
    return size == 4 and component == _LOWERCASE_EXPANSION * 2


def _valid_directory_alias(value: object) -> bool:
    """Return whether a directory coordinate matches producer normalization."""
    if not _valid_utf8_text(value):
        return False
    if not value:
        return True
    if (
        value != _rust_lower(value)
        or chr(92) in value
        or value.startswith("/")
        or value.endswith("/")
        or "//" in value
    ):
        return False
    base = _collision_family(value)
    return all(
        _valid_public_path_component(component)
        and _valid_obfuscated_component(component)
        for component in base.split("/")
    )


def _valid_extension(value: object) -> bool:
    """Return whether an extension matches producer normalization."""
    return _valid_utf8_text(value) and bool(value) and value == value.lower()


_RESERVED_HOST_STEMS = frozenset({
    "AUX",
    "CLOCK$",
    "CON",
    "CONIN$",
    "CONOUT$",
    "NUL",
    "PRN",
})
_RESERVED_HOST_SUFFIXES = frozenset({
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "¹",
    "²",
    "³",
})


def _is_unicode_path_modifier(character: str) -> bool:
    """Return whether one character can conceal a portable path identity."""
    code = ord(character)
    return (
        code == 0x061C
        or 0x200B <= code <= 0x200F
        or 0x2028 <= code <= 0x202E
        or 0x2060 <= code <= 0x2064
        or 0x2066 <= code <= 0x206F
        or 0xFE00 <= code <= 0xFE0F
        or code == 0xFEFF
    )


def _is_reserved_host_alias(component: str) -> bool:
    """Return whether one component aliases a Windows device identity."""
    stem = component.split(".", 1)[0].rstrip(" .").upper()
    if stem in _RESERVED_HOST_STEMS:
        return True
    for prefix in ("COM", "LPT"):
        suffix = stem.removeprefix(prefix)
        if suffix != stem and suffix in _RESERVED_HOST_SUFFIXES:
            return True
    return False


def _valid_public_path_component(component: str) -> bool:
    """Return whether one component follows the portable filesystem policy."""
    if component in {"", ".", ".."} or component.endswith((".", " ")):
        return False
    if any(character in '<>:"|?*' for character in component):
        return False
    if any(
        ord(character) < 32 or 127 <= ord(character) <= 159
        for character in component
    ):
        return False
    if any(_is_unicode_path_modifier(character) for character in component):
        return False
    if len(component.encode("utf-16-le")) // 2 > 255:
        return False
    return not _is_reserved_host_alias(component)


def _valid_public_relative_path(value: object) -> bool:
    """Return whether one metadata path is portable and source-root relative."""
    if not _valid_utf8_text(value) or not value or chr(92) in value:
        return False
    if value.startswith("/") or value.endswith("/") or "//" in value:
        return False
    return all(
        _valid_public_path_component(component)
        for component in value.split("/")
    )


def _valid_required_file_metadata(value: object) -> bool:
    """Return whether one public required-file record has canonical shape."""
    if not isinstance(value, dict) or tuple(value) != ("path", "min"):
        return False
    minimum = value["min"]
    if (
        not _valid_public_relative_path(value["path"])
        or isinstance(minimum, bool)
        or not isinstance(minimum, int)
        or not 0 <= minimum <= _MAX_COUNT
    ):
        return False
    return (value["path"], minimum) in _MANIFEST_REQUIRED_FILES


def _validate_schema_metadata(record: dict[str, Any]) -> None:
    """Require only public manifest header metadata shapes."""
    canonical_order = ("schema", "kind_taxonomy", "required_files")
    if set(record) - set(canonical_order):
        raise LedgerInputError("count ledger schema record has unknown fields")
    expected_order = tuple(
        field for field in canonical_order if field in record
    )
    if tuple(record) != expected_order:
        raise LedgerInputError(
            "count ledger schema member order is not canonical"
        )
    if "kind_taxonomy" in record:
        taxonomy = record["kind_taxonomy"]
        if not isinstance(taxonomy, list) or taxonomy != list(
            _MANIFEST_KIND_TAXONOMY
        ):
            raise LedgerInputError("count ledger kind taxonomy is invalid")
    if "required_files" in record:
        required_files = record["required_files"]
        canonical_required_files = [
            {"path": path, "min": minimum}
            for path, minimum in _MANIFEST_REQUIRED_FILE_METADATA
        ]
        if (
            not isinstance(required_files, list)
            or not all(
                _valid_required_file_metadata(requirement)
                for requirement in required_files
            )
            or required_files != canonical_required_files
        ):
            raise LedgerInputError(
                "count ledger required-files metadata is invalid"
            )


def _is_schema_record(record: dict[str, Any], line_number: int) -> bool:
    """Validate and identify an optional first-line manifest schema record."""
    if "schema" not in record:
        return False
    if line_number != 1 or record.get("schema") != _MANIFEST_SCHEMA:
        raise LedgerInputError("count ledger schema record is invalid")
    _validate_schema_metadata(record)
    return True


def _coordinate_record(record: dict[str, Any]) -> tuple[Coordinate, int]:
    """Project one manifest row to the public-safe calibration coordinate."""
    if set(record) - {"dir", "ext", "min", "count", "kind"}:
        raise LedgerInputError("count ledger record has unknown fields")
    has_minimum = "min" in record
    has_observed = "count" in record
    if has_minimum == has_observed:
        raise LedgerInputError(
            "count ledger record must select one count field"
        )
    count_field = "count" if has_observed else "min"
    expected_order = ("dir", "ext", count_field) + (
        ("kind",) if "kind" in record else ()
    )
    if tuple(record) != expected_order:
        raise LedgerInputError("count ledger member order is not canonical")
    directory = record.get("dir")
    extension = record.get("ext")
    count = record.get(count_field)
    kind = record.get("kind")
    if not _valid_directory_alias(directory) or not _valid_extension(extension):
        raise InvalidCoordinateError
    expected_kind = _expected_manifest_kind(directory, extension)
    if (
        expected_kind == "error"
        or ("kind" in record and not _valid_manifest_kind(kind))
        or ("kind" in record and kind != expected_kind)
    ):
        raise LedgerInputError("count ledger kind metadata is invalid")
    if (
        isinstance(count, bool)
        or not isinstance(count, int)
        or count < 0
        or count > _MAX_COUNT
    ):
        raise InvalidCountError
    if count_field == "count" and count == 0:
        raise InvalidCountError
    return (directory, extension), count


def _ledger_lines(text: str) -> list[str]:
    """Split JSONL only on LF while tolerating one terminal LF."""
    if not text:
        return []
    lines = text.split("\n")
    if text.endswith("\n"):
        lines.pop()
    return lines


def parse_count_ledger(text: str) -> dict[Coordinate, int]:
    """Parse public manifest coordinates without retaining metadata payloads."""
    counts: dict[Coordinate, int] = {}
    count_field: str | None = None
    previous_coordinate: Coordinate | None = None
    for line_number, line in enumerate(_ledger_lines(text), start=1):
        record = _parse_jsonl_record(line, line_number)
        if _is_schema_record(record, line_number):
            continue
        coordinate, count = _coordinate_record(record)
        record_count_field = "count" if "count" in record else "min"
        if count_field is None:
            count_field = record_count_field
        elif count_field != record_count_field:
            raise LedgerInputError("count ledger mixes count meanings")
        if coordinate in counts:
            raise LedgerInputError(
                f"count ledger repeats a coordinate at line {line_number}"
            )
        if previous_coordinate is not None and coordinate < previous_coordinate:
            raise LedgerInputError(
                "count ledger coordinate order is not canonical"
            )
        counts[coordinate] = count
        previous_coordinate = coordinate
    if not counts:
        raise LedgerInputError("count ledger contains no coordinates")
    _validate_ledger_collision_families(counts)
    _validate(counts)
    return counts


def load_count_ledger(path: Path) -> dict[Coordinate, int]:
    """Load one stable canonical JSONL ledger without disclosing its path."""
    expected = _regular_ledger_identity(path)
    try:
        with path.open("rb") as handle:
            opened = _ledger_identity(os.fstat(handle.fileno()))
            if opened != expected or _current_ledger_identity(path) != expected:
                raise LedgerInputError("count ledger changed while reading")
            data = handle.read()
            finished = _ledger_identity(os.fstat(handle.fileno()))
    except OSError as error:
        raise LedgerInputError(
            "count ledger could not be read as UTF-8"
        ) from error
    if (
        finished != expected
        or _current_ledger_identity(path) != expected
        or len(data) != expected.size
    ):
        raise LedgerInputError("count ledger changed while reading")
    try:
        text = data.decode("utf-8")
    except UnicodeError as error:
        raise LedgerInputError(
            "count ledger could not be read as UTF-8"
        ) from error
    if "\r" in text:
        raise LedgerInputError("count ledger must use LF line endings")
    if not text.endswith("\n"):
        raise LedgerInputError("count ledger must end with LF")
    return parse_count_ledger(text)


def measure(
    reference: Mapping[Coordinate, int],
    candidate: Mapping[Coordinate, int],
) -> SimilarityEvidence:
    """Measure nonnegative count vectors without selecting a threshold.

    Raises:
        EmptyReferenceError: If the reference vector has no units.
        EmptyUnionError: If both vectors produce an empty union.

    """
    reference_snapshot = _validated_snapshot(reference)
    candidate_snapshot = _validated_snapshot(candidate)
    if not candidate_snapshot:
        raise LedgerInputError("candidate count vector contains no coordinates")
    reference_units = sum(reference_snapshot.values())
    candidate_units = sum(candidate_snapshot.values())
    if reference_units == 0:
        raise EmptyReferenceError
    comparable_reference = _collapse_collision_families(reference_snapshot)
    comparable_candidate = _collapse_collision_families(candidate_snapshot)
    coordinates = set(comparable_reference) | set(comparable_candidate)
    shared = sum(
        min(comparable_reference.get(key, 0), comparable_candidate.get(key, 0))
        for key in coordinates
    )
    union = sum(
        max(comparable_reference.get(key, 0), comparable_candidate.get(key, 0))
        for key in coordinates
    )
    if union == 0:
        raise EmptyUnionError
    return SimilarityEvidence(
        reference_units=reference_units,
        candidate_units=candidate_units,
        shared_units=shared,
        union_units=union,
        reference_coverage=Fraction(shared, reference_units),
        weighted_jaccard=Fraction(shared, union),
    )


def _generated_collision_ordinal(
    directory: str,
) -> tuple[str, int] | None:
    """Return one producer-shaped collision family and ordinal when present."""
    base, marker, ordinal = directory.rpartition("~")
    if not marker or not base:
        return None
    if len(ordinal) < 2 or not ordinal.isascii() or not ordinal.isdigit():
        return None
    if ordinal == "00" or (len(ordinal) > 2 and ordinal.startswith("0")):
        return None
    if len(ordinal) > len(_MAX_COUNT_TEXT) or (
        len(ordinal) == len(_MAX_COUNT_TEXT) and ordinal > _MAX_COUNT_TEXT
    ):
        return None
    return base, int(ordinal)


def _collision_family(directory: str) -> str:
    collision = _generated_collision_ordinal(directory)
    return directory if collision is None else collision[0]


def _validate_ledger_collision_families(
    values: Mapping[Coordinate, int],
) -> None:
    """Require collision aliases to match one complete producer family."""
    plain_directories: set[str] = set()
    families: dict[str, set[int]] = {}
    for coordinate in values:
        directory = coordinate[0]
        collision = _generated_collision_ordinal(directory)
        if collision is None:
            plain_directories.add(directory)
            continue
        base, ordinal = collision
        families.setdefault(base, set()).add(ordinal)
    for base, ordinals in families.items():
        ordered = sorted(ordinals)
        if (
            base in plain_directories
            or len(ordered) < 2
            or any(
                ordinal != expected
                for expected, ordinal in enumerate(ordered, start=1)
            )
        ):
            raise LedgerInputError(
                "count ledger collision ordinals are not canonical"
            )


def _collapse_collision_families(
    values: Mapping[Coordinate, int],
) -> dict[Coordinate, int]:
    collapsed: dict[Coordinate, int] = {}
    for (directory, extension), count in values.items():
        key = (_collision_family(directory), extension)
        collapsed[key] = collapsed.get(key, 0) + count
    return collapsed


def _validated_snapshot(
    values: Mapping[Coordinate, int],
) -> dict[Coordinate, int]:
    """Capture and validate one stable programmatic count-vector snapshot."""
    snapshot = dict(values.items())
    _validate(snapshot)
    _validate_ledger_collision_families(snapshot)
    return snapshot


def _validate(values: Mapping[Coordinate, int]) -> None:
    total = 0
    for key, count in values.items():
        if (
            not isinstance(key, tuple)
            or len(key) != 2
            or not _valid_directory_alias(key[0])
            or not _valid_extension(key[1])
            or _expected_manifest_kind(key[0], key[1]) == "error"
        ):
            raise InvalidCoordinateError
        if (
            isinstance(count, bool)
            or not isinstance(count, int)
            or count < 0
            or count > _MAX_COUNT
            or count > _MAX_COUNT - total
        ):
            raise InvalidCountError
        if key == ("", "png") and count != 0:
            raise InvalidCountError
        total += count


def _fraction_text(value: Fraction) -> str:
    return f"{value.numerator}/{value.denominator}"


def _evidence_text(evidence: SimilarityEvidence) -> str:
    fields = (
        f"reference_units={evidence.reference_units}",
        f"candidate_units={evidence.candidate_units}",
        f"shared_units={evidence.shared_units}",
        f"union_units={evidence.union_units}",
        f"reference_coverage={_fraction_text(evidence.reference_coverage)}",
        f"weighted_jaccard={_fraction_text(evidence.weighted_jaccard)}",
    )
    return "source-similarity\t" + "\t".join(fields) + "\n"


def main(argv: Sequence[str] | None = None) -> int:
    """Measure two public count ledgers without making an admission decision."""
    arguments = list(sys.argv[1:] if argv is None else argv)
    if len(arguments) != 2:
        sys.stderr.write(
            "usage: source-similarity <reference.jsonl> <candidate.jsonl>\n"
        )
        return 2
    try:
        reference = load_count_ledger(Path(arguments[0]))
        candidate = load_count_ledger(Path(arguments[1]))
        evidence = measure(reference, candidate)
    except SimilarityInputError as error:
        sys.stderr.write(f"source-similarity: {error}\n")
        return 1
    sys.stdout.write(_evidence_text(evidence))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
