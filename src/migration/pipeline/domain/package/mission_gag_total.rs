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
//   - Source `SetTotalGags` level-total evidence.
// - Must-Not:
//   - Infer viewed, completed, saved, or progression gag state.
// - Allows:
//   - Validate exact level and positive total-count source lexemes.
// - Split-When:
//   - Gag catalog membership gains independent source authority.
// - Merge-When:
//   - Final collectible catalog compilation owns these exact totals.
// - Summary:
//   - Source gag-total preflight.
// - Description:
//   - Preserves one reviewed total gag count for each authored level row.
// - Usage:
//   - Runs after lossless mission scope projection.
// - Defaults:
//   - Duplicate levels, malformed counts, or range drift fail closed.
//

//! Source-backed per-level `SetTotalGags` totals.

use std::collections::BTreeSet;

use super::MissionScopeReport;

/// One reviewed source gag total for a base-game level.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionGagTotalBinding {
    source_ordinal: usize,
    source_level: String,
    level: u8,
    source_total: String,
    total: u16,
}

impl MissionGagTotalBinding {
    /// Return source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return exact authored level lexeme.
    #[must_use]
    pub fn source_level(&self) -> &str {
        &self.source_level
    }

    /// Return validated base-game level number.
    #[must_use]
    pub const fn level(&self) -> u8 {
        self.level
    }

    /// Return exact authored total-count lexeme.
    #[must_use]
    pub fn source_total(&self) -> &str {
        &self.source_total
    }

    /// Return validated positive source total.
    #[must_use]
    pub const fn total(&self) -> u16 {
        self.total
    }
}

/// Reviewed source gag totals in authored order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionGagTotalReport {
    bindings: Vec<MissionGagTotalBinding>,
}

impl MissionGagTotalReport {
    /// Return per-level totals in authored order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionGagTotalBinding] {
        &self.bindings
    }
}

/// Compile every unscoped source `SetTotalGags` command.
///
/// # Errors
///
/// Returns an error for semantic-role or arity drift, duplicate levels,
/// malformed values, out-of-range levels, or zero totals.
pub fn preflight_mission_gag_totals(
    scopes: &MissionScopeReport,
) -> Result<MissionGagTotalReport, String> {
    let mut bindings = Vec::new();
    let mut levels = BTreeSet::new();
    for command in scopes
        .unscoped_commands()
        .iter()
        .filter(|command| command.name() == "settotalgags")
    {
        if command.semantic_role() != "mission-script" {
            return Err("SetTotalGags semantic role changed".to_owned());
        }
        let [level_source, total_source] = command.arguments() else {
            return Err("SetTotalGags must have two arguments".to_owned());
        };
        let level_source = required_unsigned(level_source, "level")?;
        let level = level_source
            .parse::<u8>()
            .map_err(|_error| "SetTotalGags level is not numeric".to_owned())?;
        if !(1..=7).contains(&level) {
            return Err("SetTotalGags level is outside base levels".to_owned());
        }
        if !levels.insert(level) {
            return Err("SetTotalGags level is duplicated".to_owned());
        }
        let total_source = required_unsigned(total_source, "total")?;
        let total = total_source
            .parse::<u16>()
            .map_err(|_error| "SetTotalGags total is not numeric".to_owned())?;
        if total == 0 {
            return Err("SetTotalGags total must be positive".to_owned());
        }
        bindings.push(MissionGagTotalBinding {
            source_ordinal: command.source_ordinal(),
            source_level: level_source,
            level,
            source_total: total_source,
            total,
        });
    }
    Ok(MissionGagTotalReport { bindings })
}

fn required_unsigned(value: &str, role: &str) -> Result<String, String> {
    if value.is_empty()
        || value != value.trim()
        || value.chars().any(char::is_control)
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(format!("SetTotalGags {role} is malformed"));
    }
    Ok(value.to_owned())
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_gag_total/tests.rs"]
mod tests;
