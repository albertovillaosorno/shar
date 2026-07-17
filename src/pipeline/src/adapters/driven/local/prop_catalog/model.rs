// File:
//   - model.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/model.rs
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
//   - Typed mission prop candidate, source, and exported-asset records.
// - Must-Not:
//   - Read files, serialize FBX, or publish directories.
// - Allows:
//   - Stable ordering, family labels, route labels, and report aggregation.
// - Split-When:
//   - Candidate and published-record lifecycles diverge independently.
// - Merge-When:
//   - Another prop-catalog module owns the same data without distinct policy.
// - Summary:
//   - Keeps mission prop export records explicit.
// - Description:
//   - Represents model-only sources and justified static or rigid-animation
//     routes.
// - Usage:
//   - Shared by prop discovery, preparation, catalog rendering, and
//     publication.
// - Defaults:
//   - Ordering is family, package, owner, and container identity.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md
//
// Large file:
//   - false
//

//! Typed mission prop catalog records.

use std::cmp::Ordering;
use std::path::PathBuf;

use fbx::adapters::driven::binary_character_writer::CharacterBinaryFbxSummary;

/// Catalog family that owns one mission source occurrence.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum PropFamily {
    /// Mission-specific models, including race flags and finish-line geometry.
    Missions,
}

impl PropFamily {
    /// Stable directory and JSON label.
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Missions => "missions",
        }
    }
}

/// FBX representation justified by normalized model evidence.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum PropRoute {
    /// Meshes connect directly to the FBX export root with no synthetic rig.
    Static,
    /// Rigid meshes retain their authored skeleton and exact PTRN clip.
    RigidAnimated,
}

impl PropRoute {
    /// Stable JSON route label.
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Static => "static-model",
            Self::RigidAnimated => "rigid-animated-model",
        }
    }
}

/// One normalized source occurrence before semantic deduplication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PropCandidate {
    /// Mission catalog family.
    pub(super) family: PropFamily,
    /// Generated package-index identity.
    pub(super) package_id: String,
    /// Generated package subcategory.
    pub(super) subcategory: String,
    /// Package root relative to the normalized staging directory.
    pub(super) relative_root: PathBuf,
    /// Top-level container kind or direct component family.
    pub(super) owner_kind: String,
    /// Human-readable source owner identity.
    pub(super) owner_name: String,
    /// Stable container key inside the package.
    pub(super) container_key: String,
    /// Selected decoded mesh member ids without family or extension.
    pub(super) mesh_ids: Vec<String>,
    /// Composite member id for one rigid animated route.
    pub(super) composite_id: Option<String>,
    /// Skeleton member id for one rigid animated route.
    pub(super) skeleton_id: Option<String>,
    /// Exact PTRN animation member id for one rigid animated route.
    pub(super) animation_id: Option<String>,
    /// Justified static or rigid-animation representation.
    pub(super) route: PropRoute,
}

impl Ord for PropCandidate {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        (
            self.family,
            &self.package_id,
            &self.owner_name,
            &self.container_key,
        )
            .cmp(
                &(
                    other.family,
                    &other.package_id,
                    &other.owner_name,
                    &other.container_key,
                ),
            )
    }
}

impl PartialOrd for PropCandidate {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// One source occurrence retained as provenance for a deduplicated asset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PropAlias {
    /// Source package id.
    pub(super) package_id: String,
    /// Source package subcategory.
    pub(super) subcategory: String,
    /// Top-level container kind.
    pub(super) owner_kind: String,
    /// Human-readable source owner identity.
    pub(super) owner_name: String,
    /// Stable source container key.
    pub(super) container_key: String,
}

impl From<&PropCandidate> for PropAlias {
    fn from(candidate: &PropCandidate) -> Self {
        Self {
            package_id: candidate
                .package_id
                .clone(),
            subcategory: candidate
                .subcategory
                .clone(),
            owner_kind: candidate
                .owner_kind
                .clone(),
            owner_name: candidate
                .owner_name
                .clone(),
            container_key: candidate
                .container_key
                .clone(),
        }
    }
}

/// One referenced external texture written beside an FBX.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct TextureRecord {
    /// Portable file name under the asset texture directory.
    pub(super) file_name: String,
    /// Exact byte count.
    pub(super) bytes: u64,
    /// Lowercase SHA-256.
    pub(super) sha256: String,
}

/// One deduplicated and published model asset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ExportedProp {
    /// Stable content-derived asset id.
    pub(super) asset_id: String,
    /// Mission catalog family.
    pub(super) family: PropFamily,
    /// Static or rigid-animation representation.
    pub(super) route: PropRoute,
    /// Semantic model signature used for deduplication.
    pub(super) signature: String,
    /// Relative FBX path from catalog root.
    pub(super) fbx_path: String,
    /// FBX byte count.
    pub(super) fbx_bytes: u64,
    /// FBX SHA-256.
    pub(super) fbx_sha256: String,
    /// Binary object-family summary.
    pub(super) summary: CharacterBinaryFbxSummary,
    /// Referenced texture records in file-name order.
    pub(super) textures: Vec<TextureRecord>,
    /// Canonical source and all duplicate occurrences.
    pub(super) aliases: Vec<PropAlias>,
}

/// Aggregate counters emitted by one complete batch.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct PropCatalogCounts {
    /// Source packages re-extracted from the game tree.
    pub(super) source_packages: usize,
    /// Model-bearing occurrences before deduplication.
    pub(super) occurrences: usize,
    /// Unique FBX assets after semantic deduplication.
    pub(super) assets: usize,
    /// Unique mission FBX assets.
    pub(super) mission_assets: usize,
    /// Static unique assets.
    pub(super) static_assets: usize,
    /// Rigid animated unique assets.
    pub(super) animated_assets: usize,
}
