// File:
//   - lib.rs
// Path:
//   - src/game-manifest/src/lib.rs
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
//   - The game-manifest public hexagonal facade.
// - Must-Not:
//   - Hide dependency direction or select concrete adapters implicitly.
// - Allows:
//   - Expose layered APIs and deliberate compatibility re-exports.
// - Split-When:
//   - Split when a public bounded context becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same crate-level contracts.
// - Summary:
//   - Public facade for manifest generation, validation, and auditing.
// - Description:
//   - Separates pure manifest rules, application commands, ports, and adapters.
// - Usage:
//   - Imported by workspace crates and thin executables.
// - Defaults:
//   - No tree or artifact path is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Hexagonal facade for game manifest workflows.
//!
//! Domain classification stays pure, application commands depend on ports, and
//! concrete filesystem behavior remains in adapters.
pub mod adapters;
pub mod application;
#[path = "domain/domain.rs"]
pub mod domain;
pub mod ports;

pub use adapters::count_by_dir_ext;
pub use application::{
    EXPANDED_SCHEMA_LINE, GenerateExpandedManifest, GenerateExpandedReport,
    GenerateManifest, GenerateManifestReport, ManifestError, StructuralAudit,
    StructuralAuditReport, ValidateManifest, ValidateManifestReport,
};
pub use domain::{
    BACKUP_EXTENSION, DirCount, DirExtCounts, EXPANDED_MANIFEST_FILE_NAME,
    GENERATED_IMAGE_EXTENSION, KIND_TAXONOMY, MANIFEST_FILE_NAME, NO_EXTENSION,
    OPTIONAL_EXTENSION, classify_manifest_bucket, count_by_dir_ext_paths,
    extension_of, kind_taxonomy_jsonl, obfuscate_component,
};
pub use ports::{GameTree, PathKind, TextArtifactStore};
