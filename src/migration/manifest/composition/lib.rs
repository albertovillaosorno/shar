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
//   - Manifest lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Manifest lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Manifest lib.rs.

#[path = "adapters.rs"]
pub mod adapters;
#[path = "application/mod.rs"]
pub mod application;
#[path = "../domain/mod.rs"]
pub mod domain;
#[path = "ports/mod.rs"]
pub mod ports;

pub use adapters::count_by_dir_ext;
pub use application::{
    EXPANDED_SCHEMA_LINE, GenerateExpandedManifest, GenerateExpandedReport,
    GenerateManifest, GenerateManifestReport, ManifestError, ObserveManifest,
    ObserveManifestReport, StructuralAudit,
    StructuralAuditReport, ValidateManifest, ValidateManifestReport,
};
pub use domain::{
    BACKUP_EXTENSION, DirCount, DirExtCounts, EXACT_FILE_REQUIREMENTS,
    EXPANDED_MANIFEST_FILE_NAME, GENERATED_IMAGE_EXTENSION, KIND_TAXONOMY,
    MANIFEST_FILE_NAME, NO_EXTENSION,
    classify_manifest_bucket, count_by_dir_ext_paths, exact_file_shortfalls,
    extension_of, kind_taxonomy_jsonl, obfuscate_component,
};
pub use ports::{GameTree, PathKind, TextArtifactStore};
