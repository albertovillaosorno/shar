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
//   - Adapter outbound outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Adapter outbound outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Adapter outbound outbound adapter.

mod filesystem_movie_auditor;
mod tsv_audit_manifest;

pub use filesystem_movie_auditor::FilesystemMovieAuditor;
pub use tsv_audit_manifest::TsvAuditManifestSink;
