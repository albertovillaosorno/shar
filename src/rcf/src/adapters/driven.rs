// File:
//   - driven.rs
// Path:
//   - src/rcf/src/adapters/driven.rs
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
//   - Replaceable outbound adapters for archive bytes and extracted payloads.
// - Must-Not:
//   - Parse CLI arguments or change RCF domain and application semantics.
// - Allows:
//   - Filesystem IO behind the archive source and entry sink ports.
// - Split-When:
//   - Split when a new storage provider requires an independent adapter family.
// - Merge-When:
//   - Another adapter facade owns the same outbound implementations.
// - Summary:
//   - Driven adapter facade for RCF archive IO.
// - Description:
//   - Exposes concrete filesystem implementations of RCF outbound ports.
// - Usage:
//   - Constructed by driving adapters and integration tests.
// - Defaults:
//   - No output path is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driven adapters implementing RCF archive IO ports.
//!
//! Concrete storage mechanisms remain replaceable behind the port traits used
//! by application use cases.
pub mod filesystem;

pub use filesystem::{FileArchiveSource, FileEntrySink};
