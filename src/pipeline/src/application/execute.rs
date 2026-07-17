// File:
//   - execute.rs
// Path:
//   - src/pipeline/src/application/execute.rs
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
//   - Process-neutral orchestration of pipeline operations through one port.
// - Must-Not:
//   - Select local adapters, traverse storage, or invoke external tools.
// - Allows:
//   - Dispatch explicit extraction, manifest, audit, and planning requests.
// - Split-When:
//   - Split when one operation family gains an independent application
//   - policy.
// - Merge-When:
//   - Another application service owns the same operation dispatch contract.
// - Summary:
//   - Pipeline application service.
// - Description:
//   - Keeps inbound adapters independent from concrete local phase
//   - execution.
// - Usage:
//   - Constructed by driving adapters with an explicit operations provider.
// - Defaults:
//   - No provider or path is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Process-neutral pipeline application orchestration.
//!
//! Every operation is delegated through the outbound operations port.
use std::path::Path;

use crate::domain::{
    PhaseThreePackageSelector, PipelineConfig, PipelineOutcome, PipelineReport,
    StageReport,
};
use crate::ports::{FbxExportOptions, PipelineOperations};

/// Application service bound to one explicit pipeline operations provider.
#[derive(Debug, Clone, Copy)]
pub struct PipelineService<'provider, Provider: ?Sized> {
    /// Outbound provider selected by the composition root.
    provider: &'provider Provider,
}

impl<'provider, Provider> PipelineService<'provider, Provider>
where
    Provider: PipelineOperations + ?Sized,
{
    /// Binds application orchestration to one explicit provider.
    #[must_use]
    pub const fn new(provider: &'provider Provider) -> Self {
        Self {
            provider,
        }
    }

    /// Runs the complete ordered extraction pipeline.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn run(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport> {
        self.provider
            .run(config)
    }

    /// Exports only movie packages.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn export_movies(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport> {
        self.provider
            .export_movies(config)
    }

    /// Exports only LMLM packages.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn export_lmlm(
        &self,
        config: &PipelineConfig,
    ) -> PipelineOutcome<PipelineReport> {
        self.provider
            .export_lmlm(config)
    }

    /// Writes the phase-two minor-unit manifest.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn manifest_minor_units(
        &self,
        game_root: &Path,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        self.provider
            .manifest_minor_units(
                game_root,
                extracted_root,
            )
    }

    /// Fills derived minor-unit metadata.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn fill_minor_unit_metadata(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        self.provider
            .fill_minor_unit_metadata(extracted_root)
    }

    /// Applies deterministic minor-unit metadata edits.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn edit_minor_unit_metadata(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        self.provider
            .edit_minor_unit_metadata(extracted_root)
    }

    /// Writes the minor-unit package index.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn index_minor_units(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        self.provider
            .index_minor_units(extracted_root)
    }

    /// Audits the minor-unit manifest and package evidence.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn audit_minor_units(
        &self,
        extracted_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        self.provider
            .audit_minor_units(extracted_root)
    }

    /// Writes one selected phase-three FBX manifest.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn write_fbx_manifest(
        &self,
        index_path: &Path,
        selector: &PhaseThreePackageSelector,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport> {
        self.provider
            .write_fbx_manifest(
                index_path, selector, output_dir,
            )
    }

    /// Exports every skinned character package as a verified FBX catalog.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn export_character_catalog(
        &self,
        index_path: &Path,
        output_dir: &Path,
        base_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        self.provider
            .export_character_catalog(
                index_path, output_dir, base_root,
            )
    }

    /// Exports one canonical standalone Wasp Camera FBX artifact.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn export_wasp_camera(
        &self,
        index_path: &Path,
        output_dir: &Path,
        base_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        self.provider
            .export_wasp_camera(
                index_path, output_dir, base_root,
            )
    }

    /// Exports one canonical standalone Wrench model FBX artifact.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn export_wrench(
        &self,
        index_path: &Path,
        output_dir: &Path,
        base_root: &Path,
    ) -> PipelineOutcome<StageReport> {
        self.provider
            .export_wrench(
                index_path, output_dir, base_root,
            )
    }

    /// Exports the complete original-game model prop catalog.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn export_prop_catalog(
        &self,
        index_path: &Path,
        game_root: &Path,
        output_dir: &Path,
    ) -> PipelineOutcome<StageReport> {
        self.provider
            .export_prop_catalog(
                index_path, game_root, output_dir,
            )
    }

    /// Exports one selected phase-three package as an FBX artifact.
    ///
    /// # Errors
    ///
    /// Returns the provider's validated pipeline failure.
    pub fn export_fbx_package(
        &self,
        index_path: &Path,
        selector: &PhaseThreePackageSelector,
        output_dir: &Path,
        base_root: &Path,
        options: FbxExportOptions,
    ) -> PipelineOutcome<StageReport> {
        self.provider
            .export_fbx_package(
                index_path, selector, output_dir, base_root, options,
            )
    }
}
