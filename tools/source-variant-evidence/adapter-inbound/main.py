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


@dataclass(frozen=True, slots=True)
class FileIdentity:
    """Stable filesystem identity captured around one evidence read."""

    device: int
    inode: int
    modified_ns: int
    changed_ns: int
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


def _identity(metadata: os.stat_result) -> FileIdentity:
    """Project path-free metadata needed to detect evidence-file drift."""
    return FileIdentity(
        device=metadata.st_dev,
        inode=metadata.st_ino,
        modified_ns=metadata.st_mtime_ns,
        changed_ns=metadata.st_ctime_ns,
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


def load_reference(path: Path) -> bytes:
    """Read one nonempty common-byte reference without disclosing its path.

    Raises:
        ReferenceReadError: If the complete bytes cannot be read.
        ReferenceChangedError: If size evidence changes during the read.
        EmptyReferenceError: If the reference contains no bytes.

    """
    expected = _regular_file_identity(path)
    try:
        with path.open("rb") as handle:
            opened = _identity(os.fstat(handle.fileno()))
            data = handle.read()
            finished = _identity(os.fstat(handle.fileno()))
    except OSError as error:
        raise ReferenceReadError from error
    if (
        opened != expected
        or finished != expected
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
            while chunk := handle.read(_CHUNK_BYTES):
                observed += len(chunk)
                matched = _match_chunk(reference, matched, chunk)
            finished = _identity(os.fstat(handle.fileno()))
    except OSError as error:
        raise CandidateReadError from error
    if (
        opened != expected
        or finished != expected
        or _current_identity(path) != expected
        or observed != expected.size
    ):
        raise CandidateChangedError
    return VariantEvidence(
        candidate_bytes=observed,
        matched_bytes=matched,
        reference_bytes=len(reference),
    )


def _read_candidate_snapshot(path: Path) -> bytes:
    """Read one stable candidate snapshot with path-free failures.

    Raises:
        CandidateReadError: If candidate bytes cannot be read completely.
        CandidateChangedError: If candidate identity changes during the read.

    """
    expected = _regular_file_identity(path)
    try:
        with path.open("rb") as handle:
            opened = _identity(os.fstat(handle.fileno()))
            data = handle.read()
            finished = _identity(os.fstat(handle.fileno()))
    except OSError as error:
        raise CandidateReadError from error
    if (
        opened != expected
        or finished != expected
        or _current_identity(path) != expected
        or len(data) != expected.size
    ):
        raise CandidateChangedError
    return data


def _offset_mask(flags: bytearray) -> bytes:
    """Pack one selected-offset flag per source byte, high bit first."""
    mask = bytearray((len(flags) + 7) // 8)
    for offset, selected in enumerate(flags):
        if not selected:
            continue
        byte_index, bit_index = divmod(offset, 8)
        mask[byte_index] |= 1 << (7 - bit_index)
    return bytes(mask)


def _ordered_projection(
    reference: bytes,
    candidate: bytes,
) -> OffsetProjectionAlternative:
    """Derive the deterministic earliest-match layout for one candidate.

    Raises:
        ProjectionMismatchError: If the complete reference is not found
            in order.

    """
    flags = bytearray(len(candidate))
    matched = 0
    span = 0
    for offset, value in enumerate(candidate):
        if matched == len(reference):
            break
        if value == reference[matched]:
            flags[offset] = 1
            matched += 1
            span = offset + 1
    if matched != len(reference):
        raise ProjectionMismatchError
    del flags[span:]
    return OffsetProjectionAlternative(
        span_bytes=span,
        mask=_offset_mask(flags),
    )


def build_offset_projection(
    reference: bytes,
    paths: Sequence[Path],
) -> OffsetProjection:
    """Derive deduplicated ordered-subsequence layouts for local variants.

    Raises:
        EmptyReferenceError: If the supplied common artifact is empty.
        ProjectionMismatchError: If one candidate cannot reproduce it in order.

    """
    if not reference:
        raise EmptyReferenceError
    if not paths:
        raise ProjectionMismatchError
    alternatives: list[OffsetProjectionAlternative] = []
    for path in paths:
        candidate = _read_candidate_snapshot(path)
        alternative = _ordered_projection(reference, candidate)
        if alternative not in alternatives:
            alternatives.append(alternative)
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
        reference = load_reference(Path(arguments[0]))
        variants = [Path(value) for value in arguments[1:]]
        if projection_mode:
            projection = build_offset_projection(reference, variants)
            output = _projection_text(projection)
        else:
            evidence = [measure_variant(reference, path) for path in variants]
            output = "".join(starmap(_render, enumerate(evidence, start=1)))
    except VariantEvidenceError as error:
        sys.stderr.write(f"source-variant-evidence: {error}\n")
        return 1
    sys.stdout.write(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
