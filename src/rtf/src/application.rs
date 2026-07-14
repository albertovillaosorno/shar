// File:
//   - application.rs
// Path:
//   - src/rtf/src/application.rs
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
//   - RTF conversion application use cases.
// - Must-Not:
//   - Implement parser internals, filesystem IO, or CLI presentation.
// - Allows:
//   - Coordinate domain conversion through explicit ports.
// - Split-When:
//   - Split when a use-case family becomes independently deployable.
// - Merge-When:
//   - Another facade owns the same application commands.
// - Summary:
//   - Application facade for RTF README conversion.
// - Description:
//   - Exposes complete document conversion without selecting concrete adapters.
// - Usage:
//   - Called by driving adapters and library clients.
// - Defaults:
//   - No source provider is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! RTF conversion application use cases.
//!
//! Commands coordinate pure conversion through replaceable source ports.
mod convert_readme;

pub use convert_readme::{ConvertReadme, ConvertReadmeError};
