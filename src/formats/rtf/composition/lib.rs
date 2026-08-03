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
//   - Rtf lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Rtf lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Rtf lib.rs.

#[path = "adapters.rs"]
pub mod adapters;
#[path = "application/mod.rs"]
pub mod application;
#[path = "../domain/mod.rs"]
pub mod domain;
#[path = "ports/mod.rs"]
pub mod ports;

pub use application::{ConvertReadme, ConvertReadmeError};
pub use domain::{format_unix_date, rtf_to_markdown};
pub use ports::{MarkdownSink, RtfSnapshot, RtfSource};
