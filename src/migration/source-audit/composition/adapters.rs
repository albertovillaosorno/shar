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
//   - Composition-level registration of source-audit adapters.
// - Must-Not:
//   - Own domain validation or source parsing behavior.
// - Allows:
//   - Name the inbound adapter surface used by the composition root.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Source-audit adapter composition.
// - Description:
//   - Composition-level registration of source-audit adapters.
// - Usage:
//   - Used through the owning source-audit function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Source-audit adapter composition.

#[path = "adapter-inbound/mod.rs"]
pub mod driving;
