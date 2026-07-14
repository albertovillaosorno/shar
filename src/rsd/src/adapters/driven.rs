// File:
//   - driven.rs
// Path:
//   - src/rsd/src/adapters/driven.rs
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
//   - Replaceable outbound adapters for RSD discovery and publication.
// - Must-Not:
//   - Parse CLI requests or alter audio domain semantics.
// - Allows:
//   - Filesystem traversal and transactional WAV publication behind ports.
// - Split-When:
//   - Split when another storage provider gains an independent adapter family.
// - Merge-When:
//   - Another facade owns the same outbound implementations.
// - Summary:
//   - Driven adapter facade for RSD export.
// - Description:
//   - Exposes filesystem and transactional implementations of export ports.
// - Usage:
//   - Constructed by driving adapters and integration tests.
// - Defaults:
//   - No source or output path is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driven adapters implementing RSD outbound ports.
//!
//! Traversal and transactional publication remain outside domain and
//! application modules.
mod filesystem;
mod transaction;

pub use filesystem::FilesystemExporter;
