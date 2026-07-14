// File:
//   - driven.rs
// Path:
//   - src/rmv/src/adapters/driven.rs
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
//   - Replaceable outbound adapters for RMV audit storage and publication.
// - Must-Not:
//   - Parse CLI requests or alter domain decisions.
// - Allows:
//   - Filesystem discovery and TSV publication behind explicit ports.
// - Split-When:
//   - Split when a provider gains an independent adapter family.
// - Merge-When:
//   - Another facade owns the same outbound implementations.
// - Summary:
//   - Driven adapter facade for RMV workflows.
// - Description:
//   - Exposes concrete movie auditing and manifest sink implementations.
// - Usage:
//   - Constructed by driving composition roots and integration tests.
// - Defaults:
//   - No root or output path is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driven adapters implementing RMV outbound ports.
//!
//! Storage and serialization mechanisms remain replaceable behind application
//! port contracts.
mod filesystem_movie_auditor;
mod tsv_audit_manifest;

pub use filesystem_movie_auditor::FilesystemMovieAuditor;
pub use tsv_audit_manifest::TsvAuditManifestSink;
