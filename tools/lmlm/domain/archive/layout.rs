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
//   - Layout domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Layout domain module.
// - Description:
//   - Implements the declared domain module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Layout domain module.

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
