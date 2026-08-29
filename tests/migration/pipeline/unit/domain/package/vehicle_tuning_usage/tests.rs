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
//   - Vehicle-tuning contextual usage binding regressions.
// - Must-Not:
//   - Read game files, infer profile ownership, or map tuning setters.
// - Allows:
//   - Synthetic exact vehicle/config catalogs and reviewed command shapes.
// - Split-When:
//   - Native tuning application tests gain an independent runtime boundary.
// - Merge-When:
//   - Contextual tuning binding loses independent policy.
// - Summary:
//   - Vehicle-tuning usage binder tests.
// - Description:
//   - Proves exact usage provenance, unresolved paths, and profile reuse.
// - Usage:
//   - Included only by the vehicle-tuning usage domain module under cfg(test).
// - Defaults:
//   - Ambiguity and malformed reviewed commands fail closed.
//

//! Vehicle-tuning contextual usage binding regressions.

use super::*;

fn vehicles() -> MissionReferenceCatalog {
    MissionReferenceCatalog::from_vehicle_entries_for_tests(&[
        ("car_a", "car-a", "cars/traffic/car-a"),
        ("car_b", "car-b", "cars/traffic/car-b"),
    ])
}

fn tuning() -> Result<VehicleTuningSourceCatalog, String> {
    VehicleTuningSourceCatalog::from_entries_for_tests(&[
        (
            r"level01\shared.con",
            "tuning-shared",
            "tuning-package",
            "vehicle-tuning/level01",
        ),
    ])
}

#[expect(
    clippy::too_many_arguments,
    reason = "Synthetic command occurrence keeps all provenance explicit."
)]
fn bind(
    command: &str,
    arguments: &[&str],
    scope: VehicleTuningUsageScope,
    mission_id: Option<&str>,
    stage: Option<usize>,
    objective: Option<usize>,
    ordinal: usize,
    vehicle_catalog: &MissionReferenceCatalog,
    tuning_catalog: &VehicleTuningSourceCatalog,
) -> Result<Option<VehicleTuningUsageBinding>, String> {
    let arguments = arguments
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    let mut bindings = Vec::new();
    bind_usage_parts(
        &mut bindings,
        "mission-source",
        ordinal,
        command,
        &arguments,
        scope,
        mission_id,
        stage,
        objective,
        vehicle_catalog,
        tuning_catalog,
    )?;
    Ok(bindings.pop())
}

#[test]
fn binds_stage_vehicle_to_exact_profile_and_vehicle() -> Result<(), String> {
    let vehicle_catalog = vehicles();
    let tuning_catalog = tuning()?;
    let binding = bind(
        "addstagevehicle",
        &["car_a", "start", "chase", r"level01\shared.con", "none"],
        VehicleTuningUsageScope::Stage,
        Some("m1"),
        Some(2),
        None,
        17,
        &vehicle_catalog,
        &tuning_catalog,
    )?
    .ok_or_else(|| "stage tuning usage disappeared".to_owned())?;
    assert_eq!(binding.mission_source_id(), "mission-source");
    assert_eq!(binding.source_ordinal(), 17);
    assert_eq!(binding.command(), "addstagevehicle");
    assert_eq!(binding.scope(), VehicleTuningUsageScope::Stage);
    assert_eq!(binding.owner_mission_id(), Some("m1"));
    assert_eq!(binding.owner_stage_sequence_ordinal(), Some(2));
    assert_eq!(binding.con_file(), r"level01\shared.con");
    assert_eq!(binding.vehicle().package_id(), "car-a");
    let profile = binding
        .tuning_source()
        .ok_or_else(|| "exact tuning source did not resolve".to_owned())?;
    assert_eq!(profile.source_id(), "tuning-shared");
    Ok(())
}

#[test]
fn preserves_unresolved_profile_without_path_repair() -> Result<(), String> {
    let vehicle_catalog = vehicles();
    let tuning_catalog = tuning()?;
    let binding = bind(
        "addstagevehicle",
        &["car_a", "start", "chase", r"level05\M4chase.con"],
        VehicleTuningUsageScope::Stage,
        Some("m4"),
        Some(0),
        None,
        8,
        &vehicle_catalog,
        &tuning_catalog,
    )?
    .ok_or_else(|| "unresolved tuning usage disappeared".to_owned())?;
    assert_eq!(binding.con_file(), r"level05\M4chase.con");
    assert!(binding.tuning_source().is_none());
    Ok(())
}

#[test]
fn binds_reviewed_unscoped_init_and_chase_shapes() -> Result<(), String> {
    let vehicle_catalog = vehicles();
    let tuning_catalog = tuning()?;
    let init = bind(
        "initlevelplayervehicle",
        &["car_a", "start", "OTHER", r"level01\shared.con"],
        VehicleTuningUsageScope::Unscoped,
        None,
        None,
        None,
        3,
        &vehicle_catalog,
        &tuning_catalog,
    )?
    .ok_or_else(|| "player tuning usage disappeared".to_owned())?;
    let chase = bind(
        CREATE_CHASE_MANAGER_COMMAND,
        &["car_b", r"level01\shared.con", "3"],
        VehicleTuningUsageScope::Unscoped,
        None,
        None,
        None,
        7,
        &vehicle_catalog,
        &tuning_catalog,
    )?
    .ok_or_else(|| "chase tuning usage disappeared".to_owned())?;
    assert_eq!(init.vehicle().package_id(), "car-a");
    assert_eq!(chase.vehicle().package_id(), "car-b");
    assert_eq!(
        init.tuning_source().map(VehicleTuningSourceReference::source_id),
        chase.tuning_source().map(VehicleTuningSourceReference::source_id),
    );
    Ok(())
}


#[test]
fn preserves_objective_owned_stage_vehicle_usage() -> Result<(), String> {
    let vehicle_catalog = vehicles();
    let tuning_catalog = tuning()?;
    let binding = bind(
        "addstagevehicle",
        &["car_a", "start", "chase", r"level01\shared.con", "none"],
        VehicleTuningUsageScope::Objective,
        Some("m1"),
        Some(4),
        Some(23),
        24,
        &vehicle_catalog,
        &tuning_catalog,
    )?
    .ok_or_else(|| "objective tuning usage disappeared".to_owned())?;
    assert_eq!(binding.scope(), VehicleTuningUsageScope::Objective);
    assert_eq!(binding.owner_stage_sequence_ordinal(), Some(4));
    assert_eq!(binding.owner_objective_source_ordinal(), Some(23));
    assert_eq!(binding.source_ordinal(), 24);
    Ok(())
}

#[test]
fn rejects_scope_arity_vehicle_and_path_drift() -> Result<(), String> {
    let vehicle_catalog = vehicles();
    let tuning_catalog = tuning()?;
    let cases = [
        (
            "addstagevehicle",
            vec!["car_a", "start", "chase", r"level01\shared.con"],
            VehicleTuningUsageScope::Unscoped,
        ),
        (
            CREATE_CHASE_MANAGER_COMMAND,
            vec!["car_a", r"level01\shared.con"],
            VehicleTuningUsageScope::Unscoped,
        ),
        (
            CREATE_CHASE_MANAGER_COMMAND,
            vec!["missing", r"level01\shared.con", "3"],
            VehicleTuningUsageScope::Unscoped,
        ),
        (
            CREATE_CHASE_MANAGER_COMMAND,
            vec!["car_a", "../shared.con", "3"],
            VehicleTuningUsageScope::Unscoped,
        ),
    ];
    for (command, arguments, scope) in cases {
        if bind(
            command,
            &arguments,
            scope,
            None,
            None,
            None,
            1,
            &vehicle_catalog,
            &tuning_catalog,
        )
        .is_ok()
        {
            return Err(format!("drifted tuning usage was accepted: {command}"));
        }
    }
    Ok(())
}

fn production_tuning_row() -> String {
    concat!(
        "{\"package_id\":\"extracted-game-scripts-cars-missions-level01\",",
        "\"package_root\":\"extracted/game/scripts/cars/missions/level01\",",
        "\"package_category\":\"vehicle-tuning\",",
        "\"package_subcategory\":\"vehicle-tuning/mission/level-01\",",
        "\"unit_count\":1,\"text_key_count\":0,",
        "\"unit_ids\":[\"config-a\"],",
        "\"world_ids\":[],\"texture_ids\":[],",
        "\"material_ids\":[],\"model_ids\":[],",
        "\"physics_ids\":[],\"animation_ids\":[],",
        "\"scene_ids\":[],\"locator_ids\":[],",
        "\"camera_ids\":[],\"light_ids\":[],",
        "\"particle_ids\":[],\"controller_ids\":[],",
        "\"audio_ids\":[],\"movie_ids\":[],",
        "\"script_ids\":[],\"text_ids\":[],",
        "\"ui_ids\":[],\"metadata_ids\":[\"config-a\"],",
        "\"error_ids\":[],\"source_unit_ids\":[],",
        "\"text_key_ids\":[],",
        "\"members\":[{\"id\":\"config-a\",",
        "\"role\":\"metadata\",",
        "\"path\":\"extracted/game/scripts/cars/",
        "Missions/level01/M1race.con.json\",",
        "\"type\":\"config\",\"kind\":\"vehicle-tuning\",",
        "\"source_chunk_kind\":\"none\"}],",
        "\"text_keys\":[]}",
    )
    .to_owned()
}

fn production_tuning_index() -> Result<PhaseThreePackageIndex, String> {
    PhaseThreePackageIndex::from_jsonl(&production_tuning_row())
        .map_err(|error| error.to_string())
}

#[test]
fn production_catalog_binds_exact_normalized_member_path()
-> Result<(), String> {
    let index = production_tuning_index()?;
    let catalog = VehicleTuningSourceCatalog::from_package_index(&index)?;
    let source = catalog
        .resolve_optional(r"Missions\level01\M1race.con")?
        .ok_or_else(|| {
            "production tuning catalog did not resolve member".to_owned()
        })?;
    if source.source_id() != "config-a"
        || source.package_id()
            != "extracted-game-scripts-cars-missions-level01"
        || source.package_subcategory() != "vehicle-tuning/mission/level-01"
    {
        return Err(
            "production tuning catalog lost package provenance".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn production_catalog_rejects_tuning_taxonomy_drift() -> Result<(), String> {
    let base = production_tuning_row();
    let drifts = [
        base.replace(
            "\"package_subcategory\":\"vehicle-tuning/mission/level-01\"",
            "\"package_subcategory\":\"missions/level-01\"",
        ),
        base.replace(
            "\"metadata_ids\":[\"config-a\"]",
            "\"metadata_ids\":[]",
        )
        .replace(
            "\"script_ids\":[]",
            "\"script_ids\":[\"config-a\"]",
        )
        .replace(
            "\"role\":\"metadata\"",
            "\"role\":\"script\"",
        ),
        base.replace(
            "\"source_chunk_kind\":\"none\"",
            "\"source_chunk_kind\":\"mesh\"",
        ),
    ];
    for drift in drifts {
        let index = PhaseThreePackageIndex::from_jsonl(&drift)
            .map_err(|error| error.to_string())?;
        if VehicleTuningSourceCatalog::from_package_index(&index).is_ok() {
            return Err(
                "drifted tuning package taxonomy entered usage catalog"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

#[test]
fn production_catalog_accepts_scoped_tuning_taxonomy() -> Result<(), String> {
    let row = production_tuning_row().replace(
        "\"package_subcategory\":\"vehicle-tuning/mission/level-01\"",
        "\"package_subcategory\":\"missions/level-01/vehicle-tuning/m1\"",
    );
    let index = PhaseThreePackageIndex::from_jsonl(&row)
        .map_err(|error| error.to_string())?;
    let catalog = VehicleTuningSourceCatalog::from_package_index(&index)?;
    let source = catalog
        .resolve_optional(r"Missions\level01\M1race.con")?
        .ok_or_else(|| "scoped tuning taxonomy did not resolve".to_owned())?;
    if source.package_subcategory()
        != "missions/level-01/vehicle-tuning/m1"
    {
        return Err("scoped tuning taxonomy lost exact provenance".to_owned());
    }
    Ok(())
}
