// File:
//   - lib.rs
// Path: src/foundation/sha256/lib.rs
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
//   - The public facade for dependency-free SHA-256 behavior.
// - Must-Not:
//   - Implement hashing behavior or perform external effects.
// - Allows:
//   - Re-export stable digest and hexadecimal primitives.
// - Split-When:
//   - Another independently versioned public surface appears.
// - Merge-When:
//   - The function no longer needs a separate package boundary.
// - Summary:
//   - SHA-256 public facade.
// - Description:
//   - Preserves the package API while implementation remains in domain.
// - Usage:
//   - Imported by repository functions that require exact artifact identities.
// - Defaults:
//   - No implementation behavior is duplicated here.
//
// Related documents:
// - docs/adr/engineering/architecture/project-core-separation.md
//
// Large file:
//   - false
//

//! Public facade for dependency-free SHA-256 behavior.

pub mod domain;

pub use domain::{digest, digest_hex, hex};
