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
//   - Pure normalized mod-package records and deterministic package errors.
// - Must-Not:
//   - Own serialization, hashing, filesystem policy, or package activation.
// - Allows:
//   - Model storage-independent package identities and relationships.
// - Split-When:
//   - Package records gain independently versioned domain lifecycles.
// - Merge-When:
//   - Another domain module owns the identical package record contract.
// - Summary:
//   - Pure normalized mod-package domain model.
// - Description:
//   - Defines package records without external effect capabilities.
// - Usage:
//   - Used through the owning mod-package composition facade.
// - Defaults:
//   - Invalid package data is rejected by composition validation.
//

//! Pure storage-independent SHAR mod-package domain records.

use std::fmt;

/// Exact schema identifier for the first normalized SHAR mod contract.
pub const CONTRACT_VERSION: &str = "shar.mod-package.v1";

/// Content/native execution boundary declared by one package.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageKind {
    /// Data/assets only; no package-provided executable code.
    Content,
    /// Contains or requires package-provided native code.
    Native,
}

/// Trust boundary required before activation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustLevel {
    /// Static package validation is sufficient for the content boundary.
    ContentOnly,
    /// Native execution requires a separate explicit user trust decision.
    NativeExplicit,
}

/// Exact dependency on one canonical package revision.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dependency {
    /// Canonical package identity.
    pub canonical_id: String,
    /// Exact accepted package revision for contract v1.
    pub revision: String,
}

/// One exact package member identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    /// Canonical slash-separated package-relative path.
    pub path: String,
    /// Exact member byte length.
    pub bytes: u64,
    /// Lowercase SHA-256 digest of exact member bytes.
    pub sha256: String,
    /// Inspectable media type such as `application/json`.
    pub media_type: String,
    /// Stable semantic role such as `localization/text`.
    pub role: String,
}

/// Human/legal provenance carried with one package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    /// Ordered, unique provenance/authorship labels.
    pub authors: Vec<String>,
    /// Human-readable source/provenance declaration.
    pub source: String,
    /// License identifier or `NOASSERTION` when rights are source-dependent.
    pub license: String,
}

/// Storage-independent canonical package declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageManifest {
    /// Exact package contract version.
    pub contract_version: String,
    /// Stable package identity independent of storage.
    pub canonical_id: String,
    /// Deterministic package revision.
    pub package_revision: String,
    /// Content/native boundary.
    pub package_kind: PackageKind,
    /// Explicit deterministic load-order priority.
    pub priority: i32,
    /// Exact package dependencies, sorted by identity/revision.
    pub dependencies: Vec<Dependency>,
    /// Canonical package identities that cannot be active together.
    pub conflicts: Vec<String>,
    /// Canonical package identities explicitly superseded by this package.
    pub supersedes: Vec<String>,
    /// Required runtime/product capabilities.
    pub required_capabilities: Vec<String>,
    /// Exact target identifiers; empty means portable content when allowed.
    pub supported_targets: Vec<String>,
    /// Exact package members sorted by canonical path.
    pub members: Vec<Member>,
    /// Authorship/source/license evidence.
    pub provenance: Provenance,
    /// Required activation trust boundary.
    pub trust_level: TrustLevel,
}

/// Deterministic package-contract failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageError {
    message: String,
}

impl PackageError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl fmt::Display for PackageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for PackageError {}
