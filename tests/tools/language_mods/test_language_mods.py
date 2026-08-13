# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT

"""Tests for deterministic official-language source bundles."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest

_ROOT = Path(__file__).resolve().parents[3]
_TOOL = _ROOT / "tools" / "language-mods" / "main.py"
_spec = importlib.util.spec_from_file_location("shar_language_mod_test", _TOOL)
if _spec is None or _spec.loader is None:
    raise RuntimeError("cannot load language-mod exporter")
_MOD = importlib.util.module_from_spec(_spec)
sys.modules[_spec.name] = _MOD
_spec.loader.exec_module(_MOD)


def _game(root: Path) -> Path:
    game = root / "game"
    text = game / "art/frontend/scrooby2/resource/txtbible/srr2.txt"
    text.parent.mkdir(parents=True)
    rows = [
        "Languages\tEFGIS\t\t\t\t\t\t\t",
        "\t\t\t\t\t\t\t\t",
        "Screen\tPHRASE TABLE\tSPACE\tE\tF\tG\tI\tS\tNOTES",
        "\tTERM\tCRITICAL\tENGLISH\tFRENCH\tGERMAN\tITALIAN\tSPANISH\t",
        "\t\t\t\t\t\t\t\t",
        "MENU\tHELLO\t\tHello\tBonjour\tHallo\tCiao\tHola\tnote",
    ]
    text.write_bytes(("\r\n".join(rows) + "\r\n").encode("cp1252"))
    for code in "FGIS":
        text.with_name(f"srr2.{code}").write_bytes(code.encode("ascii"))
    (game / "dialogf.rcf").write_bytes(b"french-dialogue")
    (game / "dialogg.rcf").write_bytes(b"german-dialogue")
    (game / "dialogs.rcf").write_bytes(b"spanish-dialogue")
    (game / "Lisez-moi.rtf").write_bytes(b"french-readme")
    (game / "Liesmich.rtf").write_bytes(b"german-readme")
    (game / "Léeme.rtf").write_bytes(b"spanish-readme")
    return game


class LanguageModExporterTests(unittest.TestCase):
    def test_french_bundle_preserves_sources_and_exposes_text(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-language-") as raw:
            root = Path(raw)
            game = _game(root)
            output = root / "french"
            manifest = _MOD.export_language(game, output, _MOD.LANGUAGES["french"])
            self.assertEqual(manifest["missing_optional_sources"], [])
            self.assertEqual((output / "source/dialogf.rcf").read_bytes(), b"french-dialogue")
            record = json.loads((output / "text.jsonl").read_text(encoding="utf-8"))
            self.assertEqual(record["english"], "Hello")
            self.assertEqual(record["value"], "Bonjour")

    def test_italian_without_translated_source_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-language-") as raw:
            root = Path(raw)
            game = _game(root)
            text = game / "art/frontend/scrooby2/resource/txtbible/srr2.txt"
            body = text.read_bytes().decode("cp1252").replace("Ciao", "???")
            text.write_bytes(body.encode("cp1252"))
            with self.assertRaisesRegex(_MOD.ExportError, "no translated text"):
                _MOD.export_language(
                    game,
                    root / "italian",
                    _MOD.LANGUAGES["italian"],
                )


    def test_two_exports_are_byte_deterministic(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-language-") as raw:
            root = Path(raw)
            game = _game(root)
            first = root / "first"
            second = root / "second"
            _MOD.export_language(game, first, _MOD.LANGUAGES["spanish"])
            _MOD.export_language(game, second, _MOD.LANGUAGES["spanish"])
            self.assertEqual(
                (first / "manifest.json").read_bytes(),
                (second / "manifest.json").read_bytes(),
            )
            self.assertEqual(
                (first / "text.jsonl").read_bytes(),
                (second / "text.jsonl").read_bytes(),
            )

    def test_output_inside_source_game_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-language-") as raw:
            root = Path(raw)
            game = _game(root)
            with self.assertRaisesRegex(_MOD.ExportError, "outside"):
                _MOD.export_language(game, game / "generated", _MOD.LANGUAGES["german"])


if __name__ == "__main__":
    unittest.main()
