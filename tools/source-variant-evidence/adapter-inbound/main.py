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
#   - Read-only ordered-byte evidence across local source variants.
# - Must-Not:
#   - Derive or publish common payloads, expose private paths, or admit sources.
# - Allows:
#   - One private common-byte artifact and one or more lawful local candidates.
# - Split-When:
#   - A replay-source projection gains a reviewed independent contract.
# - Merge-When:
#   - Source admission owns identical private variant evidence.
# - Summary:
#   - Verifies common-byte subsequence evidence without exposing payloads.
# - Description:
#   - Streams local candidates and counts ordered matches against one snapshot.
# - Usage:
#   - Supply a private common artifact followed by local variant files.
# - Defaults:
#   - No admission decision or source transformation is produced.
#

"""Read-only private source-variant common-byte evidence."""

from __future__ import annotations

from collections.abc import Sequence
from dataclasses import dataclass
from itertools import starmap
import json
import os
from pathlib import Path
import stat
import sys

_CHUNK_BYTES = 1024 * 1024
_MAX_PROJECTION_ALTERNATIVES = 256
_ALGORITHM_SETTINGS_SCHEMA = "shar.algorithm.settings.v1"
_ALGORITHM_SETTINGS_RELATIVE = Path(
    "src/foundation/algorithm/composition/adapter-inbound/settings.json"
)
_ALGORITHM_SETTINGS_FIELDS = (
    "schema",
    "minimum_source_files",
    "minimum_source_bytes",
    "maximum_source_files",
    "maximum_target_files",
    "maximum_file_bytes",
    "maximum_source_bytes",
    "maximum_target_bytes",
)
_U64_MAX = (1 << 64) - 1


class VariantEvidenceError(ValueError):
    """Base path-free source-variant evidence failure."""


class InputInspectionError(VariantEvidenceError):
    """One local evidence input cannot be inspected."""

    def __init__(self) -> None:
        """Initialize the canonical path-free inspection failure."""
        super().__init__("source-variant evidence input cannot be inspected")


class InputTypeError(VariantEvidenceError):
    """One local evidence input is not a regular file."""

    def __init__(self) -> None:
        """Initialize the canonical regular-file failure."""
        super().__init__("source-variant evidence input must be a regular file")


class InputRedirectError(VariantEvidenceError):
    """One local evidence input traverses a redirected parent directory."""

    def __init__(self) -> None:
        """Initialize the canonical parent-redirect failure."""
        super().__init__(
            "source-variant evidence parent must be a real directory"
        )


class EmptyReferenceError(VariantEvidenceError):
    """The private common-byte reference contains no bytes."""

    def __init__(self) -> None:
        """Initialize the canonical empty-reference failure."""
        super().__init__("common-byte reference must not be empty")


class ReferenceReadError(VariantEvidenceError):
    """The private common-byte reference cannot be read completely."""

    def __init__(self) -> None:
        """Initialize the canonical reference-read failure."""
        super().__init__("common-byte reference cannot be read")


class ReferenceChangedError(VariantEvidenceError):
    """The private common-byte reference changed during its snapshot."""

    def __init__(self) -> None:
        """Initialize the canonical reference-drift failure."""
        super().__init__("common-byte reference changed while reading")


class CandidateReadError(VariantEvidenceError):
    """One candidate variant cannot be read completely."""

    def __init__(self) -> None:
        """Initialize the canonical candidate-read failure."""
        super().__init__("candidate variant cannot be read")


class CandidateChangedError(VariantEvidenceError):
    """One candidate variant changed during evidence collection."""

    def __init__(self) -> None:
        """Initialize the canonical candidate-drift failure."""
        super().__init__("candidate variant changed while reading")


class ProjectionMismatchError(VariantEvidenceError):
    """The common artifact is not an ordered subsequence of one variant."""

    def __init__(self) -> None:
        """Initialize the canonical projection mismatch failure."""
        super().__init__(
            "common-byte reference is not an ordered subsequence of a variant"
        )


class ProjectionSettingsError(VariantEvidenceError):
    """The active generic algorithm settings cannot be read safely."""

    def __init__(self) -> None:
        """Initialize the canonical path-free settings failure."""
        super().__init__("active algorithm projection settings are invalid")


class ProjectionResourceError(VariantEvidenceError):
    """Projection evidence exceeds the active generic algorithm resources."""

    def __init__(self) -> None:
        """Initialize the canonical projection resource-limit failure."""
        super().__init__("source projection exceeds active algorithm limits")


class ProjectionLimitError(VariantEvidenceError):
    """Distinct projection layouts exceed the generic algorithm limit."""

    def __init__(self) -> None:
        """Initialize the canonical projection alternative limit failure."""
        super().__init__(
            "source projection has too many distinct layout alternatives"
        )


@dataclass(frozen=True, slots=True)
class FileIdentity:
    """Stable filesystem identity captured around one evidence read."""

    device: int
    inode: int
    modified_ns: int
    ctime_ns: int
    size: int


@dataclass(frozen=True, slots=True)
class VariantEvidence:
    """Path-free aggregate evidence for one candidate variant."""

    candidate_bytes: int
    matched_bytes: int
    reference_bytes: int

    @property
    def complete(self) -> bool:
        """Whether every reference byte was observed in order."""
        return self.matched_bytes == self.reference_bytes


@dataclass(frozen=True, slots=True)
class OffsetProjectionAlternative:
    """One public-safe positional layout for a private source variant."""

    span_bytes: int
    mask: bytes

    @property
    def selected_bytes(self) -> int:
        """Number of source bytes selected by the mask."""
        return sum(value.bit_count() for value in self.mask)


@dataclass(frozen=True, slots=True)
class OffsetProjection:
    """Deduplicated public-safe layouts for private local variants."""

    alternatives: tuple[OffsetProjectionAlternative, ...]


def _repository_root() -> Path:
    """Return the repository root from this tool's tracked location."""
    return Path(__file__).resolve().parents[3]


def _settings_object_without_duplicates(
    pairs: list[tuple[str, object]],
) -> dict[str, object]:
    """Build one settings object while rejecting duplicate JSON fields.

    Raises:
        ProjectionSettingsError: If one field appears more than once.

    """
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise ProjectionSettingsError
        result[key] = value
    return result


def _algorithm_maximum_file_bytes_from_text(text: str) -> int:
    """Validate Rust-compatible active settings and return the file limit.

    Raises:
        ProjectionSettingsError: If active settings cannot be validated.

    """
    try:
        document = json.loads(
            text,
            object_pairs_hook=_settings_object_without_duplicates,
        )
    except (TypeError, ValueError) as error:
        raise ProjectionSettingsError from error
    if not isinstance(document, dict) or set(document) != set(
        _ALGORITHM_SETTINGS_FIELDS
    ):
        raise ProjectionSettingsError
    if document["schema"] != _ALGORITHM_SETTINGS_SCHEMA:
        raise ProjectionSettingsError
    values: dict[str, int] = {}
    for field in _ALGORITHM_SETTINGS_FIELDS[1:]:
        value = document[field]
        if (
            isinstance(value, bool)
            or not isinstance(value, int)
            or not 0 <= value <= _U64_MAX
        ):
            raise ProjectionSettingsError
        values[field] = value
    if (
        values["minimum_source_files"] == 0
        or values["minimum_source_bytes"] == 0
        or values["maximum_source_files"] < values["minimum_source_files"]
        or values["maximum_source_bytes"] < values["minimum_source_bytes"]
    ):
        raise ProjectionSettingsError
    if (
        values["maximum_target_files"] == 0
        or values["maximum_file_bytes"] == 0
        or values["maximum_target_bytes"] == 0
    ):
        raise ProjectionSettingsError
    if (
        values["maximum_source_files"] * values["maximum_file_bytes"]
        < values["minimum_source_bytes"]
    ):
        raise ProjectionSettingsError
    return values["maximum_file_bytes"]


def _algorithm_maximum_file_bytes() -> int:
    """Load one stable non-redirected active algorithm settings snapshot.

    Raises:
        ProjectionSettingsError: If active settings cannot be validated.

    """
    path = _repository_root() / _ALGORITHM_SETTINGS_RELATIVE
    try:
        expected = _regular_file_identity(path)
    except VariantEvidenceError as error:
        raise ProjectionSettingsError from error
    try:
        with path.open("rb") as handle:
            opened = _identity(os.fstat(handle.fileno()))
            if opened != expected or _current_identity(path) != expected:
                raise ProjectionSettingsError
            data = handle.read()
            finished = _identity(os.fstat(handle.fileno()))
    except OSError as error:
        raise ProjectionSettingsError from error
    if (
        finished != expected
        or _current_identity(path) != expected
        or len(data) != expected.size
    ):
        raise ProjectionSettingsError
    try:
        text = data.decode("utf-8")
    except UnicodeError as error:
        raise ProjectionSettingsError from error
    return _algorithm_maximum_file_bytes_from_text(text)


def _validate_projection_resources(
    projection: OffsetProjection,
    maximum_file_bytes: int,
) -> None:
    """Require spans and aggregate mask bytes to fit active authoring limits.

    Raises:
        ProjectionResourceError: If one projection exceeds the active limit.

    """
    mask_bytes = 0
    for alternative in projection.alternatives:
        if alternative.span_bytes > maximum_file_bytes:
            raise ProjectionResourceError
        mask_bytes += len(alternative.mask)
        if mask_bytes > maximum_file_bytes:
            raise ProjectionResourceError


def _identity(metadata: os.stat_result) -> FileIdentity:
    """Project path-free metadata needed to detect evidence-file drift."""
    return FileIdentity(
        device=metadata.st_dev,
        inode=metadata.st_ino,
        modified_ns=metadata.st_mtime_ns,
        ctime_ns=metadata.st_ctime_ns,
        size=metadata.st_size,
    )


def _regular_file_identity(path: Path) -> FileIdentity:
    """Return one regular-file identity without following a final redirect.

    Raises:
        InputInspectionError: If filesystem metadata cannot be inspected.
        InputRedirectError: If a lexical parent redirects elsewhere.
        InputTypeError: If the path does not identify a regular file.

    """
    try:
        redirected_parent = any(
            parent.is_symlink() or os.path.isjunction(parent)
            for parent in (path.parent, *path.parent.parents)
        )
        if redirected_parent:
            raise InputRedirectError
        metadata = path.lstat()
    except OSError as error:
        raise InputInspectionError from error
    if not stat.S_ISREG(metadata.st_mode):
        raise InputTypeError
    return _identity(metadata)


def _current_identity(path: Path) -> FileIdentity | None:
    """Return one final regular-file identity, or `None` after any drift."""
    try:
        return _regular_file_identity(path)
    except VariantEvidenceError:
        return None


def load_reference(
    path: Path,
    *,
    maximum_file_bytes: int | None = None,
) -> bytes:
    """Read one nonempty common-byte reference without disclosing its path.

    Raises:
        ReferenceReadError: If the complete bytes cannot be read.
        ReferenceChangedError: If size evidence changes during the read.
        EmptyReferenceError: If the reference contains no bytes.
        ProjectionResourceError: If projection mode exceeds the active limit.

    """
    expected = _regular_file_identity(path)
    if maximum_file_bytes is not None and expected.size > maximum_file_bytes:
        raise ProjectionResourceError
    try:
        with path.open("rb") as handle:
            opened = _identity(os.fstat(handle.fileno()))
            if opened != expected or _current_identity(path) != expected:
                raise ReferenceChangedError
            data = handle.read()
            finished = _identity(os.fstat(handle.fileno()))
    except OSError as error:
        raise ReferenceReadError from error
    if (
        finished != expected
        or _current_identity(path) != expected
        or len(data) != expected.size
    ):
        raise ReferenceChangedError
    if not data:
        raise EmptyReferenceError
    return data


def _match_chunk(reference: bytes, matched: int, chunk: bytes) -> int:
    """Advance one ordered reference cursor across a candidate chunk."""
    if matched == len(reference):
        return matched
    for value in chunk:
        if value == reference[matched]:
            matched += 1
            if matched == len(reference):
                break
    return matched


def measure_variant(reference: bytes, path: Path) -> VariantEvidence:
    """Measure ordered reference-byte matches while streaming one candidate.

    Raises:
        EmptyReferenceError: If the reference contains no bytes.
        CandidateReadError: If the complete candidate cannot be read.
        CandidateChangedError: If size evidence changes during the read.

    """
    if not reference:
        raise EmptyReferenceError
    expected = _regular_file_identity(path)
    matched = 0
    observed = 0
    try:
        with path.open("rb") as handle:
            opened = _identity(os.fstat(handle.fileno()))
            if opened != expected or _current_identity(path) != expected:
                raise CandidateChangedError
            while chunk := handle.read(_CHUNK_BYTES):
                observed += len(chunk)
                matched = _match_chunk(reference, matched, chunk)
            finished = _identity(os.fstat(handle.fileno()))
    except OSError as error:
        raise CandidateReadError from error
    if (
        finished != expected
        or _current_identity(path) != expected
        or observed != expected.size
    ):
        raise CandidateChangedError
    return VariantEvidence(
        candidate_bytes=observed,
        matched_bytes=matched,
        reference_bytes=len(reference),
    )


def _read_candidate_snapshot(
    path: Path,
    *,
    maximum_file_bytes: int | None = None,
) -> bytes:
    """Read one stable candidate snapshot with path-free failures.

    Raises:
        CandidateReadError: If candidate bytes cannot be read completely.
        CandidateChangedError: If candidate identity changes during the read.
        ProjectionResourceError: If the candidate exceeds the active limit.

    """
    expected = _regular_file_identity(path)
    if maximum_file_bytes is not None and expected.size > maximum_file_bytes:
        raise ProjectionResourceError
    try:
        with path.open("rb") as handle:
            opened = _identity(os.fstat(handle.fileno()))
            if opened != expected or _current_identity(path) != expected:
                raise CandidateChangedError
            data = handle.read()
            finished = _identity(os.fstat(handle.fileno()))
    except OSError as error:
        raise CandidateReadError from error
    if (
        finished != expected
        or _current_identity(path) != expected
        or len(data) != expected.size
    ):
        raise CandidateChangedError
    return data


def _ordered_projection(
    reference: bytes,
    candidate: bytes,
) -> OffsetProjectionAlternative:
    """Derive the deterministic earliest-match layout for one candidate.

    Raises:
        ProjectionMismatchError: If the complete reference is not found
            in order.

    """
    mask = bytearray()
    matched = 0
    span = 0
    for offset, value in enumerate(candidate):
        if matched == len(reference):
            break
        if value == reference[matched]:
            byte_index, bit_index = divmod(offset, 8)
            missing = byte_index + 1 - len(mask)
            if missing > 0:
                mask.extend(b"\x00" * missing)
            mask[byte_index] |= 1 << (7 - bit_index)
            matched += 1
            span = offset + 1
    if matched != len(reference):
        raise ProjectionMismatchError
    return OffsetProjectionAlternative(
        span_bytes=span,
        mask=bytes(mask),
    )


def build_offset_projection(
    reference: bytes,
    paths: Sequence[Path],
    *,
    maximum_file_bytes: int | None = None,
) -> OffsetProjection:
    """Derive deduplicated ordered-subsequence layouts for local variants.

    Raises:
        EmptyReferenceError: If the supplied common artifact is empty.
        ProjectionMismatchError: If one candidate cannot reproduce it in order.
        ProjectionLimitError: If distinct layouts exceed the algorithm limit.
        ProjectionResourceError: If projection resources exceed active limits.

    """
    if not reference:
        raise EmptyReferenceError
    if not paths:
        raise ProjectionMismatchError
    maximum = (
        _algorithm_maximum_file_bytes()
        if maximum_file_bytes is None
        else maximum_file_bytes
    )
    if len(reference) > maximum:
        raise ProjectionResourceError
    alternatives: list[OffsetProjectionAlternative] = []
    for path in paths:
        candidate = _read_candidate_snapshot(
            path,
            maximum_file_bytes=maximum,
        )
        alternative = _ordered_projection(reference, candidate)
        if alternative not in alternatives:
            alternatives.append(alternative)
            if len(alternatives) > _MAX_PROJECTION_ALTERNATIVES:
                raise ProjectionLimitError
            _validate_projection_resources(
                OffsetProjection(alternatives=tuple(alternatives)),
                maximum,
            )
    return OffsetProjection(alternatives=tuple(alternatives))


def _mask_chunks(mask: bytes) -> list[str]:
    """Encode one offset mask as canonical bounded hexadecimal chunks."""
    encoded = mask.hex()
    return [encoded[index : index + 64] for index in range(0, len(encoded), 64)]


def _projection_text(projection: OffsetProjection) -> str:
    """Render one canonical algorithm source-projection descriptor."""
    document = {
        "kind": "offset-mask-set-v1",
        "alternatives": [
            {
                "span_bytes": alternative.span_bytes,
                "mask": _mask_chunks(alternative.mask),
            }
            for alternative in projection.alternatives
        ],
    }
    return json.dumps(document, indent=2, ensure_ascii=True) + "\n"


def _render(index: int, evidence: VariantEvidence) -> str:
    """Render one path-free evidence row."""
    complete = "true" if evidence.complete else "false"
    return (
        f"variant={index}\t"
        f"candidate_bytes={evidence.candidate_bytes}\t"
        f"matched_bytes={evidence.matched_bytes}\t"
        f"reference_bytes={evidence.reference_bytes}\t"
        f"complete={complete}\n"
    )


def main(argv: Sequence[str] | None = None) -> int:
    """Run path-free variant evidence or emit a verified offset projection."""
    arguments = list(sys.argv[1:] if argv is None else argv)
    projection_mode = bool(arguments and arguments[0] == "--projection")
    if projection_mode:
        arguments.pop(0)
    if len(arguments) < 2:
        sys.stderr.write(
            "usage: source-variant-evidence [--projection] "
            "<common-file> <variant-file>...\n"
        )
        return 2
    try:
        variants = [Path(value) for value in arguments[1:]]
        if projection_mode:
            maximum_file_bytes = _algorithm_maximum_file_bytes()
            reference = load_reference(
                Path(arguments[0]),
                maximum_file_bytes=maximum_file_bytes,
            )
            projection = build_offset_projection(
                reference,
                variants,
                maximum_file_bytes=maximum_file_bytes,
            )
            output = _projection_text(projection)
        else:
            reference = load_reference(Path(arguments[0]))
            evidence = [measure_variant(reference, path) for path in variants]
            output = "".join(starmap(_render, enumerate(evidence, start=1)))
    except VariantEvidenceError as error:
        sys.stderr.write(f"source-variant-evidence: {error}\n")
        return 1
    sys.stdout.write(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
