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
        candidate = {("aa", "p3d"): 4, ("zz", "png"): 4}
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
        reference = {
            (f"aa~{ordinal:02d}", "p3d"): 1 for ordinal in range(1, 100)
        }
        reference["aa~100", "p3d"] = 1
        candidate = {("aa", "p3d"): 100}
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
        for extension in ("private", "dll"):
            with (
                self.subTest(unclassifiable_extension=extension),
                self.assertRaises(_MOD.InvalidCoordinateError),
            ):
                _MOD.measure({("aa", extension): 1}, {})
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
        header = (
            (_ROOT / "game/manifest/game.jsonl")
            .read_text(encoding="utf-8")
            .splitlines()[0]
        )
        ledger = (
            header
            + "\n"
            + '{"dir":"aa","ext":"p3d","min":2,"kind":"p3d_container"}\n'
        )
        self.assertEqual(
            _MOD.parse_count_ledger(ledger),
            {("aa", "p3d"): 2},
        )

    def test_parser_accepts_observed_count_rows(self) -> None:
        ledger = '{"dir":"aa","ext":"p3d","count":2,"kind":"p3d_container"}'
        self.assertEqual(_MOD.parse_count_ledger(ledger), {("aa", "p3d"): 2})

    def test_parser_rejects_ambiguous_jsonl_records(self) -> None:
        valid_pair = (
            '{"dir":"\\ud83d\\ude80\\ud83d\\ude80","ext":"p3d","min":1}'
        )
        self.assertEqual(
            _MOD.parse_count_ledger(valid_pair),
            {("🚀🚀", "p3d"): 1},
        )
        expanded_lowercase = '{"dir":"i\\u0307z","ext":"p3d","min":1}'
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
                '"required_files":['
                '{"path":"README.rtf","min":1},'
                '{"path":"README.rtf","min":1}]}'
            ),
            (
                '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
                '"required_files":[{"path":"private/source","min":1}]}'
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
                '"required_files":[{"path":"' + ("a" * 256) + '","min":1}]}'
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

    def test_parser_rejects_noncanonical_kind_metadata(self) -> None:
        rows = (
            '{"dir":"aa","ext":"p3d","count":1,"kind":"audio"}\n',
            '{"dir":"aa","ext":"mystery","count":1}\n',
            ('{"dir":"","ext":"png","count":1,"kind":"image"}\n'),
        )
        for row in rows:
            with (
                self.subTest(row=row),
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "kind metadata",
                ),
            ):
                _MOD.parse_count_ledger(row)

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
                    "open",
                    side_effect=AssertionError("special ledger was opened"),
                ) as open_file,
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "regular file",
                ),
            ):
                _MOD.load_count_ledger(special)
            open_file.assert_not_called()

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
                        return_code = _MOD.main([
                            str(reference),
                            str(candidate),
                        ])

                self.assertNotEqual(return_code, 0)
                self.assertEqual(stdout.getvalue(), "")
                self.assertNotIn(str(reference), stderr.getvalue())
                self.assertIn(expected, stderr.getvalue())


class SourceSimilarityJsonRobustnessTests(unittest.TestCase):
    """Keep malformed JSON failures inside the path-free input boundary."""

    def test_parser_rejects_zero_observation_but_allows_optional_minimum(
        self,
    ) -> None:
        with self.assertRaises(_MOD.InvalidCountError):
            _MOD.parse_count_ledger('{"dir":"aa","ext":"p3d","count":0}')
        self.assertEqual(
            _MOD.parse_count_ledger('{"dir":"","ext":"png","min":0}'),
            {("", "png"): 0},
        )

    def test_parser_rejects_signed_zero_counts(self) -> None:
        for count_field in ("count", "min"):
            ledger = '{"dir":"aa","ext":"p3d","' + count_field + '":-0}'
            with (
                self.subTest(count_field=count_field),
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "invalid JSONL",
                ),
            ):
                _MOD.parse_count_ledger(ledger)

    def test_parser_rejects_escaped_json_member_names(self) -> None:
        aliases = (
            '{"d\\u0069r":"aa","ext":"p3d","count":1}',
            '{"dir":"aa","e\\u0078t":"p3d","count":1}',
            '{"dir":"aa","ext":"p3d","co\\u0075nt":1}',
            '{"sch\\u0065ma":"shar-schoenwald.game-manifest-ledger.v2"}',
        )

        for ledger in aliases:
            with (
                self.subTest(ledger=ledger),
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "member name uses an escape",
                ),
            ):
                _MOD.parse_count_ledger(ledger)

    def test_parser_rejects_json_token_whitespace(self) -> None:
        aliases = (
            '{"dir": "aa","ext":"p3d","count":1}',
            '{"dir":"aa", "ext":"p3d","count":1}',
            '{"dir":"aa","ext":"p3d","count": 1}',
            '{"dir":"aa","ext":"p3d",\t"count":1}',
            '{"dir"\r:"aa","ext":"p3d","count":1}',
            '{"dir":"aa",\r"ext":"p3d","count":1}',
            '{"dir":"aa","ext"\r:\r"p3d","count":1}',
        )

        for ledger in aliases:
            with (
                self.subTest(ledger=ledger),
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "JSON tokens have whitespace",
                ),
            ):
                _MOD.parse_count_ledger(ledger)

    def test_parser_rejects_excessively_nested_json(self) -> None:
        depth = 100_000
        ledger = (
            '{"schema":"shar-schoenwald.game-manifest-ledger.v2",'
            '"required_files":' + ("[" * depth) + "0" + ("]" * depth) + "}\n"
        )

        with self.assertRaisesRegex(
            _MOD.LedgerInputError,
            "invalid JSONL",
        ):
            _MOD.parse_count_ledger(ledger)


class SourceSimilaritySchemaParityTests(unittest.TestCase):
    """Keep calibration metadata aligned with the tracked public manifest."""

    def test_parser_rejects_noncanonical_manifest_header_lists(self) -> None:
        header = json.loads(
            (_ROOT / "game/manifest/game.jsonl")
            .read_text(encoding="utf-8")
            .splitlines()[0]
        )
        variants = []

        partial_taxonomy = dict(header)
        partial_taxonomy["kind_taxonomy"] = header["kind_taxonomy"][:-1]
        variants.append(partial_taxonomy)

        reordered_taxonomy = dict(header)
        reordered_taxonomy["kind_taxonomy"] = list(
            reversed(header["kind_taxonomy"])
        )
        variants.append(reordered_taxonomy)

        partial_required = dict(header)
        partial_required["required_files"] = header["required_files"][:-1]
        variants.append(partial_required)

        reordered_required = dict(header)
        reordered_required["required_files"] = list(
            reversed(header["required_files"])
        )
        variants.append(reordered_required)

        row = '{"dir":"aa","ext":"p3d","count":1}\n'
        for variant in variants:
            ledger = json.dumps(variant, separators=(",", ":")) + "\n" + row
            with (
                self.subTest(variant=variant),
                self.assertRaises(_MOD.LedgerInputError),
            ):
                _MOD.parse_count_ledger(ledger)

    def test_tracked_public_manifest_header_is_accepted(self) -> None:
        counts = _MOD.load_count_ledger(_ROOT / "game/manifest/game.jsonl")
        self.assertTrue(counts)


class SourceSimilarityCanonicalOrderTests(unittest.TestCase):
    """Require JSONL rows to retain deterministic producer coordinate order."""

    def test_parser_rejects_reordered_json_members(self) -> None:
        header = json.loads(
            (_ROOT / "game/manifest/game.jsonl")
            .read_text(encoding="utf-8")
            .splitlines()[0]
        )
        reordered_header = {
            "required_files": header["required_files"],
            "schema": header["schema"],
            "kind_taxonomy": header["kind_taxonomy"],
        }
        reordered_requirement = dict(header)
        first = header["required_files"][0]
        reordered_requirement["required_files"] = [
            {"min": first["min"], "path": first["path"]},
            *header["required_files"][1:],
        ]
        variants = (
            '{"ext":"p3d","dir":"aa","count":1}',
            '{"dir":"aa","count":1,"ext":"p3d"}',
            '{"dir":"aa","ext":"p3d","kind":"p3d_container","count":1}',
            json.dumps(reordered_header, separators=(",", ":")),
            json.dumps(reordered_requirement, separators=(",", ":")),
        )

        for ledger in variants:
            with (
                self.subTest(ledger=ledger[:120]),
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "order|metadata",
                ),
            ):
                _MOD.parse_count_ledger(ledger)

    def test_parser_rejects_reordered_coordinates(self) -> None:
        ledgers = (
            (
                '{"dir":"bb","ext":"p3d","count":1}\n'
                '{"dir":"aa","ext":"p3d","count":1}\n'
            ),
            (
                '{"dir":"aa","ext":"rcf","count":1}\n'
                '{"dir":"aa","ext":"p3d","count":1}\n'
            ),
        )
        for ledger in ledgers:
            with (
                self.subTest(ledger=ledger),
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "coordinate order",
                ),
            ):
                _MOD.parse_count_ledger(ledger)


class SourceSimilarityLedgerFileTests(unittest.TestCase):
    """Require stable non-redirected calibration-ledger snapshots."""

    def test_ledger_identity_includes_platform_ctime(self) -> None:
        before = mock.Mock(
            st_dev=1,
            st_ino=2,
            st_mtime_ns=3,
            st_ctime_ns=4,
            st_size=5,
        )
        after = mock.Mock(
            st_dev=1,
            st_ino=2,
            st_mtime_ns=3,
            st_ctime_ns=6,
            st_size=5,
        )

        self.assertNotEqual(
            _MOD._ledger_identity(before),
            _MOD._ledger_identity(after),
        )

    def test_ledger_path_drift_fails_before_payload_read(self) -> None:
        expected = _MOD._LedgerIdentity(
            device=1,
            inode=2,
            modified_ns=3,
            ctime_ns=4,
            size=5,
        )
        handle = mock.MagicMock()
        handle.__enter__.return_value = handle
        handle.read.side_effect = AssertionError("payload was read")
        with (
            mock.patch.object(
                _MOD,
                "_regular_ledger_identity",
                return_value=expected,
            ),
            mock.patch.object(_MOD, "_ledger_identity", return_value=expected),
            mock.patch.object(
                _MOD,
                "_current_ledger_identity",
                return_value=None,
            ),
            mock.patch.object(Path, "open", return_value=handle),
            mock.patch.object(_MOD.os, "fstat", return_value=mock.Mock()),
            self.assertRaisesRegex(
                _MOD.LedgerInputError,
                "changed while reading",
            ),
        ):
            _MOD.load_count_ledger(Path("private-ledger.jsonl"))
        handle.read.assert_not_called()

    def test_ledger_symlink_is_rejected_before_payload_read(self) -> None:
        if sys.platform == "win32":
            self.skipTest("symlink fixture is Unix-focused")
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = root / "target.jsonl"
            linked = root / "linked.jsonl"
            target.write_text(
                '{"dir":"aa","ext":"p3d","count":1}\n',
                encoding="utf-8",
            )
            linked.symlink_to(target.name)

            with self.assertRaisesRegex(
                _MOD.LedgerInputError,
                "regular file",
            ):
                _MOD.load_count_ledger(linked)

    def test_ledger_parent_redirect_is_rejected(self) -> None:
        if sys.platform == "win32":
            self.skipTest("symlink fixture is Unix-focused")
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            real = root / "real"
            real.mkdir()
            ledger = real / "ledger.jsonl"
            ledger.write_text(
                '{"dir":"aa","ext":"p3d","count":1}\n',
                encoding="utf-8",
            )
            redirect = root / "redirect"
            redirect.symlink_to(real, target_is_directory=True)

            with self.assertRaisesRegex(
                _MOD.LedgerInputError,
                "real directory",
            ):
                _MOD.load_count_ledger(redirect / ledger.name)

    def test_ledger_parent_junction_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            parent = root / "junction-parent"
            parent.mkdir()
            ledger = parent / "ledger.jsonl"
            ledger.write_text(
                '{"dir":"aa","ext":"p3d","count":1}\n',
                encoding="utf-8",
            )
            with (
                mock.patch.object(
                    _MOD.os.path,
                    "isjunction",
                    side_effect=lambda path: Path(path) == parent,
                ),
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "real directory",
                ),
            ):
                _MOD.load_count_ledger(ledger)

    def test_ledger_identity_drift_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            ledger = Path(directory) / "ledger.jsonl"
            ledger.write_text(
                '{"dir":"aa","ext":"p3d","count":1}\n',
                encoding="utf-8",
            )
            with (
                mock.patch.object(
                    _MOD,
                    "_current_ledger_identity",
                    return_value=None,
                ),
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "changed while reading",
                ),
            ):
                _MOD.load_count_ledger(ledger)


class SourceSimilarityAliasShapeTests(unittest.TestCase):
    """Guard the manifest producer's bounded public directory aliases."""

    def test_non_generated_suffixes_fail_closed(self) -> None:
        candidate = {("aa", "p3d"): 2}
        for directory in (
            "aa~1",
            "aa~00",
            "aa~001",
            "aa~18446744073709551616",
        ):
            with (
                self.subTest(directory=directory),
                self.assertRaises(_MOD.InvalidCoordinateError),
            ):
                _MOD.measure({(directory, "p3d"): 2}, candidate)

    def test_programmatic_vectors_reject_noncanonical_collision_families(
        self,
    ) -> None:
        candidate = {("aa", "p3d"): 1}
        for reference in (
            {("aa~01", "p3d"): 1},
            {("aa~02", "p3d"): 1},
            {("aa~100", "p3d"): 1},
        ):
            with (
                self.subTest(reference=reference),
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "collision ordinals",
                ),
            ):
                _MOD.measure(reference, candidate)

    def test_parser_rejects_noncanonical_collision_families(self) -> None:
        ledgers = (
            '{"dir":"aa~01","ext":"p3d","count":1}\n',
            (
                '{"dir":"aa~01","ext":"p3d","count":1}\n'
                '{"dir":"aa~03","ext":"rcf","count":1}\n'
            ),
            (
                '{"dir":"aa","ext":"p3d","count":1}\n'
                '{"dir":"aa~01","ext":"rcf","count":1}\n'
                '{"dir":"aa~02","ext":"wav","count":1}\n'
            ),
        )
        for ledger in ledgers:
            with (
                self.subTest(ledger=ledger),
                self.assertRaisesRegex(
                    _MOD.LedgerInputError,
                    "collision ordinals",
                ),
            ):
                _MOD.parse_count_ledger(ledger)

    def test_parser_rejects_nonportable_obfuscated_components(self) -> None:
        for directory in (
            "a\u200b",
            "\u200b\u200b",
            "a\u0001",
            "\u0001a",
            "a:",
            "a?",
            "a*",
            "..",
            "a.",
            "a ",
        ):
            with (
                self.subTest(directory=repr(directory)),
                self.assertRaises(_MOD.InvalidCoordinateError),
            ):
                _MOD.parse_count_ledger(
                    json.dumps(
                        {
                            "dir": directory,
                            "ext": "p3d",
                            "count": 1,
                        },
                        separators=(",", ":"),
                    )
                )

    def test_parser_rejects_impossible_obfuscated_components(self) -> None:
        for directory in ("abc", "abcd", "aa~1"):
            with (
                self.subTest(directory=directory),
                self.assertRaises(_MOD.InvalidCoordinateError),
            ):
                _MOD.parse_count_ledger(
                    json.dumps(
                        {
                            "dir": directory,
                            "ext": "p3d",
                            "count": 1,
                        },
                        separators=(",", ":"),
                    )
                )

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
                    json.dumps(
                        {
                            "dir": directory,
                            "ext": "p3d",
                            "count": 1,
                        },
                        separators=(",", ":"),
                    )
                )


if __name__ == "__main__":
    unittest.main()
