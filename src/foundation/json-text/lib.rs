// File:
//   - lib.rs
// Path: src/foundation/json-text/lib.rs
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
//   - The public facade for portable JSON text behavior.
// - Must-Not:
//   - Implement domain behavior or perform external effects.
// - Allows:
//   - Re-export the stable JSON string-content escaping primitive.
// - Split-When:
//   - Another independently versioned public surface appears.
// - Merge-When:
//   - The function no longer needs a separate package boundary.
// - Summary:
//   - JSON text public facade.
// - Description:
//   - Preserves the package API while implementation remains in domain.
// - Usage:
//   - Imported by repository functions that render JSON text.
// - Defaults:
//   - No implementation behavior is duplicated here.
//
// Related documents:
// - docs/adr/engineering/architecture/project-core-separation.md
//
// Large file:
//   - false
//

//! Public facade for portable JSON text behavior.

pub mod domain;

pub use domain::escape;
