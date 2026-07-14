// File:
//   - adapters.rs
// Path:
//   - src/rsd/src/adapters.rs
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
//   - rsd module behavior for adapters.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute adapters.
// - Split-When:
//   - Split when adapters contains two independently testable contracts.
// - Merge-When:
//   - Another rsd module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Filesystem adapters for RSD export.
// - Description:
//   - Defines adapters data and behavior for rsd root.
// - Usage:
//   - Used by rsd root code that needs adapters.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Inbound and outbound adapters for RSD export workflows.
//!
//! Driving adapters compose requests while driven adapters implement traversal
//! and transactional publication behind explicit ports.
pub mod driven;
/// Inbound adapters such as the command-line composition root.
pub mod driving;

pub use driven::FilesystemExporter;
