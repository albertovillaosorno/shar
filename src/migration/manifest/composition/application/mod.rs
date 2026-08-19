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
//   - Application application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Application application service.
// - Description:
//   - Implements the declared application service responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Application application service.

mod diagnostic_path;
mod error;
mod generate_expanded;
mod generate_manifest;
mod observe_manifest;
mod path_evidence;
mod rcf_evidence;
mod structural_audit;
mod validate_manifest;

pub use error::ManifestError;
pub use generate_expanded::{
    EXPANDED_SCHEMA_LINE, GenerateExpandedManifest, GenerateExpandedReport,
};
pub use generate_manifest::{GenerateManifest, GenerateManifestReport};
pub use observe_manifest::{ObserveManifest, ObserveManifestReport};
pub use structural_audit::{StructuralAudit, StructuralAuditReport};
pub use validate_manifest::{ValidateManifest, ValidateManifestReport};
