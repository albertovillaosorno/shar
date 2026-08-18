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


if __name__ == "__main__":
    unittest.main()
