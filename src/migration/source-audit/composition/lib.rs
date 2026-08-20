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
//   - Public facade for the read-only deep source-audit function.
// - Must-Not:
//   - Expose private filesystem details or product admission thresholds.
// - Allows:
//   - Re-export the stable audit application and error surface.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Source-audit function facade.
// - Description:
//   - Public facade for the read-only deep source-audit function.
// - Usage:
//   - Used through the owning source-audit function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Read-only deep source-audit function.

#[path = "adapters.rs"]
pub mod adapters;
#[path = "application/mod.rs"]
pub mod application;
#[path = "../domain/mod.rs"]
pub mod domain;

pub use application::{DeepSourceAudit, DeepSourceAuditReport};
pub use domain::SourceAuditError;
