// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Pipeline tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Pipeline tests test module.
// - Description:
//   - Implements the declared test module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Pipeline tests test module.

use super::{PipelineError, StageReport};

#[test]
fn pipeline_error_escapes_control_characters() {
    let error = PipelineError::new("invalid\nsource\\evidence");

    assert_eq!(error.to_string(), r"invalid\nsource\evidence");
}

#[test]
fn rejects_stage_file_count_overflow() -> Result<(), String> {
    if StageReport::checked_file_total("test-stage", usize::MAX, 1).is_err() {
        Ok(())
    } else {
        Err(String::from("stage file overflow was accepted"))
    }
}

#[test]
fn rejects_stage_byte_total_overflow() -> Result<(), String> {
    if StageReport::checked_byte_total("test-stage", u64::MAX, 1).is_err() {
        Ok(())
    } else {
        Err(String::from("stage byte overflow was accepted"))
    }
}
