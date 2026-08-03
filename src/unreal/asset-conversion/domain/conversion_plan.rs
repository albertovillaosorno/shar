// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Conversion plan domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Conversion plan domain module.
// - Description:
//   - Implements the declared responsibility for asset conversion.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Conversion plan domain module.

/// Normalized source representation accepted by Unreal conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceFormat {
    /// Structured normalized records.
    Json,
    /// Pulse-code modulation audio.
    Wav,
    /// HAP-encoded video package evidence.
    Hap,
    /// Canonical binary FBX 7.7 model or animation evidence.
    Fbx,
}

/// Broad native Unreal target family selected before editor application.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeAssetFamily {
    /// Data Table, Data Asset, String Table, or purpose-built structured asset.
    StructuredData,
    /// Sound Wave and related native audio metadata.
    Audio,
    /// Media Source, media playback, and synchronized audio assets.
    Media,
    /// Static Mesh, Skeletal Mesh, Skeleton, animation, material, or camera.
    Model,
}

/// Deterministic conversion plan consumed by pipeline orchestration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConversionPlan {
    /// Opaque source or package identity.
    pub source_identity: String,
    /// Accepted normalized source format.
    pub source_format: SourceFormat,
    /// Broad native target family.
    pub target_family: NativeAssetFamily,
    /// Deterministic Unreal object path.
    pub destination: String,
    /// Ordered opaque identities required before this plan is applied.
    pub dependencies: Vec<String>,
    /// Public-safe provenance artifact identity.
    pub provenance: String,
}
