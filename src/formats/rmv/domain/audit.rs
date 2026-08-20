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
//   - Audit domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Audit domain module.
// - Description:
//   - Implements the declared domain module responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Audit domain module.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use super::{MovieKind, ProvenanceEvidence, Sha256};

/// One discovered movie and its derived output identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovieRecord {
    /// Input root that owns the source.
    pub source_root: PathBuf,
    /// Full source path selected by the adapter.
    pub source_path: PathBuf,
    /// Stable path relative to the input root.
    pub relative_path: PathBuf,
    /// Expected converted output path.
    pub output_path: PathBuf,
    /// Source byte length.
    pub bytes: u64,
    /// Detected container kind.
    pub kind: MovieKind,
    /// Source content digest.
    pub hash: Sha256,
    /// Embedded provenance evidence.
    pub provenance: ProvenanceEvidence,
}

/// Deterministic result of auditing movie roots.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuditReport {
    /// Records ordered by source root and relative path.
    pub records: Vec<MovieRecord>,
    /// Expected outputs absent or not valid Bink2 containers.
    pub missing_bk2_outputs: usize,
    /// Repeated inputs beyond the first occurrence of each digest.
    pub duplicate_inputs: usize,
}

impl AuditReport {
    /// Counts distinct source hashes.
    #[must_use]
    pub fn unique_hashes(&self) -> usize {
        self.records
            .iter()
            .map(|record| record.hash)
            .collect::<BTreeSet<_>>()
            .len()
    }

    /// Counts records by detected movie kind.
    #[must_use]
    pub fn kind_counts(&self) -> BTreeMap<MovieKind, usize> {
        let mut counts = BTreeMap::new();
        for record in &self.records {
            let count = counts.entry(record.kind).or_insert(0_usize);
            *count = count.saturating_add(1);
        }
        counts
    }
}
