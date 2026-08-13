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
#   - Standalone Python LMLM parser and conversion publication tests.
# - Must-Not:
#   - Read user mods, invoke Rust, or mutate repository game content.
# - Allows:
#   - Synthetic archives and isolated temporary conversion roots.
# - Split-When:
#   - Licensed compatibility fixtures gain an independent test lifecycle.
# - Merge-When:
#   - LMLM compatibility is removed.
# - Summary:
#   - Tests the downloadable LMLM tool using stdlib only.
# - Description:
#   - Proves strict parsing, path policy, atomic output, and evidence generation.
# - Usage:
#   - Run with Python's standard-library unittest runner.
# - Defaults:
#   - All filesystem state is temporary and synthetic.
#

"""Tests for the downloadable LMLM compatibility tool."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest
import zipfile

_ROOT = Path(__file__).resolve().parents[3]
_TOOL = _ROOT / "tools" / "lmlm"
sys.path.insert(0, str(_TOOL))


def _load(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


_ARCHIVE = _load("shar_lmlm_archive_test", _TOOL / "archive.py")
_MAIN = _load("shar_lmlm_main_test", _TOOL / "main.py")
_BLOCK = 0x200
_FIRST_ENTRY = 0x600
_ROOT_BLOCK = 0x400


def _write_name(data: bytearray, position: int, name: str) -> None:
    data[position : position + 2] = (2).to_bytes(2, "little")
    encoded = name.encode("utf-16-le") + b"\x00\x00"
    data[position + 2 : position + 2 + len(encoded)] = encoded


def _write_file(
    data: bytearray,
    position: int,
    name: str,
    offset: int,
    payload: bytes,
) -> None:
    _write_name(data, position, name)
    metadata = position + _BLOCK
    data[metadata + 0x0C : metadata + 0x14] = len(payload).to_bytes(8, "little")
    data[metadata + 0x14 : metadata + 0x1C] = offset.to_bytes(8, "little")
    data[offset : offset + len(payload)] = payload


def _archive(name: str = "Meta.ini", payload: bytes = b"fixture") -> bytes:
    data = bytearray(0x2000)
    data[0:4] = b"LSPA"
    data[4:8] = (5).to_bytes(4, "little")
    data[0x0C:0x10] = (0x0200_0000).to_bytes(4, "little")
    data[_ROOT_BLOCK + 2 : _ROOT_BLOCK + 4] = (1).to_bytes(2, "little")
    _write_file(data, _FIRST_ENTRY, name, 0x1000, payload)
    return bytes(data)


def _empty_archive() -> bytes:
    data = bytearray(0x600)
    data[0:4] = b"LSPA"
    data[4:8] = (5).to_bytes(4, "little")
    data[0x0C:0x10] = (0x0200_0000).to_bytes(4, "little")
    return bytes(data)


def _two_file_archive(
    first_name: str,
    second_name: str,
    first_offset: int = 0x1800,
    second_offset: int = 0x1A00,
    first_payload: bytes = b"a",
    second_payload: bytes = b"b",
) -> bytes:
    data = bytearray(0x2000)
    data[0:4] = b"LSPA"
    data[4:8] = (5).to_bytes(4, "little")
    data[0x0C:0x10] = (0x0200_0000).to_bytes(4, "little")
    data[_ROOT_BLOCK + 2 : _ROOT_BLOCK + 4] = (2).to_bytes(2, "little")
    _write_file(data, _FIRST_ENTRY, first_name, first_offset, first_payload)
    _write_file(
        data,
        _FIRST_ENTRY + _BLOCK * 3,
        second_name,
        second_offset,
        second_payload,
    )
    return bytes(data)


class LmlmToolTests(unittest.TestCase):
    """Exercise parser and publication without external dependencies."""

    def test_valid_archive_matches_expected_entry(self) -> None:
        data = _archive()
        entries = _ARCHIVE.parse_archive(data)
        self.assertEqual(len(entries), 1)
        self.assertEqual(entries[0].path, "Meta.ini")
        self.assertEqual(_ARCHIVE.entry_bytes(data, entries[0]), b"fixture")

    def test_unsafe_and_reserved_paths_fail_closed(self) -> None:
        for name in ["..", "CON", "bad/name", "trailing."]:
            with self.subTest(name=name):
                with self.assertRaises(_ARCHIVE.LmlmError):
                    _ARCHIVE.parse_archive(_archive(name=name))

    def test_nonzero_unclaimed_bytes_are_rejected(self) -> None:
        data = bytearray(_archive())
        data[0x0E00] = 1
        with self.assertRaisesRegex(_ARCHIVE.LmlmError, "unclaimed"):
            _ARCHIVE.parse_archive(bytes(data))

    def test_unaligned_payload_is_rejected(self) -> None:
        data = bytearray(_archive())
        metadata = _FIRST_ENTRY + _BLOCK
        data[0x1000 : 0x1007] = b"\x00" * 7
        data[metadata + 0x14 : metadata + 0x1C] = (0x1001).to_bytes(
            8,
            "little",
        )
        data[0x1001 : 0x1008] = b"fixture"
        with self.assertRaisesRegex(_ARCHIVE.LmlmError, "not block aligned"):
            _ARCHIVE.parse_archive(bytes(data))

    def test_empty_container_is_valid(self) -> None:
        self.assertEqual(_ARCHIVE.parse_archive(_empty_archive()), ())

    def test_portable_case_collision_is_rejected(self) -> None:
        data = _two_file_archive("File.txt", "file.TXT")
        with self.assertRaisesRegex(_ARCHIVE.LmlmError, "collide"):
            _ARCHIVE.parse_archive(data)

    def test_overlapping_payloads_are_rejected(self) -> None:
        data = _two_file_archive(
            "first.bin",
            "second.bin",
            first_payload=b"a" * 0x300,
            second_payload=b"b" * 0x100,
        )
        with self.assertRaisesRegex(_ARCHIVE.LmlmError, "overlap"):
            _ARCHIVE.parse_archive(data)

    def test_zip_output_cannot_live_inside_conversion_directory(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-lmlm-contained-zip-") as raw:
            root = Path(raw)
            source = root / "example.lmlm"
            output = root / "converted"
            source.write_bytes(_archive())
            with self.assertRaisesRegex(_MAIN.LmlmError, "outside"):
                _MAIN.convert(source, output, output / "inside.zip")
            self.assertFalse(output.exists())

    def test_conversion_is_atomic_and_emits_hash_evidence(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-lmlm-tool-") as raw:
            root = Path(raw)
            source = root / "example.lmlm"
            output = root / "converted"
            source.write_bytes(_archive(payload=b"payload"))

            report = _MAIN.convert(source, output, None)

            payload = output / "content" / "Meta.ini"
            self.assertEqual(payload.read_bytes(), b"payload")
            persisted = json.loads(
                (output / "conversion-report.json").read_text(encoding="utf-8")
            )
            self.assertEqual(persisted, report)
            self.assertTrue(persisted["decompilable_mods_only"])
            self.assertEqual(
                persisted["status"],
                "extracted-needs-shar-package-adaptation",
            )
            with self.assertRaisesRegex(_MAIN.LmlmError, "already exists"):
                _MAIN.convert(source, output, None)
            self.assertEqual(payload.read_bytes(), b"payload")

    def test_optional_zip_contains_only_open_workspace_files(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-lmlm-zip-") as raw:
            root = Path(raw)
            source = root / "example.lmlm"
            output = root / "converted"
            zipped = root / "converted.zip"
            source.write_bytes(_archive())

            _MAIN.convert(source, output, zipped)

            with zipfile.ZipFile(zipped) as archive:
                self.assertEqual(
                    sorted(archive.namelist()),
                    ["content/Meta.ini", "conversion-report.json"],
                )


if __name__ == "__main__":
    unittest.main()
