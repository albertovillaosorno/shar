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
from dataclasses import dataclass
from fractions import Fraction

Coordinate = tuple[str, str]


@dataclass(frozen=True, slots=True)
class SimilarityEvidence:
    """Pure calibration evidence with no admission decision."""

    reference_units: int
    candidate_units: int
    shared_units: int
    union_units: int
    reference_coverage: Fraction
    weighted_jaccard: Fraction


def measure(
    reference: Mapping[Coordinate, int],
    candidate: Mapping[Coordinate, int],
) -> SimilarityEvidence:
    """Measure nonnegative count vectors without selecting a threshold."""
    _validate(reference)
    _validate(candidate)
    reference_units = sum(reference.values())
    candidate_units = sum(candidate.values())
    if reference_units == 0:
        raise ValueError("reference count vector must not be empty")
    coordinates = set(reference) | set(candidate)
    shared = sum(
        min(reference.get(key, 0), candidate.get(key, 0)) for key in coordinates
    )
    union = sum(
        max(reference.get(key, 0), candidate.get(key, 0)) for key in coordinates
    )
    if union == 0:
        raise ValueError("count-vector union must not be empty")
    return SimilarityEvidence(
        reference_units=reference_units,
        candidate_units=candidate_units,
        shared_units=shared,
        union_units=union,
        reference_coverage=Fraction(shared, reference_units),
        weighted_jaccard=Fraction(shared, union),
    )


def _validate(values: Mapping[Coordinate, int]) -> None:
    for key, count in values.items():
        if (
            not isinstance(key, tuple)
            or len(key) != 2
            or not all(isinstance(value, str) for value in key)
        ):
            raise ValueError("coordinate must be a pair of strings")
        if isinstance(count, bool) or not isinstance(count, int) or count < 0:
            raise ValueError("coordinate count must be a nonnegative integer")
