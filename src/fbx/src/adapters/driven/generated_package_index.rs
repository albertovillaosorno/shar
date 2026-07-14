// File:
//   - generated_package_index.rs
// Path:
//   - src/fbx/src/adapters/driven/generated_package_index.rs
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
//   - The fbx adapter boundary for adapters driven generated package index.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when generated package index contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another fbx module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Generated package-index catalog adapter.
// - Description:
//   - Defines generated package index data and behavior for fbx adapters
//   - driven.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - true
//   - Reason: src/fbx/src/adapters/driven/generated_package_index.rs has 603
//   - effective lines after the required header and remains cohesive until a
//   - focused split lands.
//

//! Generated package-index catalog adapter.
//!
//! This boundary keeps generated package-index catalog adapter explicit and
//! returns deterministic results to fbx callers.
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;
use serde::Deserialize;

use crate::domain::scene::identity::is_portable_path_segment;
use crate::ports::package_index::{
    ModelPackageEvidence, PackageIndexError, PackageIndexReader,
    PackageModelFamily,
};

/// Generated package-index catalog adapter.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GeneratedPackageCatalog {
    /// Package rows keyed by their stable generated package identifier.
    rows: BTreeMap<String, IndexedPackageRow>,
}

impl GeneratedPackageCatalog {
    /// Read generated package-index JSONL from a caller-supplied path.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read or one JSONL row is not a
    /// supported package-index record.
    pub fn read_jsonl(path: &Path) -> Result<Self, PackageIndexAdapterError> {
        let text = local::read_utf8(path).map_err(
            |source| PackageIndexAdapterError::Read {
                path: path
                    .display()
                    .to_string(),
                source: source.to_string(),
            },
        )?;
        Self::from_jsonl(&text)
    }

    /// Build a catalog from generated package-index JSONL text.
    ///
    /// # Errors
    ///
    /// Returns an error when a row cannot be decoded or duplicate package ids
    /// make catalog lookup ambiguous.
    pub fn from_jsonl(text: &str) -> Result<Self, PackageIndexAdapterError> {
        let jsonl_text = text
            .strip_prefix('\u{feff}')
            .unwrap_or(text);
        let mut rows = BTreeMap::new();
        for (line_index, line) in jsonl_text
            .lines()
            .enumerate()
        {
            if line
                .trim()
                .is_empty()
            {
                continue;
            }
            let row: IndexedPackageRow = serde_json::from_str(line).map_err(
                |source| PackageIndexAdapterError::Parse {
                    line: line_index.saturating_add(1),
                    source: source.to_string(),
                },
            )?;
            let line_number = line_index.saturating_add(1);
            validate_indexed_row(
                &row,
                line_number,
            )?;
            if rows
                .insert(
                    row.package_id
                        .clone(),
                    row,
                )
                .is_some()
            {
                return Err(
                    PackageIndexAdapterError::DuplicatePackageId {
                        line: line_index.saturating_add(1),
                    },
                );
            }
        }
        if rows.is_empty() {
            return Err(PackageIndexAdapterError::EmptyCatalog);
        }
        Ok(
            Self {
                rows,
            },
        )
    }
}

/// Reject non-empty generated-index fields with surrounding whitespace.
fn validate_canonical_whitespace(
    row: &IndexedPackageRow,
    line: usize,
) -> Result<(), PackageIndexAdapterError> {
    for (field, value) in [
        (
            "package_id",
            row.package_id
                .as_str(),
        ),
        (
            "package_category",
            row.package_category
                .as_str(),
        ),
    ] {
        if value != value.trim()
            || value
                .chars()
                .any(char::is_control)
        {
            return Err(
                PackageIndexAdapterError::NonCanonicalWhitespace {
                    line,
                    field: field.to_owned(),
                },
            );
        }
    }
    for member in &row.members {
        for (field, value) in [
            (
                "member.id",
                member
                    .id
                    .as_str(),
            ),
            (
                "member.role",
                member
                    .role
                    .as_str(),
            ),
        ] {
            if value != value.trim()
                || value
                    .chars()
                    .any(char::is_control)
            {
                return Err(
                    PackageIndexAdapterError::NonCanonicalWhitespace {
                        line,
                        field: field.to_owned(),
                    },
                );
            }
        }
    }
    Ok(())
}

/// Validate generated member identities and canonical roles.
fn validate_members(
    row: &IndexedPackageRow,
    line: usize,
) -> Result<(), PackageIndexAdapterError> {
    if row
        .members
        .iter()
        .any(
            |member| {
                member
                    .id
                    .trim()
                    .is_empty()
            },
        )
    {
        return Err(
            PackageIndexAdapterError::BlankMemberId {
                line,
            },
        );
    }
    if row
        .members
        .iter()
        .any(
            |member| {
                member
                    .role
                    .trim()
                    .is_empty()
            },
        )
    {
        return Err(
            PackageIndexAdapterError::BlankMemberRole {
                line,
            },
        );
    }
    validate_canonical_whitespace(
        row, line,
    )?;
    if let Some(member) = row
        .members
        .iter()
        .find(|member| !is_safe_member_id(&member.id))
    {
        return Err(
            PackageIndexAdapterError::InvalidMemberId {
                line,
                id: member
                    .id
                    .clone(),
            },
        );
    }
    if let Some(member) = row
        .members
        .iter()
        .find(|member| !is_canonical_member_role(&member.role))
    {
        return Err(
            PackageIndexAdapterError::UnknownMemberRole {
                line,
                role: member
                    .role
                    .clone(),
            },
        );
    }
    let mut member_ids = BTreeSet::new();
    for member in &row.members {
        if !member_ids.insert(
            member
                .id
                .as_str(),
        ) {
            return Err(
                PackageIndexAdapterError::DuplicateMemberId {
                    line,
                    id: member
                        .id
                        .clone(),
                },
            );
        }
    }
    Ok(())
}

/// Validate one decoded generated-index row before catalog insertion.
fn validate_indexed_row(
    row: &IndexedPackageRow,
    line: usize,
) -> Result<(), PackageIndexAdapterError> {
    if row
        .package_id
        .trim()
        .is_empty()
    {
        return Err(
            PackageIndexAdapterError::BlankPackageId {
                line,
            },
        );
    }
    if row
        .package_category
        .trim()
        .is_empty()
    {
        return Err(
            PackageIndexAdapterError::BlankPackageCategory {
                line,
            },
        );
    }
    validate_members(
        row, line,
    )?;
    Ok(())
}

impl PackageIndexReader for GeneratedPackageCatalog {
    type Error = PackageIndexAdapterError;

    /// Internal helper for the adapter implementation.
    fn require_model_package(
        &self,
        package_id: &str,
    ) -> Result<ModelPackageEvidence, Self::Error> {
        if package_id.is_empty()
            || package_id != package_id.trim()
            || package_id
                .chars()
                .any(char::is_control)
        {
            return Err(
                PackageIndexAdapterError::InvalidPackageSelector(
                    package_id.to_owned(),
                ),
            );
        }
        let row = self
            .rows
            .get(package_id)
            .ok_or_else(
                || {
                    PackageIndexAdapterError::MissingPackage(
                        package_id.to_owned(),
                    )
                },
            )?;
        let family = model_family(row)?;
        let members = classify_members(row);
        ModelPackageEvidence::new(
            row.package_id
                .clone(),
            family,
            members.models,
            members.materials,
            members.textures,
            members.animations,
        )
        .map_err(PackageIndexAdapterError::Package)
    }
}

/// Internal helper for the adapter implementation.
fn model_family(
    row: &IndexedPackageRow
) -> Result<PackageModelFamily, PackageIndexAdapterError> {
    match row
        .package_category
        .as_str()
    {
        "props" => Ok(PackageModelFamily::Prop),
        "cars" | "ui-vehicle-previews" => Ok(PackageModelFamily::Vehicle),
        "characters" => Ok(PackageModelFamily::Character),
        "terrain-world" => Ok(PackageModelFamily::Terrain),
        category => Err(
            PackageIndexAdapterError::NotFbxEligible {
                package_id: row
                    .package_id
                    .clone(),
                category: category.to_owned(),
            },
        ),
    }
}

/// Return whether one member id is portable for component-file lookup.
fn is_safe_member_id(value: &str) -> bool {
    is_portable_path_segment(value)
}

/// Return whether one role belongs to the canonical package taxonomy.
fn is_canonical_member_role(role: &str) -> bool {
    matches!(
        role,
        "world"
            | "texture"
            | "material"
            | "model"
            | "physics"
            | "animation"
            | "scene"
            | "locator"
            | "camera"
            | "light"
            | "particle"
            | "controller"
            | "audio"
            | "movie"
            | "script"
            | "text"
            | "ui"
            | "metadata"
            | "error"
    )
}

/// Internal helper for the adapter implementation.
fn classify_members(row: &IndexedPackageRow) -> MemberBuckets {
    let mut buckets = MemberBuckets::default();
    for member in &row.members {
        match member
            .role
            .as_str()
        {
            "model" => buckets
                .models
                .push(
                    member
                        .id
                        .clone(),
                ),
            "material" => buckets
                .materials
                .push(
                    member
                        .id
                        .clone(),
                ),
            "texture" => buckets
                .textures
                .push(
                    member
                        .id
                        .clone(),
                ),
            "animation" => buckets
                .animations
                .push(
                    member
                        .id
                        .clone(),
                ),
            _ => {}
        }
    }
    buckets
        .models
        .sort();
    buckets
        .materials
        .sort();
    buckets
        .textures
        .sort();
    buckets
        .animations
        .sort();
    buckets
}

#[derive(Default)]
/// Internal data shape for the adapter implementation.
struct MemberBuckets {
    /// Model member ids.
    models: Vec<String>,
    /// Material member ids.
    materials: Vec<String>,
    /// Texture member ids.
    textures: Vec<String>,
    /// Animation member ids.
    animations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct IndexedPackageRow {
    /// Stable package id from the generated index.
    package_id: String,
    /// Stable package category from the generated index.
    package_category: String,
    /// Members that belong to the package row.
    members: Vec<IndexedMember>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct IndexedMember {
    /// Stable member id.
    id: String,
    /// Package member role.
    role: String,
    #[serde(
        default,
        rename = "kind"
    )]
    /// Decoded member kind retained for schema compatibility.
    _kind: Option<String>,
    #[serde(
        default,
        rename = "source_chunk_kind"
    )]
    /// Source chunk kind retained for schema compatibility.
    _source_chunk_kind: Option<String>,
}

/// Generated package-index adapter error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PackageIndexAdapterError {
    /// Package-index JSONL file could not be read.
    Read {
        /// Package-index path.
        path: String,
        /// IO error text.
        source: String,
    },
    /// Package-index JSONL did not contain any package rows.
    EmptyCatalog,
    /// Package-index JSONL row could not be parsed.
    Parse {
        /// One-based line number.
        line: usize,
        /// JSON error text.
        source: String,
    },
    /// Package-index row contained surrounding identity whitespace.
    NonCanonicalWhitespace {
        /// One-based line number containing the padded value.
        line: usize,
        /// Stable field name containing surrounding whitespace.
        field: String,
    },
    /// Package-index row declared an empty package id.
    BlankPackageId {
        /// One-based line number containing the empty identity.
        line: usize,
    },
    /// Package-index row declared an empty package category.
    BlankPackageCategory {
        /// One-based line number containing the empty category.
        line: usize,
    },
    /// Package-index row contained a member identity unsafe for file lookup.
    InvalidMemberId {
        /// One-based line number containing the invalid identity.
        line: usize,
        /// Invalid member identity.
        id: String,
    },
    /// Package-index row contained an empty member identity.
    BlankMemberId {
        /// One-based line number containing the empty identity.
        line: usize,
    },
    /// Package-index row contained an empty member role.
    BlankMemberRole {
        /// One-based line number containing the empty role.
        line: usize,
    },
    /// Package-index row contained a role outside the canonical taxonomy.
    UnknownMemberRole {
        /// One-based line number containing the unknown role.
        line: usize,
        /// Unknown role text.
        role: String,
    },
    /// Package-index row reused one member identity.
    DuplicateMemberId {
        /// One-based line number containing the duplicate identity.
        line: usize,
        /// Repeated member identity.
        id: String,
    },
    /// Duplicate package ids would make deterministic lookup impossible.
    DuplicatePackageId {
        /// One-based line number where the duplicate was found.
        line: usize,
    },
    /// Requested package selector was empty or non-canonical.
    InvalidPackageSelector(String),
    /// Requested package was not present.
    MissingPackage(String),
    /// Requested package is not a model-like FBX package.
    NotFbxEligible {
        /// Stable package id.
        package_id: String,
        /// Stable package category.
        category: String,
    },
    /// Package row was eligible but lacked required model members.
    Package(PackageIndexError),
}
