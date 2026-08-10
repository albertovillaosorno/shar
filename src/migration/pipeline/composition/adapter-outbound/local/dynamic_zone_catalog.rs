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
//   - Integrity preflight for already-read decoded DynamicZone locators.
// - Must-Not:
//   - Infer trigger traversal order, mission progress, or locator precedence.
// - Allows:
//   - Parse decoded type-five locators and validate Dyna package load targets.
// - Split-When:
//   - DynamicZone trigger history gains an independently evidenced runtime
//     model.
// - Merge-When:
//   - Locator intake owns this exact DynamicZone integrity check directly.
// - Summary:
//   - Local DynamicZone package-transition preflight.
// - Description:
//   - Requires P3D loads to resolve to migrated packages while allowing absent
//     unloads, which are deterministic remove-if-present effects.
// - Usage:
//   - Called after authoritative phase-three package-index intake.
// - Defaults:
//   - World Sphere effects are syntax-validated but do not bind P3D packages.
//

//! Filesystem-backed `DynamicZone` package-transition integrity preflight.

use std::collections::BTreeSet;

use serde_json::Value;

use super::mission_locator_catalog::{required_string, required_u32};
use crate::domain::{
    DynaLoadPackageTransition, DynamicZoneTraversalStep,
    PhaseThreePackageIndex, PipelineError, PipelineOutcome,
    compile_dyna_load_package_transition, parse_dyna_load_data,
};

/// Build case-insensitive package-root lookup from validated index evidence.
pub(super) fn dynamic_zone_package_roots(
    index: &PhaseThreePackageIndex,
) -> PipelineOutcome<BTreeSet<String>> {
    indexed_package_roots(index)
}

/// Preflight one already-read decoded locator as a possible `DynamicZone`.
pub(super) fn preflight_dynamic_zone_json(
    json: &str,
    source_package_root: &str,
    indexed_roots: &BTreeSet<String>,
) -> PipelineOutcome<Option<DynamicZoneTraversalStep>> {
    let Some(zone) = parse_dynamic_zone(json)? else {
        return Ok(None);
    };
    let parsed = parse_dyna_load_data(&zone.data).map_err(|error| {
        PipelineError::new(format!(
            "DynamicZone `{}` Dyna Load Data failed: {error}",
            zone.name
        ))
    })?;
    let transition =
        compile_dyna_load_package_transition(&parsed).map_err(|error| {
            PipelineError::new(format!(
                "DynamicZone `{}` package transition failed: {error}",
                zone.name
            ))
        })?;
    drop(
        transition
            .apply_order_independent_package_roots(&[])
            .map_err(|error| {
                PipelineError::new(format!(
                    "DynamicZone `{}` package ordering is unresolved: {error}",
                    zone.name
                ))
            })?,
    );
    validate_load_targets(&zone.name, &transition, indexed_roots)?;
    DynamicZoneTraversalStep::new(
        zone.name,
        source_package_root.to_owned(),
        zone.trigger_count,
        transition,
    )
    .map(Some)
    .map_err(|error| {
        PipelineError::new(format!(
            "DynamicZone traversal identity failed: {error}"
        ))
    })
}

#[derive(Debug, Eq, PartialEq)]
struct DecodedDynamicZone {
    name: String,
    data: String,
    trigger_count: u32,
}

fn parse_dynamic_zone(
    json: &str,
) -> PipelineOutcome<Option<DecodedDynamicZone>> {
    let value = serde_json::from_str::<Value>(json).map_err(|error| {
        PipelineError::new(format!("invalid decoded DynamicZone JSON: {error}"))
    })?;
    let object = value.as_object().ok_or_else(|| {
        PipelineError::new("decoded DynamicZone locator must be a JSON object")
    })?;
    if required_string(object, "schema")? != "locator" {
        return Err(PipelineError::new(
            "decoded DynamicZone locator schema is not supported",
        ));
    }
    let locator_type = required_u32(object, "locator_type")?;
    let locator_type_name = required_string(object, "locator_type_name")?;
    let type_is_dynamic = locator_type == 5;
    let name_is_dynamic = locator_type_name == "dynamic_zone";
    if type_is_dynamic != name_is_dynamic {
        return Err(PipelineError::new(
            "decoded DynamicZone locator type classification drifted",
        ));
    }
    if !type_is_dynamic {
        return Ok(None);
    }
    let raw_name = required_string(object, "name")?;
    let name = raw_name.trim_end_matches(char::from(0)).to_owned();
    if name.is_empty() || name.chars().any(char::is_control) {
        return Err(PipelineError::new(concat!(
            "decoded DynamicZone name is empty or contains ",
            "interior control data"
        )));
    }
    let interpretation = object
        .get("data_interpretation")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new(
                "decoded DynamicZone data_interpretation must be an object",
            )
        })?;
    if required_string(interpretation, "kind")? != "dynamic_zone" {
        return Err(PipelineError::new(
            "decoded DynamicZone interpretation kind drifted",
        ));
    }
    let data = required_string(interpretation, "zone")?;
    let trigger_count = required_u32(object, "num_triggers")?;
    let trigger_volumes = object
        .get("trigger_volumes")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(
                "decoded DynamicZone trigger_volumes must be an array",
            )
        })?;
    let observed_trigger_count =
        u32::try_from(trigger_volumes.len()).map_err(|_conversion_error| {
            PipelineError::new(
                "decoded DynamicZone trigger volume count exceeds u32",
            )
        })?;
    if trigger_count != observed_trigger_count {
        return Err(PipelineError::new(
            "decoded DynamicZone trigger count does not match trigger volumes",
        ));
    }
    Ok(Some(DecodedDynamicZone {
        name,
        data,
        trigger_count,
    }))
}

fn validate_load_targets(
    zone_name: &str,
    transition: &DynaLoadPackageTransition,
    indexed_roots: &BTreeSet<String>,
) -> PipelineOutcome<()> {
    for effect in transition.effects() {
        if !effect.kind().is_p3d_load() {
            continue;
        }
        let root = effect.package_root().ok_or_else(|| {
            PipelineError::new(format!(
                "DynamicZone `{zone_name}` P3D load has no package identity"
            ))
        })?;
        if !indexed_roots.contains(root) {
            return Err(PipelineError::new(format!(
                concat!(
                    "DynamicZone `{zone_name}` load target `{}` is absent ",
                    "from the package index"
                ),
                effect.source_target(),
                zone_name = zone_name
            )));
        }
    }
    Ok(())
}

fn indexed_package_roots(
    index: &PhaseThreePackageIndex,
) -> PipelineOutcome<BTreeSet<String>> {
    let mut roots = BTreeSet::new();
    for package in index.packages() {
        let normalized = normalize_index_package_root(&package.package_root)?;
        if !roots.insert(normalized) {
            return Err(PipelineError::new(concat!(
                "DynamicZone package preflight found duplicate normalized ",
                "package roots"
            )));
        }
    }
    Ok(roots)
}

fn normalize_index_package_root(root: &str) -> PipelineOutcome<String> {
    if root.is_empty()
        || root != root.trim()
        || root.starts_with('/')
        || root.ends_with('/')
        || root.contains(char::from(92))
        || root.contains(':')
        || root.chars().any(char::is_control)
        || root
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(PipelineError::new(
            "DynamicZone package index root is unsafe",
        ));
    }
    Ok(root.to_ascii_lowercase())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/dynamic_zone_catalog/tests.rs"]
mod tests;
