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
#   - Repository evidence for mission-script JSON/domain separation.
# - Must-Not:
#   - Parse mission payloads or inspect proprietary source data.
# - Allows:
#   - Inspect tracked mission-script source boundaries.
# - Split-When:
#   - Mission intake gains independently versioned boundary policy.
# - Merge-When:
#   - Another architecture test owns the same separation contract.
# - Summary:
#   - Mission-script domain boundary policy.
# - Description:
#   - Keeps JSON intake in composition and semantic evidence in domain.
# - Usage:
#   - Run through the canonical repository pytest gate.
# - Defaults:
#   - Reads only tracked repository source text.
#

"""Mission-script JSON/domain boundary policy."""

from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]
_PIPELINE = _ROOT / "src/migration/pipeline"


def test_mission_script_domain_owns_no_json_intake() -> None:
    """Keep serde and JSON decoding out of mission semantic domain code."""
    domain = (_PIPELINE / "domain/package/mission_script.rs").read_text()
    for fragment in ("serde", "serde_json", "cspell:"):
        assert fragment not in domain
    assert "preflight_mission_script(" not in domain
    assert "preflight_mission_script_document(" in domain


def test_mission_script_json_intake_lives_in_composition() -> None:
    """Keep public JSON intake on the composition side of the boundary."""
    intake = _PIPELINE / "composition/mission_script.rs"
    text = intake.read_text()
    assert "serde::Deserialize" in text
    assert "pub fn preflight_mission_script(" in text
