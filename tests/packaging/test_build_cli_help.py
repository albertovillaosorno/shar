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
#   - Cross-host help and non-GUI behavior for repository build adapters.
# - Must-Not:
#   - Open GUI windows, package targets, or create desktop shortcuts.
# - Allows:
#   - Direct CLI dispatch under test-managed import and host shims.
# - Split-When:
#   - One build adapter gains an independent command-line lifecycle.
# - Merge-When:
#   - Build adapter CLI behavior becomes one inseparable packaging contract.
# - Summary:
#   - Guards build help and non-GUI modes against optional host dependencies.
# - Description:
#   - Proves help/list parsing remains available before GUI or platform gates.
# - Usage:
#   - Run through the canonical Jig pytest gate or repository-local pytest.
# - Defaults:
#   - Loads tracked adapters directly and does not mutate repository state.
#

"""Regression tests for portable build-adapter command-line discovery."""

from __future__ import annotations

import builtins
from contextlib import redirect_stderr
from contextlib import redirect_stdout
import importlib.util
from io import StringIO
import os
from pathlib import Path
import sys
import tempfile
from types import ModuleType
import unittest
from unittest import mock

_ROOT = Path(__file__).resolve().parents[2]
_BUILD = _ROOT / "tools" / "build" / "adapter-inbound"
_ARCH = _BUILD / "arch.py"
_SHORTCUT = _BUILD / "windows_shortcut.py"


def _load(path: Path, name: str) -> ModuleType:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load build adapter: {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def _load_arch_without_tkinter() -> ModuleType:
    real_import = builtins.__import__

    def guarded_import(
        name: str,
        globals_: object = None,
        locals_: object = None,
        from_list: object = (),
        level: int = 0,
    ) -> object:
        if name == "tkinter" or name.startswith("tkinter."):
            raise ImportError("tkinter intentionally blocked by test")
        return real_import(name, globals_, locals_, from_list, level)

    with mock.patch.object(builtins, "__import__", side_effect=guarded_import):
        return _load(_ARCH, "shar_build_arch_no_tk_test")


class BuildCliHelpTests(unittest.TestCase):
    def test_arch_help_and_list_do_not_require_tkinter(self) -> None:
        """Non-GUI architecture modes must work when Tk is unavailable."""
        module = _load_arch_without_tkinter()
        with (
            mock.patch.object(sys, "argv", ["arch", "--help"]),
            redirect_stdout(StringIO()),
            self.assertRaises(SystemExit) as raised,
        ):
            module.main()
        self.assertEqual(raised.exception.code, 0)

        output = StringIO()
        with (
            mock.patch.object(sys, "argv", ["arch", "--list"]),
            redirect_stdout(output),
        ):
            self.assertEqual(module.main(), 0)
        self.assertIn("linux-x64", output.getvalue())

    @unittest.skipIf(os.name == "nt", "non-Windows platform gate regression")
    def test_windows_shortcut_help_precedes_platform_gate(self) -> None:
        """Help remains discoverable where shortcut creation is unsupported."""
        module = _load(_SHORTCUT, "shar_windows_shortcut_help_test")
        output = StringIO()
        errors = StringIO()
        with (
            mock.patch.object(module.os, "name", "posix"),
            mock.patch.object(sys, "argv", ["shortcut", "--help"]),
            redirect_stdout(output),
            redirect_stderr(errors),
            self.assertRaises(SystemExit) as raised,
        ):
            module.main()
        self.assertEqual(raised.exception.code, 0)
        self.assertIn("--target", output.getvalue())
        self.assertEqual(errors.getvalue(), "")


class WindowsShortcutTargetTests(unittest.TestCase):
    """Keep shortcut discovery bound to the packaged SHAR executable."""

    @classmethod
    def setUpClass(cls) -> None:
        cls.module = _load(_SHORTCUT, "shar_windows_shortcut_target_test")

    def test_discovery_rejects_linked_dist_root(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-shortcut-root-") as raw:
            root = Path(raw)
            dist = root / "dist"
            dist.mkdir()
            shipping = dist / "shar-Win64-Shipping.exe"
            shipping.write_bytes(b"game")
            original = self.module._is_directory_link

            def report_dist_as_link(path: Path) -> bool:
                return path == dist or original(path)

            with (
                mock.patch.object(
                    self.module,
                    "_is_directory_link",
                    side_effect=report_dist_as_link,
                ),
                self.assertRaisesRegex(
                    SystemExit,
                    "dist/ must be a real directory",
                ),
            ):
                self.module._discover_target(root)

    def test_discovery_ignores_original_helpers_and_lookalikes(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-shortcut-target-") as raw:
            root = Path(raw)
            shipping = root / "dist/windows-x64/shar-Win64-Shipping.exe"
            shipping.parent.mkdir(parents=True)
            shipping.write_bytes(b"game")
            preferred = shipping.parent / "SHAR.exe"
            preferred.write_bytes(b"launcher")
            for name in (
                "CrashReportClient.exe",
                "Simpsons.exe",
                "shareware.exe",
            ):
                (shipping.parent / name).write_bytes(b"other")

            self.assertEqual(
                self.module._discover_target(root),
                preferred.resolve(),
            )

    def test_discovery_accepts_shipping_executable_without_launcher(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-shortcut-target-") as raw:
            root = Path(raw)
            target = root / "dist/windows-x64/shar-Win64-Shipping.exe"
            target.parent.mkdir(parents=True)
            target.write_bytes(b"game")

            self.assertEqual(
                self.module._discover_target(root),
                target.resolve(),
            )

    def test_discovery_rejects_non_shar_only_executable(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-shortcut-target-") as raw:
            root = Path(raw)
            dist = root / "dist/windows-x64"
            dist.mkdir(parents=True)
            (dist / "Simpsons.exe").write_bytes(b"original")

            with self.assertRaisesRegex(SystemExit, "no packaged SHAR"):
                self.module._discover_target(root)

    def test_explicit_target_rejects_empty_or_lookalike_executable(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-shortcut-target-") as raw:
            root = Path(raw)
            for name, payload in (
                ("shareware.exe", b"lookalike"),
                ("shar-helper.exe", b"prefixed helper"),
                ("shar-Win64-Debug.exe", b"wrong configuration"),
                ("shar.exe", b""),
            ):
                with self.subTest(name=name):
                    target = root / name
                    target.write_bytes(payload)
                    with self.assertRaisesRegex(
                        SystemExit,
                        "non-empty packaged SHAR",
                    ):
                        self.module._target(target.name, root)

    def test_explicit_target_accepts_packaged_shar_names(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-shortcut-target-") as raw:
            root = Path(raw)
            for name in ("SHAR.exe", "shar-Win64-Shipping.exe"):
                with self.subTest(name=name):
                    target = root / name
                    target.write_bytes(b"game")
                    self.assertEqual(
                        self.module._target(target.name, root),
                        target.resolve(),
                    )


if __name__ == "__main__":
    unittest.main()
