# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT

"""Contract tests for host-portable Jig native-tool launchers."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import unittest

_ROOT = Path(__file__).resolve().parents[2]
_PATH = _ROOT / "tools" / "validation" / "native_dependencies.py"
_SPEC = importlib.util.spec_from_file_location(
    "shar_native_validation_dependencies_test",
    _PATH,
)
if _SPEC is None or _SPEC.loader is None:
    raise RuntimeError("cannot load native validation bootstrap")
_MODULE = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(_MODULE)


class NativeValidationBootstrapTests(unittest.TestCase):
    """Keep generated launchers path-stable and host-neutral."""

    def test_posix_launcher_forwards_arguments_and_exact_environment(self) -> None:
        launcher = _MODULE.Launcher(
            ("/repo/cargo", "+nightly-2026-07-14", "clippy"),
            (("CARGO_HOME", "/repo/cargo-home"),),
        )
        text = _MODULE._posix_launcher(launcher)
        self.assertTrue(text.startswith("#!/bin/sh\nset -eu\n"))
        self.assertIn("export CARGO_HOME=/repo/cargo-home", text)
        self.assertIn('"$@"', text)
        self.assertNotIn("windows-gnu", text)

    def test_windows_launcher_uses_same_cmd_filename_contract(self) -> None:
        launcher = _MODULE.Launcher(
            (r"C:\repo\cargo.exe", "+1.97.1"),
            (("RUSTUP_HOME", r"C:\repo\rustup"),),
        )
        text = _MODULE._windows_launcher(launcher)
        self.assertTrue(text.startswith("@echo off\r\nsetlocal\r\n"))
        self.assertIn('set "RUSTUP_HOME=C:\\repo\\rustup"', text)
        self.assertIn("%*", text)

    def test_declared_launcher_names_are_platform_neutral(self) -> None:
        host = _MODULE.HostTools(
            Path("/host/git"),
            Path("/host/node"),
            Path("/host/npm"),
        )
        launchers = _MODULE._launchers(
            _ROOT,
            host,
            Path("/repo/cargo"),
            {"CARGO_HOME": "/repo/cargo-home", "RUSTUP_HOME": "/repo/rustup"},
            Path("/repo/cspell.mjs"),
            Path("/repo/markdownlint.mjs"),
        )
        self.assertEqual(
            sorted(launchers),
            [
                "cspell.cmd",
                "git.cmd",
                "markdownlint.cmd",
                "node.cmd",
                "pytest.cmd",
                "ruff.cmd",
                "rust-nightly-cargo.cmd",
                "rust-nightly-clippy.cmd",
                "rust-nightly-fmt.cmd",
                "rust-stable-cargo.cmd",
            ],
        )
        for name in launchers:
            self.assertNotIn("windows", name)
            self.assertNotIn("linux", name)


if __name__ == "__main__":
    unittest.main()
