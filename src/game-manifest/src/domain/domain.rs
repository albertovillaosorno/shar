// File:
//   - domain.rs
// Path:
//   - src/game-manifest/src/domain/domain.rs
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
//   - Pure manifest records, taxonomy, classification, and path projections.
// - Must-Not:
//   - Avoid filesystem IO, artifact writes, CLI parsing, and adapter selection.
// - Allows:
//   - Deterministic obfuscation, normalized counting, and JSON rows.
// - Split-When:
//   - Split when one manifest subdomain becomes independently versioned.
// - Merge-When:
//   - Another domain facade owns the same manifest invariants.
// - Summary:
//   - Domain model for name-hiding game manifests.
// - Description:
//   - Defines pure manifest behavior over caller-supplied path evidence.
// - Usage:
//   - Used by application commands, ports, adapters, and library callers.
// - Defaults:
//   - Unknown classifications fail closed as `error`.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Name-hiding manifest format for `game/` content.
//!
//! The manifest preserves the directory *shape* of the game copy but hides the
//! real names: every folder name is reduced to its first and last character,
//! and file names are dropped entirely — only a per-folder, per-type count is
//! kept.
//!
//! For example `scripts/missions/something.mfk` contributes to the record:
//!
//! ```text
//! {"dir":"ss/ms","ext":"mfk","min":1}
//! ```
//!
//! `scripts` becomes `ss`, `missions` becomes `ms`, and the file name is
//! replaced by the count "1 mfk file in this folder". This lets validation
//! pinpoint which folder is short on which file type, without publishing the
//! game's real folder names or file names.
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use schoenwald_filesystem::resolve_under;

mod classification;
mod json;

pub use classification::classify_manifest_bucket;

/// File name of the manifest itself, relative to the game directory. It is
/// excluded from the counts and skipped by consumers.
pub const MANIFEST_FILE_NAME: &str = "manifest.jsonl";

/// File name of the expanded manifest, relative to the game directory.
pub const EXPANDED_MANIFEST_FILE_NAME: &str = "manifest-expanded.jsonl";

/// Controlled taxonomy of manifest file kinds.
pub const KIND_TAXONOMY: &[&str] = &[
    "error",
    "language_mod",
    "language_textbible",
    "movie",
    "p3d_container",
    "rcf_container",
    "audio",
    "music_arrangement",
    "script",
    "image",
    "generated_artifact",
    "character_outfit",
    "build-log",
    "document",
    "ui-resource",
    "sound-type",
    "metadata",
    "json-ledger",
];

/// Extension token used for files that have no extension.
pub const NO_EXTENSION: &str = "(none)";

/// Extension of the optional language mod. It is excluded from the folder
/// counts and recorded once at the root with a minimum of zero.
pub const OPTIONAL_EXTENSION: &str = "lmlm";

/// Extension of generated image files that must not become required game input.
/// Original texture/image payloads are supplied through P3D extraction instead.
pub const GENERATED_IMAGE_EXTENSION: &str = "png";

/// Extension of local backup files produced by modding tooling (for example
/// `ingame.p3d.schoenwald-original`). These are not original game content and
/// are excluded from the counts.
pub const BACKUP_EXTENSION: &str = "schoenwald-original";

/// Counts keyed by `(obfuscated directory, file extension)`.
pub type DirExtCounts = BTreeMap<
    (
        String,
        String,
    ),
    usize,
>;

/// One manifest record: an obfuscated folder path, a file extension, and the
/// minimum number of files of that type the folder must contain.
#[derive(Debug)]
pub struct DirCount {
    /// Obfuscated, `/`-separated folder path. Each name is reduced to its
    /// first and last character; colliding paths receive a stable ordinal
    /// suffix. Empty for the game root.
    pub dir: String,
    /// Lowercase file extension, or [`NO_EXTENSION`].
    pub extension: String,
    /// Minimum number of files of this type in the folder.
    pub min_count: usize,
    /// Pipeline kind for this folder/type bucket. Unknown is explicit.
    pub kind: String,
}

/// Render the file kind taxonomy as JSON Lines.
#[must_use]
pub fn kind_taxonomy_jsonl() -> String {
    let values = KIND_TAXONOMY
        .iter()
        .map(|kind| format!("\"{kind}\""))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "{{\"schema\":\"shar-schoenwald.game-manifest-ledger.v1\",",
            "\"kind_taxonomy\":[{}]}}",
        ),
        values
    )
}

/// Returns the lowercase extension of a path, or [`NO_EXTENSION`] when absent.
#[must_use]
pub fn extension_of(path: &Path) -> String {
    path.extension()
        .and_then(|extension| extension.to_str())
        .filter(|extension| !extension.is_empty())
        .map_or_else(
            || NO_EXTENSION.to_owned(),
            str::to_lowercase,
        )
}

/// Reduces a folder name to its first and last character, lowercased.
///
/// Exposed so downstream ledgers (the phase-two minor-unit manifest) reuse the
/// one canonical name-hiding rule instead of duplicating it, keeping the
/// obfuscation contract a single source of truth across manifests.
#[must_use]
pub fn obfuscate_component(name: &str) -> String {
    let first_char = name
        .chars()
        .next();
    let last_char = name
        .chars()
        .last();
    match (
        first_char, last_char,
    ) {
        (Some(first), Some(last)) => [
            first, last,
        ]
        .into_iter()
        .flat_map(char::to_lowercase)
        .collect(),
        _ => String::new(),
    }
}

/// Builds the obfuscated, `/`-separated folder path for a file, relative to the
/// game root. Returns an empty string for files directly in the root.
fn obfuscate_parent(
    root: &Path,
    file: &Path,
) -> String {
    let Some(parent) = file.parent() else {
        return String::new();
    };
    let Ok(relative) = parent.strip_prefix(root) else {
        return String::new();
    };
    let mut parts: Vec<String> = Vec::new();
    for component in relative.components() {
        if let Some(name) = component
            .as_os_str()
            .to_str()
        {
            parts.push(obfuscate_component(name));
        }
    }
    parts.join("/")
}

/// One countable source coordinate before public-path disambiguation.
#[derive(Debug)]
struct ManifestSource {
    /// Normalized source directory relative to the caller-selected root.
    relative_parent: PathBuf,
    /// Base public directory identity before collision disambiguation.
    base_dir: String,
    /// Normalized source extension used by the minimum ledger coordinate.
    extension: String,
}

/// Returns whether one root file is a generated manifest ledger.
fn is_root_manifest(
    root: &Path,
    path: &Path,
) -> bool {
    if path.parent() != Some(root) {
        return false;
    }
    let Some(name) = path
        .file_name()
        .and_then(|value| value.to_str())
    else {
        return false;
    };
    let is_minimum = name.eq_ignore_ascii_case(MANIFEST_FILE_NAME);
    let is_expanded = name.eq_ignore_ascii_case(EXPANDED_MANIFEST_FILE_NAME);
    is_minimum || is_expanded
}

/// Returns one countable source coordinate for a rooted source path.
fn manifest_source(
    root: &Path,
    path: &Path,
) -> Option<ManifestSource> {
    let relative = path
        .strip_prefix(root)
        .ok()?;
    let _resolved = resolve_under(
        root, relative,
    )
    .ok()?;
    let extension = extension_of(path);
    let at_root = path.parent() == Some(root);
    if at_root && extension == OPTIONAL_EXTENSION {
        return None;
    }
    if at_root && extension == GENERATED_IMAGE_EXTENSION {
        return None;
    }
    if extension == BACKUP_EXTENSION {
        return None;
    }
    if is_root_manifest(
        root, path,
    ) {
        return None;
    }
    let parent = path.parent()?;
    let relative_parent = parent
        .strip_prefix(root)
        .ok()?
        .to_path_buf();
    Some(
        ManifestSource {
            relative_parent,
            base_dir: obfuscate_parent(
                root, path,
            ),
            extension,
        },
    )
}

/// Collects unique countable source coordinates in deterministic path order.
fn manifest_sources(
    root: &Path,
    files: &[PathBuf],
) -> Vec<ManifestSource> {
    let mut seen = BTreeSet::new();
    let mut sources = Vec::new();
    for path in files {
        if !seen.insert(path) {
            continue;
        }
        let Some(source) = manifest_source(
            root, path,
        ) else {
            continue;
        };
        sources.push(source);
    }
    sources
}

/// Assigns one deterministic public identity to every source directory.
fn public_parent_ids(sources: &[ManifestSource]) -> BTreeMap<PathBuf, String> {
    let mut grouped = BTreeMap::<String, BTreeSet<PathBuf>>::new();
    for source in sources {
        let parents = grouped
            .entry(
                source
                    .base_dir
                    .clone(),
            )
            .or_default();
        let _inserted = parents.insert(
            source
                .relative_parent
                .clone(),
        );
    }
    let mut identities = BTreeMap::new();
    for (base_dir, parents) in grouped {
        let needs_ordinal = parents.len() > 1;
        for (offset, parent) in parents
            .into_iter()
            .enumerate()
        {
            let ordinal = offset.saturating_add(1);
            let identity = if needs_ordinal {
                format!("{base_dir}~{ordinal:02}")
            } else {
                base_dir.clone()
            };
            let _previous = identities.insert(
                parent, identity,
            );
        }
    }
    identities
}

/// Counts supplied regular-file paths by obfuscated parent and extension.
///
/// Distinct source directories that share the same base obfuscation receive
/// stable ordinal suffixes, preventing ambiguous merged evidence without
/// publishing either original directory name.
#[must_use]
pub fn count_by_dir_ext_paths(
    root: &Path,
    files: &[PathBuf],
) -> DirExtCounts {
    let sources = manifest_sources(
        root, files,
    );
    let identities = public_parent_ids(&sources);
    let mut counts = DirExtCounts::new();
    for source in sources {
        let Some(dir) = identities.get(&source.relative_parent) else {
            continue;
        };
        let key = (
            dir.clone(),
            source.extension,
        );
        let count = counts
            .entry(key)
            .or_insert(0);
        *count = count.saturating_add(1);
    }
    counts
}
