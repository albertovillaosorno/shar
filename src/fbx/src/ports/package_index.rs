// File:
//   - package_index.rs
// Path:
//   - src/fbx/src/ports/package_index.rs
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
//   - The fbx port contract for ports package index.
// - Must-Not:
//   - Contain concrete filesystem, JSON, Blender, or serialization
//   - implementations.
// - Allows:
//   - Trait and DTO definitions that keep adapters replaceable.
// - Split-When:
//   - Split when package index contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same ports boundary with no distinct
//   - invariant.
// - Summary:
//   - Model families that are eligible for FBX scene translation.
// - Description:
//   - Defines package index data and behavior for fbx ports.
// - Usage:
//   - Implemented by adapters and consumed by application use cases.
// - Defaults:
//   - No default implementation is provided by the port contract.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Model families that are eligible for FBX scene translation.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use std::collections::BTreeSet;

use crate::domain::scene::identity::is_portable_path_segment;

/// Package model family represented by generated index evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageModelFamily {
    /// Static or animated prop package.
    Prop,
    /// Vehicle model package.
    Vehicle,
    /// Character or costume package.
    Character,
    /// Terrain/world-piece package represented as mesh evidence.
    Terrain,
}

/// Stable model package evidence resolved from the generated package index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelPackageEvidence {
    /// Stable package id from the generated package index.
    pub package_id: String,
    /// Family selected only from generated package evidence.
    pub family: PackageModelFamily,
    /// Model member ids required to build scene geometry.
    pub model_member_ids: Vec<String>,
    /// Material member ids required to bind surfaces.
    pub material_member_ids: Vec<String>,
    /// Texture member ids required to bind material maps.
    pub texture_member_ids: Vec<String>,
    /// Animation member ids preserved for clip export when present.
    pub animation_member_ids: Vec<String>,
}

impl ModelPackageEvidence {
    /// Build model package evidence after the adapter has read the index row.
    ///
    /// # Errors
    ///
    /// Returns an error when the index row has no model members.
    pub fn new(
        package_id: impl Into<String>,
        family: PackageModelFamily,
        mut model_member_ids: Vec<String>,
        mut material_member_ids: Vec<String>,
        mut texture_member_ids: Vec<String>,
        mut animation_member_ids: Vec<String>,
    ) -> Result<Self, PackageIndexError> {
        let stable_package_id = package_id.into();
        if stable_package_id
            .trim()
            .is_empty()
        {
            return Err(PackageIndexError::MissingPackageId);
        }
        if model_member_ids.is_empty() {
            return Err(PackageIndexError::MissingModelMembers);
        }
        let has_blank_member = model_member_ids
            .iter()
            .chain(&material_member_ids)
            .chain(&texture_member_ids)
            .chain(&animation_member_ids)
            .any(
                |member_id| {
                    member_id
                        .trim()
                        .is_empty()
                },
            );
        if has_blank_member {
            return Err(PackageIndexError::BlankMemberId);
        }
        if stable_package_id != stable_package_id.trim()
            || stable_package_id
                .chars()
                .any(char::is_control)
            || model_member_ids
                .iter()
                .chain(&material_member_ids)
                .chain(&texture_member_ids)
                .chain(&animation_member_ids)
                .any(|member_id| !is_portable_path_segment(member_id))
        {
            return Err(PackageIndexError::NonCanonicalIdentity);
        }
        let mut unique_member_ids = BTreeSet::new();
        for member_id in model_member_ids
            .iter()
            .chain(&material_member_ids)
            .chain(&texture_member_ids)
            .chain(&animation_member_ids)
        {
            if !unique_member_ids.insert(member_id.to_ascii_lowercase()) {
                return Err(PackageIndexError::DuplicateMemberId);
            }
        }
        model_member_ids.sort();
        material_member_ids.sort();
        texture_member_ids.sort();
        animation_member_ids.sort();
        Ok(
            Self {
                package_id: stable_package_id,
                family,
                model_member_ids,
                material_member_ids,
                texture_member_ids,
                animation_member_ids,
            },
        )
    }
}

/// Package-index reader failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageIndexError {
    /// Package evidence requires a stable non-blank package id.
    MissingPackageId,
    /// Package or member identity contained surrounding whitespace.
    NonCanonicalIdentity,
    /// A package member list contained an empty identity.
    BlankMemberId,
    /// A package member identity appeared more than once across role lists.
    DuplicateMemberId,
    /// FBX model exports require at least one model member.
    MissingModelMembers,
}

/// Port that resolves phase-three package ids into generated package evidence.
pub trait PackageIndexReader {
    /// Adapter-specific index error type.
    type Error;

    /// Resolve one stable package id from the generated phase-three index.
    ///
    /// # Errors
    ///
    /// Returns an adapter-specific error when the package id is missing, the
    /// row is not model-like, or required model evidence is absent.
    fn require_model_package(
        &self,
        package_id: &str,
    ) -> Result<ModelPackageEvidence, Self::Error>;
}
