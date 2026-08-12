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
#   - Atomic no-replace directory publication for source-bound transforms.
# - Must-Not:
#   - Overwrite a destination created by another writer.
#   - Silently downgrade unsupported host publication semantics.
# - Allows:
#   - Inputs: staging and destination pathlib paths owned by the caller.
#   - Outputs: one atomic directory rename when the destination is absent.
#   - Side effects: filesystem rename only.
# - Split-When:
#   - Split when another publication primitive gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the exact same no-replace directory ABI.
# - Summary:
#   - Portable atomic no-replace directory publication.
# - Description:
#   - Implements Windows rename and Linux renameat2 no-replace semantics.
# - Usage:
#   - Used by source-bound tree materializers at their final publication step.
# - Defaults:
#   - Unsupported hosts and publication collisions fail closed.
#

"""Portable atomic no-replace directory publication."""

from __future__ import annotations

import ctypes
import errno
import os
from pathlib import Path
import sys
from typing import Final
from typing import Protocol
from typing import cast

_WINDOWS_OS_NAME: Final = "nt"
_LINUX_PLATFORM: Final = "linux"
_AT_FDCWD: Final = -100
_RENAME_NOREPLACE: Final = 1


class _RenameAt2(Protocol):
    """Typed view of the Linux libc renameat2 entry point."""

    argtypes: tuple[object, ...]
    restype: object

    def __call__(self, *arguments: int | bytes) -> int:
        """Rename one path with the fixed Linux renameat2 ABI arguments."""
        ...


def _linux_rename_noreplace(source: Path, destination: Path) -> None:
    try:
        library = ctypes.CDLL(None, use_errno=True)
        renameat2 = cast("_RenameAt2", cast("object", library.renameat2))
        renameat2.argtypes = (
            ctypes.c_int,
            ctypes.c_char_p,
            ctypes.c_int,
            ctypes.c_char_p,
            ctypes.c_uint,
        )
        renameat2.restype = ctypes.c_int
    except (AttributeError, OSError) as error:
        raise OSError(
            errno.ENOTSUP,
            "Linux renameat2(RENAME_NOREPLACE) is unavailable",
            destination,
        ) from error
    result = renameat2(
        _AT_FDCWD,
        os.fsencode(source),
        _AT_FDCWD,
        os.fsencode(destination),
        _RENAME_NOREPLACE,
    )
    if result != 0:
        error_number = ctypes.get_errno()
        raise OSError(
            error_number,
            os.strerror(error_number),
            destination,
        )


def _validate_publication_inputs(
    staging: object,
    destination: object,
    *,
    os_name: object,
    platform: object,
) -> tuple[Path, Path, str, str]:
    if not isinstance(staging, Path) or not isinstance(destination, Path):
        raise OSError(
            errno.EINVAL,
            "directory publication paths must use pathlib Path",
        )
    if type(os_name) is not str or type(platform) is not str:
        raise OSError(
            errno.EINVAL,
            "directory publication host selectors must use exact strings",
        )
    if staging == destination:
        raise OSError(
            errno.EINVAL,
            "directory publication staging and destination must differ",
            destination,
        )
    return staging, destination, os_name, platform


def publish_directory_no_replace(
    staging: Path,
    destination: Path,
    *,
    os_name: str = os.name,
    platform: str = sys.platform,
) -> None:
    """Atomically publish one directory without replacing an existing target.

    Raises:
        OSError: The target exists or atomic no-replace publication is
            unavailable.

    """
    staging, destination, os_name, platform = _validate_publication_inputs(
        staging,
        destination,
        os_name=os_name,
        platform=platform,
    )
    if os_name == _WINDOWS_OS_NAME:
        _ = staging.rename(destination)
        return
    if platform == _LINUX_PLATFORM:
        _linux_rename_noreplace(staging, destination)
        return
    raise OSError(
        errno.ENOTSUP,
        "atomic no-replace directory publication is unsupported",
        destination,
    )
