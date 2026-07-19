// File:
//   - movement_model.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     movement_model.rs
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
//   - Typed evidence produced by one package coordinate movement.
// - Must-Not:
//   - Read source files, transform coordinates, or serialize catalogs.
// - Allows:
//   - Carry movement matrices, subject coverage, and before/after records.
// - Summary:
//   - World coordinate movement publication model.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Typed movement evidence shared by world export and catalog rendering.

use crate::domain::coordinate_movement::CoordinateMatrix;

/// One package-level movement and all observable coordinate evidence.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct WorldCoordinateMovementRecord {
    /// Stable movement identity.
    pub(super) id: String,
    /// Package whose records are moved.
    pub(super) package_id: String,
    /// Source-space row-vector affine matrix.
    pub(super) matrix: CoordinateMatrix,
    /// Complete coordinate-family contract, including future runtime records.
    pub(super) subjects: Vec<String>,
    /// Render meshes transformed before FBX publication.
    pub(super) moved_render_meshes: usize,
    /// Collision meshes transformed before exclusion or native conversion.
    pub(super) moved_collision_meshes: usize,
    /// Aggregate render bound before movement.
    pub(super) source_render_bounds: Option<(
        [f32; 3],
        [f32; 3],
    )>,
    /// Aggregate render bound after movement.
    pub(super) moved_render_bounds: Option<(
        [f32; 3],
        [f32; 3],
    )>,
    /// Locator, trigger, camera, light, and physics coordinate evidence.
    pub(super) records: Vec<WorldMovedCoordinateRecord>,
}

/// One transformed source record with optional point, direction, or placement.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct WorldMovedCoordinateRecord {
    /// Stable source-relative component path.
    pub(super) source_path: String,
    /// Decoded source identity without fixed-width padding.
    pub(super) identity: String,
    /// Coordinate subject represented by this record.
    pub(super) subject: String,
    /// Optional source position.
    pub(super) source_position: Option<[f32; 3]>,
    /// Optional transformed position.
    pub(super) moved_position: Option<[f32; 3]>,
    /// Optional source direction or basis vector.
    pub(super) source_direction: Option<[f32; 3]>,
    /// Optional transformed direction or basis vector.
    pub(super) moved_direction: Option<[f32; 3]>,
    /// Optional source placement matrix.
    pub(super) source_matrix: Option<CoordinateMatrix>,
    /// Optional transformed placement matrix.
    pub(super) moved_matrix: Option<CoordinateMatrix>,
}

impl WorldMovedCoordinateRecord {
    /// Construct one point record.
    pub(super) fn point(
        source_path: String,
        identity: String,
        subject: &str,
        source: [f32; 3],
        moved: [f32; 3],
    ) -> Self {
        Self {
            source_path,
            identity,
            subject: subject.to_owned(),
            source_position: Some(source),
            moved_position: Some(moved),
            source_direction: None,
            moved_direction: None,
            source_matrix: None,
            moved_matrix: None,
        }
    }

    /// Construct one direction record.
    pub(super) fn direction(
        source_path: String,
        identity: String,
        subject: &str,
        source: [f32; 3],
        moved: [f32; 3],
    ) -> Self {
        Self {
            source_path,
            identity,
            subject: subject.to_owned(),
            source_position: None,
            moved_position: None,
            source_direction: Some(source),
            moved_direction: Some(moved),
            source_matrix: None,
            moved_matrix: None,
        }
    }

    /// Construct one placement-matrix record.
    pub(super) fn placement(
        source_path: String,
        identity: String,
        subject: &str,
        source: CoordinateMatrix,
        moved: CoordinateMatrix,
    ) -> Self {
        Self {
            source_path,
            identity,
            subject: subject.to_owned(),
            source_position: None,
            moved_position: None,
            source_direction: None,
            moved_direction: None,
            source_matrix: Some(source),
            moved_matrix: Some(moved),
        }
    }
}
