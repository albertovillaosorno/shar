"""Repository guards for source-bound icon migration ownership."""

from __future__ import annotations

import json
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]
_PLAN = _ROOT / "src" / "migration" / "icon" / "icon_algorithm.txt"
_CANONICAL_SOURCE = {
    "input": 0,
    "path": "",
    "bytes": 5806,
    "sha256": "3b0b72f7b2eff173a81669ee465625e67e70dda8c18282bdeeda6d2a203df328",
}
_TARGET_IDENTITIES = {
    "android.svg": "0f533d54b75b87478843a64854be9aba4e639c596d2ba65deb5059c0fa0b00d3",
    "ios.svg": "05acbec55cc6dc23b2eedbb576b382e6f94a7b1f82404158917ed8ab79dba9f0",
    "macos-linux.svg": "6e123e2f0b70404aaedd4346beb0c272736de4f61e9ab26ea52802c84483ab3a",
    "master/hit-and-run.svg": (
        "4e501a914d7490bf87db48e2fa5b9c60e1a5711c726e7c14ece666f923210e79"
    ),
    "master/sign.svg": "4dc6b32a838680aaf3bb026fb894ed51e2a68a8105ce27f43205844f4a6f112e",
    "master/the-simpsons.svg": (
        "002e160dc4f087d7d50d65a3ab245f9984428b649d7dd2509501af5c61ceaf02"
    ),
    "windows-linux.svg": (
        "d64674cfb04c6dbb09296e846f9e5ab62d0cd70dee6373ac83c419eb441ca6d9"
    ),
}


def _plan() -> dict[str, object]:
    return json.loads(_PLAN.read_text(encoding="utf-8"))


def test_icon_algorithm_is_bound_only_to_canonical_source_identity() -> None:
    plan = _plan()

    assert plan["schema"] == "shar.algorithm.v1"
    assert plan["source"] == [_CANONICAL_SOURCE]
    assert plan["target_kind"] == "directory"


def test_icon_algorithm_records_exact_recovered_target_identities() -> None:
    plan = _plan()
    targets = plan["target"]

    assert isinstance(targets, list)
    observed = {
        target["path"]: target["sha256"]
        for target in targets
        if isinstance(target, dict)
    }
    assert observed == _TARGET_IDENTITIES


def test_icon_algorithm_contains_no_plaintext_svg_payload() -> None:
    plan_bytes = _PLAN.read_bytes()

    assert b"<svg" not in plan_bytes.lower()
    assert b"the simpsons" not in plan_bytes.lower()
    assert b"uninst.ico" not in plan_bytes.lower()


def test_icon_local_assets_and_outputs_are_ignored() -> None:
    ignores = {
        line.strip()
        for line in (_ROOT / ".gitignore").read_text(encoding="utf-8").splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    }

    assert "src/migration/icon/assets" in ignores
    assert "src/migration/icon/out" in ignores
