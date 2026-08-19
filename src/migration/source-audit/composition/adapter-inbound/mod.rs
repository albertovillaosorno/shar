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
//   - Inbound adapter module registration for source-audit.
// - Must-Not:
//   - Implement audit behavior or filesystem traversal policy.
// - Allows:
//   - Expose approved inbound source-audit adapters.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Source-audit inbound adapter registry.
// - Description:
//   - Inbound adapter module registration for source-audit.
// - Usage:
//   - Used through the owning source-audit function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Inbound source-audit adapters.

pub mod cli;
