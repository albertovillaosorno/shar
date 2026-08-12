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
#   - The repository behavior implemented by this source file.
# - Must-Not:
#   - Bypass the contracts or authority boundaries of its owning package.
# - Allows:
#   - Inputs: values admitted by the file's public or internal interface.
#   - Outputs: deterministic values or effects declared by that interface.
#   - Side effects: only those explicitly owned by the implementation.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another file owns the exact same responsibility.
# - Summary:
#   - Tree-level structural and stable-anchor source admission.
# - Description:
#   - Implements the responsibility summarized by this module.
# - Usage:
#   - Used through the owning package, executable, or document boundary.
# - Defaults:
#   - Invalid inputs or broken invariants fail closed.
#

"""Tree-level structural and stable-anchor source admission."""

from __future__ import annotations

from dataclasses import dataclass
import math
from pathlib import PurePosixPath
from typing import TYPE_CHECKING
from typing import cast

from .fingerprints import AnchorPolicy
from .fingerprints import anchor_coverage
from .fingerprints import stable_anchors

if TYPE_CHECKING:
    from .fingerprints import StableAnchor

_BACKSLASH = "\\"
_DOT = "."
_PARENT = ".."
_ZERO = 0
_ONE = 1
_STRUCTURAL_POLICY = AnchorPolicy(window_bytes=16, selection_modulus=8)
_ANCHOR_POLICY = AnchorPolicy(window_bytes=32, selection_modulus=64)


class AdmissionPolicyError(ValueError):
    """Raised when tree admission policy is internally invalid."""


class AdmissionError(RuntimeError):
    """Raised when candidate source evidence does not satisfy admission."""


@dataclass(frozen=True, slots=True, order=True)
class IdentityFile:
    """One consumer-canonicalized file admitted to source identity."""

    path: str
    canonical: bytes

    def __post_init__(self) -> None:
        """Require one normalized portable relative file path.

        Raises:
            AdmissionPolicyError: The identity path is unsafe or non-canonical.

        """
        if type(self.path) is not str:
            message = "identity path must use the exact string type"
            raise AdmissionPolicyError(message)
        if type(self.canonical) is not bytes:
            message = "identity canonical content must use exact bytes"
            raise AdmissionPolicyError(message)
        candidate = PurePosixPath(self.path)
        unsafe = (
            not self.path
            or _BACKSLASH in self.path
            or self.path == _DOT
            or candidate.is_absolute()
            or _PARENT in candidate.parts
        )
        if unsafe or candidate.as_posix() != self.path:
            message = f"invalid identity path: {self.path!r}"
            raise AdmissionPolicyError(message)


@dataclass(frozen=True, slots=True)
class IdentityTree:
    """Explicit set of consumer-selected files participating in identity."""

    files: tuple[IdentityFile, ...]

    def __post_init__(self) -> None:
        """Require sorted unique paths for deterministic evidence.

        Raises:
            AdmissionPolicyError: Files are duplicated or out of order.

        """
        if type(self.files) is not tuple:
            message = "identity tree files must use the exact immutable tuple"
            raise AdmissionPolicyError(message)
        if any(type(item) is not IdentityFile for item in self.files):
            message = "identity tree contains a foreign file record"
            raise AdmissionPolicyError(message)
        paths = tuple(item.path for item in self.files)
        if paths != tuple(sorted(set(paths))):
            message = "identity tree paths must be unique and sorted"
            raise AdmissionPolicyError(message)


@dataclass(frozen=True, slots=True)
class AdmissionPolicy:
    """Thresholds and distribution requirements for source admission."""

    minimum_source_similarity: float
    minimum_anchor_coverage: float
    minimum_anchor_files: int
    minimum_anchors_per_file: int
    structural_policy: AnchorPolicy = _STRUCTURAL_POLICY
    anchor_policy: AnchorPolicy = _ANCHOR_POLICY

    def __post_init__(self) -> None:
        """Validate threshold and positive-count requirements.

        Raises:
            AdmissionPolicyError: A threshold or evidence count is invalid.

        """
        _validate_fraction(
            "minimum_source_similarity", self.minimum_source_similarity
        )
        _validate_fraction(
            "minimum_anchor_coverage", self.minimum_anchor_coverage
        )
        if (
            type(self.minimum_anchor_files) is not int
            or self.minimum_anchor_files < _ONE
        ):
            message = "minimum_anchor_files must be a positive integer"
            raise AdmissionPolicyError(message)
        if (
            type(self.minimum_anchors_per_file) is not int
            or self.minimum_anchors_per_file < _ONE
        ):
            message = "minimum_anchors_per_file must be a positive integer"
            raise AdmissionPolicyError(message)
        if type(self.structural_policy) is not AnchorPolicy:
            message = "structural_policy must use the exact AnchorPolicy type"
            raise AdmissionPolicyError(message)
        if type(self.anchor_policy) is not AnchorPolicy:
            message = "anchor_policy must use the exact AnchorPolicy type"
            raise AdmissionPolicyError(message)


def _require_evidence_fraction(value: object, context: str) -> float:
    if type(value) is not float or not math.isfinite(value):
        message = f"{context} must use a finite exact float"
        raise AdmissionPolicyError(message)
    if value < _ZERO or value > _ONE:
        message = f"{context} must be in [0, 1]"
        raise AdmissionPolicyError(message)
    return value


def _require_evidence_count(value: object, context: str) -> int:
    if type(value) is not int or value < _ZERO:
        message = f"{context} must use a non-negative exact integer"
        raise AdmissionPolicyError(message)
    return value


def _require_evidence_path(value: object) -> str:
    if type(value) is not str:
        message = "file admission path must use the exact string type"
        raise AdmissionPolicyError(message)
    candidate = PurePosixPath(value)
    unsafe = (
        not value
        or _BACKSLASH in value
        or value == _DOT
        or candidate.is_absolute()
        or _PARENT in candidate.parts
        or candidate.as_posix() != value
    )
    if unsafe:
        message = f"invalid file admission path: {value!r}"
        raise AdmissionPolicyError(message)
    return value


def _require_reason_tuple(value: object) -> tuple[str, ...]:
    if type(value) is not tuple:
        message = "tree admission reasons must use the exact immutable tuple"
        raise AdmissionPolicyError(message)
    items = cast("tuple[object, ...]", value)
    if any(type(item) is not str or not item for item in items):
        message = "tree admission reasons must contain non-empty exact strings"
        raise AdmissionPolicyError(message)
    return cast("tuple[str, ...]", value)


def _require_file_anchor_relation(
    coverage: float | None,
    reference: int,
    matched: int,
) -> None:
    if matched > reference:
        message = "matched anchor count cannot exceed reference anchors"
        raise AdmissionPolicyError(message)
    if coverage is not None and (
        reference == _ZERO or coverage != matched / reference
    ):
        message = "file anchor coverage does not match anchor counts"
        raise AdmissionPolicyError(message)


def _validate_file_admission_evidence(evidence: FileAdmissionEvidence) -> None:
    _ = _require_evidence_path(evidence.path)
    _ = _require_evidence_fraction(
        evidence.structural_similarity,
        "file structural similarity",
    )
    coverage = (
        _require_evidence_fraction(
            evidence.anchor_coverage,
            "file anchor coverage",
        )
        if evidence.anchor_coverage is not None
        else None
    )
    reference = _require_evidence_count(
        evidence.reference_anchor_count,
        "reference anchor count",
    )
    matched = _require_evidence_count(
        evidence.matched_anchor_count,
        "matched anchor count",
    )
    _require_file_anchor_relation(coverage, reference, matched)


def _require_file_evidence_tuple(
    value: object,
) -> tuple[FileAdmissionEvidence, ...]:
    if type(value) is not tuple:
        message = "tree admission files must use the exact immutable tuple"
        raise AdmissionPolicyError(message)
    items = cast("tuple[object, ...]", value)
    if any(type(item) is not FileAdmissionEvidence for item in items):
        message = "tree admission files must use exact immutable records"
        raise AdmissionPolicyError(message)
    files = cast("tuple[FileAdmissionEvidence, ...]", value)
    if not files:
        message = "tree admission evidence requires at least one file"
        raise AdmissionPolicyError(message)
    paths = tuple(item.path for item in files)
    if paths != tuple(sorted(set(paths))):
        message = "tree admission file paths must be unique and sorted"
        raise AdmissionPolicyError(message)
    return files


def _tree_file_aggregates(
    files: tuple[FileAdmissionEvidence, ...],
) -> tuple[float, float, int]:
    source_values = tuple(item.structural_similarity for item in files)
    source = math.fsum(source_values) / len(source_values)
    covered = tuple(
        item.anchor_coverage
        for item in files
        if item.anchor_coverage is not None
    )
    anchor = math.fsum(covered) / len(covered) if covered else 0.0
    return source, anchor, len(covered)


def _require_equal_evidence_value(
    actual: float,
    expected: float,
    message: str,
) -> None:
    if actual != expected:
        raise AdmissionPolicyError(message)


def _validate_tree_admission_evidence(evidence: TreeAdmissionEvidence) -> None:
    files = _require_file_evidence_tuple(evidence.files)
    aggregates = _tree_file_aggregates(files)
    _require_equal_evidence_value(
        _require_evidence_fraction(
            evidence.source_similarity,
            "tree source similarity",
        ),
        aggregates[0],
        "tree source similarity does not match file evidence",
    )
    _require_equal_evidence_value(
        _require_evidence_fraction(
            evidence.anchor_coverage,
            "tree anchor coverage",
        ),
        aggregates[1],
        "tree anchor coverage does not match file evidence",
    )
    eligible = _require_evidence_count(
        evidence.eligible_anchor_files,
        "eligible anchor file count",
    )
    _require_equal_evidence_value(
        eligible,
        aggregates[2],
        "eligible anchor file count does not match file evidence",
    )
    satisfied = _require_evidence_count(
        evidence.satisfied_anchor_files,
        "satisfied anchor file count",
    )
    if satisfied > eligible:
        message = "satisfied anchor file count cannot exceed eligible files"
        raise AdmissionPolicyError(message)
    _ = _require_reason_tuple(evidence.reasons)


@dataclass(frozen=True, slots=True)
class FileAdmissionEvidence:
    """Structural and anchor evidence for one reference identity file."""

    path: str
    structural_similarity: float
    anchor_coverage: float | None
    reference_anchor_count: int
    matched_anchor_count: int

    def __post_init__(self) -> None:
        """Require internally coherent per-file source evidence."""
        _validate_file_admission_evidence(self)


@dataclass(frozen=True, slots=True)
class TreeAdmissionEvidence:
    """Aggregated source-lineage evidence with deterministic failure reasons."""

    source_similarity: float
    anchor_coverage: float
    eligible_anchor_files: int
    satisfied_anchor_files: int
    files: tuple[FileAdmissionEvidence, ...]
    reasons: tuple[str, ...]

    def __post_init__(self) -> None:
        """Require aggregate evidence consistent with per-file records."""
        _validate_tree_admission_evidence(self)

    @property
    def admitted(self) -> bool:
        """Whether every source-lineage requirement passed.

        Returns:
            True exactly when no deterministic rejection reason exists.

        """
        return not self.reasons


def identity_tree(files: dict[str, bytes]) -> IdentityTree:
    """Build a sorted explicit identity tree from consumer-canonicalized bytes.

    Returns:
        A deterministic identity tree.

    Raises:
        AdmissionPolicyError: The mapping or any path/content value is invalid.

    """
    if type(files) is not dict:
        message = "identity source files must use the exact dictionary type"
        raise AdmissionPolicyError(message)
    for path, canonical in files.items():
        if type(path) is not str:
            message = "identity source path must use the exact string type"
            raise AdmissionPolicyError(message)
        if type(canonical) is not bytes:
            message = "identity source content must use exact bytes"
            raise AdmissionPolicyError(message)
    records = tuple(
        IdentityFile(path=path, canonical=canonical)
        for path, canonical in sorted(files.items())
    )
    return IdentityTree(files=records)


def _validate_fraction(name: str, value: object) -> None:
    if type(value) is int:
        number = float(value)
    elif type(value) is float:
        number = value
    else:
        message = f"{name} must be a finite numeric fraction in [0, 1]"
        raise AdmissionPolicyError(message)
    if not math.isfinite(number) or number < _ZERO or number > _ONE:
        message = f"{name} must be a finite fraction in [0, 1], got {value}"
        raise AdmissionPolicyError(message)


def _digest_set(anchors: tuple[StableAnchor, ...]) -> frozenset[bytes]:
    return frozenset(anchor.digest for anchor in anchors)


def _structural_similarity(
    reference: bytes,
    candidate: bytes,
    policy: AnchorPolicy,
) -> float:
    if reference == candidate:
        return 1.0
    reference_digests = _digest_set(stable_anchors(reference, policy))
    candidate_digests = _digest_set(stable_anchors(candidate, policy))
    if not reference_digests or not candidate_digests:
        return 0.0
    intersection = len(reference_digests & candidate_digests)
    return (
        2.0 * intersection / (len(reference_digests) + len(candidate_digests))
    )


def _file_evidence(
    reference: IdentityFile,
    candidate: IdentityFile | None,
    policy: AdmissionPolicy,
) -> FileAdmissionEvidence:
    candidate_bytes = b"" if candidate is None else candidate.canonical
    structural = _structural_similarity(
        reference.canonical,
        candidate_bytes,
        policy.structural_policy,
    )
    reference_anchors = stable_anchors(
        reference.canonical, policy.anchor_policy
    )
    candidate_anchors = stable_anchors(candidate_bytes, policy.anchor_policy)
    coverage = anchor_coverage(reference_anchors, candidate_anchors)
    eligible = coverage.total >= policy.minimum_anchors_per_file
    return FileAdmissionEvidence(
        path=reference.path,
        structural_similarity=structural,
        anchor_coverage=coverage.ratio if eligible else None,
        reference_anchor_count=coverage.total,
        matched_anchor_count=coverage.matched,
    )


def _mean(values: tuple[float, ...]) -> float:
    if not values:
        return 0.0
    return math.fsum(values) / len(values)


@dataclass(frozen=True, slots=True)
class _AggregateEvidence:
    source_similarity: float
    anchor_average: float
    eligible_files: int
    satisfied_files: int


def _reason_tuple(
    aggregate: _AggregateEvidence,
    policy: AdmissionPolicy,
) -> tuple[str, ...]:
    reasons: list[str] = []
    if aggregate.source_similarity < policy.minimum_source_similarity:
        reasons.append("insufficient structural source similarity")
    if aggregate.anchor_average < policy.minimum_anchor_coverage:
        reasons.append("insufficient stable-anchor coverage")
    if aggregate.eligible_files < policy.minimum_anchor_files:
        reasons.append("insufficient files with anchor evidence")
    if aggregate.satisfied_files < policy.minimum_anchor_files:
        reasons.append("stable-anchor evidence is not sufficiently distributed")
    return tuple(reasons)


def evaluate_admission(
    reference: IdentityTree,
    candidate: IdentityTree,
    policy: AdmissionPolicy,
) -> TreeAdmissionEvidence:
    """Evaluate structural and distributed-anchor source lineage.

    Every reference file receives equal weight. Consumers choose which files
    belong to identity before calling this function, so opaque assets cannot
    gain influence merely by being large.

    Returns:
        Deterministic per-file and aggregate admission evidence.

    Raises:
        AdmissionPolicyError: Input trees or policy use foreign runtime types.
        AdmissionError: The reference identity tree is empty.

    """
    if (
        type(reference) is not IdentityTree
        or type(candidate) is not IdentityTree
    ):
        message = (
            "admission identity inputs must use the exact IdentityTree type"
        )
        raise AdmissionPolicyError(message)
    if type(policy) is not AdmissionPolicy:
        message = "admission policy must use the exact AdmissionPolicy type"
        raise AdmissionPolicyError(message)
    if not reference.files:
        message = "reference identity tree contains no source evidence"
        raise AdmissionError(message)
    candidate_by_path = {item.path: item for item in candidate.files}
    files = tuple(
        _file_evidence(item, candidate_by_path.get(item.path), policy)
        for item in reference.files
    )
    source_similarity = _mean(
        tuple(item.structural_similarity for item in files)
    )
    eligible = tuple(item for item in files if item.anchor_coverage is not None)
    anchor_average = _mean(
        tuple(item.anchor_coverage or 0.0 for item in eligible)
    )
    satisfied = sum(
        item.matched_anchor_count > _ZERO
        and (item.anchor_coverage or 0.0) >= policy.minimum_anchor_coverage
        for item in eligible
    )
    aggregate = _AggregateEvidence(
        source_similarity=source_similarity,
        anchor_average=anchor_average,
        eligible_files=len(eligible),
        satisfied_files=satisfied,
    )
    reasons = _reason_tuple(aggregate, policy)
    return TreeAdmissionEvidence(
        source_similarity=source_similarity,
        anchor_coverage=anchor_average,
        eligible_anchor_files=len(eligible),
        satisfied_anchor_files=satisfied,
        files=files,
        reasons=reasons,
    )


def require_admission(
    reference: IdentityTree,
    candidate: IdentityTree,
    policy: AdmissionPolicy,
) -> TreeAdmissionEvidence:
    """Require tree admission and return its evidence.

    Returns:
        Passing deterministic source-lineage evidence.

    Raises:
        AdmissionError: Candidate source lineage is insufficient.

    """
    evidence = evaluate_admission(reference, candidate, policy)
    if not evidence.admitted:
        message = "; ".join(evidence.reasons)
        raise AdmissionError(message)
    return evidence
