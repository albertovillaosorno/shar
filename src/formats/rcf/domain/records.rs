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
//   - Records domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Records domain module.
// - Description:
//   - Implements the declared domain module responsibility for rcf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Records domain module.

/// Parsed archive header shared by RCF validation and extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchiveHeader {
    /// Number of entries declared by the resource table.
    pub entry_count: usize,
    /// Absolute byte offset of the name table.
    pub name_table_offset: u64,
    /// Alignment value declared by the archive.
    pub alignment: u32,
    /// Declared absolute start of the first stored file payload.
    pub first_file_offset: u64,
}

/// One resource-table record before name resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexRecord {
    /// Opaque lookup hash used by the original runtime.
    pub hash: u32,
    /// Absolute byte offset of the payload.
    pub offset: u64,
    /// Payload length in bytes.
    pub length: u64,
}

/// One resolved archive entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveEntry {
    /// Archive-relative entry name using forward slashes.
    pub name: String,
    /// Opaque lookup hash used by the original runtime.
    pub hash: u32,
    /// Absolute byte offset of the payload.
    pub offset: u64,
    /// Payload length in bytes.
    pub length: u64,
    /// Last modification time stored in the detailed file information.
    pub modification_time: u32,
}

/// Parsed RCF archive index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Archive {
    /// Header summary.
    pub header: ArchiveHeader,
    /// Resolved entries in archive name-table order.
    pub entries: Vec<ArchiveEntry>,
    /// Original archive length in bytes.
    pub archive_size: u64,
}

impl Archive {
    /// Returns the sum of all extracted payload lengths.
    #[must_use]
    pub fn payload_bytes(&self) -> u64 {
        self.entries.iter().map(|entry| entry.length).sum()
    }

    /// Returns the number of zero-length entries.
    #[must_use]
    pub fn zero_length_entries(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.length == 0)
            .count()
    }
}
