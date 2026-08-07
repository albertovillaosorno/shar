// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

// cspell:ignore selectmission closemission addstage closestage addobjective
// cspell:ignore closeobjective
//! Tests unit tests.

use std::path::Path;

use super::decode_straggler_text;

#[test]
fn decodes_windows_1252_text_stragglers() {
    let result = decode_straggler_text(
        b"Logitech\xae Force",
        Path::new("synthetic/era.txt"),
        "txt",
    );
    assert!(
        result.as_deref() == Ok("Logitech\u{ae} Force"),
        "era Windows-1252 bytes must decode deterministically"
    );
}

#[test]
fn rejects_undefined_windows_1252_text_stragglers() {
    let result = decode_straggler_text(
        &[0x81_u8],
        Path::new("synthetic/invalid.txt"),
        "txt",
    );
    assert!(
        result.is_err(),
        "bytes Windows-1252 leaves undefined must fail closed"
    );
}

#[test]
fn mission_v2_renderer_matches_semantic_preflight() -> Result<(), String> {
    let source = concat!(
        "SelectMission(\"m1\");\n",
        "AddStage(0);\n",
        "AddObjective(\"goto\");\n",
        "CloseObjective();\n",
        "CloseStage();\n",
        "CloseMission();\n",
    );
    let rendered = super::semantic_json_from_text(
        Path::new("scripts/missions/level01/m1i.mfk"),
        "mfk",
        source.as_bytes(),
        source,
    );
    let evidence = crate::domain::preflight_mission_script(&rendered)?;
    if evidence.statement_count() != 6 || evidence.invocations().len() != 6 {
        return Err(
            "rendered mission evidence changed during preflight".to_owned()
        );
    }
    Ok(())
}
