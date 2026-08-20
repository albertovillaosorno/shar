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
