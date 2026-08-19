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
//   - Domain module registration for source-audit records.
// - Must-Not:
//   - Own adapters, filesystem access, or process behavior.
// - Allows:
//   - Expose approved path-free source-audit domain records.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Source-audit domain registry.
// - Description:
//   - Domain module registration for source-audit records.
// - Usage:
//   - Used through the owning source-audit function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Source-audit domain records.

mod error;

pub use error::SourceAuditError;
