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

from contextlib import redirect_stderr
from contextlib import redirect_stdout
from fractions import Fraction
import importlib.util
from io import StringIO
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

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


class ChangingCountMapping:
    """Count source that changes after its first read."""

    def __init__(self) -> None:
        self.reads = 0

    def _next_value(self) -> int:
        self.reads += 1
        return 1 if self.reads == 1 else -1

    def items(self) -> tuple[tuple[tuple[str, str], int], ...]:
        """Return one coordinate paired with the next observed count."""
        return ((("aa", "p3d"), self._next_value()),)

    def values(self) -> tuple[int, ...]:
        """Return the next observed count as a values collection."""
        return (self._next_value(),)


class SourceSimilarityTests(unittest.TestCase):
    def test_identical_vectors_are_complete(self) -> None:
        vector = {("aa", "p3d"): 4, ("bb", "rcf"): 2}
        evidence = _MOD.measure(vector, vector)
        self.assertEqual(evidence.reference_coverage, Fraction(1, 1))
        self.assertEqual(evidence.weighted_jaccard, Fraction(1, 1))

        changing_reference = ChangingCountMapping()
        changing_evidence = _MOD.measure(
            changing_reference,
            {("aa", "p3d"): 1},
        )
        self.assertEqual(changing_reference.reads, 1)
        self.assertEqual(changing_evidence.reference_units, 1)
        self.assertEqual(changing_evidence.reference_coverage, Fraction(1, 1))
        self.assertEqual(changing_evidence.weighted_jaccard, Fraction(1, 1))

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

    def test_invalid_or_empty_reference_vectors_fail_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "must not be empty"):
            _MOD.measure({}, {("aa", "p3d"): 1})
        with self.assertRaisesRegex(ValueError, "nonnegative integer"):
            _MOD.measure({("aa", "p3d"): -1}, {})
        with self.assertRaisesRegex(ValueError, "pair of strings"):
            _MOD.measure({"aa": 1}, {})
        with self.assertRaises(_MOD.InvalidCoordinateError):
            _MOD.measure({("aa", ""): 1}, {})
        for coordinate in (("\ud800", "p3d"), ("aa", "\ud800")):
            with (
                self.subTest(coordinate=repr(coordinate)),
                self.assertRaises(_MOD.InvalidCoordinateError),
            ):
                _MOD.measure({coordinate: 1}, {})
        for extension in ("P3D", "ÄBC"):
            with (
                self.subTest(extension=extension),
                self.assertRaises(_MOD.InvalidCoordinateError),
            ):
                _MOD.measure({("aa", extension): 1}, {})
        for directory in ("AA", "Aa", r"aa\bb", "/aa", "aa/", "aa//bb"):
            with (
                self.subTest(directory=directory),
                self.assertRaises(_MOD.InvalidCoordinateError),
            ):
                _MOD.measure({(directory, "p3d"): 1}, {})
        _MOD.measure({("", "p3d"): 1}, {})

    def test_parser_accepts_generated_manifest_metadata_and_kind(self) -> None:
        ledger = (
            '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
            '"kind_taxonomy":[],"required_files":[]}\n'
            '{"dir":"aa","ext":"p3d","min":2,'
            '"kind":"p3d_container"}\n'
        )
        self.assertEqual(
            _MOD.parse_count_ledger(ledger),
            {("aa", "p3d"): 2},
        )

    def test_parser_accepts_observed_count_rows(self) -> None:
        ledger = (
            '{"dir":"aa","ext":"p3d","count":2,'
            '"kind":"p3d_container"}'
        )
        self.assertEqual(_MOD.parse_count_ledger(ledger), {("aa", "p3d"): 2})

    def test_parser_rejects_ambiguous_jsonl_records(self) -> None:
        valid_pair = (
            '{"dir":"\\ud83d\\ude80\\ud83d\\ude80",'
            '"ext":"p3d","min":1}'
        )
        self.assertEqual(
            _MOD.parse_count_ledger(valid_pair),
            {("🚀🚀", "p3d"): 1},
        )
        expanded_lowercase = (
            '{"dir":"i\\u0307z","ext":"p3d","min":1}'
        )
        self.assertEqual(
            _MOD.parse_count_ledger(expanded_lowercase),
            {("i\u0307z", "p3d"): 1},
        )

        records = (
            ' {"dir":"aa","ext":"p3d","min":1}',
            '{"dir":"aa","ext":"p3d","min":1} ',
            '\t{"dir":"aa","ext":"p3d","min":1}',
            '{"dir":"aa","ext":"p3d","min":1}\t',
            '{"dir":"aa","dir":"bb","ext":"p3d","min":1}',
            '{"dir":"aa","ext":"p3d","min":1,"extra":true}',
            '{"dir":"aa","ext":"p3d","min":1,"count":1}',
            '{"dir":"aa","ext":"p3d"}',
            '{"dir":"aa","ext":"","min":1}',
            '{"dir":"aa","ext":"P3D","min":1}',
            '{"dir":"aa","ext":"ÄBC","min":1}',
            '{"dir":"AA","ext":"p3d","min":1}',
            '{"dir":"aa\\\\bb","ext":"p3d","min":1}',
            '{"dir":"/aa","ext":"p3d","min":1}',
            '{"dir":"aa/","ext":"p3d","min":1}',
            '{"dir":"aa//bb","ext":"p3d","min":1}',
            '{"dir":"aa","ext":"p3d","min":1,"kind":null}',
            '{"dir":"aa","ext":"p3d","min":1,"kind":{"private":1}}',
            '{"dir":"aa","ext":"p3d","min":1,"kind":"private-data"}',
            '{"dir":"\\ud800","ext":"p3d","min":1}',
            '{"dir":"aa","ext":"\\ud800","min":1}',
            '{"dir":"aa","ext":"p3d","min":1,"kind":"\\ud800"}',
            '{"dir":"aa","ext":"p3d","min":18446744073709551616}',
            '{"dir":"aa","ext":"p3d","min":' + ("9" * 5000) + "}",
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"private_metadata":"not-public"}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"kind_taxonomy":null}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":null}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"kind_taxonomy":["\\ud800"],"required_files":[]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"kind_taxonomy":{},"required_files":[]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"kind_taxonomy":["private-data"],"required_files":[]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"kind_taxonomy":["audio","audio"],"required_files":[]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"kind_taxonomy":[],"required_files":{}}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"\\ud800","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"README.rtf","min":1,'
                '"private":"metadata"}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"/private/source","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"../private","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"private\\source","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"C:private/source","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"AUX/file","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"private./file","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"private /file","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"bad?name/file","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"private\u0001/file","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"private\u200b/file","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"'
                + ("a" * 256)
                + '","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"README.rtf",'
                '"min":18446744073709551616}]}'
            ),
        )
        for record in records:
            with (
                self.subTest(record=record[:120]),
                self.assertRaises(_MOD.SimilarityInputError),
            ):
                _MOD.parse_count_ledger(record)

        aggregate_overflow = (
            '{"dir":"aa","ext":"p3d","min":18446744073709551615}\n'
            '{"dir":"bb","ext":"p3d","min":1}\n'
        )
        with self.assertRaises(_MOD.InvalidCountError):
            _MOD.parse_count_ledger(aggregate_overflow)

    def test_parser_rejects_mixed_count_meanings(self) -> None:
        ledger = (
            '{"dir":"aa","ext":"p3d","min":1}\n'
            '{"dir":"bb","ext":"rcf","count":1}\n'
        )
        with self.assertRaisesRegex(ValueError, "mixes count meanings"):
            _MOD.parse_count_ledger(ledger)

    def test_cli_measures_public_jsonl_ledgers_without_admission(self) -> None:
        metadata = '{"schema":"shar-schoenwald.game-manifest-ledger.v2"}\n'
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            reference = root / "reference.jsonl"
            candidate = root / "candidate.jsonl"
            reference.write_text(
                metadata
                + '{"dir":"aa~01","ext":"p3d","min":2}\n'
                + '{"dir":"aa~02","ext":"p3d","min":3}\n',
                encoding="utf-8",
            )
            candidate.write_text(
                metadata + '{"dir":"aa","ext":"p3d","min":3}\n',
                encoding="utf-8",
            )
            stdout = StringIO()
            stderr = StringIO()
            with redirect_stdout(stdout), redirect_stderr(stderr):
                return_code = _MOD.main([str(reference), str(candidate)])

        self.assertEqual(return_code, 0, stderr.getvalue())
        self.assertEqual(
            stdout.getvalue(),
            "source-similarity\treference_units=5\tcandidate_units=3"
            "\tshared_units=3\tunion_units=5\treference_coverage=3/5"
            "\tweighted_jaccard=3/5\n",
        )
        self.assertNotIn("accepted", stdout.getvalue())
        self.assertEqual(stderr.getvalue(), "")

    def test_duplicate_public_coordinates_fail_closed(self) -> None:
        ledger = (
            '{"dir":"aa","ext":"p3d","min":1}\n'
            '{"dir":"aa","ext":"p3d","min":2}\n'
        )
        with self.assertRaisesRegex(ValueError, "repeats a coordinate"):
            _MOD.parse_count_ledger(ledger)

    def test_cli_rejects_unreadable_ledger_without_disclosing_path(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            reference = root / "private-missing-reference.jsonl"
            candidate = root / "candidate.jsonl"
            candidate.write_text(
                '{"dir":"aa","ext":"p3d","min":1}\n',
                encoding="utf-8",
            )
            stdout = StringIO()
            stderr = StringIO()
            with redirect_stdout(stdout), redirect_stderr(stderr):
                return_code = _MOD.main([str(reference), str(candidate)])

        self.assertNotEqual(return_code, 0)
        self.assertEqual(stdout.getvalue(), "")
        self.assertNotIn(str(reference), stderr.getvalue())
        self.assertIn("could not be read", stderr.getvalue())

        with tempfile.TemporaryDirectory() as directory:
            special = Path(directory)
            with (
                mock.patch.object(
                    Path,
                    "read_bytes",
                    side_effect=AssertionError("special ledger was read"),
                ) as read_bytes,
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "regular file",
                ),
            ):
                _MOD.load_count_ledger(special)
            read_bytes.assert_not_called()

    def test_cli_rejects_malformed_ledger_without_disclosing_path(self) -> None:
        malformed_ledgers = (
            ("not-json\n", "invalid JSONL"),
            ('{"dir":"aa","ext":"p3d","min":1}', "end with LF"),
            ('{"dir":"aa","ext":"p3d","min":1}\r\n', "LF line endings"),
            (
                (
                    '{"dir":"aa","ext":"p3d","min":1}'
                    '\u2028{"dir":"bb","ext":"p3d","min":1}\n'
                ),
                "invalid JSONL",
            ),
        )
        for contents, expected in malformed_ledgers:
            with self.subTest(expected=expected):
                with tempfile.TemporaryDirectory() as directory:
                    root = Path(directory)
                    reference = root / "private-reference-name.jsonl"
                    candidate = root / "candidate.jsonl"
                    reference.write_text(contents, encoding="utf-8")
                    candidate.write_text(
                        '{"dir":"aa","ext":"p3d","min":1}\n',
                        encoding="utf-8",
                    )
                    stdout = StringIO()
                    stderr = StringIO()
                    with redirect_stdout(stdout), redirect_stderr(stderr):
                        return_code = _MOD.main(
                            [str(reference), str(candidate)]
                        )

                self.assertNotEqual(return_code, 0)
                self.assertEqual(stdout.getvalue(), "")
                self.assertNotIn(str(reference), stderr.getvalue())
                self.assertIn(expected, stderr.getvalue())


class SourceSimilarityAliasShapeTests(unittest.TestCase):
    """Guard the manifest producer's bounded public directory aliases."""

    def test_non_generated_suffix_remains_distinct(self) -> None:
        reference = {("aa~1", "p3d"): 2}
        candidate = {("aa", "p3d"): 2}
        evidence = _MOD.measure(reference, candidate)
        self.assertEqual(evidence.reference_coverage, Fraction(0, 1))
        self.assertEqual(evidence.weighted_jaccard, Fraction(0, 1))

        for directory in (
            "aa~00",
            "aa~001",
            "aa~18446744073709551616",
        ):
            with (
                self.subTest(directory=directory),
                self.assertRaises(_MOD.InvalidCoordinateError),
            ):
                _MOD.measure({(directory, "p3d"): 2}, candidate)

    def test_parser_rejects_private_directory_names_outside_alias_shape(
        self,
    ) -> None:
        for directory in (
            "private-source-name",
            "aa/private",
            "aa/visible",
        ):
            with (
                self.subTest(directory=directory),
                self.assertRaises(_MOD.InvalidCoordinateError),
            ):
                _MOD.parse_count_ledger(
                    json.dumps({
                        "dir": directory,
                        "ext": "p3d",
                        "count": 1,
                    })
                )


if __name__ == "__main__":
    unittest.main()
