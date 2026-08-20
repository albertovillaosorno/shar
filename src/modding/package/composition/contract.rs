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
//   - Mod-package validation, JSON wire format, hashing, and path identity.
// - Must-Not:
//   - Execute package code, activate packages, or inspect private game data.
// - Allows:
//   - Validate package records and encode exact contract-v1 JSON.
// - Split-When:
//   - Package codecs and validation rules gain independent lifecycles.
// - Merge-When:
//   - Another composition module owns the same normalized package contract.
// - Summary:
//   - Normalized mod-package contract composition.
// - Description:
//   - Keeps effect-capable package validation outside the pure domain model.
// - Usage:
//   - Used through the public mod-package facade and language composition.
// - Defaults:
//   - Invalid, ambiguous, or nondeterministic package data fails closed.
//

//! Validation and JSON composition for normalized SHAR mod packages.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};
use shar_sha256::Sha256;
use unicode_normalization::UnicodeNormalization as _;

use crate::domain::{
    CONTRACT_VERSION, Dependency, Member, PackageError, PackageKind,
    PackageManifest, Provenance, TrustLevel,
};

const MAX_ID_BYTES: usize = 96;
const MAX_REVISION_BYTES: usize = 96;
const MAX_LIST_ITEMS: usize = 4_096;
const MAX_MEMBERS: usize = 50_000;
const MAX_MEMBER_BYTES: u64 = 8 * 1024 * 1024 * 1024;
const MAX_PACKAGE_BYTES: u64 = 64 * 1024 * 1024 * 1024;
const MAX_PRIORITY_ABS: i32 = 1_000_000;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PackageKindDocument {
    Content,
    Native,
}

impl From<PackageKindDocument> for PackageKind {
    fn from(value: PackageKindDocument) -> Self {
        match value {
            PackageKindDocument::Content => Self::Content,
            PackageKindDocument::Native => Self::Native,
        }
    }
}

impl From<PackageKind> for PackageKindDocument {
    fn from(value: PackageKind) -> Self {
        match value {
            PackageKind::Content => Self::Content,
            PackageKind::Native => Self::Native,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TrustLevelDocument {
    ContentOnly,
    NativeExplicit,
}

impl From<TrustLevelDocument> for TrustLevel {
    fn from(value: TrustLevelDocument) -> Self {
        match value {
            TrustLevelDocument::ContentOnly => Self::ContentOnly,
            TrustLevelDocument::NativeExplicit => Self::NativeExplicit,
        }
    }
}

impl From<TrustLevel> for TrustLevelDocument {
    fn from(value: TrustLevel) -> Self {
        match value {
            TrustLevel::ContentOnly => Self::ContentOnly,
            TrustLevel::NativeExplicit => Self::NativeExplicit,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DependencyDocument {
    canonical_id: String,
    revision: String,
}

impl From<DependencyDocument> for Dependency {
    fn from(value: DependencyDocument) -> Self {
        Self {
            canonical_id: value.canonical_id,
            revision: value.revision,
        }
    }
}

impl From<&Dependency> for DependencyDocument {
    fn from(value: &Dependency) -> Self {
        Self {
            canonical_id: value.canonical_id.clone(),
            revision: value.revision.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MemberDocument {
    path: String,
    bytes: u64,
    sha256: String,
    media_type: String,
    role: String,
}

impl From<MemberDocument> for Member {
    fn from(value: MemberDocument) -> Self {
        Self {
            path: value.path,
            bytes: value.bytes,
            sha256: value.sha256,
            media_type: value.media_type,
            role: value.role,
        }
    }
}

impl From<&Member> for MemberDocument {
    fn from(value: &Member) -> Self {
        Self {
            path: value.path.clone(),
            bytes: value.bytes,
            sha256: value.sha256.clone(),
            media_type: value.media_type.clone(),
            role: value.role.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProvenanceDocument {
    authors: Vec<String>,
    source: String,
    license: String,
}

impl From<ProvenanceDocument> for Provenance {
    fn from(value: ProvenanceDocument) -> Self {
        Self {
            authors: value.authors,
            source: value.source,
            license: value.license,
        }
    }
}

impl From<&Provenance> for ProvenanceDocument {
    fn from(value: &Provenance) -> Self {
        Self {
            authors: value.authors.clone(),
            source: value.source.clone(),
            license: value.license.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PackageManifestDocument {
    contract_version: String,
    canonical_id: String,
    package_revision: String,
    package_kind: PackageKindDocument,
    priority: i32,
    dependencies: Vec<DependencyDocument>,
    conflicts: Vec<String>,
    supersedes: Vec<String>,
    required_capabilities: Vec<String>,
    supported_targets: Vec<String>,
    members: Vec<MemberDocument>,
    provenance: ProvenanceDocument,
    trust_level: TrustLevelDocument,
}

impl From<PackageManifestDocument> for PackageManifest {
    fn from(value: PackageManifestDocument) -> Self {
        Self {
            contract_version: value.contract_version,
            canonical_id: value.canonical_id,
            package_revision: value.package_revision,
            package_kind: value.package_kind.into(),
            priority: value.priority,
            dependencies: value
                .dependencies
                .into_iter()
                .map(Into::into)
                .collect(),
            conflicts: value.conflicts,
            supersedes: value.supersedes,
            required_capabilities: value.required_capabilities,
            supported_targets: value.supported_targets,
            members: value.members.into_iter().map(Into::into).collect(),
            provenance: value.provenance.into(),
            trust_level: value.trust_level.into(),
        }
    }
}

impl From<&PackageManifest> for PackageManifestDocument {
    fn from(value: &PackageManifest) -> Self {
        Self {
            contract_version: value.contract_version.clone(),
            canonical_id: value.canonical_id.clone(),
            package_revision: value.package_revision.clone(),
            package_kind: value.package_kind.into(),
            priority: value.priority,
            dependencies: value.dependencies.iter().map(Into::into).collect(),
            conflicts: value.conflicts.clone(),
            supersedes: value.supersedes.clone(),
            required_capabilities: value.required_capabilities.clone(),
            supported_targets: value.supported_targets.clone(),
            members: value.members.iter().map(Into::into).collect(),
            provenance: (&value.provenance).into(),
            trust_level: value.trust_level.into(),
        }
    }
}

fn is_canonical_token(
    value: &str,
    maximum: usize,
    separators: &[char],
) -> bool {
    if value.is_empty() || value.len() > maximum || !value.is_ascii() {
        return false;
    }
    let mut previous_separator = false;
    for (index, character) in value.chars().enumerate() {
        let separator = separators.contains(&character);
        if !(character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || separator)
        {
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

fn validate_semantic_token(
    value: &str,
    label: &str,
) -> Result<(), PackageError> {
    if !is_canonical_token(value, 160, &['.', '-', '_', '/', ':']) {
        return Err(PackageError::new(format!(
            "{label} must be canonical lowercase ASCII token data"
        )));
    }
    Ok(())
}

fn validate_sorted_unique(
    values: &[String],
    label: &str,
) -> Result<(), PackageError> {
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

fn validate_semantic_list(
    values: &[String],
    label: &str,
) -> Result<(), PackageError> {
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
    let _resolved =
        schoenwald_filesystem::resolve_under(Path::new("package"), relative)
            .map_err(|error| {
                PackageError::new(format!("invalid member path: {error}"))
            })?;
    let mut identity = String::new();
    for character in normalized.chars() {
        for lowercase in character.to_lowercase() {
            identity.push(lowercase);
        }
    }
    Ok(identity)
}

fn validate_member(member: &Member) -> Result<String, PackageError> {
    let identity = normalized_member_identity(&member.path)?;
    if member.path == "mod.json" {
        return Err(PackageError::new(
            "mod.json is reserved for the package declaration",
        ));
    }
    if member.bytes > MAX_MEMBER_BYTES {
        return Err(PackageError::new("package member exceeds byte limit"));
    }
    if !validate_sha256(&member.sha256) {
        return Err(PackageError::new(
            "member SHA-256 must be lowercase hexadecimal",
        ));
    }
    validate_semantic_token(&member.media_type, "member media type")?;
    validate_semantic_token(&member.role, "member role")?;
    Ok(identity)
}

impl PackageManifest {
    /// Parses one exact contract-v1 JSON object and validates canonical form.
    ///
    /// # Errors
    /// Returns a deterministic failure for malformed JSON or invalid package
    /// data.
    pub fn from_json(text: &str) -> Result<Self, PackageError> {
        let document: PackageManifestDocument = serde_json::from_str(text)
            .map_err(|error| {
                PackageError::new(format!("invalid package JSON: {error}"))
            })?;
        let manifest = Self::from(document);
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
        let document = PackageManifestDocument::from(self);
        let mut text =
            serde_json::to_string_pretty(&document).map_err(|error| {
                PackageError::new(format!("package JSON failed: {error}"))
            })?;
        text.push('\n');
        Ok(text)
    }

    /// Validates canonical deterministic package metadata without filesystem
    /// I/O.
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
        if self.priority < -MAX_PRIORITY_ABS || self.priority > MAX_PRIORITY_ABS
        {
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
        for (index, dependency) in self.dependencies.iter().enumerate() {
            validate_id(&dependency.canonical_id, "dependency id")?;
            validate_revision(&dependency.revision)?;
            if dependency.canonical_id == self.canonical_id {
                return Err(PackageError::new(
                    "package must not depend on itself",
                ));
            }
            if index > 0
                && self.dependencies.get(index.saturating_sub(1)).is_some_and(
                    |previous| previous.canonical_id == dependency.canonical_id,
                )
            {
                return Err(PackageError::new(
                    "dependency package ids must be unique",
                ));
            }
        }
        validate_identity_list(
            &self.conflicts,
            "conflicts",
            &self.canonical_id,
        )?;
        validate_identity_list(
            &self.supersedes,
            "supersedes",
            &self.canonical_id,
        )?;
        if self
            .conflicts
            .iter()
            .any(|value| self.supersedes.binary_search(value).is_ok())
        {
            return Err(PackageError::new(
                "one package id must not be both conflict and \
                 supersession data",
            ));
        }
        for dependency in &self.dependencies {
            if self
                .conflicts
                .binary_search(&dependency.canonical_id)
                .is_ok()
            {
                return Err(PackageError::new(
                    "dependency package ids must not also be conflicts",
                ));
            }
            if self
                .supersedes
                .binary_search(&dependency.canonical_id)
                .is_ok()
            {
                return Err(PackageError::new(
                    "dependency package ids must not also be superseded",
                ));
            }
        }
        validate_semantic_list(
            &self.required_capabilities,
            "required capabilities",
        )?;
        validate_semantic_list(&self.supported_targets, "supported targets")?;
        match (self.package_kind, self.trust_level) {
            (PackageKind::Content, TrustLevel::ContentOnly) => {},
            (PackageKind::Native, TrustLevel::NativeExplicit) => {
                if self.supported_targets.is_empty() {
                    return Err(PackageError::new(
                        "native packages require explicit supported targets",
                    ));
                }
            },
            _ => {
                return Err(PackageError::new(
                    "package kind and trust level do not match",
                ));
            },
        }
        self.validate_members()?;
        let expected_revision = content_revision(&self.members)?;
        if self.package_revision != expected_revision {
            return Err(PackageError::new(
                "package revision does not match canonical members",
            ));
        }
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
            if previous_path
                .is_some_and(|previous| previous >= member.path.as_str())
            {
                return Err(PackageError::new(
                    "members must be strictly sorted by canonical path",
                ));
            }
            previous_path = Some(&member.path);
            let identity = validate_member(member)?;
            if !portable_identities.insert(identity) {
                return Err(PackageError::new(
                    "member paths collide after portable normalization",
                ));
            }
            total_bytes =
                total_bytes.checked_add(member.bytes).ok_or_else(|| {
                    PackageError::new("package byte count overflow")
                })?;
            if total_bytes > MAX_PACKAGE_BYTES {
                return Err(PackageError::new(
                    "package exceeds total byte limit",
                ));
            }
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
            if author.trim() != author
                || author.is_empty()
                || author.len() > 160
            {
                return Err(PackageError::new(
                    "invalid provenance author label",
                ));
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

fn ordered_candidate_packages(
    packages: &[PackageManifest],
) -> Result<Vec<&PackageManifest>, PackageError> {
    if packages.len() > MAX_LIST_ITEMS {
        return Err(PackageError::new(
            "candidate package count exceeds contract limit",
        ));
    }
    let mut ordered = packages.iter().collect::<Vec<_>>();
    ordered.sort_by(|left, right| {
        left.canonical_id
            .cmp(&right.canonical_id)
            .then_with(|| left.package_revision.cmp(&right.package_revision))
    });
    if ordered.windows(2).any(|pair| {
        pair.first().is_some_and(|left| {
            pair.get(1)
                .is_some_and(|right| left.canonical_id == right.canonical_id)
        })
    }) {
        return Err(PackageError::new(
            "candidate packages contain duplicate canonical identities",
        ));
    }
    for package in &ordered {
        package.validate()?;
    }
    Ok(ordered)
}

/// Rejects direct conflicts inside one simultaneously active candidate set.
///
/// A conflict declaration names another canonical package identity. A conflict
/// with a package that is not active is inert; a conflict with any active
/// candidate fails deterministically regardless of discovery order.
///
/// # Errors
/// Returns a deterministic failure for invalid declarations, duplicate package
/// identities, or a declared conflict with another active package.
pub fn validate_active_conflicts(
    packages: &[PackageManifest],
) -> Result<(), PackageError> {
    let ordered = ordered_candidate_packages(packages)?;
    let active_ids = ordered
        .iter()
        .map(|package| package.canonical_id.as_str())
        .collect::<BTreeSet<_>>();
    for package in ordered {
        for conflict in &package.conflicts {
            if active_ids.contains(conflict.as_str()) {
                return Err(PackageError::new(format!(
                    "active package conflict between {} and {}",
                    package.canonical_id, conflict
                )));
            }
        }
    }
    Ok(())
}

/// Resolves exact package dependencies into one discovery-order-independent
/// load order.
///
/// This helper orders only dependency precedence. Explicit priority,
/// supersession, conflicts, capabilities, and activation policy remain separate
/// admission concerns.
///
/// # Errors
/// Returns a deterministic failure for invalid declarations, duplicate package
/// identities, missing or mismatched exact dependencies, or dependency cycles.
pub fn dependency_load_order(
    packages: &[PackageManifest],
) -> Result<Vec<String>, PackageError> {
    let ordered = ordered_candidate_packages(packages)?;

    let by_id = ordered
        .iter()
        .map(|package| (package.canonical_id.as_str(), *package))
        .collect::<BTreeMap<_, _>>();
    let mut dependency_count = ordered
        .iter()
        .map(|package| {
            (package.canonical_id.clone(), package.dependencies.len())
        })
        .collect::<BTreeMap<_, _>>();
    let mut dependents = BTreeMap::<&str, Vec<&str>>::new();
    for package in &ordered {
        for dependency in &package.dependencies {
            let dependency_id = dependency.canonical_id.as_str();
            let Some(required) = by_id.get(dependency_id) else {
                return Err(PackageError::new(format!(
                    "missing dependency {} for {}",
                    dependency.canonical_id, package.canonical_id
                )));
            };
            if required.package_revision != dependency.revision {
                return Err(PackageError::new(format!(
                    "dependency revision mismatch for {} required by {}",
                    dependency.canonical_id, package.canonical_id
                )));
            }
            dependents
                .entry(dependency.canonical_id.as_str())
                .or_default()
                .push(package.canonical_id.as_str());
        }
    }
    for values in dependents.values_mut() {
        values.sort_unstable();
    }

    let mut ready = BTreeSet::new();
    for package in &ordered {
        if dependency_count.get(&package.canonical_id).copied() == Some(0)
            && !ready.insert(package.canonical_id.as_str())
        {
            return Err(PackageError::new(
                "dependency ready queue contains a duplicate package",
            ));
        }
    }
    let mut result = Vec::with_capacity(ordered.len());
    while let Some(next) = ready.pop_first() {
        result.push(next.to_owned());
        for dependent in dependents.get(next).into_iter().flatten() {
            let Some(count) = dependency_count.get_mut(*dependent) else {
                return Err(PackageError::new(
                    "dependency graph lost a candidate package",
                ));
            };
            let Some(updated) = count.checked_sub(1) else {
                return Err(PackageError::new(
                    "dependency graph count underflow",
                ));
            };
            *count = updated;
            if updated == 0 && !ready.insert(dependent) {
                return Err(PackageError::new(
                    "dependency package became ready more than once",
                ));
            }
        }
    }
    if result.len() != ordered.len() {
        return Err(PackageError::new(
            "candidate package dependency graph contains a cycle",
        ));
    }
    Ok(result)
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
    let byte_count = u64::try_from(bytes.len()).map_err(|_error| {
        PackageError::new("member byte length does not fit u64")
    })?;
    let member = Member {
        path: path.to_owned(),
        bytes: byte_count,
        sha256: shar_sha256::digest_hex(bytes),
        media_type: media_type.to_owned(),
        role: role.to_owned(),
    };
    let _identity = validate_member(&member)?;
    Ok(member)
}

fn update_length_prefixed(
    state: &mut Sha256,
    value: &str,
    label: &str,
) -> Result<(), PackageError> {
    let bytes = value.as_bytes();
    let length = u64::try_from(bytes.len()).map_err(|_error| {
        PackageError::new(format!("{label} length does not fit u64"))
    })?;
    state.update(&length.to_be_bytes());
    state.update(bytes);
    Ok(())
}

/// Derives one deterministic lowercase revision from an already-sorted member
/// set.
///
/// # Errors
/// Returns a deterministic failure when members are noncanonical.
pub fn content_revision(members: &[Member]) -> Result<String, PackageError> {
    if members.is_empty() {
        return Err(PackageError::new(
            "cannot derive revision from empty members",
        ));
    }
    if members.len() > MAX_MEMBERS {
        return Err(PackageError::new(
            "revision member count exceeds contract limit",
        ));
    }
    let mut state = Sha256::new();
    let mut previous_path: Option<&str> = None;
    let mut identities = BTreeSet::new();
    let mut total_bytes = 0_u64;
    for member in members {
        if previous_path
            .is_some_and(|previous| previous >= member.path.as_str())
        {
            return Err(PackageError::new(
                "revision members must be strictly sorted by canonical path",
            ));
        }
        previous_path = Some(&member.path);
        let identity = validate_member(member)?;
        if !identities.insert(identity) {
            return Err(PackageError::new(
                "revision member paths collide after portable normalization",
            ));
        }
        total_bytes =
            total_bytes.checked_add(member.bytes).ok_or_else(|| {
                PackageError::new("revision package byte count overflow")
            })?;
        if total_bytes > MAX_PACKAGE_BYTES {
            return Err(PackageError::new(
                "revision package exceeds total byte limit",
            ));
        }
        update_length_prefixed(&mut state, &member.path, "member path")?;
        state.update(&member.bytes.to_be_bytes());
        state.update(member.sha256.as_bytes());
        update_length_prefixed(
            &mut state,
            &member.media_type,
            "member media type",
        )?;
        update_length_prefixed(&mut state, &member.role, "member role")?;
    }
    Ok(state.finalize_hex())
}
