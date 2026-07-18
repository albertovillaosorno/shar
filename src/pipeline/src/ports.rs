// File:
//   - ports.rs
// Path:
//   - src/pipeline/src/ports.rs
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
//   - Outbound contracts required by pipeline application use cases.
// - Must-Not:
//   - Implement local storage or command presentation behavior.
// - Allows:
//   - Request process-neutral output inventory evidence.
// - Split-When:
//   - Split when another provider family gains an independent lifecycle.
// - Merge-When:
//   - Another facade owns the same pipeline outbound contracts.
// - Summary:
//   - Pipeline outbound ports.
// - Description:
//   - Keeps application use cases independent from concrete providers.
// - Usage:
//   - Implemented by driven adapters and consumed by application commands.
// - Defaults:
//   - Ports receive every root and selection explicitly.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Outbound ports for pipeline application use cases.
//!
//! Concrete filesystem and tool providers remain in driven adapters.
use std::path::Path;

use crate::domain::{
    OutputSummary, PhaseThreePackageSelector, PipelineConfig, PipelineOutcome,
    PipelineReport, StageReport,
};

/// Optional storage policy requested for one phase-three FBX export.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FbxExportOptions {
    /// Embed PNG payloads for explicit legacy compatibility.
    pub embed_textures: bool,
}

/// Supplies output inventory evidence for one generated root.
pub trait OutputInventory {
    /// Inventories one output root and caller-selected directories.
    ///
    /// # Errors
    ///
    /// Returns a pipeline failure when storage evidence cannot be inspected.
    fn summarize(
        &self,
        root: &Path,
        directories: &'static [&'static str],
    ) -> PipelineOutcome<OutputSummary>;
}

/// Executes validated pipeline workflows behind one explicit provider.
pub trait PipelineOperations {
    /// Runs the complete ordered extraction pipeline.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn run(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport>;

    /// Exports only movie packages.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_movies(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport>;

    /// Exports only LMLM packages.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_lmlm(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport>;

    /// Writes the phase-two minor-unit manifest.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn manifest_minor_units(
        &self,
        game_root: &Path,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Fills derived minor-unit metadata.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn fill_minor_unit_metadata(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Applies deterministic minor-unit metadata edits.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn edit_minor_unit_metadata(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Writes the minor-unit package index.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn index_minor_units(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Audits the minor-unit manifest and package evidence.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn audit_minor_units(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Writes one selected phase-three FBX manifest.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn write_fbx_manifest(
        &self,
        index_path: &Path,
        selector: &PhaseThreePackageSelector,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Exports every skinned character package as a verified FBX catalog.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_character_catalog(
        &self,
        index_path: &Path,
        output_dir: &Path,
        base_root: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Exports one canonical standalone Wasp Camera FBX artifact.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_wasp_camera(
        &self,
        index_path: &Path,
        output_dir: &Path,
        base_root: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Exports one canonical standalone Wrench model FBX artifact.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_wrench(
        &self,
        index_path: &Path,
        output_dir: &Path,
        base_root: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Exports the complete non-world card and mission prop catalog.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_prop_catalog(
        &self,
        index_path: &Path,
        game_root: &Path,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Exports the complete semantically separated vehicle FBX catalog.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_vehicle_catalog(
        &self,
        index_path: &Path,
        game_root: &Path,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Exports every terrain-world model prop under hash-free names.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_world_prop_catalog(
        &self,
        index_path: &Path,
        game_root: &Path,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Exports one separated static master-world FBX for all main game levels.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_world_master(
        &self,
        index_path: &Path,
        game_root: &Path,
        coordinate_root: &Path,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport>;

    /// Exports one selected phase-three package as an FBX artifact.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_fbx_package(
        &self,
        index_path: &Path,
        selector: &PhaseThreePackageSelector,
        output_dir: &Path,
        base_root: &Path,
        options: FbxExportOptions,
    ) -> PipelineOutcome<StageReport>;
}
