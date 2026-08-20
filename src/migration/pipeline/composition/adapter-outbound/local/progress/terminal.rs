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
//   - Terminal outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Terminal outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Terminal outbound adapter.

use std::io::Write as _;

/// Write one complete progress summary line.
pub(super) fn line(text: &str) {
    write(&format!("{text}\n"));
}

/// Replace the current live progress line.
pub(super) fn live(text: &str) {
    write(&format!("\r{text}  \u{1b}[K"));
}

/// Clear the current live progress line.
pub(super) fn clear_live() {
    write("\r\u{1b}[K");
}

/// Perform one best-effort stderr write and flush.
fn write(text: &str) {
    let stderr = std::io::stderr();
    let mut stream = stderr.lock();
    drop(stream.write_all(text.as_bytes()));
    drop(stream.flush());
}
