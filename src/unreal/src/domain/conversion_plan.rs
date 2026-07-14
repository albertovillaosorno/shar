// File:
//   - conversion_plan.rs
// Path:
//   - src/unreal/src/domain/conversion_plan.rs
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
//   - Accepted conversion formats and deterministic native target plan records.
// - Must-Not:
//   - Read source files, invoke Unreal Engine, encode MCP calls, or select an
//   - IO adapter.
// - Allows:
//   - Typed conversion evidence, target-family selection, dependencies, and
//   - object identities supplied by pipeline orchestration.
// - Split-When:
//   - One source family gains independently testable planning invariants.
// - Merge-When:
//   - Another module owns the same source-format and target-plan contract.
// - Summary:
//   - Native Unreal asset conversion plan records.
// - Description:
//   - Represents normalized JSON, WAV, HAP, and FBX evidence without editor
//   - transport or live project behavior.
// - Usage:
//   - Constructed by pipeline orchestration before a separate terminal client
//   - applies the plan through native Unreal tools.
// - Defaults:
//   - No format, target, destination, dependency, or editor action is implicit.
//
// ADRs:
// - docs/adr/unreal/architecture.md
// - docs/adr/pipeline/eleven-phase-remake-delivery-roadmap.md
//
// Large file:
//   - false
//

//! Typed records for the four accepted Unreal conversion source families.
//!
//! Pipeline owns validation and construction until Phase 6 implementation.

/// Accepted normalized source format.
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
