// File:
//   - application.rs
// Path:
//   - src/rsd/src/application.rs
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
//   - rsd module behavior for application.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute application.
// - Split-When:
//   - Split when application contains two independently testable contracts.
// - Merge-When:
//   - Another rsd module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Application services for batch RSD export.
// - Description:
//   - Defines application data and behavior for rsd root.
// - Usage:
//   - Used by rsd root code that needs application.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Application services for batch RSD export.
//!
//! This boundary keeps application services for batch rsd export explicit and
//! returns deterministic results to rsd callers.
mod export;

pub use export::ExportRoots;

pub use crate::domain::{ExportReport, SourceRootReport};
pub use crate::ports::Exporter;
