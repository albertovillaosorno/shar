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

use std::collections::{BTreeMap, BTreeSet};
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
/// Returns an error when JSONL fields are malformed or component identities conflict.
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
    let mut ordinals = BTreeSet::new();
    let mut paths = BTreeMap::new();
    let mut relations: BTreeMap<usize, (usize, usize, usize)> = BTreeMap::new();
    for line in text.lines().filter(|line| line.contains("\"path\"")) {
        let value: serde_json::Value =
            serde_json::from_str(line).map_err(|error| {
                PipelineError::new(format!(
                    "prop component ledger JSON failed for {}: {error}",
                    path.display()
                ))
            })?;
        let parent_ordinal = required_usize(&value, "parent_ordinal")?;
        let row = LedgerRow {
            ordinal: required_usize(&value, "ordinal")?,
            depth: required_usize(&value, "depth")?,
            container_ordinal: required_usize(&value, "container_ordinal")?,
            name: required_string(&value, "name")?,
            path: required_string(&value, "path")?,
            kind: required_string(&value, "kind")?,
        };
        if !ordinals.insert(row.ordinal) {
            return Err(PipelineError::new(format!(
                "prop ledger repeats component ordinal {}",
                row.ordinal
            )));
        }
        let path_identity = row
            .path
            .chars()
            .flat_map(char::to_uppercase)
            .collect::<String>();
        if let Some(existing_path) = paths.insert(path_identity, row.path.clone()) {
            if existing_path == row.path {
                return Err(PipelineError::new(format!(
                    "prop ledger repeats component path {}",
                    row.path
                )));
            }
            return Err(PipelineError::new(format!(
                "prop ledger component paths collide portably: {existing_path} and {}",
                row.path
            )));
        }
        if row.depth == 1 && parent_ordinal != 0 {
            return Err(PipelineError::new(format!(
                "prop root component ordinal {} declares parent ordinal {}",
                row.ordinal, parent_ordinal
            )));
        }
        if row.depth > 1 && parent_ordinal == 0 {
            return Err(PipelineError::new(format!(
                "prop nested component ordinal {} declares root parent",
                row.ordinal
            )));
        }
        if row.depth == 1 && row.container_ordinal != row.ordinal {
            return Err(PipelineError::new(format!(
                "prop root component ordinal {} declares container ordinal {}",
                row.ordinal, row.container_ordinal
            )));
        }
        if row.depth == 1 {
            let _previous = owners.insert(row.ordinal, row.clone());
        }
        let _previous = relations.insert(
            row.ordinal,
            (row.depth, parent_ordinal, row.container_ordinal),
        );
        groups.entry(row.container_ordinal).or_default().push(row);
    }
    for (ordinal, (depth, parent_ordinal, container_ordinal)) in &relations {
        let Some((parent_depth, _parent_parent, parent_container)) =
            relations.get(parent_ordinal)
        else {
            continue;
        };
        let expected_depth = parent_depth.checked_add(1).ok_or_else(|| {
            PipelineError::new(format!(
                "prop parent component ordinal {parent_ordinal} depth overflows"
            ))
        })?;
        if *depth != expected_depth {
            return Err(PipelineError::new(format!(
                "prop component ordinal {ordinal} depth {depth} disagrees with parent ordinal {parent_ordinal} depth {parent_depth}"
            )));
        }
        if container_ordinal != parent_container {
            return Err(PipelineError::new(format!(
                "prop component ordinal {ordinal} container {container_ordinal} disagrees with parent ordinal {parent_ordinal} container {parent_container}"
            )));
        }
    }
    for owner_ordinal in groups.keys() {
        if !owners.contains_key(owner_ordinal) {
            return Err(PipelineError::new(format!(
                "prop ledger references missing owner ordinal {owner_ordinal}"
            )));
        }
    }
    Ok(WorldLedger { owners, groups })
}
