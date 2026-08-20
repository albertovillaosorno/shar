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
//   - Markdown sink outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Markdown sink outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Markdown sink outbound port.

use std::io;
use std::path::Path;

/// Publishes a complete converted document.
pub trait MarkdownSink {
    /// Writes one complete document to an explicit path.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when publication fails.
    fn write(&self, path: &Path, document: &str) -> io::Result<()>;
}
