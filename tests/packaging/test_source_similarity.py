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
#   - Source-similarity calibration regression tests.
# - Must-Not:
#   - Select production thresholds or serialize proprietary source content.
# - Allows:
#   - Content-free count-vector evidence and malformed-input fixtures.
# - Split-When:
#   - Calibration behavior gains independently versioned test surfaces.
# - Merge-When:
#   - Similarity measurement and admission become one inseparable contract.
# - Summary:
#   - Guards evidence-only structural similarity calibration.
# - Description:
#   - Exercises exact rational measurement without a production admission gate.
# - Usage:
#   - Run through the canonical Jig pytest gate or repository-local pytest.
# - Defaults:
#   - Loads the repository-owned source-similarity adapter directly.
#

"""Regression tests for evidence-only source similarity calibration."""

from __future__ import annotations

from fractions import Fraction
import importlib.util
from pathlib import Path
import sys
import unittest

_ROOT = Path(__file__).resolve().parents[2]
_PATH = _ROOT / "tools/source-similarity/adapter-inbound/main.py"
_SPEC = importlib.util.spec_from_file_location(
    "shar_source_similarity_test",
    _PATH,
)
if _SPEC is None or _SPEC.loader is None:
    raise RuntimeError("cannot load source similarity calibrator")
_MOD = importlib.util.module_from_spec(_SPEC)
sys.modules[_SPEC.name] = _MOD
_SPEC.loader.exec_module(_MOD)


class SourceSimilarityTests(unittest.TestCase):
    def test_identical_vectors_are_complete(self) -> None:
        vector = {("aa", "p3d"): 4, ("bb", "rcf"): 2}
        evidence = _MOD.measure(vector, vector)
        self.assertEqual(evidence.reference_coverage, Fraction(1, 1))
        self.assertEqual(evidence.weighted_jaccard, Fraction(1, 1))

    def test_missing_counts_reduce_coverage_without_admission_result(
        self,
    ) -> None:
        reference = {("aa", "p3d"): 8, ("bb", "rcf"): 2}
        candidate = {("aa", "p3d"): 4, ("bb", "rcf"): 2}
        evidence = _MOD.measure(reference, candidate)
        self.assertEqual(evidence.reference_coverage, Fraction(3, 5))
        self.assertEqual(evidence.weighted_jaccard, Fraction(3, 5))
        self.assertFalse(hasattr(evidence, "accepted"))

    def test_extra_candidate_content_affects_similarity_not_reference_coverage(
        self,
    ) -> None:
        reference = {("aa", "p3d"): 4}
        candidate = {("aa", "p3d"): 4, ("zz", "bin"): 4}
        evidence = _MOD.measure(reference, candidate)
        self.assertEqual(evidence.reference_coverage, Fraction(1, 1))
        self.assertEqual(evidence.weighted_jaccard, Fraction(1, 2))

    def test_collision_ordinals_do_not_hide_shared_structure(self) -> None:
        reference = {("aa~01", "p3d"): 2, ("aa~02", "p3d"): 3}
        candidate = {("aa", "p3d"): 3}
        evidence = _MOD.measure(reference, candidate)
        self.assertEqual(evidence.reference_coverage, Fraction(3, 5))
        self.assertEqual(evidence.weighted_jaccard, Fraction(3, 5))

    def test_wide_generated_collision_ordinal_joins_family(self) -> None:
        reference = {("aa~100", "p3d"): 2}
        candidate = {("aa", "p3d"): 2}
        evidence = _MOD.measure(reference, candidate)
        self.assertEqual(evidence.reference_coverage, Fraction(1, 1))
        self.assertEqual(evidence.weighted_jaccard, Fraction(1, 1))

    def test_non_generated_suffix_remains_distinct(self) -> None:
        for directory in ("aa~1", "aa~00", "aa~001"):
            with self.subTest(directory=directory):
                reference = {(directory, "p3d"): 2}
                candidate = {("aa", "p3d"): 2}
                evidence = _MOD.measure(reference, candidate)
                self.assertEqual(evidence.reference_coverage, Fraction(0, 1))
                self.assertEqual(evidence.weighted_jaccard, Fraction(0, 1))

    def test_invalid_or_empty_reference_vectors_fail_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "must not be empty"):
            _MOD.measure({}, {("aa", "p3d"): 1})
        with self.assertRaisesRegex(ValueError, "nonnegative integer"):
            _MOD.measure({("aa", "p3d"): -1}, {})
        with self.assertRaisesRegex(ValueError, "pair of strings"):
            _MOD.measure({"aa": 1}, {})


if __name__ == "__main__":
    unittest.main()
