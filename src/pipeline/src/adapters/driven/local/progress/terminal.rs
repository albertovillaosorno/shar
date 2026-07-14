// File:
//   - terminal.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/progress/terminal.rs
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
//   - Best-effort stderr writes for live pipeline progress.
// - Must-Not:
//   - Fail extraction because a terminal stream is unavailable.
// - Allows:
//   - Write complete summary lines, replace live lines, and clear live output.
// - Split-When:
//   - Split when another terminal protocol needs distinct rendering policy.
// - Merge-When:
//   - The progress facade can own stream writes without violating SRP.
// - Summary:
//   - Writes pipeline progress text to stderr without print macros.
// - Description:
//   - Encapsulates best-effort terminal writes and flushing for progress UX.
// - Usage:
//   - Called only by the local progress lifecycle after rendering text.
// - Defaults:
//   - Stream failures are ignored because diagnostics must not stop extraction.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Writes complete and live pipeline progress text to stderr.
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
