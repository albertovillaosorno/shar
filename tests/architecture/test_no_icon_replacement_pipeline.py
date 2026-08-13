# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT

"""Architecture guard for the canonical source icon policy."""

from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]


def test_repository_has_no_icon_replacement_pipeline() -> None:
    """Keep source identity separate from artistic icon generation."""
    assert not (_ROOT / "src" / "unreal" / "icon").exists(), (
        "src/unreal/icon is forbidden: Simpsons.ico is canonical source "
        "evidence, not input to a replacement-art generator"
    )
