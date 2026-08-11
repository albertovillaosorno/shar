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
//   - Unit evidence for source gag totals.
// - Must-Not:
//   - Assert viewed, completed, saved, or progression gag state.
// - Allows:
//   - Verify exact level/count lexemes and fail-closed validation.
// - Split-When:
//   - Gag catalog membership tests gain independent authority.
// - Merge-When:
//   - Collectible catalog tests own these source totals.
// - Summary:
//   - SetTotalGags unit tests.
// - Description:
//   - Proves per-level source totals remain distinct from player progress.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Duplicate levels, invalid counts, and invalid levels fail closed.
//

//! Unit evidence for source-backed gag totals.

use super::*;

#[test]
fn parses_reviewed_source_values() -> Result<(), String> {
    assert_eq!(required_unsigned("15", "total")?, "15");
    assert!(required_unsigned(" 15", "total").is_err());
    assert!(required_unsigned("1.5", "total").is_err());
    Ok(())
}

#[test]
fn report_defaults_empty_without_source_commands() {
    let report = MissionGagTotalReport::default();
    assert!(report.bindings().is_empty());
}
