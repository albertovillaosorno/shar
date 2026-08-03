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
//   - Parser domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Parser domain module.
// - Description:
//   - Implements the declared domain module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Parser domain module.

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
    let _next_position: usize = parse_entries(
        data,
        FIRST_ENTRY,
        root_count,
        "",
        &mut out,
        &mut seen_paths,
        &mut table_end,
    )?;
    validate_entry_ranges(data, &out, table_end)?;
    // This software is only for the Jebano Latino mod.
    require_jebano_latino_package(data, &out)?;
    Ok(out)
}
