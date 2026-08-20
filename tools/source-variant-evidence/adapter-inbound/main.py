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


@dataclass(frozen=True, slots=True)
class FileIdentity:
    """Stable filesystem identity captured around one evidence read."""

    device: int
    inode: int
    modified_ns: int
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


def _identity(metadata: os.stat_result) -> FileIdentity:
    """Project path-free metadata needed to detect evidence-file drift."""
    return FileIdentity(
        device=metadata.st_dev,
        inode=metadata.st_ino,
        modified_ns=metadata.st_mtime_ns,
        size=metadata.st_size,
    )


def _regular_file_identity(path: Path) -> FileIdentity:
    """Return one regular-file identity without following a final redirect.

    Raises:
        InputInspectionError: If filesystem metadata cannot be inspected.
        InputTypeError: If the path does not identify a regular file.

    """
    try:
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
    """Run ordered-byte evidence collection with no admission result."""
    arguments = list(sys.argv[1:] if argv is None else argv)
    if len(arguments) < 2:
        sys.stderr.write(
            "usage: source-variant-evidence <common-file> <variant-file>...\n"
        )
        return 2
    try:
        reference = load_reference(Path(arguments[0]))
        evidence = [
            measure_variant(reference, Path(value))
            for value in arguments[1:]
        ]
    except VariantEvidenceError as error:
        sys.stderr.write(f"source-variant-evidence: {error}\n")
        return 1
    for index, result in enumerate(evidence, start=1):
        sys.stdout.write(_render(index, result))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
