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
//   - World ledger outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - World ledger outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! World ledger outbound adapter.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use super::inventory_common::{required_string, required_usize};
use crate::domain::PipelineError;

/// One normalized component-ledger row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct LedgerRow {
    /// Component ordinal in the normalized source document.
    pub(super) ordinal: usize,
    /// Parsed chunk depth.
    pub(super) depth: usize,
    /// Direct root-child container ordinal.
    pub(super) container_ordinal: usize,
    /// Cleaned component identity.
    pub(super) name: String,
    /// Normalized component path below `components/`.
    pub(super) path: String,
    /// Normalized component family label.
    pub(super) kind: String,
}

/// Parsed ledger with direct owners and all rows grouped by owner ordinal.
#[derive(Debug)]
pub(super) struct WorldLedger {
    /// Direct root-child owner rows keyed by ordinal.
    pub(super) owners: BTreeMap<usize, LedgerRow>,
    /// Nested component rows grouped by root-child owner ordinal.
    pub(super) groups: BTreeMap<usize, Vec<LedgerRow>>,
}

/// Read and group one normalized package component ledger.
///
/// # Errors
///
/// Returns an error when JSONL fields are malformed or owner rows conflict.
pub(super) fn read_world_ledger(
    root: &Path,
) -> Result<WorldLedger, PipelineError> {
    let path = root.join("components.jsonl");
    let text = fs::read_to_string(&path).map_err(|error| {
        PipelineError::new(format!(
            "prop component ledger read failed for {}: {error}",
            path.display()
        ))
    })?;
    let mut owners = BTreeMap::new();
    let mut groups: BTreeMap<usize, Vec<LedgerRow>> = BTreeMap::new();
    for line in text.lines().filter(|line| line.contains("\"path\"")) {
        let value: serde_json::Value =
            serde_json::from_str(line).map_err(|error| {
                PipelineError::new(format!(
                    "prop component ledger JSON failed for {}: {error}",
                    path.display()
                ))
            })?;
        let row = LedgerRow {
            ordinal: required_usize(&value, "ordinal")?,
            depth: required_usize(&value, "depth")?,
            container_ordinal: required_usize(&value, "container_ordinal")?,
            name: required_string(&value, "name")?,
            path: required_string(&value, "path")?,
            kind: required_string(&value, "kind")?,
        };
        if row.depth == 1 && owners.insert(row.ordinal, row.clone()).is_some() {
            return Err(PipelineError::new(format!(
                "prop ledger repeats owner ordinal {}",
                row.ordinal
            )));
        }
        groups.entry(row.container_ordinal).or_default().push(row);
    }
    Ok(WorldLedger { owners, groups })
}
