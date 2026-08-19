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

from __future__ import annotations

from collections.abc import Mapping
from collections.abc import Sequence
from dataclasses import dataclass
from fractions import Fraction
import json
from pathlib import Path
import sys
from typing import Any

Coordinate = tuple[str, str]
_MANIFEST_SCHEMA = "shar-schoenwald.game-manifest-ledger.v2"
_MAX_COUNT = (1 << 64) - 1


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
        super().__init__("coordinate must be a pair of strings")


class InvalidCountError(SimilarityInputError):
    """A coordinate count is not a nonnegative integer."""

    def __init__(self) -> None:
        """Initialize the canonical coordinate-count failure."""
        super().__init__(
            "coordinate count must be a nonnegative integer within 64-bit range"
        )


class LedgerInputError(SimilarityInputError):
    """A public JSONL count ledger is malformed or unreadable."""


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
class SimilarityEvidence:
    """Pure calibration evidence with no admission decision."""

    reference_units: int
    candidate_units: int
    shared_units: int
    union_units: int
    reference_coverage: Fraction
    weighted_jaccard: Fraction


def _parse_jsonl_record(line: str, line_number: int) -> dict[str, Any]:
    """Parse one JSONL object while preserving a path-free line diagnostic."""
    if not line.strip():
        raise LedgerInputError(
            f"count ledger has empty JSONL record at line {line_number}"
        )
    try:
        record: Any = json.loads(
            line,
            object_pairs_hook=_object_without_duplicates,
        )
    except ValueError as error:
        raise LedgerInputError(
            f"count ledger contains invalid JSONL at line {line_number}"
        ) from error
    if not isinstance(record, dict):
        raise LedgerInputError(
            f"count ledger record must be an object at line {line_number}"
        )
    return record


def _valid_required_file_metadata(value: object) -> bool:
    """Return whether one public required-file record has canonical shape."""
    if not isinstance(value, dict) or set(value) != {"path", "min"}:
        return False
    minimum = value["min"]
    return (
        isinstance(value["path"], str)
        and not isinstance(minimum, bool)
        and isinstance(minimum, int)
        and minimum >= 0
    )


def _validate_schema_metadata(record: dict[str, Any]) -> None:
    """Require only public manifest header metadata shapes."""
    if set(record) - {"schema", "kind_taxonomy", "required_files"}:
        raise LedgerInputError("count ledger schema record has unknown fields")
    taxonomy = record.get("kind_taxonomy")
    if taxonomy is not None and (
        not isinstance(taxonomy, list)
        or not all(isinstance(kind, str) for kind in taxonomy)
    ):
        raise LedgerInputError("count ledger kind taxonomy is invalid")
    required_files = record.get("required_files")
    if required_files is None:
        return
    if not isinstance(required_files, list) or not all(
        _valid_required_file_metadata(requirement)
        for requirement in required_files
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
    directory = record.get("dir")
    extension = record.get("ext")
    count = record.get("count" if has_observed else "min")
    kind = record.get("kind")
    if (
        not isinstance(directory, str)
        or not isinstance(extension, str)
        or not extension
    ):
        raise InvalidCoordinateError
    if kind is not None and not isinstance(kind, str):
        raise LedgerInputError("count ledger kind metadata must be a string")
    if (
        isinstance(count, bool)
        or not isinstance(count, int)
        or count < 0
        or count > _MAX_COUNT
    ):
        raise InvalidCountError
    return (directory, extension), count


def parse_count_ledger(text: str) -> dict[Coordinate, int]:
    """Parse public manifest coordinates without retaining metadata payloads."""
    counts: dict[Coordinate, int] = {}
    count_field: str | None = None
    for line_number, line in enumerate(text.splitlines(), start=1):
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
        counts[coordinate] = count
    _validate(counts)
    return counts


def load_count_ledger(path: Path) -> dict[Coordinate, int]:
    """Load one public JSONL ledger without disclosing its local path."""
    try:
        text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        raise LedgerInputError(
            "count ledger could not be read as UTF-8"
        ) from error
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
    _validate(reference)
    _validate(candidate)
    reference_units = sum(reference.values())
    candidate_units = sum(candidate.values())
    if reference_units == 0:
        raise EmptyReferenceError
    comparable_reference = _collapse_collision_families(reference)
    comparable_candidate = _collapse_collision_families(candidate)
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


def _collision_family(directory: str) -> str:
    base, marker, ordinal = directory.rpartition("~")
    if not marker or not base:
        return directory
    if len(ordinal) < 2 or not ordinal.isascii() or not ordinal.isdigit():
        return directory
    if ordinal == "00" or (len(ordinal) > 2 and ordinal.startswith("0")):
        return directory
    return base


def _collapse_collision_families(
    values: Mapping[Coordinate, int],
) -> dict[Coordinate, int]:
    collapsed: dict[Coordinate, int] = {}
    for (directory, extension), count in values.items():
        key = (_collision_family(directory), extension)
        collapsed[key] = collapsed.get(key, 0) + count
    return collapsed


def _validate(values: Mapping[Coordinate, int]) -> None:
    total = 0
    for key, count in values.items():
        if (
            not isinstance(key, tuple)
            or len(key) != 2
            or not all(isinstance(value, str) for value in key)
            or not key[1]
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
