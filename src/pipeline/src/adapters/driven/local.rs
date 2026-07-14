// File:
//   - local.rs
// Path:
//   - src/pipeline/src/adapters/driven/local.rs
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
//   - Concrete local execution of extraction, normalization, and planning.
// - Must-Not:
//   - Parse process arguments or change process-neutral application
//   - contracts.
// - Allows:
//   - Compose local storage, external tools, and format-specific adapters.
// - Split-When:
//   - Split when one provider family gains an independent deployment
//   - lifecycle.
// - Merge-When:
//   - Another driven adapter owns the same local pipeline execution
//   - contract.
// - Summary:
//   - Local pipeline operations adapter.
// - Description:
//   - Implements the operations port using repository-local files and tools.
// - Usage:
//   - Selected explicitly by the CLI driving composition.
// - Defaults:
//   - Every root and selector remains caller supplied.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Driven adapter implementing local pipeline operations.
//!
//! Storage and tool mechanisms remain outside the application core.
use std::path::Path;

use crate::domain::{
    PhaseThreePackageSelector, PipelineConfig, PipelineOutcome, PipelineReport,
    StageReport,
};
use crate::ports::{FbxExportOptions, PipelineOperations};

mod fbx_export;
mod fbx_manifest;
mod filesystem;
mod one;
mod progress;
mod two;

pub(in crate::adapters) use progress::{
    Verbosity as ProgressVerbosity, install as install_progress,
};

/// Stateless local provider for pipeline operations.
#[derive(Debug, Default, Clone, Copy)]
pub struct LocalPipeline;

impl PipelineOperations for LocalPipeline {
    fn run(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport> {
        one::extract::ExtractGameAssets::run(config)
    }

    fn export_movies(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport> {
        one::extract::ExtractGameAssets::export_movies_only(config)
    }

    fn export_lmlm(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport> {
        one::extract::ExtractGameAssets::export_lmlm_only(config)
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
