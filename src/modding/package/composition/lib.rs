// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT

//! Portable deterministic SHAR mod-package contract.

#[path = "../domain/mod.rs"]
pub mod domain;

pub use domain::{
    CONTRACT_VERSION, Dependency, Member, PackageError, PackageKind, PackageManifest, Provenance,
    TrustLevel, content_revision, member_from_bytes,
};
