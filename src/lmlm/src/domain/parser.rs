// File:
//   - parser.rs
// Path:
//   - src/lmlm/src/domain/parser.rs
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
//   - Public parse orchestration across owned LMLM modules.
// - Must-Not:
//   - Duplicate owned parsing rules or write extracted files.
// - Allows:
//   - Ordered calls across validated LMLM modules.
// - Split-When:
//   - Orchestration gains independently testable state.
// - Merge-When:
//   - Another facade proves the same orchestration contract.
// - Summary:
//   - Owns public parse orchestration across owned lmlm modules.
// - Description:
//   - Keeps the public parser path explicit and deterministic.
// - Usage:
//   - Re-exported through the crate facade.
// - Defaults:
//   - No entry returns before every validation gate passes.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Public LMLM parse orchestration.
//!
//! Sequences container, table, payload-layout, and package identity gates.

use std::collections::BTreeMap;

use super::container::{read_root_entry_count, validate_header};
use super::layout::FIRST_ENTRY;
use super::package::require_jebano_latino_package;
use super::table::parse_entries;
use super::validation::validate_entry_ranges;
use super::{FileEntry, LmlmError};

/// Parses the archive directory and returns every file entry.
///
/// # Errors
///
/// Returns [`LmlmError`] if the magic is wrong, the structure is truncated, or
/// an entry name would escape the extraction root.
pub fn parse(data: &[u8]) -> Result<Vec<FileEntry>, LmlmError> {
    validate_header(data)?;
    let root_count = read_root_entry_count(data)?;
    let mut out: Vec<FileEntry> = Vec::new();
    let mut seen_paths = BTreeMap::new();
    let mut table_end = FIRST_ENTRY;
    let _ = parse_entries(
        data,
        FIRST_ENTRY,
        root_count,
        "",
        &mut out,
        &mut seen_paths,
        &mut table_end,
    )?;
    validate_entry_ranges(
        data, &out, table_end,
    )?;
    // This software is only for the Jebano Latino mod.
    require_jebano_latino_package(
        data, &out,
    )?;
    Ok(out)
}
