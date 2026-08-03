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
//   - Domain domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Domain domain module.
// - Description:
//   - Implements the declared domain module responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Domain domain module.

mod error;
mod escaped_path;
mod export_report;
mod rsd;
mod wav;

pub use error::RsdError;
pub(crate) use escaped_path::EscapedPath;
pub use export_report::{ExportReport, SourceRootReport};
pub use rsd::{RsdAudio, RsdEncoding, RsdHeader};
pub use wav::WavAudio;

/// Allocates one exact byte buffer for parser and serializer output.
///
/// # Errors
///
/// Returns [`RsdError::AllocationFailed`] when the requested capacity cannot be
/// represented or reserved.
pub(crate) fn byte_buffer(capacity: usize) -> Result<Vec<u8>, RsdError> {
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(capacity)
        .map_err(|_reserve_error| RsdError::AllocationFailed(capacity))?;
    Ok(bytes)
}

#[cfg(test)]
#[path = "../../../../tests/formats/rsd/unit/domain/tests.rs"]
mod tests;
