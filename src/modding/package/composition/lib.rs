// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Public facade for the normalized SHAR mod-package contract.
// - Must-Not:
//   - Execute or activate package-provided code.
// - Allows:
//   - Expose package records, validation, codecs, and revision helpers.
// - Split-When:
//   - Public package-contract surfaces gain independent lifecycles.
// - Merge-When:
//   - Another facade owns the identical normalized package API.
// - Summary:
//   - Normalized mod-package crate facade.
// - Description:
//   - Exposes pure records with composition-owned validation and codecs.
// - Usage:
//   - Used by language composition and package contract consumers.
// - Defaults:
//   - Invalid package declarations fail closed.
//

//! Portable deterministic SHAR mod-package contract.

mod contract;
#[path = "../domain/mod.rs"]
pub mod domain;

pub use contract::{content_revision, member_from_bytes};
pub use domain::{
    CONTRACT_VERSION, Dependency, Member, PackageError, PackageKind,
    PackageManifest, Provenance, TrustLevel,
};
