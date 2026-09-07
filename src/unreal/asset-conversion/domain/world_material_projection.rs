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
//   - Deterministic world-material projection values and validation.
// - Must-Not:
//   - Read catalogs, select Unreal master materials, or contact Unreal Editor.
// - Allows:
//   - Verified world material bindings, writer slots, and presentation hashes.
// - Split-When:
//   - Native master-material selection gains a reviewed schema lifecycle.
// - Merge-When:
//   - Another domain module owns the identical material projection contract.
// - Summary:
//   - World material projection domain contract.
// - Description:
//   - Validates exact world binding-to-slot joins and deduplicates only equal
//   - effective presentation identities for later native construction.
// - Usage:
//   - Built from verified world catalog v8 rows before Unreal planning.
// - Defaults:
//   - Hash drift, texture drift, slot drift, and ambiguous joins fail closed.
//

//! Deterministic world-material projection values and validation.

use std::collections::{BTreeMap, BTreeSet};

use super::WorldMaterialRasterProjection;

/// World catalog schema that supplies the projection evidence.
pub const WORLD_MATERIAL_SOURCE_SCHEMA: &str =
    "shar.world-package-collection.v8";

/// Exact overlapping surface semantics already resolved by the FBX writer.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorldMaterialSemantics {
    /// Alpha-blended or otherwise transparent presentation.
    pub transparent: bool,
    /// Glass presentation.
    pub glass: bool,
    /// Mirror presentation.
    pub mirror: bool,
    /// Reflective presentation that is not a mirror.
    pub reflective: bool,
    /// Luminous presentation.
    pub light_emitter: bool,
    /// Non-luminous visual-effect presentation.
    pub visual_effect: bool,
}

impl WorldMaterialSemantics {
    fn suffix(self) -> Option<String> {
        let mut labels = Vec::new();
        if self.glass {
            labels.push("glass");
        } else if self.transparent {
            labels.push("transparent");
        }
        if self.mirror {
            labels.push("mirror");
        }
        if self.reflective && !self.mirror {
            labels.push("reflective");
        }
        if self.light_emitter {
            labels.push("light-emitter");
        }
        if self.visual_effect {
            labels.push("vfx");
        }
        (!labels.is_empty()).then(|| labels.join("-"))
    }
}

/// One verified base material binding from a world FBX artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldMaterialBindingSource {
    /// Canonical FBX input material identity.
    pub material_name: String,
    /// Adjacent normalized base-color PNG filename when textured.
    pub texture_file_name: Option<String>,
    /// Exact normalized PNG digest when textured.
    pub texture_sha256: Option<String>,
    /// Runtime-addressable source shader binding digest.
    pub binding_sha256: String,
    /// Initial visual-presentation digest before geometry semantic fan-out.
    pub presentation_sha256: String,
    /// Exact decoded diffuse color.
    pub base_color_rgba8: [u8; 4],
    /// Source-backed raster family and per-instance alpha state.
    pub raster: WorldMaterialRasterProjection,
    /// Initial source-backed surface semantics.
    pub semantics: WorldMaterialSemantics,
}

/// One exact semantic material slot emitted by the binary FBX writer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldMaterialSlotSource {
    /// Exact FBX material object identity.
    pub slot_name: String,
    /// Base binding referenced by source primitive groups.
    pub source_material_name: String,
    /// Binding digest copied from the joined base binding.
    pub binding_sha256: String,
    /// Initial presentation digest copied from the joined base binding.
    pub presentation_sha256: String,
    /// Effective presentation digest after geometry semantics are merged.
    pub slot_presentation_sha256: String,
    /// Effective writer-resolved surface semantics.
    pub semantics: WorldMaterialSemantics,
}

/// One deduplicated effective presentation eligible for one Material Instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldMaterialPresentation {
    /// Effective presentation digest and stable deduplication identity.
    pub slot_presentation_sha256: String,
    /// Normalized base-color PNG filename when textured.
    pub texture_file_name: Option<String>,
    /// Exact normalized PNG digest when textured.
    pub texture_sha256: Option<String>,
    /// Exact decoded diffuse color.
    pub base_color_rgba8: [u8; 4],
    /// Source-backed raster family and per-instance alpha state.
    pub raster: WorldMaterialRasterProjection,
    /// Exact effective surface semantics.
    pub semantics: WorldMaterialSemantics,
}

/// One exact FBX slot assignment retaining runtime-addressable binding
/// identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldMaterialSlotAssignment {
    /// Exact FBX material object identity.
    pub slot_name: String,
    /// Exact joined base material identity.
    pub source_material_name: String,
    /// Runtime-addressable source shader binding digest.
    pub binding_sha256: String,
    /// Initial visual presentation digest.
    pub presentation_sha256: String,
    /// Effective presentation identity selected for native construction.
    pub slot_presentation_sha256: String,
}

/// Validated internal slot state before projection publication.
struct ValidatedWorldMaterialSlots<'source> {
    presentations: BTreeMap<String, WorldMaterialPresentation>,
    assignments: Vec<WorldMaterialSlotAssignment>,
    used_bindings: BTreeSet<&'source str>,
}

/// Validated deterministic world material projection for one FBX artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldMaterialProjection {
    presentations: Vec<WorldMaterialPresentation>,
    assignments: Vec<WorldMaterialSlotAssignment>,
}

impl WorldMaterialProjection {
    /// Validate v8 binding and slot rows and build one deterministic
    /// projection.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed hashes or textures, duplicate identities,
    /// incomplete joins, writer-slot drift, or conflicting presentation reuse.
    pub fn build(
        bindings: &[WorldMaterialBindingSource],
        slots: &[WorldMaterialSlotSource],
    ) -> Result<Self, String> {
        let binding_map = validate_bindings(bindings)?;
        let validated = validate_slots(slots, &binding_map)?;
        let expected_bindings =
            binding_map.keys().copied().collect::<BTreeSet<_>>();
        if validated.used_bindings != expected_bindings {
            return Err(
                "world material binding lacks an exact writer slot".to_owned()
            );
        }
        Ok(Self {
            presentations: validated.presentations.into_values().collect(),
            assignments: validated.assignments,
        })
    }

    /// Return effective presentations sorted by presentation digest.
    #[must_use]
    pub fn presentations(&self) -> &[WorldMaterialPresentation] {
        &self.presentations
    }

    /// Return exact slot assignments sorted by slot name.
    #[must_use]
    pub fn assignments(&self) -> &[WorldMaterialSlotAssignment] {
        &self.assignments
    }
}

fn validate_bindings(
    bindings: &[WorldMaterialBindingSource],
) -> Result<BTreeMap<&str, &WorldMaterialBindingSource>, String> {
    let mut by_name = BTreeMap::new();
    let mut hashes = BTreeSet::new();
    for binding in bindings {
        validate_material_name(&binding.material_name)?;
        validate_sha256(
            &binding.binding_sha256,
            "world material binding hash",
        )?;
        validate_sha256(
            &binding.presentation_sha256,
            "world material presentation hash",
        )?;
        validate_texture(binding)?;
        let expected_name = binding.semantics.suffix().map_or_else(
            || format!("material-{}", binding.binding_sha256),
            |suffix| format!("material-{}-{suffix}", binding.binding_sha256),
        );
        if binding.material_name != expected_name {
            return Err(
                "world material name does not bind its digest".to_owned()
            );
        }
        if by_name
            .insert(binding.material_name.as_str(), binding)
            .is_some()
        {
            return Err("duplicate world material binding name".to_owned());
        }
        if !hashes.insert(binding.binding_sha256.as_str()) {
            return Err("duplicate world material binding hash".to_owned());
        }
    }
    Ok(by_name)
}

fn validate_slots<'source>(
    slots: &[WorldMaterialSlotSource],
    bindings: &BTreeMap<&'source str, &'source WorldMaterialBindingSource>,
) -> Result<ValidatedWorldMaterialSlots<'source>, String> {
    let mut presentations = BTreeMap::new();
    let mut assignments = Vec::with_capacity(slots.len());
    let mut slot_names = BTreeSet::new();
    let mut used_bindings = BTreeSet::new();
    for slot in slots {
        validate_material_name(&slot.slot_name)?;
        validate_sha256(&slot.binding_sha256, "world slot binding hash")?;
        validate_sha256(
            &slot.presentation_sha256,
            "world slot presentation hash",
        )?;
        validate_sha256(
            &slot.slot_presentation_sha256,
            "world effective presentation hash",
        )?;
        if !slot_names.insert(slot.slot_name.as_str()) {
            return Err("duplicate world material slot name".to_owned());
        }
        let binding = bindings
            .get(slot.source_material_name.as_str())
            .copied()
            .ok_or_else(|| {
                "world material slot references unknown binding".to_owned()
            })?;
        if slot.binding_sha256 != binding.binding_sha256
            || slot.presentation_sha256 != binding.presentation_sha256
        {
            return Err("world material slot binding join drifted".to_owned());
        }
        let expected_name = slot.semantics.suffix().map_or_else(
            || slot.source_material_name.clone(),
            |suffix| format!("{}__{suffix}", slot.source_material_name),
        );
        if slot.slot_name != expected_name {
            return Err(
                "world material slot identity drifted from writer".to_owned()
            );
        }
        let presentation = WorldMaterialPresentation {
            slot_presentation_sha256: slot.slot_presentation_sha256.clone(),
            texture_file_name: binding.texture_file_name.clone(),
            texture_sha256: binding.texture_sha256.clone(),
            base_color_rgba8: binding.base_color_rgba8,
            raster: binding.raster,
            semantics: slot.semantics,
        };
        match presentations.get(&slot.slot_presentation_sha256) {
            Some(existing) if existing != &presentation => {
                return Err(
                    "world effective presentation hash has conflicting state"
                        .to_owned(),
                );
            },
            Some(_) => {},
            None => {
                if presentations
                    .insert(slot.slot_presentation_sha256.clone(), presentation)
                    .is_some()
                {
                    return Err(
                        "world effective presentation insertion drifted"
                            .to_owned(),
                    );
                }
            },
        }
        let _inserted = used_bindings.insert(binding.material_name.as_str());
        assignments.push(WorldMaterialSlotAssignment {
            slot_name: slot.slot_name.clone(),
            source_material_name: slot.source_material_name.clone(),
            binding_sha256: slot.binding_sha256.clone(),
            presentation_sha256: slot.presentation_sha256.clone(),
            slot_presentation_sha256: slot.slot_presentation_sha256.clone(),
        });
    }
    assignments.sort_by(|left, right| left.slot_name.cmp(&right.slot_name));
    Ok(ValidatedWorldMaterialSlots {
        presentations,
        assignments,
        used_bindings,
    })
}

fn validate_texture(
    binding: &WorldMaterialBindingSource,
) -> Result<(), String> {
    match (&binding.texture_file_name, &binding.texture_sha256) {
        (None, None) => Ok(()),
        (Some(file_name), Some(digest)) => {
            validate_sha256(digest, "world material texture hash")?;
            if file_name != &format!("texture-{digest}.png") {
                return Err(
                    "world material texture identity drifted".to_owned()
                );
            }
            Ok(())
        },
        _ => Err("world material texture evidence is incomplete".to_owned()),
    }
}

fn validate_material_name(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 240
        || value.contains('/')
        || value.contains(char::from(92))
        || value.chars().any(char::is_control)
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_')
        })
    {
        return Err("invalid world material identity".to_owned());
    }
    Ok(())
}

fn validate_sha256(value: &str, label: &str) -> Result<(), String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(format!("invalid {label}"));
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact test-module path syntax is indivisible
#[path = "../../../../tests/unreal/asset-conversion/unit/domain/world_material_projection/tests.rs"]
mod tests;
