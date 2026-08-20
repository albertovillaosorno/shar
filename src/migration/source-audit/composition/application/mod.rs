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
//   - Application-service module registration for source-audit.
// - Must-Not:
//   - Implement adapter or domain responsibilities.
// - Allows:
//   - Expose approved deep source-audit application services.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Source-audit application registry.
// - Description:
//   - Application-service module registration for source-audit.
// - Usage:
//   - Used through the owning source-audit function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Source-audit application services.

mod audit;

pub use audit::{DeepSourceAudit, DeepSourceAuditReport};
