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
#   - Regression evidence for user-facing lawful source selection.
# - Must-Not:
#   - Depend on a real proprietary installation or expose private fixture paths.
# - Allows:
#   - Build synthetic temporary source trees and load the selection adapter.
# - Split-When:
#   - GUI event behavior requires an independent test surface.
# - Merge-When:
#   - Another test module owns the identical source-selection contract.
# - Summary:
#   - User source-selection regression tests.
# - Description:
#   - Proves directory, text, and executable selection is read-only.
# - Usage:
#   - Run with the repository Python test environment.
# - Defaults:
#   - All source fixtures are synthetic and disposable.
#

"""Regression tests for user-facing lawful source selection."""

from __future__ import annotations

import importlib.util
import os
from pathlib import Path
import tempfile
from types import ModuleType

import pytest

_ROOT = Path(__file__).resolve().parents[3]
_MODULE = (
    _ROOT
    / "src/user/source-selection/adapter-inbound/source_selection.py"
)


def _load() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "shar_user_source_selection", _MODULE
    )
    assert specification is not None
    assert specification.loader is not None
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


def _source(root: Path) -> tuple[Path, Path]:
    source = root / "installed-game"
    source.mkdir()
    executable = source / "Simpsons.exe"
    executable.write_bytes(b"synthetic-executable")
    return source, executable


def test_directory_text_and_drop_resolve_same_root() -> None:
    module = _load()
    with tempfile.TemporaryDirectory(prefix="shar-user-source-") as value:
        source, executable = _source(Path(value))
        before = executable.read_bytes()

        directory = module.resolve_source_selection(source)
        typed = module.resolve_source_selection(f'  "{source}"  ')
        dropped = module.resolve_source_selection(f"{{{executable}}}")

        assert directory == source.resolve()
        assert typed == source.resolve()
        assert dropped == source.resolve()
        assert executable.read_bytes() == before


def test_non_game_file_is_rejected_without_private_path() -> None:
    module = _load()
    with tempfile.TemporaryDirectory(prefix="shar-user-private-") as value:
        other = Path(value) / "README.rtf"
        other.write_bytes(b"synthetic")

        try:
            module.resolve_source_selection(other)
        except module.SourceSelectionError as error:
            message = str(error)
        else:
            raise AssertionError("non-game source file was accepted")

        assert str(other) not in message
        assert "Simpsons.exe" in message


def test_missing_selection_is_rejected_without_private_path() -> None:
    module = _load()
    with tempfile.TemporaryDirectory(prefix="shar-user-missing-") as value:
        missing = Path(value) / "private-installation"

        try:
            module.resolve_source_selection(missing)
        except module.SourceSelectionError as error:
            message = str(error)
        else:
            raise AssertionError("missing source path was accepted")

        assert str(missing) not in message
        assert message == "selected source path does not exist"


def test_directory_requires_one_direct_canonical_executable() -> None:
    module = _load()
    with tempfile.TemporaryDirectory(prefix="shar-user-flat-") as value:
        source = Path(value) / "installed-game"
        nested = source / "copy"
        nested.mkdir(parents=True)
        (nested / "Simpsons.exe").write_bytes(b"synthetic")

        try:
            module.resolve_source_selection(source)
        except module.SourceSelectionError as error:
            message = str(error)
        else:
            raise AssertionError("nested-only source executable was accepted")

        assert str(source) not in message
        assert "direct Simpsons.exe" in message


def test_redirected_wrong_file_name_is_rejected() -> None:
    if os.name == "nt":
        pytest.skip("symlink setup is Unix-focused")
    module = _load()
    with tempfile.TemporaryDirectory(prefix="shar-user-redirect-") as value:
        source, executable = _source(Path(value))
        redirect = source / "README.rtf"
        redirect.symlink_to(executable.name)

        with pytest.raises(module.SourceSelectionError, match=r"Simpsons\.exe"):
            module.resolve_source_selection(redirect)


def test_nested_executable_redirect_is_rejected() -> None:
    if os.name == "nt":
        pytest.skip("symlink setup is Unix-focused")
    module = _load()
    with tempfile.TemporaryDirectory(prefix="shar-user-nested-link-") as value:
        source, executable = _source(Path(value))
        nested = source / "copy"
        nested.mkdir()
        (nested / "Simpsons.exe").symlink_to(executable)

        pattern = r"nested Simpsons\.exe"
        with pytest.raises(module.SourceSelectionError, match=pattern):
            module.resolve_source_selection(source)


def test_redirected_source_directory_is_rejected() -> None:
    if os.name == "nt":
        pytest.skip("symlink setup is Unix-focused")
    module = _load()
    with tempfile.TemporaryDirectory(
        prefix="shar-user-directory-link-"
    ) as value:
        root = Path(value)
        source, _ = _source(root)
        redirect = root / "redirected-game"
        redirect.symlink_to(source, target_is_directory=True)

        pattern = r"real source directory"
        with pytest.raises(module.SourceSelectionError, match=pattern):
            module.resolve_source_selection(redirect)


def test_direct_executable_redirect_is_rejected() -> None:
    if os.name == "nt":
        pytest.skip("symlink setup is Unix-focused")
    module = _load()
    with tempfile.TemporaryDirectory(prefix="shar-user-direct-link-") as value:
        root = Path(value)
        source = root / "installed-game"
        outside = root / "outside"
        source.mkdir()
        outside.mkdir()
        target = outside / "Simpsons.exe"
        target.write_bytes(b"synthetic-executable")
        redirect = source / "Simpsons.exe"
        redirect.symlink_to(target)

        pattern = r"real Simpsons\.exe"
        with pytest.raises(module.SourceSelectionError, match=pattern):
            module.resolve_source_selection(source)
        with pytest.raises(module.SourceSelectionError, match=pattern):
            module.resolve_source_selection(redirect)
