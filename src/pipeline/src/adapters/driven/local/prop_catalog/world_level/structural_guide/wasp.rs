// File:
//   - wasp.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     structural_guide/wasp.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Confidential: false
// License-File: LICENSE
// Path-Rule: All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Source-backed Wasp Camera activation, locator resolution, and placement.
// - Must-Not:
//   - Infer omitted targets, include commented script calls, or serialize FBX.
// - Allows:
//   - Reuse the canonical rest-pose body, decoded locators, and reviewed world
//     family movements.
// - Summary:
//   - Adds the 140 counted campaign Wasp Cameras to the structural guide.
//
// LARGE-FILE:
// - owner: Structural-guide Wasp placement
// - reason: Script activation, package locator authority, body presentation,
//   and transformed instances enforce one source-backed placement contract.
// - split: Body selection remains owned by the standalone Wasp exporter.
// - validation: Exact 20-per-level parsing and 140-placement tests.
// - review: Split if another flying-actor family uses a different script form.

//! Source-backed placement of the 140 counted Wasp Camera targets.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::super::super::prepared::PreparedTexture;
use super::super::export::MasterContent;
use super::super::movement::exterior_movement_for_level;
use super::super::transform::{bake_mesh, translation};
use crate::adapters::driven::local::wasp_camera::collect_wasp_guide_source;
use crate::domain::PipelineError;
use crate::domain::package::{PhaseThreePackageIndex, PhaseThreePackageRow};

const LEVELS: std::ops::RangeInclusive<u8> = 1..=7;
const EXPECTED_PER_LEVEL: usize = 20;
const EXPECTED_PLACEMENTS: usize = 140;
const BODY_MESHES_PER_PLACEMENT: usize = 14;

#[derive(Clone, Debug, Eq, PartialEq)]
struct ActiveSpawn {
    actor: String,
    locator: String,
}

#[derive(Deserialize)]
struct LocatorRecord {
    name: String,
    position: [f32; 3],
}

/// Append all counted Wasp Cameras to one canonical guide source.
pub(super) fn append(
    index_path: &Path,
    game_root: &Path,
    texture_staging: &Path,
    content: &mut MasterContent,
) -> Result<usize, PipelineError> {
    let repository_root = repository_root(index_path)?;
    let source = collect_wasp_guide_source(
        index_path,
        &repository_root,
        texture_staging,
    )?;
    merge_materials(
        content,
        source.materials,
    )?;
    merge_textures(
        content,
        source.textures,
    )?;
    if source
        .meshes
        .len()
        != BODY_MESHES_PER_PLACEMENT
    {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide Wasp body mesh count changed: {}",
                    source
                        .meshes
                        .len()
                ),
            ),
        );
    }
    let index = PhaseThreePackageIndex::read(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let mut placement_count = 0_usize;
    let mut placement_ids = BTreeSet::new();
    for level in LEVELS {
        let spawns = active_spawns(
            game_root, level,
        )?;
        let locators = locator_positions(
            &index,
            &repository_root,
            level,
        )?;
        let movement = exterior_movement_for_level(level).ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "structural-guide Wasp movement is missing for Level \
                         {level}"
                    ),
                )
            },
        )?;
        for spawn in spawns {
            let locator_key = normalized_identity(&spawn.locator);
            let position = locators
                .get(&locator_key)
                .copied()
                .ok_or_else(
                    || {
                        PipelineError::new(
                            format!(
                                "structural-guide Wasp locator is missing: \
                                 Level {level}:{}",
                                spawn.locator
                            ),
                        )
                    },
                )?;
            let placement_id = format!(
                "wasp-placement-l{level:02}-{}",
                normalized_identity(&spawn.actor)
            );
            if !placement_ids.insert(placement_id.clone()) {
                return Err(
                    PipelineError::new(
                        format!(
                            "structural-guide Wasp placement repeats: \
                             {placement_id}"
                        ),
                    ),
                );
            }
            for source_mesh in &source.meshes {
                let mut mesh = source_mesh.clone();
                let part = normalized_identity(&mesh.name);
                bake_mesh(
                    &mut mesh,
                    &translation(position),
                    format!("{placement_id}-{part}"),
                )?;
                let final_name = mesh
                    .name
                    .clone();
                bake_mesh(
                    &mut mesh,
                    &movement.matrix(),
                    final_name,
                )?;
                content
                    .meshes
                    .push(mesh);
            }
            placement_count = placement_count
                .checked_add(1)
                .ok_or_else(
                    || {
                        PipelineError::new(
                            "structural-guide Wasp placement count overflowed",
                        )
                    },
                )?;
        }
    }
    if placement_count != EXPECTED_PLACEMENTS {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide Wasp placement count changed: \
                     {placement_count}"
                ),
            ),
        );
    }
    Ok(placement_count)
}

fn active_spawns(
    game_root: &Path,
    level: u8,
) -> Result<Vec<ActiveSpawn>, PipelineError> {
    let path = game_root
        .join("scripts")
        .join("missions")
        .join(format!("level{level:02}"))
        .join("leveli.mfk");
    let text = fs::read_to_string(&path).map_err(
        |error| {
            PipelineError::new(
                format!(
                    "structural-guide Wasp script read failed: {}:{error}",
                    path.display()
                ),
            )
        },
    )?;
    let mut spawns = Vec::new();
    for line in text.lines() {
        let code = line
            .split_once("//")
            .map_or(
                line,
                |(code, _)| code,
            );
        if !code.contains("AddSpawnPointByLocatorScript") {
            continue;
        }
        let arguments = quoted_arguments(code);
        if arguments.len() < 4
            || !arguments[1].eq_ignore_ascii_case("beecamera")
        {
            continue;
        }
        spawns.push(
            ActiveSpawn {
                actor: arguments[0].to_owned(),
                locator: arguments[3].to_owned(),
            },
        );
    }
    if spawns.len() != EXPECTED_PER_LEVEL {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide active Wasp count changed for Level \
                     {level}: {}",
                    spawns.len()
                ),
            ),
        );
    }
    let identities = spawns
        .iter()
        .map(|spawn| normalized_identity(&spawn.actor))
        .collect::<BTreeSet<_>>();
    if identities.len() != EXPECTED_PER_LEVEL {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide active Wasp identities repeat for Level \
                     {level}"
                ),
            ),
        );
    }
    Ok(spawns)
}

fn locator_positions(
    index: &PhaseThreePackageIndex,
    repository_root: &Path,
    level: u8,
) -> Result<BTreeMap<String, [f32; 3]>, PipelineError> {
    let package_id = format!("extracted-art-missions-level{level:02}-wasps");
    let expected_root = format!("extracted/art/missions/level{level:02}/wasps");
    let package = index
        .find_package(&package_id)
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "structural-guide Wasp package is missing: \
                         {package_id}"
                    ),
                )
            },
        )?;
    validate_package_root(
        package,
        &expected_root,
    )?;
    let mut locators = BTreeMap::new();
    for member in package.members() {
        if !member
            .path
            .contains("/components/srr_locator/")
            || !member
                .path
                .ends_with(".json")
        {
            continue;
        }
        let path = repository_root.join(&member.path);
        let bytes = fs::read(&path).map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "structural-guide Wasp locator read failed: {}:{error}",
                        path.display()
                    ),
                )
            },
        )?;
        let record: LocatorRecord = serde_json::from_slice(&bytes).map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "structural-guide Wasp locator parse failed: \
                         {}:{error}",
                        path.display()
                    ),
                )
            },
        )?;
        if record
            .position
            .iter()
            .any(|value| !value.is_finite())
        {
            return Err(
                PipelineError::new(
                    format!(
                        "structural-guide Wasp locator is non-finite: {}",
                        record.name
                    ),
                ),
            );
        }
        let key = normalized_identity(
            record
                .name
                .trim_end_matches('\0'),
        );
        if locators
            .insert(
                key.clone(),
                record.position,
            )
            .is_some()
        {
            return Err(
                PipelineError::new(
                    format!(
                        "structural-guide Wasp locator repeats: Level \
                         {level}:{key}"
                    ),
                ),
            );
        }
    }
    if locators.len() < EXPECTED_PER_LEVEL {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide Wasp locator package is incomplete: \
                     Level {level}:{}",
                    locators.len()
                ),
            ),
        );
    }
    Ok(locators)
}

fn validate_package_root(
    package: &PhaseThreePackageRow,
    expected_root: &str,
) -> Result<(), PipelineError> {
    if package.package_root == expected_root {
        Ok(())
    } else {
        Err(
            PipelineError::new(
                format!(
                    "structural-guide Wasp package root changed: expected \
                     {expected_root}, found {}",
                    package.package_root
                ),
            ),
        )
    }
}

fn merge_materials(
    content: &mut MasterContent,
    materials: Vec<fbx::domain::texture::MaterialBinding>,
) -> Result<(), PipelineError> {
    for material in materials {
        let key = material
            .material_name
            .clone();
        if let Some(existing) = content
            .materials
            .get(&key)
        {
            if existing != &material {
                return Err(
                    PipelineError::new(
                        format!(
                            "structural-guide Wasp material conflicts: {key}"
                        ),
                    ),
                );
            }
        } else {
            content
                .materials
                .insert(
                    key, material,
                );
        }
    }
    Ok(())
}

fn merge_textures(
    content: &mut MasterContent,
    textures: Vec<
        crate::adapters::driven::local::wasp_camera::WaspGuideTexture,
    >,
) -> Result<(), PipelineError> {
    for texture in textures {
        let prepared = PreparedTexture {
            file_name: texture
                .file_name
                .clone(),
            bytes: texture.bytes,
            sha256: texture.sha256,
        };
        if let Some(existing) = content
            .textures
            .get(&prepared.file_name)
        {
            if existing != &prepared {
                return Err(
                    PipelineError::new(
                        format!(
                            "structural-guide Wasp texture conflicts: {}",
                            prepared.file_name
                        ),
                    ),
                );
            }
        } else {
            content
                .textures
                .insert(
                    prepared
                        .file_name
                        .clone(),
                    prepared,
                );
        }
    }
    Ok(())
}

fn repository_root(index_path: &Path) -> Result<PathBuf, PipelineError> {
    let minor_unit = index_path
        .parent()
        .ok_or_else(
            || {
                PipelineError::new(
                    "structural-guide package index has no parent",
                )
            },
        )?;
    let extracted = minor_unit
        .parent()
        .ok_or_else(
            || {
                PipelineError::new(
                    "structural-guide package index has no extracted root",
                )
            },
        )?;
    let root = extracted
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let root = if root
        .as_os_str()
        .is_empty()
    {
        PathBuf::from(".")
    } else {
        root.to_path_buf()
    };
    if !root
        .join("extracted/art/l01_fx")
        .is_dir()
    {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide decoded repository root is invalid: {}",
                    root.display()
                ),
            ),
        );
    }
    Ok(root)
}

fn quoted_arguments(line: &str) -> Vec<&str> {
    let mut arguments = Vec::new();
    let mut start = None;
    for (index, character) in line.char_indices() {
        if character != '"' {
            continue;
        }
        if let Some(open) = start.take() {
            if let Some(value) = line.get(open..index) {
                arguments.push(value);
            }
        } else {
            start = Some(index + character.len_utf8());
        }
    }
    arguments
}

fn normalized_identity(value: &str) -> String {
    value
        .trim_end_matches('\0')
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(
            |character| {
                if character.is_ascii_alphanumeric() {
                    character
                } else {
                    '-'
                }
            },
        )
        .collect::<String>()
        .trim_matches('-')
        .to_owned()
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::{active_spawns, quoted_arguments};

    fn game_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("game")
    }

    #[test]
    fn commented_wasp_spawn_is_not_active() {
        let line = "//AddSpawnPointByLocatorScript(\"static_bee2\",\"\
                    beecamera\",\"Shelley\",\"w_powerplant2\",\"20\",\"60\");";
        let code = line
            .split_once("//")
            .map_or(
                line,
                |(code, _)| code,
            );
        assert!(quoted_arguments(code).is_empty());
    }

    #[test]
    fn every_level_has_exactly_twenty_active_wasps() -> Result<(), String> {
        for level in 1..=7 {
            let spawns = active_spawns(
                &game_root(),
                level,
            )
            .map_err(|error| error.to_string())?;
            if spawns.len() != 20 {
                return Err(
                    format!(
                        "Level {level} has {} Wasp rows",
                        spawns.len()
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn level_four_static_actor_uses_authored_locator() -> Result<(), String> {
        let spawns = active_spawns(
            &game_root(),
            4,
        )
        .map_err(|error| error.to_string())?;
        let locator = spawns
            .iter()
            .find(|spawn| spawn.actor == "static_bee2")
            .map(
                |spawn| {
                    spawn
                        .locator
                        .as_str()
                },
            );
        if locator == Some("w_powerplant2") {
            Ok(())
        } else {
            Err(format!("unexpected static Wasp locator: {locator:?}"))
        }
    }
}
