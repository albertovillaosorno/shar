// File:
//   - lib.rs
// Path:
//   - src/rtf/src/lib.rs
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
//   - The RTF public hexagonal facade.
// - Must-Not:
//   - Hide dependency direction or select concrete adapters implicitly.
// - Allows:
//   - Expose layered APIs and deliberate conversion compatibility re-exports.
// - Split-When:
//   - Split when a public bounded context becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same crate-level contracts.
// - Summary:
//   - Public facade for RTF parsing and README conversion.
// - Description:
//   - Separates pure conversion, application policy, ports, and adapters.
// - Usage:
//   - Imported by workspace crates and the thin executable.
// - Defaults:
//   - No source or destination is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Hexagonal facade for RTF parsing and README conversion.
//!
//! Domain conversion stays pure, application commands depend on ports, and
//! concrete filesystem behavior remains in adapters.
pub mod adapters;
pub mod application;
#[path = "domain/domain.rs"]
pub mod domain;
pub mod ports;

pub use application::{ConvertReadme, ConvertReadmeError};
pub use domain::{format_unix_date, rtf_to_markdown};
pub use ports::{MarkdownSink, RtfSnapshot, RtfSource};
