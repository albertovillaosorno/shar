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
#   - Repository validation regression tests.
# - Must-Not:
#   - Publish private game inputs or mutate external repositories.
# - Allows:
#   - Repository-local policy and bootstrap inspection.
# - Split-When:
#   - One validation policy gains an independent lifecycle.
# - Merge-When:
#   - The guarded policies become one inseparable repository contract.
# - Summary:
#   - Guards repository validation policy.
# - Description:
#   - Exercises tracked configuration and repository-local validation behavior.
# - Usage:
#   - Run through the canonical Jig pytest gate or repository-local pytest.
# - Defaults:
#   - Reads the current repository and writes only test-managed temporary state.
#

"""Contract tests for host-portable Jig native-tool launchers."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import tempfile
import tomllib
import unittest

_ROOT = Path(__file__).resolve().parents[2]
_PATH = (
    _ROOT
    / "tools"
    / "validation"
    / "adapter-inbound"
    / "native_dependencies.py"
)
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

    def test_posix_launcher_forwards_arguments_and_exact_environment(
        self,
    ) -> None:
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

    def test_jig_tool_paths_use_portable_validation_launchers(self) -> None:
        path = _ROOT / ".jig" / "settings" / "tools.toml"
        with path.open("rb") as stream:
            config = tomllib.load(stream)
        tools = config["entry"]
        expected = {
            "cspell": ".dependencies/validation/bin/cspell.cmd",
            "git": ".dependencies/validation/bin/git.cmd",
            "markdownlint": ".dependencies/validation/bin/markdownlint.cmd",
            "pytest": ".dependencies/validation/bin/pytest.cmd",
            "ruff": ".dependencies/validation/bin/ruff.cmd",
            "rust_nightly_cargo": (
                ".dependencies/validation/rust/"
                "nightly-2026-07-14/bin/cargo.cmd"
            ),
            "rust_nightly_cargo_clippy": (
                ".dependencies/validation/rust/"
                "nightly-2026-07-14/bin/clippy.cmd"
            ),
            "rust_nightly_cargo_fmt": (
                ".dependencies/validation/rust/"
                "nightly-2026-07-14/bin/fmt.cmd"
            ),
            "rust_stable_cargo": (
                ".dependencies/validation/rust/stable-1.97.1/bin/cargo.cmd"
            ),
        }
        observed = {
            name: values["path"]
            for name, values in tools.items()
            if name in expected
        }
        self.assertEqual(observed, expected)

    def test_jig_git_version_matches_portable_bootstrap(self) -> None:
        path = _ROOT / ".jig" / "settings" / "versions.toml"
        with path.open("rb") as stream:
            config = tomllib.load(stream)
        expected = _MODULE._GIT_VERSION
        authority = config["authority"]["git"]
        self.assertEqual(authority["current"], expected)
        self.assertEqual(authority["stable"], expected)

    def test_jig_native_cargo_home_is_a_real_repo_local_directory(self) -> None:
        prepare_home = getattr(_MODULE, "_prepare_jig_cargo_home", None)
        self.assertIsNotNone(prepare_home)
        if prepare_home is None:
            return
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            cargo_home = prepare_home(root)
            self.assertEqual(cargo_home, root / ".dependencies/cargo-home")
            self.assertTrue(cargo_home.is_dir())
            self.assertFalse(cargo_home.is_symlink())

    def test_declared_launcher_names_are_platform_neutral(self) -> None:
        host = _MODULE.HostTools(
            Path("/host/git"),
            Path("/host/node"),
            Path("/host/npm"),
        )
        launchers = _MODULE._launchers(
            _ROOT,
            host,
            cargo=Path("/repo/cargo"),
            rust_environment={
                "CARGO_HOME": "/repo/cargo-home",
                "RUSTUP_HOME": "/repo/rustup",
            },
            cspell=Path("/repo/cspell.mjs"),
            markdownlint=Path("/repo/markdownlint.mjs"),
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
