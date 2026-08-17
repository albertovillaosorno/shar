// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT

//! Storage-independent SHAR mod-package model and validation.

use std::collections::BTreeSet;
use std::fmt;
use std::path::Path;

use serde::{Deserialize, Serialize};
use shar_sha256::Sha256;
use unicode_normalization::UnicodeNormalization as _;

/// Exact schema identifier for the first normalized SHAR mod contract.
pub const CONTRACT_VERSION: &str = "shar.mod-package.v1";
const MAX_ID_BYTES: usize = 96;
const MAX_REVISION_BYTES: usize = 96;
const MAX_LIST_ITEMS: usize = 4_096;
const MAX_MEMBERS: usize = 50_000;
const MAX_MEMBER_BYTES: u64 = 8 * 1024 * 1024 * 1024;
const MAX_PACKAGE_BYTES: u64 = 64 * 1024 * 1024 * 1024;
const MAX_PRIORITY_ABS: i32 = 1_000_000;

/// Content/native execution boundary declared by one package.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PackageKind {
    /// Data/assets only; no package-provided executable code.
    Content,
    /// Contains or requires package-provided native code.
    Native,
}

/// Trust boundary required before activation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TrustLevel {
    /// Static package validation is sufficient for the content boundary.
    ContentOnly,
    /// Native execution requires a separate explicit user trust decision.
    NativeExplicit,
}

/// Exact dependency on one canonical package revision.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct Dependency {
    /// Canonical package identity.
    pub canonical_id: String,
    /// Exact accepted package revision for contract v1.
    pub revision: String,
}

/// One exact package member identity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    /// Ordered, unique provenance/authorship labels.
    pub authors: Vec<String>,
    /// Human-readable source/provenance declaration.
    pub source: String,
    /// License identifier or `NOASSERTION` when rights are source-dependent.
    pub license: String,
}

/// Storage-independent canonical package declaration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
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
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for PackageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for PackageError {}

fn is_canonical_token(value: &str, maximum: usize, separators: &[char]) -> bool {
    if value.is_empty() || value.len() > maximum || !value.is_ascii() {
        return false;
    }
    let mut previous_separator = false;
    for (index, character) in value.chars().enumerate() {
        let separator = separators.contains(&character);
        if !(character.is_ascii_lowercase() || character.is_ascii_digit() || separator) {
            return false;
        }
        if separator && (index == 0 || previous_separator) {
            return false;
        }
        previous_separator = separator;
    }
    !previous_separator
}

fn validate_id(value: &str, label: &str) -> Result<(), PackageError> {
    if !is_canonical_token(value, MAX_ID_BYTES, &['.', '-']) {
        return Err(PackageError::new(format!(
            "{label} must be lowercase ASCII identity data"
        )));
    }
    Ok(())
}

fn validate_revision(value: &str) -> Result<(), PackageError> {
    if !is_canonical_token(value, MAX_REVISION_BYTES, &['.', '-', '_']) {
        return Err(PackageError::new(
            "package revision must be lowercase ASCII identity data",
        ));
    }
    Ok(())
}

fn validate_semantic_token(value: &str, label: &str) -> Result<(), PackageError> {
    if !is_canonical_token(value, 160, &['.', '-', '_', '/', ':']) {
        return Err(PackageError::new(format!(
            "{label} must be canonical lowercase ASCII token data"
        )));
    }
    Ok(())
}

fn validate_sorted_unique(values: &[String], label: &str) -> Result<(), PackageError> {
    if values.len() > MAX_LIST_ITEMS {
        return Err(PackageError::new(format!("{label} exceeds item limit")));
    }
    if values.windows(2).any(|pair| pair.first() >= pair.get(1)) {
        return Err(PackageError::new(format!(
            "{label} must be strictly sorted and unique"
        )));
    }
    Ok(())
}

fn validate_identity_list(
    values: &[String],
    label: &str,
    own_id: &str,
) -> Result<(), PackageError> {
    validate_sorted_unique(values, label)?;
    for value in values {
        validate_id(value, label)?;
        if value == own_id {
            return Err(PackageError::new(format!(
                "{label} must not contain the package itself"
            )));
        }
    }
    Ok(())
}

fn validate_semantic_list(values: &[String], label: &str) -> Result<(), PackageError> {
    validate_sorted_unique(values, label)?;
    for value in values {
        validate_semantic_token(value, label)?;
    }
    Ok(())
}

fn validate_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn normalized_member_identity(path: &str) -> Result<String, PackageError> {
    if path.contains('\\') {
        return Err(PackageError::new(
            "member paths must use canonical forward slashes",
        ));
    }
    let normalized = path.nfc().collect::<String>();
    if normalized != path {
        return Err(PackageError::new(
            "member path must already be NFC-normalized",
        ));
    }
    let relative = Path::new(path);
    let _resolved = schoenwald_filesystem::resolve_under(Path::new("package"), relative)
        .map_err(|error| PackageError::new(format!("invalid member path: {error}")))?;
    let mut identity = String::new();
    for character in normalized.chars() {
        for lowercase in character.to_lowercase() {
            identity.push(lowercase);
        }
    }
    Ok(identity)
}

impl PackageManifest {
    /// Parses one exact contract-v1 JSON object and validates canonical form.
    ///
    /// # Errors
    /// Returns a deterministic failure for malformed JSON or invalid package data.
    pub fn from_json(text: &str) -> Result<Self, PackageError> {
        let manifest: Self = serde_json::from_str(text)
            .map_err(|error| PackageError::new(format!("invalid package JSON: {error}")))?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Serializes one already-valid package declaration deterministically.
    ///
    /// # Errors
    /// Returns a deterministic failure when package data is invalid or JSON
    /// serialization fails.
    pub fn to_pretty_json(&self) -> Result<String, PackageError> {
        self.validate()?;
        let mut text = serde_json::to_string_pretty(self)
            .map_err(|error| PackageError::new(format!("package JSON failed: {error}")))?;
        text.push('\n');
        Ok(text)
    }

    /// Validates canonical deterministic package metadata without filesystem I/O.
    ///
    /// # Errors
    /// Returns a deterministic failure when any contract invariant is violated.
    pub fn validate(&self) -> Result<(), PackageError> {
        if self.contract_version != CONTRACT_VERSION {
            return Err(PackageError::new(
                "unsupported mod-package contract version",
            ));
        }
        validate_id(&self.canonical_id, "canonical package id")?;
        validate_revision(&self.package_revision)?;
        if self.priority < -MAX_PRIORITY_ABS || self.priority > MAX_PRIORITY_ABS {
            return Err(PackageError::new(
                "package priority exceeds contract bounds",
            ));
        }
        if self.dependencies.len() > MAX_LIST_ITEMS {
            return Err(PackageError::new("dependencies exceed item limit"));
        }
        if self
            .dependencies
            .windows(2)
            .any(|pair| pair.first() >= pair.get(1))
        {
            return Err(PackageError::new(
                "dependencies must be strictly sorted and unique",
            ));
        }
        for dependency in &self.dependencies {
            validate_id(&dependency.canonical_id, "dependency id")?;
            validate_revision(&dependency.revision)?;
            if dependency.canonical_id == self.canonical_id {
                return Err(PackageError::new("package must not depend on itself"));
            }
        }
        validate_identity_list(&self.conflicts, "conflicts", &self.canonical_id)?;
        validate_identity_list(&self.supersedes, "supersedes", &self.canonical_id)?;
        if self
            .conflicts
            .iter()
            .any(|value| self.supersedes.binary_search(value).is_ok())
        {
            return Err(PackageError::new(
                "one package id must not be both conflict and supersession data",
            ));
        }
        validate_semantic_list(&self.required_capabilities, "required capabilities")?;
        validate_semantic_list(&self.supported_targets, "supported targets")?;
        match (self.package_kind, self.trust_level) {
            (PackageKind::Content, TrustLevel::ContentOnly) => {}
            (PackageKind::Native, TrustLevel::NativeExplicit) => {
                if self.supported_targets.is_empty() {
                    return Err(PackageError::new(
                        "native packages require explicit supported targets",
                    ));
                }
            }
            _ => {
                return Err(PackageError::new(
                    "package kind and trust level do not match",
                ));
            }
        }
        self.validate_members()?;
        self.validate_provenance()
    }

    fn validate_members(&self) -> Result<(), PackageError> {
        if self.members.is_empty() {
            return Err(PackageError::new(
                "package must contain at least one member",
            ));
        }
        if self.members.len() > MAX_MEMBERS {
            return Err(PackageError::new(
                "package member count exceeds contract limit",
            ));
        }
        let mut portable_identities = BTreeSet::new();
        let mut previous_path: Option<&str> = None;
        let mut total_bytes = 0_u64;
        for member in &self.members {
            if previous_path.is_some_and(|previous| previous >= member.path.as_str()) {
                return Err(PackageError::new(
                    "members must be strictly sorted by canonical path",
                ));
            }
            previous_path = Some(&member.path);
            let identity = normalized_member_identity(&member.path)?;
            if !portable_identities.insert(identity) {
                return Err(PackageError::new(
                    "member paths collide after portable normalization",
                ));
            }
            if member.path == "mod.json" {
                return Err(PackageError::new(
                    "mod.json is reserved for the package declaration",
                ));
            }
            if member.bytes > MAX_MEMBER_BYTES {
                return Err(PackageError::new("package member exceeds byte limit"));
            }
            total_bytes = total_bytes
                .checked_add(member.bytes)
                .ok_or_else(|| PackageError::new("package byte count overflow"))?;
            if total_bytes > MAX_PACKAGE_BYTES {
                return Err(PackageError::new("package exceeds total byte limit"));
            }
            if !validate_sha256(&member.sha256) {
                return Err(PackageError::new(
                    "member SHA-256 must be lowercase hexadecimal",
                ));
            }
            validate_semantic_token(&member.media_type, "member media type")?;
            validate_semantic_token(&member.role, "member role")?;
        }
        Ok(())
    }

    fn validate_provenance(&self) -> Result<(), PackageError> {
        validate_sorted_unique(&self.provenance.authors, "provenance authors")?;
        if self.provenance.authors.is_empty() {
            return Err(PackageError::new(
                "provenance requires at least one author label",
            ));
        }
        for author in &self.provenance.authors {
            if author.trim() != author || author.is_empty() || author.len() > 160 {
                return Err(PackageError::new("invalid provenance author label"));
            }
        }
        for (label, value) in [
            ("provenance source", self.provenance.source.as_str()),
            ("provenance license", self.provenance.license.as_str()),
        ] {
            if value.trim() != value || value.is_empty() || value.len() > 512 {
                return Err(PackageError::new(format!("invalid {label}")));
            }
        }
        Ok(())
    }
}

/// Constructs one member identity directly from exact bytes.
///
/// # Errors
/// Returns a deterministic failure when path/media-role metadata is invalid or
/// the byte length exceeds the contract limit.
pub fn member_from_bytes(
    path: &str,
    media_type: &str,
    role: &str,
    bytes: &[u8],
) -> Result<Member, PackageError> {
    let byte_count = u64::try_from(bytes.len())
        .map_err(|_error| PackageError::new("member byte length does not fit u64"))?;
    let member = Member {
        path: path.to_owned(),
        bytes: byte_count,
        sha256: shar_sha256::digest_hex(bytes),
        media_type: media_type.to_owned(),
        role: role.to_owned(),
    };
    if member.bytes > MAX_MEMBER_BYTES {
        return Err(PackageError::new("package member exceeds byte limit"));
    }
    let _identity = normalized_member_identity(&member.path)?;
    validate_semantic_token(&member.media_type, "member media type")?;
    validate_semantic_token(&member.role, "member role")?;
    Ok(member)
}

/// Derives one deterministic lowercase revision from an already-sorted member set.
///
/// # Errors
/// Returns a deterministic failure when members are noncanonical.
pub fn content_revision(members: &[Member]) -> Result<String, PackageError> {
    if members.is_empty() {
        return Err(PackageError::new(
            "cannot derive revision from empty members",
        ));
    }
    let mut state = Sha256::new();
    let mut previous_path: Option<&str> = None;
    let mut identities = BTreeSet::new();
    for member in members {
        if previous_path.is_some_and(|previous| previous >= member.path.as_str()) {
            return Err(PackageError::new(
                "revision members must be strictly sorted by canonical path",
            ));
        }
        previous_path = Some(&member.path);
        let identity = normalized_member_identity(&member.path)?;
        if !identities.insert(identity) {
            return Err(PackageError::new(
                "revision member paths collide after portable normalization",
            ));
        }
        if !validate_sha256(&member.sha256) {
            return Err(PackageError::new(
                "member SHA-256 must be lowercase hexadecimal",
            ));
        }
        let path_bytes = member.path.as_bytes();
        let path_length = u64::try_from(path_bytes.len())
            .map_err(|_error| PackageError::new("member path length does not fit u64"))?;
        state.update(&path_length.to_be_bytes());
        state.update(path_bytes);
        state.update(&member.bytes.to_be_bytes());
        state.update(member.sha256.as_bytes());
        state.update(member.media_type.as_bytes());
        state.update(member.role.as_bytes());
    }
    Ok(state.finalize_hex())
}
