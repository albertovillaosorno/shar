// File:
//   - binary_identity.rs
// Path:
//   - src/fbx/src/adapters/driven/binary_identity.rs
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
//   - Collision-free deterministic object ids inside one binary FBX document.
// - Must-Not:
//   - Select packages, validate package membership, or serialize FBX nodes.
// - Allows:
//   - Derive bounded geometry, material, bone, and cluster object ids.
// - Split-When:
//   - One object family gains a different identity lifecycle.
// - Merge-When:
//   - Animation and character objects no longer share identity ranges.
// - Summary:
//   - Binary FBX object identity allocator.
// - Description:
//   - Keeps serializer-local object ranges separate from phase-three package
//   - planning and from the binary container codec.
// - Usage:
//   - Consumed by the binary character and animation serializers.
// - Defaults:
//   - Rejects ordinals outside the collision-free documented ranges.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Deterministic object identity inside one binary FBX document.

/// Object-id base for geometry, mesh-model, and skin-deformer triples.
const GEOMETRY_ID_BASE: u64 = 2_000_000;
/// Object-id base for deduplicated material, texture, and video triples.
const MATERIAL_ID_BASE: u64 = 3_000_000;
/// Object-id base for limb-node model and node-attribute pairs.
const BONE_ID_BASE: u64 = 4_000_000;
/// Object-id base for one cluster per geometry-and-bone pair.
const CLUSTER_ID_BASE: u64 = 5_000_000_000;
/// Maximum group ordinal keeping every object family collision-free.
const MAX_GROUP_ORDINAL: usize = 99_999;
/// Maximum bone ordinal keeping every object family collision-free.
const MAX_BONE_ORDINAL: usize = 99_999;

/// Deterministic object ids for one geometry triple.
#[derive(Clone, Copy)]
pub(super) struct GeometryIds {
    /// Geometry object id.
    pub(super) geometry: u64,
    /// Mesh model object id.
    pub(super) model: u64,
    /// Skin deformer object id.
    pub(super) deformer: u64,
}

/// Deterministic object ids for one bone pair.
#[derive(Clone, Copy)]
pub(super) struct BoneIds {
    /// Limb-node model object id.
    pub(super) model: u64,
    /// Node-attribute object id.
    pub(super) attribute: u64,
}

/// Deterministic object ids for one material triple.
#[derive(Clone, Copy)]
pub(super) struct MaterialIds {
    /// Material object id.
    pub(super) material: u64,
    /// Texture object id.
    pub(super) texture: u64,
    /// Video object id.
    pub(super) video: u64,
}

/// Derive bounded deterministic ids for one geometry ordinal.
// Explicit bounds prove the following arithmetic remains collision-free.
#[expect(
    clippy::arithmetic_side_effects,
    reason = "The explicit maximum ordinal check proves every following \
              object-id operation remains within u64 bounds."
)]
pub(super) fn geometry_ids(
    ordinal: usize
) -> Result<GeometryIds, BinaryIdentityError> {
    let ordinal_value = group_ordinal(ordinal)?;
    let base = GEOMETRY_ID_BASE + ordinal_value * 10;
    Ok(
        GeometryIds {
            geometry: base + 1,
            model: base + 2,
            deformer: base + 3,
        },
    )
}

/// Derive bounded deterministic ids for one material ordinal.
// Explicit bounds prove the following arithmetic remains collision-free.
#[expect(
    clippy::arithmetic_side_effects,
    reason = "The explicit maximum ordinal check proves every following \
              object-id operation remains within u64 bounds."
)]
pub(super) fn material_ids(
    ordinal: usize
) -> Result<MaterialIds, BinaryIdentityError> {
    let ordinal_value = group_ordinal(ordinal)?;
    let base = MATERIAL_ID_BASE + ordinal_value * 10;
    Ok(
        MaterialIds {
            material: base + 1,
            texture: base + 2,
            video: base + 3,
        },
    )
}

/// Derive bounded deterministic ids for one bone ordinal.
// Explicit bounds prove the following arithmetic remains collision-free.
#[expect(
    clippy::arithmetic_side_effects,
    reason = "The explicit maximum ordinal check proves every following \
              object-id operation remains within u64 bounds."
)]
pub(super) fn bone_ids(ordinal: usize) -> Result<BoneIds, BinaryIdentityError> {
    let ordinal_value = bone_ordinal(ordinal)?;
    let base = BONE_ID_BASE + ordinal_value * 10;
    Ok(
        BoneIds {
            model: base + 1,
            attribute: base + 2,
        },
    )
}

/// Derive one bounded deterministic cluster id.
// Explicit bounds prove the following arithmetic remains collision-free.
#[expect(
    clippy::arithmetic_side_effects,
    reason = "The explicit maximum ordinal checks prove every following \
              object-id operation remains within u64 bounds."
)]
pub(super) fn cluster_id(
    group: usize,
    bone: usize,
) -> Result<u64, BinaryIdentityError> {
    let group_value = group_ordinal(group)?;
    let bone_value = bone_ordinal(bone)?;
    Ok(CLUSTER_ID_BASE + group_value * 1_000_000 + bone_value * 10)
}

/// Validate and narrow one group-family ordinal.
fn group_ordinal(ordinal: usize) -> Result<u64, BinaryIdentityError> {
    if ordinal > MAX_GROUP_ORDINAL {
        return Err(
            BinaryIdentityError::GroupOrdinalTooLarge {
                ordinal,
            },
        );
    }
    let Ok(value) = u64::try_from(ordinal) else {
        return Err(
            BinaryIdentityError::GroupOrdinalTooLarge {
                ordinal,
            },
        );
    };
    Ok(value)
}

/// Validate and narrow one bone-family ordinal.
fn bone_ordinal(ordinal: usize) -> Result<u64, BinaryIdentityError> {
    if ordinal > MAX_BONE_ORDINAL {
        return Err(
            BinaryIdentityError::BoneOrdinalTooLarge {
                ordinal,
            },
        );
    }
    let Ok(value) = u64::try_from(ordinal) else {
        return Err(
            BinaryIdentityError::BoneOrdinalTooLarge {
                ordinal,
            },
        );
    };
    Ok(value)
}

/// Binary FBX object identity failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum BinaryIdentityError {
    /// Group ordinal exceeded the deterministic id range.
    GroupOrdinalTooLarge {
        /// Rejected group ordinal.
        ordinal: usize,
    },
    /// Bone ordinal exceeded the deterministic id range.
    BoneOrdinalTooLarge {
        /// Rejected bone ordinal.
        ordinal: usize,
    },
}
