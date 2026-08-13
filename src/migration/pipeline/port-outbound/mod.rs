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
//   - Port outbound outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Port outbound outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Port outbound outbound port.

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
    fn run(&self, config: &PipelineConfig) -> PipelineOutcome<PipelineReport>;

    /// Exports only movie packages.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_movies(
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

    /// Audits, indexes, and prepares canonical Unreal staging output.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn prepare_unreal(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport>;

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

    /// Exports every current direct package-level FBX into catalog v2.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_complete_fbx_catalog(
        &self,
        index_path: &Path,
        output_dir: &Path,
        manifest_path: &Path,
        base_root: &Path,
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

    /// Exports one canonical one-mesh Unreal structural guide.
    ///
    /// # Errors
    ///
    /// Returns a validated pipeline failure.
    fn export_structural_guide(
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
