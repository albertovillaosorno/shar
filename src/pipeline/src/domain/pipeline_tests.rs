// File:
//   - pipeline_tests.rs
// Path:
//   - src/pipeline/src/domain/pipeline_tests.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Regression coverage for checked stage-report metrics.
// - Must-Not:
//   - Perform IO, invoke adapters, or inspect repository data.
// - Allows:
//   - Exercise pure report arithmetic at representable limits.
// - Split-When:
//   - Another report invariant gains independent regression ownership.
// - Merge-When:
//   - Stage-report tests move beside a dedicated metrics value.
// - Summary:
//   - Stage-report arithmetic regressions.
// - Description:
//   - Verifies file and byte totals fail closed instead of saturating.
// - Usage:
//   - Included by pipeline.rs under cfg(test).
// - Defaults:
//   - Tests use deterministic scalar limits only.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression tests for checked stage-report metrics.
//!
//! Each case proves report arithmetic rejects values beyond its scalar range.

use super::StageReport;

#[test]
fn rejects_stage_file_count_overflow() -> Result<(), String> {
    if StageReport::checked_file_total(
        "test-stage",
        usize::MAX,
        1,
    )
    .is_err()
    {
        Ok(())
    } else {
        Err(String::from("stage file overflow was accepted"))
    }
}

#[test]
fn rejects_stage_byte_total_overflow() -> Result<(), String> {
    if StageReport::checked_byte_total(
        "test-stage",
        u64::MAX,
        1,
    )
    .is_err()
    {
        Ok(())
    } else {
        Err(String::from("stage byte overflow was accepted"))
    }
}
