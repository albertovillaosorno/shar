// File:
//   - movement_records_tests.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     movement_records_tests.rs
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
//   - Synthetic non-mesh coordinate movement regression coverage.
// - Must-Not:
//   - Depend on extracted game files or manual FBX evidence.
// - Allows:
//   - Create isolated decoded locator, light, and physics fixtures.
// - Summary:
//   - Verifies one movement reaches every currently decoded record family.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Tests decoded coordinate movement without private or generated assets.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::collect_moved_records;
use crate::domain::coordinate_movement::{
    CoordinateMovement, CoordinateSubject,
};

/// Unique synthetic-fixture nonce.
static NONCE: AtomicU64 = AtomicU64::new(0);
/// Representative movement subjects.
const SUBJECTS: &[CoordinateSubject] = &[
    CoordinateSubject::Geometry,
    CoordinateSubject::Collision,
    CoordinateSubject::Trigger,
    CoordinateSubject::Camera,
    CoordinateSubject::Locator,
    CoordinateSubject::Light,
];
/// Representative reflection and translation.
const MOVEMENT: CoordinateMovement = CoordinateMovement::new(
    "fixture-movement",
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 10.0,
        20.0, 30.0, 1.0,
    ],
    SUBJECTS,
);

/// Isolated decoded package removed after one test.
struct Fixture {
    /// Synthetic package root.
    root: PathBuf,
}

impl Fixture {
    /// Create one complete decoded coordinate fixture.
    fn create() -> Result<Self, String> {
        let nonce = NONCE.fetch_add(
            1,
            Ordering::Relaxed,
        );
        let root = std::env::temp_dir().join(
            format!(
                "pipeline-movement-records-{}-{nonce}",
                std::process::id()
            ),
        );
        match fs::remove_dir_all(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }
        write(
            &root,
            "components/srr_locator/camera.json",
            locator_json(),
        )?;
        write(
            &root,
            "components/light/light.json",
            light_json(),
        )?;
        write(
            &root,
            "components/srr_static_phys_dsg/physics.json",
            physics_json(),
        )?;
        Ok(
            Self {
                root,
            },
        )
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.root));
    }
}

#[test]
fn movement_reaches_locator_trigger_light_and_physics_records()
-> Result<(), String> {
    let fixture = Fixture::create()?;
    let records = collect_moved_records(
        &fixture.root,
        MOVEMENT,
    )
    .map_err(|error| error.to_string())?;
    let counts = records
        .iter()
        .fold(
            std::collections::BTreeMap::<&str, usize>::new(),
            |mut counts, record| {
                let count = counts
                    .entry(&record.subject)
                    .or_default();
                *count = count.saturating_add(1);
                counts
            },
        );
    for (subject, expected) in [
        (
            "camera", 4_usize,
        ),
        (
            "trigger", 2_usize,
        ),
        (
            "light", 2_usize,
        ),
        (
            "collision",
            4_usize,
        ),
    ] {
        if counts
            .get(subject)
            .copied()
            != Some(expected)
        {
            return Err(
                format!(
                    "movement record count changed for {subject}: {counts:?}"
                ),
            );
        }
    }
    let camera = records
        .iter()
        .find(
            |record| {
                record.subject == "camera"
                    && record
                        .source_position
                        .is_some()
            },
        )
        .ok_or_else(|| String::from("camera movement record is missing"))?;
    if camera.moved_position
        != Some(
            [
                11.0, 22.0, 27.0,
            ],
        )
    {
        return Err(
            format!(
                "camera position moved incorrectly: {:?}",
                camera.moved_position
            ),
        );
    }
    let trigger_matrix = records
        .iter()
        .find(
            |record| {
                record
                    .moved_matrix
                    .is_some()
            },
        )
        .ok_or_else(|| String::from("trigger matrix record is missing"))?;
    let moved_matrix = trigger_matrix
        .moved_matrix
        .ok_or_else(|| String::from("trigger matrix value is missing"))?;
    if moved_matrix[12..15]
        != [
            14.0, 25.0, 24.0,
        ]
    {
        return Err(
            format!("trigger matrix moved incorrectly: {moved_matrix:?}"),
        );
    }
    Ok(())
}

/// Write one synthetic decoded component.
fn write(
    root: &std::path::Path,
    relative: &str,
    value: &'static str,
) -> Result<(), String> {
    let path = root.join(relative);
    let parent = path
        .parent()
        .ok_or_else(|| String::from("fixture parent is missing"))?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::write(
        path, value,
    )
    .map_err(|error| error.to_string())
}

/// Synthetic camera locator with basis and trigger evidence.
const fn locator_json() -> &'static str {
    r#"{
      "name":"int_kwik_camera01",
      "position":[1,2,3],
      "data_interpretation":{"basis":[[1,0,0],[0,1,0],[0,0,1]]},
      "trigger_volumes":[{
        "name":"camera-trigger",
        "position":[4,5,6],
        "matrix":[[1,0,0,0],[0,1,0,0],[0,0,1,0],[4,5,6,1]]
      }]
    }"#
}

/// Synthetic light with point and direction evidence.
const fn light_json() -> &'static str {
    r#"{
      "name":"fixture-light",
      "extras":[
        {"kind":"position","value":[7,8,9]},
        {"kind":"direction","value":[0,0,1]}
      ]
    }"#
}

/// Synthetic static-physics oriented box.
const fn physics_json() -> &'static str {
    r#"{
      "name":"fixture-physics",
      "collision_objects":[{
        "volumes":[{
          "primitives":[{
            "kind":"obbox",
            "vectors":[[10,11,12],[1,0,0],[0,1,0],[0,0,1]]
          }]
        }]
      }]
    }"#
}
