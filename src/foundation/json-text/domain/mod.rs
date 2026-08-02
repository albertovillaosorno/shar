// File:
//   - mod.rs
// Path: src/foundation/json-text/domain/mod.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Pure JSON text domain primitives.
// - Must-Not:
//   - Perform filesystem, process, network, or environment access.
// - Allows:
//   - Deterministic in-memory JSON string-content escaping.
// - Split-When:
//   - Typed document behavior gains an independently versioned contract.
// - Merge-When:
//   - Another domain module owns the same escaping primitive.
// - Summary:
//   - JSON text domain facade.
// - Description:
//   - Exposes portable JSON string escaping through the canonical domain kind.
// - Usage:
//   - Re-exported by the function public facade.
// - Defaults:
//   - No document rendering or external effect is selected implicitly.
//
// Related documents:
// - docs/adr/engineering/architecture/project-core-separation.md
//
// Large file:
//   - false
//

//! Pure JSON text domain primitives.

mod escaping;

pub use escaping::escape;
