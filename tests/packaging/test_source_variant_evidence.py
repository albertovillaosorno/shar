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
#   - Regression evidence for private source-variant comparison.
# - Must-Not:
#   - Use proprietary fixtures, expose local paths, or decide source admission.
# - Allows:
#   - Synthetic byte sequences and disposable local files.
# - Split-When:
#   - Variant evidence gains another independently versioned surface.
# - Merge-When:
#   - Another test owns identical ordered-byte comparison behavior.
# - Summary:
#   - Tests path-free ordered common-byte evidence.
# - Description:
#   - Proves exact ordered matches, partial evidence, and safe input failures.
# - Usage:
#   - Run through repository-local pytest or the canonical Jig gate.
# - Defaults:
#   - Synthetic bytes only.
#

"""Regression tests for private source-variant evidence."""

from __future__ import annotations

from contextlib import redirect_stderr
from contextlib import redirect_stdout
import importlib.util
from io import StringIO
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

_ROOT = Path(__file__).resolve().parents[2]
_PATH = _ROOT / "tools/source-variant-evidence/adapter-inbound/main.py"
_SPEC = importlib.util.spec_from_file_location(
    "shar_source_variant_evidence_test",
    _PATH,
)
if _SPEC is None or _SPEC.loader is None:
    raise RuntimeError("cannot load source variant evidence tool")
_MOD = importlib.util.module_from_spec(_SPEC)
sys.modules[_SPEC.name] = _MOD
_SPEC.loader.exec_module(_MOD)


class SourceVariantEvidenceTests(unittest.TestCase):
    def test_file_identity_includes_platform_ctime(self) -> None:
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

        self.assertNotEqual(_MOD._identity(before), _MOD._identity(after))

    def test_open_or_path_drift_fails_before_payload_read(self) -> None:
        expected = _MOD.FileIdentity(
            device=1,
            inode=2,
            modified_ns=3,
            ctime_ns=4,
            size=5,
        )
        changed = _MOD.FileIdentity(
            device=1,
            inode=2,
            modified_ns=3,
            ctime_ns=6,
            size=5,
        )
        operations = (
            (
                "reference",
                _MOD.load_reference,
                _MOD.ReferenceChangedError,
            ),
            (
                "streamed-candidate",
                lambda path: _MOD.measure_variant(b"x", path),
                _MOD.CandidateChangedError,
            ),
            (
                "candidate-snapshot",
                _MOD._read_candidate_snapshot,
                _MOD.CandidateChangedError,
            ),
        )
        drift_modes = (
            ("opened", changed, expected),
            ("current-path", expected, None),
        )

        for label, operation, error_type in operations:
            for drift, opened, current in drift_modes:
                handle = mock.MagicMock()
                handle.__enter__.return_value = handle
                handle.read.side_effect = AssertionError("payload was read")
                with (
                    self.subTest(label=label, drift=drift),
                    mock.patch.object(
                        _MOD,
                        "_regular_file_identity",
                        return_value=expected,
                    ),
                    mock.patch.object(
                        _MOD,
                        "_identity",
                        return_value=opened,
                    ),
                    mock.patch.object(
                        _MOD,
                        "_current_identity",
                        return_value=current,
                    ),
                    mock.patch.object(Path, "open", return_value=handle),
                    mock.patch.object(
                        _MOD.os,
                        "fstat",
                        return_value=mock.Mock(),
                    ),
                    self.assertRaises(error_type),
                ):
                    operation(Path("private-input.bin"))
                handle.read.assert_not_called()

    def test_complete_subsequence_reports_aggregate_evidence(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-variant-") as raw:
            root = Path(raw)
            candidate = root / "private-candidate.bin"
            candidate.write_bytes(b"aXbYYcZZd")

            evidence = _MOD.measure_variant(b"abcd", candidate)

            self.assertEqual(evidence.candidate_bytes, 9)
            self.assertEqual(evidence.matched_bytes, 4)
            self.assertEqual(evidence.reference_bytes, 4)
            self.assertTrue(evidence.complete)

    def test_partial_subsequence_is_evidence_not_failure(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-variant-") as raw:
            candidate = Path(raw) / "candidate.bin"
            candidate.write_bytes(b"aXb")

            evidence = _MOD.measure_variant(b"abcd", candidate)

            self.assertEqual(evidence.matched_bytes, 2)
            self.assertFalse(evidence.complete)

    def test_cli_omits_private_paths_and_admission_language(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-private-variant-") as raw:
            root = Path(raw)
            reference = root / "private-reference.bin"
            candidate = root / "private-candidate.bin"
            reference.write_bytes(b"ace")
            candidate.write_bytes(b"abcde")
            stdout = StringIO()
            stderr = StringIO()

            with redirect_stdout(stdout), redirect_stderr(stderr):
                status = _MOD.main([str(reference), str(candidate)])

            output = stdout.getvalue()
            self.assertEqual(status, 0)
            self.assertEqual(stderr.getvalue(), "")
            self.assertIn("variant=1", output)
            self.assertIn("complete=true", output)
            self.assertNotIn(str(root), output)
            self.assertNotIn("accepted", output.lower())

    def test_invalid_inputs_fail_without_path_disclosure(self) -> None:
        private = Path("/private/example/common.bin")
        stderr = StringIO()

        with redirect_stderr(stderr):
            status = _MOD.main([str(private), "variant.bin"])

        self.assertEqual(status, 1)
        self.assertNotIn(str(private), stderr.getvalue())
        self.assertIn("cannot be inspected", stderr.getvalue())

    def test_redirected_parent_is_rejected(self) -> None:
        if sys.platform == "win32":
            self.skipTest("symlink fixture is Unix-focused")
        with tempfile.TemporaryDirectory(prefix="shar-variant-parent-") as raw:
            root = Path(raw)
            real = root / "real"
            real.mkdir()
            candidate = real / "candidate.bin"
            candidate.write_bytes(b"abcd")
            redirect = root / "redirect"
            redirect.symlink_to(real, target_is_directory=True)

            with self.assertRaisesRegex(
                _MOD.VariantEvidenceError,
                "real directory",
            ):
                _MOD.measure_variant(b"abcd", redirect / candidate.name)

    def test_junction_parent_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-variant-junction-"
        ) as raw:
            root = Path(raw)
            parent = root / "junction-parent"
            parent.mkdir()
            candidate = parent / "candidate.bin"
            candidate.write_bytes(b"abcd")
            with (
                mock.patch.object(
                    _MOD.os.path,
                    "isjunction",
                    side_effect=lambda path: Path(path) == parent,
                ),
                self.assertRaisesRegex(
                    _MOD.VariantEvidenceError,
                    "real directory",
                ),
            ):
                _MOD.measure_variant(b"abcd", candidate)

    def test_candidate_identity_drift_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-variant-") as raw:
            candidate = Path(raw) / "candidate.bin"
            candidate.write_bytes(b"abcd")

            with (
                mock.patch.object(_MOD, "_current_identity", return_value=None),
                self.assertRaises(_MOD.CandidateChangedError),
            ):
                _MOD.measure_variant(b"abcd", candidate)

    def test_later_invalid_variant_emits_no_partial_evidence(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-private-variant-") as raw:
            root = Path(raw)
            reference = root / "common.bin"
            candidate = root / "candidate.bin"
            missing = root / "private-missing.bin"
            reference.write_bytes(b"ace")
            candidate.write_bytes(b"abcde")
            stdout = StringIO()
            stderr = StringIO()

            with redirect_stdout(stdout), redirect_stderr(stderr):
                status = _MOD.main([
                    str(reference),
                    str(candidate),
                    str(missing),
                ])

            self.assertEqual(status, 1)
            self.assertEqual(stdout.getvalue(), "")
            self.assertNotIn(str(root), stderr.getvalue())


class SourceVariantProjectionTests(unittest.TestCase):
    """Guard projection derivation and generic-algorithm compatibility."""

    def test_projection_settings_match_rust_validation_shape(self) -> None:
        settings = {
            "schema": "shar.algorithm.settings.v1",
            "minimum_source_files": 1,
            "minimum_source_bytes": 2,
            "maximum_source_files": 3,
            "maximum_target_files": 4,
            "maximum_file_bytes": 5,
            "maximum_source_bytes": 6,
            "maximum_target_bytes": 7,
        }
        encoded = json.dumps(settings, separators=(",", ":"))
        self.assertEqual(
            _MOD._algorithm_maximum_file_bytes_from_text(encoded),
            5,
        )

        malformed = (
            encoded[:-1] + ',"maximum_file_bytes":8}',
            json.dumps({**settings, "unknown": 1}),
            json.dumps({
                key: value for key, value in settings.items() if key != "schema"
            }),
            json.dumps({**settings, "maximum_file_bytes": True}),
            json.dumps({**settings, "maximum_source_bytes": 1}),
            json.dumps({**settings, "maximum_source_files": 0}),
        )
        for document in malformed:
            with (
                self.subTest(document=document),
                self.assertRaises(_MOD.ProjectionSettingsError),
            ):
                _MOD._algorithm_maximum_file_bytes_from_text(document)

    def test_projection_settings_redirect_fails_before_read(self) -> None:
        with (
            mock.patch.object(
                _MOD,
                "_regular_file_identity",
                side_effect=_MOD.InputRedirectError,
            ),
            mock.patch.object(
                Path,
                "open",
                side_effect=AssertionError("settings payload was read"),
            ) as opened,
            self.assertRaises(_MOD.ProjectionSettingsError),
        ):
            _MOD._algorithm_maximum_file_bytes()
        opened.assert_not_called()

    def test_projection_enforces_active_file_resources(self) -> None:
        oversized_span = _MOD.OffsetProjection((
            _MOD.OffsetProjectionAlternative(span_bytes=9, mask=b"\x80\x00"),
        ))
        aggregate_masks = _MOD.OffsetProjection((
            _MOD.OffsetProjectionAlternative(span_bytes=8, mask=b"\x80"),
            _MOD.OffsetProjectionAlternative(span_bytes=8, mask=b"\x40"),
        ))

        with self.assertRaises(_MOD.ProjectionResourceError):
            _MOD._validate_projection_resources(oversized_span, 8)
        with self.assertRaises(_MOD.ProjectionResourceError):
            _MOD._validate_projection_resources(aggregate_masks, 1)

    def test_projection_rejects_oversized_candidate_before_read(self) -> None:
        path = Path("private-candidate.bin")
        identity = _MOD.FileIdentity(
            device=1,
            inode=2,
            modified_ns=3,
            ctime_ns=4,
            size=9,
        )
        with (
            mock.patch.object(
                _MOD,
                "_regular_file_identity",
                return_value=identity,
            ),
            mock.patch.object(
                Path,
                "open",
                side_effect=AssertionError("payload was read"),
            ) as opened,
            self.assertRaises(_MOD.ProjectionResourceError),
        ):
            _MOD._read_candidate_snapshot(path, maximum_file_bytes=8)
        opened.assert_not_called()

    def test_projection_mask_does_not_reserve_for_candidate_tail(self) -> None:
        candidate = b"x" + (b"a" * 8191)
        with mock.patch.object(
            _MOD,
            "bytearray",
            wraps=bytearray,
            create=True,
        ) as packed_mask:
            alternative = _MOD._ordered_projection(b"x", candidate)

        packed_mask.assert_called_once_with()
        self.assertEqual(alternative.span_bytes, 1)
        self.assertEqual(alternative.mask, b"\x80")

    def test_projection_packs_sparse_long_span_mask(self) -> None:
        candidate = (b"a" * 8192) + b"x"

        alternative = _MOD._ordered_projection(b"x", candidate)

        self.assertEqual(alternative.span_bytes, 8193)
        self.assertEqual(len(alternative.mask), 1025)
        self.assertEqual(alternative.mask[:-1], b"\x00" * 1024)
        self.assertEqual(alternative.mask[-1], 0x80)

    def test_projection_mode_emits_distinct_layouts_without_hash(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-projection-") as raw:
            root = Path(raw)
            reference = root / "common.bin"
            first = root / "first.bin"
            second = root / "second.bin"
            reference.write_bytes(b"ace")
            first.write_bytes(b"a1c2e")
            second.write_bytes(b"ZZaXcYe")

            projection = _MOD.build_offset_projection(
                b"ace",
                [first, second],
            )
            self.assertEqual(len(projection.alternatives), 2)
            self.assertEqual(projection.alternatives[0].span_bytes, 5)
            self.assertEqual(projection.alternatives[0].mask, b"\xa8")
            self.assertEqual(projection.alternatives[0].selected_bytes, 3)
            self.assertEqual(projection.alternatives[1].span_bytes, 7)
            self.assertEqual(projection.alternatives[1].mask, b"\x2a")
            self.assertEqual(projection.alternatives[1].selected_bytes, 3)

            stdout = StringIO()
            stderr = StringIO()
            with redirect_stdout(stdout), redirect_stderr(stderr):
                status = _MOD.main([
                    "--projection",
                    str(reference),
                    str(first),
                    str(second),
                ])

            self.assertEqual(status, 0)
            self.assertEqual(stderr.getvalue(), "")
            document = json.loads(stdout.getvalue())
            self.assertEqual(document["kind"], "offset-mask-set-v1")
            self.assertEqual(
                document["alternatives"],
                [
                    {"span_bytes": 5, "mask": ["a8"]},
                    {"span_bytes": 7, "mask": ["2a"]},
                ],
            )
            self.assertNotIn("sha256", document)
            self.assertNotIn(str(root), stdout.getvalue())

    def test_projection_rejects_more_than_algorithm_layout_limit(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-projection-limit-"
        ) as raw:
            root = Path(raw)
            variants = []
            for offset in range(_MOD._MAX_PROJECTION_ALTERNATIVES + 1):
                candidate = root / f"candidate-{offset:03}.bin"
                candidate.write_bytes((b"a" * offset) + b"x")
                variants.append(candidate)

            with self.assertRaisesRegex(
                _MOD.ProjectionLimitError,
                "too many distinct layout alternatives",
            ):
                _MOD.build_offset_projection(b"x", variants)

    def test_projection_mismatch_fails_without_partial_output(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-projection-") as raw:
            root = Path(raw)
            reference = root / "common.bin"
            first = root / "first.bin"
            second = root / "second.bin"
            reference.write_bytes(b"ace")
            first.write_bytes(b"a1c2e")
            second.write_bytes(b"aXcYq")
            stdout = StringIO()
            stderr = StringIO()

            with redirect_stdout(stdout), redirect_stderr(stderr):
                status = _MOD.main([
                    "--projection",
                    str(reference),
                    str(first),
                    str(second),
                ])

            self.assertEqual(status, 1)
            self.assertEqual(stdout.getvalue(), "")
            self.assertIn("ordered subsequence", stderr.getvalue())
            self.assertNotIn(str(root), stderr.getvalue())

    def test_empty_reference_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-variant-") as raw:
            root = Path(raw)
            reference = root / "common.bin"
            reference.write_bytes(b"")

            with self.assertRaisesRegex(
                _MOD.VariantEvidenceError,
                "must not be empty",
            ):
                _MOD.load_reference(reference)


if __name__ == "__main__":
    unittest.main()
