// File:
//   - layout.rs
// Path:
//   - src/lmlm/src/domain/layout.rs
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
//   - Fixed constants for the supported LSPA container layout.
// - Must-Not:
//   - Write extracted files or bypass checked parser boundaries.
// - Allows:
//   - Operations required by this single LMLM responsibility.
// - Split-When:
//   - One contained invariant gains independent state or a distinct API.
// - Merge-When:
//   - Another LMLM module proves the same invariant without distinction.
// - Summary:
//   - Owns fixed constants for the supported lspa container layout.
// - Description:
//   - Keeps this parser responsibility deterministic and fail closed.
// - Usage:
//   - Imported only by owned LMLM modules.
// - Defaults:
//   - Malformed input never uses unchecked arithmetic.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Fixed layout constants for the supported LSPA container.
//!
//! Centralizes block sizes, offsets, header values, and package markers.

// Sibling parser modules share these contracts without exposing them publicly.
#![expect(
    clippy::redundant_pub_crate,
    reason = "sibling parser modules require crate-visible contracts while \
              the private module prevents external API exposure"
)]

/// Size of each structural block.
pub(crate) const BLOCK: usize = 0x200;
/// Structural block size for archive-declared offsets.
pub(crate) const BLOCK_U64: u64 = 0x200;
/// Offset of the root directory block.
pub(crate) const ROOT_BLOCK: usize = 0x400;
/// Offset of the first entry.
pub(crate) const FIRST_ENTRY: usize = 0x600;
/// Supported entry-kind word at the start of each name block.
pub(crate) const ENTRY_KIND: u16 = 2;
/// Maximum supported directory nesting below the archive root.
pub(crate) const MAX_DIRECTORY_DEPTH: usize = 64;
/// Container magic.
pub(crate) const MAGIC: &[u8; 4] = b"LSPA";
/// Offset of the LSPA container version field.
pub(crate) const VERSION_OFFSET: usize = 4;
/// Supported LSPA container version.
pub(crate) const VERSION: u32 = 5;
/// Offset of the LSPA container flags field.
pub(crate) const HEADER_FLAGS_OFFSET: usize = 0x0c;
/// Supported LSPA container flags.
pub(crate) const HEADER_FLAGS: u32 = 0x0200_0000;
/// Root metadata entry that owns the supported package identity.
pub(crate) const PACKAGE_METADATA_PATH: &str = "Meta.ini";
/// LF marker pins extraction to the one local package this parser was built
/// for.
pub(crate) const JEBANO_TITLE_LF: &[u8] =
    b"[Miscellaneous]\nTitle=The Simpsons: Hit & Run - Version Latino";
