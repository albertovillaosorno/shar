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
//   - Texture authority outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Texture authority outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Texture authority outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use shar_sha256::digest_hex;

use super::extraction::{is_world_package, relative_art_root};
use super::inventory_common::{clean_identity, required_string};
use crate::domain::PipelineError;
use crate::domain::package::{PhaseThreePackageIndex, PhaseThreePackageRow};

/// One published texture source with package scope and exact content identity.
#[derive(Clone, Debug, Eq, PartialEq)]
struct TextureSource {
    /// Generated package subcategory used for scope preference.
    subcategory: String,
    /// Normalized PNG payload path.
    path: PathBuf,
    /// Exact lowercase SHA-256 of the payload.
    sha256: String,
}

/// Logical texture identities mapped to every selected package occurrence.
#[derive(Debug)]
pub(super) struct SharedTextureAuthority {
    /// Cleaned logical texture identity to every normalized occurrence.
    sources: BTreeMap<String, Vec<TextureSource>>,
}

/// Add every normalized texture occurrence from one selected package.
///
/// # Errors
///
/// Returns an error when package paths, ledger JSON, or texture payloads fail.
fn ingest_package(
    package: &PhaseThreePackageRow,
    normalized_root: &Path,
    sources: &mut BTreeMap<String, Vec<TextureSource>>,
) -> Result<(), PipelineError> {
    let relative = relative_art_root(package)?;
    let root = normalized_root.join(relative);
    let manifest = root.join("components.jsonl");
    if !manifest.is_file() {
        return Ok(());
    }
    let text = fs::read_to_string(&manifest).map_err(|error| {
        PipelineError::new(format!(
            "shared texture ledger read failed for {}: {error}",
            manifest.display()
        ))
    })?;
    for line in text.lines().filter(|line| line.contains("\"path\"")) {
        if let Some((logical, source)) = texture_source_from_line(
            line,
            &manifest,
            &root,
            &package.subcategory,
        )? {
            sources.entry(logical).or_default().push(source);
        }
    }
    Ok(())
}

/// Parse one ledger row into a scoped texture source when it is a texture.
///
/// # Errors
///
/// Returns an error when ledger fields, paths, or payload bytes are invalid.
fn texture_source_from_line(
    line: &str,
    manifest: &Path,
    root: &Path,
    subcategory: &str,
) -> Result<Option<(String, TextureSource)>, PipelineError> {
    let value: Value = serde_json::from_str(line).map_err(|error| {
        PipelineError::new(format!(
            "shared texture ledger JSON failed for {}: {error}",
            manifest.display()
        ))
    })?;
    if value.get("kind").and_then(Value::as_str) != Some("texture") {
        return Ok(None);
    }
    let logical = clean_identity(&required_string(&value, "name")?);
    let relative_path = required_string(&value, "path")?;
    let file_name = relative_path
        .strip_prefix("texture/")
        .filter(|member| {
            !member.is_empty()
                && !member.contains('/')
                && !member.contains('\\')
        })
        .ok_or_else(|| {
            PipelineError::new(format!(
                "shared texture path is not portable: {relative_path}"
            ))
        })?;
    let path = root.join("components").join("texture").join(file_name);
    let bytes = fs::read(&path).map_err(|error| {
        PipelineError::new(format!(
            "shared texture payload read failed for {}: {error}",
            path.display()
        ))
    })?;
    Ok(Some((logical, TextureSource {
        subcategory: subcategory.to_owned(),
        path,
        sha256: digest_hex(&bytes),
    })))
}

impl SharedTextureAuthority {
    /// Build one authority from normalized package ledgers.
    ///
    /// # Errors
    ///
    /// Returns an error when a ledger row, path, texture payload, or digest
    /// cannot be read safely.
    pub(super) fn build(
        index: &PhaseThreePackageIndex,
        normalized_root: &Path,
    ) -> Result<Self, PipelineError> {
        let mut sources: BTreeMap<String, Vec<TextureSource>> = BTreeMap::new();
        for package in index
            .packages()
            .iter()
            .filter(|package| is_world_package(package))
        {
            ingest_package(package, normalized_root, &mut sources)?;
        }
        for entries in sources.values_mut() {
            entries.sort_by(|left, right| {
                (&left.subcategory, &left.path)
                    .cmp(&(&right.subcategory, &right.path))
            });
            entries.dedup();
        }
        Ok(Self { sources })
    }

    /// Resolve one missing local texture using package-scope authority.
    ///
    /// # Errors
    ///
    /// Returns an error when the preferred scope contains conflicting payloads.
    pub(super) fn resolve(
        &self,
        texture_reference: &str,
        source_subcategory: &str,
    ) -> Result<Option<&Path>, PipelineError> {
        let logical = clean_identity(texture_reference);
        let Some(all) = self.sources.get(&logical) else {
            return Ok(None);
        };
        let level = level_scope(source_subcategory);
        let same_level = level.map_or_else(Vec::new, |scope| {
            all.iter()
                .filter(|source| {
                    level_scope(&source.subcategory) == Some(scope)
                })
                .collect::<Vec<_>>()
        });
        let terrain = same_level
            .iter()
            .copied()
            .filter(|source| source.subcategory.ends_with("/terrain-mesh"))
            .collect::<Vec<_>>();
        let broader = broader_scope(source_subcategory);
        let same_broader = broader.map_or_else(Vec::new, |scope| {
            all.iter()
                .filter(|source| {
                    broader_scope(&source.subcategory) == Some(scope)
                })
                .collect::<Vec<_>>()
        });
        let preferred = if !terrain.is_empty() {
            terrain
        } else if !same_level.is_empty() {
            same_level
        } else if !same_broader.is_empty() {
            same_broader
        } else {
            all.iter().collect()
        };
        unique_payload(&logical, &preferred)
    }
}

/// Return one path only when every preferred source has identical bytes.
fn unique_payload<'source>(
    logical: &str,
    candidates: &[&'source TextureSource],
) -> Result<Option<&'source Path>, PipelineError> {
    let digests = candidates
        .iter()
        .map(|source| source.sha256.as_str())
        .collect::<BTreeSet<_>>();
    if digests.len() > 1 {
        return Err(PipelineError::new(format!(
            concat!(
                "shared texture identity is ambiguous in ",
                "preferred scope: {} ({})"
            ),
            logical,
            digests.len()
        )));
    }
    Ok(candidates.first().map(|source| source.path.as_path()))
}

/// Extract one stable `level-NN` segment from a package subcategory.
fn level_scope(subcategory: &str) -> Option<&str> {
    subcategory
        .split('/')
        .find(|segment| segment.starts_with("level-"))
}

/// Return one non-level package family scope for generic and bonus packages.
fn broader_scope(subcategory: &str) -> Option<&str> {
    [
        "terrain-world/bonus-area/",
        "missions/generic/",
        "missions/h2h/",
    ]
    .into_iter()
    .find(|prefix| subcategory.starts_with(prefix))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/texture_authority/tests.rs"]
mod tests;
