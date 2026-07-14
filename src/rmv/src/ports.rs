// File:
//   - ports.rs
// Path:
//   - src/rmv/src/ports.rs
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
//   - RMV outbound port declarations.
// - Must-Not:
//   - Implement storage behavior or command-line policy.
// - Allows:
//   - Traits isolating use cases from external mechanisms.
// - Split-When:
//   - Split when one port family becomes an independent context.
// - Merge-When:
//   - Another facade owns the same port declarations.
// - Summary:
//   - Hexagonal ports for RMV audit workflows.
// - Description:
//   - Exposes replaceable boundaries used by application use cases.
// - Usage:
//   - Implemented by driven adapters and supplied by driving adapters.
// - Defaults:
//   - Ports infer no storage or output locations.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Hexagonal ports for RMV audit workflows.
//!
//! Application code depends on these traits while storage and serialization
//! remain replaceable driven adapters.
mod audit_manifest;
mod movie_auditor;

pub use audit_manifest::AuditManifestSink;
pub use movie_auditor::MovieAuditor;
