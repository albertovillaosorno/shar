// File:
//   - mod.rs
// Path: src/foundation/sha256/domain/mod.rs
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
//   - Pure SHA-256 domain primitives.
// - Must-Not:
//   - Read files, use network access, or expose mutable hash state.
// - Allows:
//   - Deterministic hashing and lowercase hexadecimal projection.
// - Split-When:
//   - Incremental hashing gains an independently testable state machine.
// - Merge-When:
//   - Another domain module owns the same exact SHA-256 primitive.
// - Summary:
//   - SHA-256 domain facade.
// - Description:
//   - Exposes dependency-free hashing through the canonical domain kind.
// - Usage:
//   - Re-exported by the function public facade.
// - Defaults:
//   - No input source or external effect is selected implicitly.
//
// Related documents:
// - docs/adr/engineering/architecture/project-core-separation.md
//
// Large file:
//   - false
//

//! Pure SHA-256 domain primitives.

mod sha256;

pub use sha256::{digest, digest_hex, hex};
