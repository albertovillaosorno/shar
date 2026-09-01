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
use super::inventory_common::{
    clean_identity, ledger_member_id, required_string, required_usize,
};
use crate::domain::PipelineError;
use crate::domain::package::{
    PackageRole, PhaseThreePackageIndex, PhaseThreePackageRow,
};

/// One published texture source with package scope and exact content identity.
#[derive(Clone, Debug, Eq, PartialEq)]
struct TextureSource {
    /// Generated owning package identity.
    package_id: String,
    /// Generated package subcategory used for scope preference.
    subcategory: String,
    /// Stable phase-three package-member identity for this occurrence.
    package_member_id: String,
    /// Exact normalized texture member identity.
    member_id: String,
    /// Exact source texture component ordinal.
    source_ordinal: usize,
    /// Normalized PNG payload path.
    path: PathBuf,
    /// Exact lowercase SHA-256 of the payload.
    sha256: String,
}

/// Public-safe physical texture occurrence retained as deferred source
/// evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SharedTextureOccurrenceEvidence {
    /// Generated owning package identity.
    pub(super) package_id: String,
    /// Generated package subcategory used for scope preference.
    pub(super) subcategory: String,
    /// Stable phase-three package-member identity for this occurrence.
    pub(super) package_member_id: String,
    /// Exact normalized texture member identity.
    pub(super) member_id: String,
    /// Exact source texture component ordinal.
    pub(super) source_ordinal: usize,
    /// Exact lowercase SHA-256 of the normalized PNG payload.
    pub(super) sha256: String,
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
            package,
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
    package: &PhaseThreePackageRow,
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
    let logical = clean_identity(&required_string(&value, "name")?)?;
    let relative_path = required_string(&value, "path")?;
    let member_id = ledger_member_id(&relative_path, "texture")?;
    let source_ordinal = required_usize(&value, "ordinal")?;
    let package_member_id = phase_three_texture_member_id(
        package,
        &relative_path,
        source_ordinal,
    )?;
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
        package_id: package.package_id.clone(),
        subcategory: package.subcategory.clone(),
        package_member_id,
        member_id,
        source_ordinal,
        path,
        sha256: digest_hex(&bytes),
    })))
}


/// Resolve one ledger texture row to its exact phase-three member identity.
fn phase_three_texture_member_id(
    package: &PhaseThreePackageRow,
    relative_path: &str,
    source_ordinal: usize,
) -> Result<String, PipelineError> {
    let expected_path =
        format!("{}/components/{relative_path}", package.package_root);
    let matches = package
        .members()
        .iter()
        .filter(|member| {
            member.role == PackageRole::Texture
                && member.kind == "p3d-texture"
                && member.source_chunk_kind == "texture"
                && member.source_chunk_ordinal == Some(source_ordinal)
                && member.path == expected_path
        })
        .collect::<Vec<_>>();
    let [member] = matches.as_slice() else {
        return Err(PipelineError::new(format!(
            concat!(
                "world texture occurrence has no unique phase-three member: ",
                "{}@{}"
            ),
            relative_path,
            source_ordinal
        )));
    };
    Ok(member.id.clone())
}

#[cfg(test)]
pub(super) struct TextureOccurrenceFixture<'a> {
    pub(super) logical: &'a str,
    pub(super) package_id: &'a str,
    pub(super) subcategory: &'a str,
    pub(super) package_member_id: &'a str,
    pub(super) member_id: &'a str,
    pub(super) source_ordinal: usize,
    pub(super) path: &'a str,
    pub(super) sha256: &'a str,
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

    /// Return every physical occurrence in the same preferred scope used by
    /// material resolution without selecting one payload.
    ///
    /// # Errors
    ///
    /// Returns an error when the logical texture identity is malformed.
    pub(super) fn preferred_occurrences(
        &self,
        texture_reference: &str,
        source_subcategory: &str,
    ) -> Result<Vec<SharedTextureOccurrenceEvidence>, PipelineError> {
        let mut occurrences = self
            .preferred_sources(texture_reference, source_subcategory)?
            .into_iter()
            .map(|source| SharedTextureOccurrenceEvidence {
                package_id: source.package_id.clone(),
                subcategory: source.subcategory.clone(),
                package_member_id: source.package_member_id.clone(),
                member_id: source.member_id.clone(),
                source_ordinal: source.source_ordinal,
                sha256: source.sha256.clone(),
            })
            .collect::<Vec<_>>();
        occurrences.sort_by(|left, right| {
            (&left.subcategory, left.source_ordinal, &left.member_id).cmp(&(
                &right.subcategory,
                right.source_ordinal,
                &right.member_id,
            ))
        });
        Ok(occurrences)
    }

    /// Build a small authority from explicit physical occurrences for tests.
    #[cfg(test)]
    pub(super) fn from_occurrences_for_tests(
        occurrences: &[TextureOccurrenceFixture<'_>],
    ) -> Self {
        let mut sources = BTreeMap::new();
        for occurrence in occurrences {
            sources
                .entry(occurrence.logical.to_owned())
                .or_insert_with(Vec::new)
                .push(TextureSource {
                    package_id: occurrence.package_id.to_owned(),
                    subcategory: occurrence.subcategory.to_owned(),
                    package_member_id: occurrence.package_member_id.to_owned(),
                    member_id: occurrence.member_id.to_owned(),
                    source_ordinal: occurrence.source_ordinal,
                    path: PathBuf::from(occurrence.path),
                    sha256: occurrence.sha256.to_owned(),
                });
        }
        Self { sources }
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
        let logical = clean_identity(texture_reference)?;
        let preferred = self.preferred_sources(&logical, source_subcategory)?;
        unique_payload(&logical, &preferred)
    }

    /// Select the deterministic preferred physical scope without interpreting
    /// payload identity.
    fn preferred_sources(
        &self,
        texture_reference: &str,
        source_subcategory: &str,
    ) -> Result<Vec<&TextureSource>, PipelineError> {
        let logical = clean_identity(texture_reference)?;
        let Some(all) = self.sources.get(&logical) else {
            return Ok(Vec::new());
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
        Ok(if !terrain.is_empty() {
            terrain
        } else if !same_level.is_empty() {
            same_level
        } else if !same_broader.is_empty() {
            same_broader
        } else {
            all.iter().collect()
        })
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
