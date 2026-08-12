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
#   - Content-defined stable fingerprints for source-lineage evidence.
# - Description:
#   - Implements the responsibility summarized by this module.
# - Usage:
#   - Used through the owning package, executable, or document boundary.
# - Defaults:
#   - Invalid inputs or broken invariants fail closed.
#

"""Content-defined stable fingerprints for source-lineage evidence."""

from __future__ import annotations

from dataclasses import dataclass
import hashlib

_DEFAULT_WINDOW_BYTES = 32
_DEFAULT_SELECTION_MODULUS = 64
_ROLLING_BASE = 257
_ROLLING_BITS = 64
_ROLLING_MODULUS = 1 << _ROLLING_BITS
_ROLLING_MASK = _ROLLING_MODULUS - 1
_ZERO = 0
_ONE = 1
_SHA256_BYTES = hashlib.sha256().digest_size


class FingerprintPolicyError(ValueError):
    """Raised when stable-anchor policy is internally invalid."""


@dataclass(frozen=True, slots=True)
class AnchorPolicy:
    """Deterministic content-defined anchor selection policy."""

    window_bytes: int = _DEFAULT_WINDOW_BYTES
    selection_modulus: int = _DEFAULT_SELECTION_MODULUS

    def __post_init__(self) -> None:
        """Reject unusable anchor parameters.

        Raises:
            FingerprintPolicyError: A configured integer is not positive.

        """
        if (
            type(self.window_bytes) is not int
            or type(self.selection_modulus) is not int
        ):
            message = "anchor window and selection modulus must be integers"
            raise FingerprintPolicyError(message)
        if self.window_bytes < _ONE or self.selection_modulus < _ONE:
            message = (
                "anchor window and selection modulus must be positive integers"
            )
            raise FingerprintPolicyError(message)


_DEFAULT_POLICY = AnchorPolicy()


@dataclass(frozen=True, slots=True, order=True)
class StableAnchor:
    """One selected source-content fingerprint and its authoring offset."""

    digest: bytes
    offset: int

    def __post_init__(self) -> None:
        """Require one exact SHA-256 digest and non-negative byte offset.

        Raises:
            FingerprintPolicyError: Digest or offset metadata is invalid.

        """
        if type(self.digest) is not bytes or len(self.digest) != _SHA256_BYTES:
            message = "stable anchor digest must be exactly 32 bytes"
            raise FingerprintPolicyError(message)
        if type(self.offset) is not int or self.offset < _ZERO:
            message = "stable anchor offset must be a non-negative integer"
            raise FingerprintPolicyError(message)


def _require_coverage_count(value: object, context: str) -> int:
    if type(value) is not int or value < _ZERO:
        message = f"{context} must be a non-negative integer"
        raise FingerprintPolicyError(message)
    return value


def _require_coverage_ratio(value: object) -> float:
    if type(value) is not float or not 0.0 <= value <= 1.0:
        message = "anchor coverage ratio must be an exact float in [0, 1]"
        raise FingerprintPolicyError(message)
    return value


def _validate_anchor_coverage(coverage: AnchorCoverage) -> None:
    matched = _require_coverage_count(
        coverage.matched,
        "matched anchor coverage",
    )
    total = _require_coverage_count(
        coverage.total,
        "total anchor coverage",
    )
    if matched > total:
        message = "matched anchor coverage cannot exceed total anchors"
        raise FingerprintPolicyError(message)
    ratio = _require_coverage_ratio(coverage.ratio)
    expected = 1.0 if total == _ZERO else matched / total
    if ratio != expected:
        message = "anchor coverage ratio does not match anchor counts"
        raise FingerprintPolicyError(message)


@dataclass(frozen=True, slots=True)
class AnchorCoverage:
    """Measured overlap between reference and candidate stable anchors."""

    matched: int
    total: int
    ratio: float

    def __post_init__(self) -> None:
        """Require coverage counts and ratio to describe the same overlap."""
        _validate_anchor_coverage(self)


def _digest_window(window: bytes) -> bytes:
    return hashlib.sha256(window).digest()


def _small_input_anchor(data: bytes) -> tuple[StableAnchor, ...]:
    if not data:
        return ()
    return (StableAnchor(digest=_digest_window(data), offset=_ZERO),)


def _initial_rolling_hash(data: bytes, window_bytes: int) -> int:
    value = _ZERO
    for byte in data[:window_bytes]:
        value = ((value * _ROLLING_BASE) + byte) & _ROLLING_MASK
    return value


def _rolling_power(window_bytes: int) -> int:
    return pow(_ROLLING_BASE, window_bytes - _ONE, _ROLLING_MODULUS)


def _roll_hash(
    value: int,
    outgoing: int,
    incoming: int,
    *,
    leading_power: int,
) -> int:
    without_leading = (value - (outgoing * leading_power)) & _ROLLING_MASK
    return ((without_leading * _ROLLING_BASE) + incoming) & _ROLLING_MASK


def _selected_anchor(
    data: bytes, offset: int, window_bytes: int
) -> StableAnchor:
    window = data[offset : offset + window_bytes]
    return StableAnchor(digest=_digest_window(window), offset=offset)


def _scan_anchor_windows(
    data: bytes,
    policy: AnchorPolicy,
) -> tuple[dict[bytes, StableAnchor], int]:
    last_offset = len(data) - policy.window_bytes
    selected: dict[bytes, StableAnchor] = {}
    rolling = _initial_rolling_hash(data, policy.window_bytes)
    leading_power = _rolling_power(policy.window_bytes)
    fallback_value = rolling
    fallback_offset = _ZERO
    for offset in range(last_offset + _ONE):
        if rolling < fallback_value:
            fallback_value = rolling
            fallback_offset = offset
        if rolling % policy.selection_modulus == _ZERO:
            anchor = _selected_anchor(data, offset, policy.window_bytes)
            _ = selected.setdefault(anchor.digest, anchor)
        if offset < last_offset:
            rolling = _roll_hash(
                rolling,
                data[offset],
                data[offset + policy.window_bytes],
                leading_power=leading_power,
            )
    return selected, fallback_offset


def stable_anchors(
    data: bytes,
    policy: AnchorPolicy = _DEFAULT_POLICY,
) -> tuple[StableAnchor, ...]:
    """Select deterministic anchors from sliding content windows.

    A 64-bit polynomial rolling hash evaluates every fixed-size content window.
    The rolling value selects sparse windows without a cryptographic hash call
    at every byte offset; selected windows are then fingerprinted with SHA-256.
    Insertions shift offsets but unchanged windows retain the same selector
    value
    and final digest. If no window is selected, the minimum rolling-hash window
    becomes a deterministic fallback.

    Returns:
        Unique selected SHA-256 digests ordered by authoring offset.

    Raises:
        FingerprintPolicyError: Input bytes or policy metadata is invalid.

    """
    if type(data) is not bytes:
        message = "stable anchor input must use exact bytes"
        raise FingerprintPolicyError(message)
    if type(policy) is not AnchorPolicy:
        message = "stable anchor policy must use the exact AnchorPolicy type"
        raise FingerprintPolicyError(message)
    if len(data) < policy.window_bytes:
        return _small_input_anchor(data)
    selected, fallback_offset = _scan_anchor_windows(data, policy)
    if not selected:
        fallback = _selected_anchor(data, fallback_offset, policy.window_bytes)
        selected[fallback.digest] = fallback
    return tuple(sorted(selected.values(), key=lambda anchor: anchor.offset))


def anchor_coverage(
    reference: tuple[StableAnchor, ...],
    candidate: tuple[StableAnchor, ...],
) -> AnchorCoverage:
    """Measure how many unique reference anchor digests survive in a candidate.

    Returns:
        Matched count, reference count, and matched/reference ratio.

    Raises:
        FingerprintPolicyError: Anchor collections contain foreign records.

    """
    if type(reference) is not tuple or type(candidate) is not tuple:
        message = "anchor coverage inputs must use exact immutable tuples"
        raise FingerprintPolicyError(message)
    if any(
        type(anchor) is not StableAnchor for anchor in (*reference, *candidate)
    ):
        message = "anchor coverage inputs contain a foreign anchor record"
        raise FingerprintPolicyError(message)
    reference_digests = {anchor.digest for anchor in reference}
    candidate_digests = {anchor.digest for anchor in candidate}
    total = len(reference_digests)
    if total == _ZERO:
        return AnchorCoverage(matched=_ZERO, total=_ZERO, ratio=1.0)
    matched = len(reference_digests & candidate_digests)
    return AnchorCoverage(
        matched=matched,
        total=total,
        ratio=matched / total,
    )
