// File:
//   - adapters.rs
// Path:
//   - src/p3d/src/adapters.rs
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
//   - p3d module behavior for adapters.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute adapters.
// - Split-When:
//   - Split when adapters contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Defines driven for this module boundary.
// - Description:
//   - Defines adapters data and behavior for p3d root.
// - Usage:
//   - Used by p3d root code that needs adapters.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Inbound and outbound adapters for Pure3D workflows.
//!
//! Driving adapters compose requests while driven adapters implement decoding
//! and package publication behind explicit ports.
pub mod driven;
/// Inbound adapters such as single and batch command-line composition.
pub mod driving;
