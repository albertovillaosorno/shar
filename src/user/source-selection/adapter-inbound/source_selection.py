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
#   - User-facing normalization of one lawful source directory or Simpsons.exe.
# - Must-Not:
#   - Modify source files, validate proprietary payloads, or expose paths.
# - Allows:
#   - Resolve directory, pasted path, or dropped executable input to one root.
# - Split-When:
#   - GUI event binding or validation gains an independent lifecycle.
# - Merge-When:
#   - Another user adapter owns the identical source-selection behavior.
# - Summary:
#   - Read-only lawful source-root selection adapter.
# - Description:
#   - Normalizes user selections and requires one canonical direct executable.
# - Usage:
#   - Called before fast and deep source validation in the user release flow.
# - Defaults:
#   - Invalid selections fail with path-free diagnostics.
#

"""Resolve one user-selected lawful game source without modifying it."""

from __future__ import annotations

import os
from pathlib import Path

_EXECUTABLE_NAME = "Simpsons.exe"
_WRAPPERS = {'"': '"', "'": "'", "{": "}"}


class SourceSelectionError(ValueError):
    """Source selection cannot resolve to one flat game installation root."""


def _is_directory_redirect(path: Path) -> bool:
    """Return whether one directory identity redirects."""
    return path.is_symlink() or os.path.isjunction(path)


def _real_directory_root(path: Path) -> Path:
    """Resolve one real source directory without accepting redirects.

    Raises:
        SourceSelectionError: If the directory is a redirect.

    """
    if _is_directory_redirect(path):
        message = (
            "selected source directory must be a real source directory"
        )
        raise SourceSelectionError(message)
    return path.resolve()


def _selection_path(selection: str | Path) -> Path:
    """Normalize one file-dialog, typed, pasted, or drop-event path value.

    Raises:
        SourceSelectionError: If the path is empty or cannot be resolved.

    """
    text = str(selection).strip()
    if len(text) >= 2 and _WRAPPERS.get(text[0]) == text[-1]:
        text = text[1:-1].strip()
    if not text:
        message = "selected source path does not exist"
        raise SourceSelectionError(message)
    try:
        candidate = Path(text).expanduser()
        if not candidate.is_absolute():
            candidate = Path.cwd() / candidate
    except (OSError, RuntimeError) as error:
        message = "selected source path cannot be resolved"
        raise SourceSelectionError(message) from error
    else:
        return candidate


def _candidate_root(selection: str | Path) -> Path:
    """Resolve one existing directory or canonical executable to its root.

    Raises:
        SourceSelectionError: If the selection is missing or not game input.

    """
    candidate = _selection_path(selection)
    try:
        if candidate.is_file():
            if candidate.name.casefold() != _EXECUTABLE_NAME.casefold():
                message = "selected source file must be Simpsons.exe"
                raise SourceSelectionError(message)
            if candidate.is_symlink():
                message = "selected source file must be a real Simpsons.exe"
                raise SourceSelectionError(message)
            return candidate.resolve().parent
        if candidate.is_dir():
            return _real_directory_root(candidate)
    except OSError as error:
        message = "selected source path cannot be inspected"
        raise SourceSelectionError(message) from error
    message = "selected source path does not exist"
    raise SourceSelectionError(message)


def resolve_source_selection(selection: str | Path) -> Path:
    """Return one flat source root selected by directory, text, or executable.

    The function only inspects filesystem metadata. It never creates, writes,
    renames, removes, or patches anything beneath the selected source.

    Raises:
        SourceSelectionError: If the selection is missing, not a game source,
            or contains another nested canonical executable.

    """
    root = _candidate_root(selection)
    direct = root / _EXECUTABLE_NAME
    try:
        if direct.is_symlink():
            message = "selected source must contain a real Simpsons.exe"
            raise SourceSelectionError(message)
        if not direct.is_file():
            message = "selected source does not contain a direct Simpsons.exe"
            raise SourceSelectionError(message)
        nested = tuple(
            path
            for path in root.rglob(_EXECUTABLE_NAME)
            if path != direct
        )
    except OSError as error:
        message = "selected source cannot be inspected safely"
        raise SourceSelectionError(message) from error
    if nested:
        message = "selected source contains another nested Simpsons.exe"
        raise SourceSelectionError(message)
    return root
