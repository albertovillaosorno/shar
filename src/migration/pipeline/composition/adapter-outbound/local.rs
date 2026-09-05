// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Local outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Local outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Local outbound adapter.

use std::path::Path;

use crate::domain::{
    PhaseThreePackageSelector, PipelineConfig, PipelineOutcome, PipelineReport,
    StageReport,
};
use crate::ports::{FbxExportOptions, PipelineOperations};

mod character_catalog;
mod dynamic_zone_catalog;
mod fbx_catalog_publish;
mod fbx_export;
mod fbx_manifest;
mod filesystem;
mod mission_camera_catalog;
mod mission_completion_dialog_context;
mod mission_dialogue_info_context;
mod mission_definition_context;
mod mission_locator_catalog;
mod mission_locator_context;
mod mission_music_context;
mod mission_tuning_context;
mod mission_order_context;
mod one;
mod progress;
mod prop_catalog;
mod run_registry;
mod two;
mod unreal_fbx_catalog;
mod unreal_plans;
mod unreal_prepare;
mod unreal_vehicle_catalog;
mod ui_scrooby_joined_raster;
mod ui_scrooby_layout;
mod ui_scrooby_project;
mod ui_scrooby_resources;
mod ui_sprite_raster;
mod vehicle_catalog;
mod vehicle_tuning_context;
mod vehicle_tuning_usage_context;
mod vertex_expression_context;
mod wasp_camera;
mod wrench;

pub(in crate::adapters) use progress::{
    Verbosity as ProgressVerbosity, install as install_progress,
};
pub(in crate::adapters) use run_registry::{
    RunMode, RunRegistry, RunStartError, check_cancellation,
};

/// Stateless local provider for pipeline operations.
#[derive(Debug, Default, Clone, Copy)]
pub struct LocalPipeline;

impl PipelineOperations for LocalPipeline {
    fn run(&self, config: &PipelineConfig) -> PipelineOutcome<PipelineReport> {
        one::extract::ExtractGameAssets::run(config)
    }

    fn export_movies(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport> {
        one::extract::ExtractGameAssets::export_movies_only(config)
    }

    fn manifest_minor_units(
        &self,
        game_root: &Path,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        two::units::manifest_minor_unit::write_manifest_minor_units(
            game_root,
            extracted_root,
        )
    }

    fn fill_minor_unit_metadata(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        two::units::metadata_fill::fill_minor_unit_metadata(extracted_root)
    }

    fn edit_minor_unit_metadata(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        two::units::editor::edit_minor_unit_metadata(extracted_root)
    }

    fn index_minor_units(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        two::units::index::write_minor_unit_index(extracted_root)
    }

    fn audit_minor_units(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        two::units::audit_minor_units::audit_minor_units(extracted_root)
    }

    fn prepare_unreal(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport> {
        let audit = two::units::audit_minor_units::audit_minor_units(
            &config.extracted_root,
        )?;
        let index =
            two::units::index::write_minor_unit_index(&config.extracted_root)?;
        let staging = unreal_prepare::prepare_unreal(config)?;
        Ok(PipelineReport {
            stages: vec![audit, index, staging],
        })
    }

    fn write_fbx_manifest(
        &self,
        index_path: &Path,
        selector: &PhaseThreePackageSelector,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport> {
        fbx_manifest::write_phase_three_fbx_manifest(
            index_path, selector, output_dir,
        )
    }

    fn export_complete_fbx_catalog(
        &self,
        index_path: &Path,
        output_dir: &Path,
        manifest_path: &Path,
        base_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        fbx_catalog_publish::export_complete_fbx_catalog(
            index_path,
            output_dir,
            manifest_path,
            base_root,
        )
    }

    fn export_character_catalog(
        &self,
        index_path: &Path,
        output_dir: &Path,
        base_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        character_catalog::export_character_catalog(
            index_path, output_dir, base_root,
        )
    }

    fn export_wasp_camera(
        &self,
        index_path: &Path,
        output_dir: &Path,
        base_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        wasp_camera::export_wasp_camera(index_path, output_dir, base_root)
    }

    fn export_wrench(
        &self,
        index_path: &Path,
        output_dir: &Path,
        base_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        wrench::export_wrench(index_path, output_dir, base_root)
    }

    fn export_prop_catalog(
        &self,
        index_path: &Path,
        game_root: &Path,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport> {
        prop_catalog::export_prop_catalog(index_path, game_root, output_dir)
    }

    fn export_vehicle_catalog(
        &self,
        index_path: &Path,
        game_root: &Path,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport> {
        vehicle_catalog::export_vehicle_catalog(
            index_path, game_root, output_dir,
        )
    }

    fn export_world_prop_catalog(
        &self,
        index_path: &Path,
        game_root: &Path,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport> {
        prop_catalog::export_world_prop_catalog(
            index_path, game_root, output_dir,
        )
    }

    fn export_world_master(
        &self,
        index_path: &Path,
        game_root: &Path,
        coordinate_root: &Path,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport> {
        prop_catalog::export_world_master(
            index_path,
            game_root,
            coordinate_root,
            output_dir,
        )
    }

    fn export_structural_guide(
        &self,
        index_path: &Path,
        game_root: &Path,
        coordinate_root: &Path,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport> {
        prop_catalog::export_structural_guide(
            index_path,
            game_root,
            coordinate_root,
            output_dir,
        )
    }

    fn export_fbx_package(
        &self,
        index_path: &Path,
        selector: &PhaseThreePackageSelector,
        output_dir: &Path,
        base_root: &Path,
        options: FbxExportOptions,
    ) -> PipelineOutcome<StageReport> {
        fbx_export::export_fbx_package(
            index_path, selector, output_dir, base_root, options,
        )
    }
}
