// File:
//   - lib.rs
// Path:
//   - src/fbx/src/lib.rs
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
//   - The fbx public library facade for lib.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute public crate facade.
// - Split-When:
//   - Split when public crate facade contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another fbx module owns the same library facade boundary with no distinct
//   - invariant.
// - Summary:
//   - The crate keeps scene semantics in Rust and exposes adapter boundaries
//   - for.
// - Description:
//   - Defines public crate facade data and behavior for fbx root.
// - Usage:
//   - Imported by workspace crates through the public library surface.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Hexagonal FBX scene export for the Unreal rebuild.
//!
//! The crate keeps scene semantics in Rust and exposes adapter boundaries for
//! serializers and validators so props, terrain, vehicles, and characters can
//! share one model-export engine.
/// Inbound and outbound adapters for serialization, validation, and IO.
pub mod adapters;
/// Use cases that assemble domain scenes and invoke ports.
pub mod application;
/// Package-independent scene, mesh, material, and texture model.
#[path = "domain/domain.rs"]
pub mod domain;
/// Hexagonal ports used by future readers, writers, and validators.
pub mod ports;
