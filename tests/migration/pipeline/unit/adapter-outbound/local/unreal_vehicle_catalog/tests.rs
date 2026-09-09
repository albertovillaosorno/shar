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
//   - Generated vehicle FBX catalog verifier tests.
// - Must-Not:
//   - Read proprietary assets or contact Unreal Editor.
// - Allows:
//   - Synthetic binary FBX files and isolated filesystem fixtures.
// - Split-When:
//   - Split when vehicle catalog verification gains an independent lifecycle.
// - Merge-When:
//   - Merge when another adapter test owns identical vehicle evidence.
// - Summary:
//   - Generated vehicle catalog verifier tests.
// - Description:
//   - Proves exact FBX/physics verification and stale-byte rejection.
// - Usage:
//   - Included only by the owning local adapter under cfg(test).
// - Defaults:
//   - Missing catalogs remain absent and malformed evidence fails closed.
//

//! Generated vehicle catalog verifier tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::json;
use shar_sha256::digest_hex;

use super::{FBX_VERSION, verified_vehicle_fbx_catalog};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);

struct TempRoot(PathBuf);

impl TempRoot {
    fn new(label: &str) -> Result<Self, String> {
        let sequence = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = PathBuf::from(".temp").join(format!(
            "unreal-vehicle-catalog-{label}-{}-{sequence}",
            std::process::id()
        ));
        if path.exists() {
            fs::remove_dir_all(&path).map_err(|error| error.to_string())?;
        }
        Ok(Self(path))
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _result = fs::remove_dir_all(&self.0);
    }
}

fn fbx_bytes() -> Vec<u8> {
    let mut bytes = b"Kaydara FBX Binary  \0\x1a\0".to_vec();
    bytes.extend_from_slice(&FBX_VERSION.to_le_bytes());
    bytes.extend_from_slice(b"vehicle-fixture");
    bytes
}

fn collision_bytes() -> &'static [u8] {
    br#"{"schema":"simulation_collision_object","value":1}"#
}

fn physics_bytes() -> &'static [u8] {
    br#"{"schema":"simulation_physics_object","value":2}"#
}

fn write_catalog(
    root: &Path,
    declared_size: Option<u64>,
) -> Result<(), String> {
    let bytes = fbx_bytes();
    let vehicle_dir = root.join("sedana");
    fs::create_dir_all(&vehicle_dir).map_err(|error| error.to_string())?;
    fs::write(vehicle_dir.join("sedana.fbx"), &bytes)
        .map_err(|error| error.to_string())?;
    let physics_dir = vehicle_dir.join("physics");
    fs::create_dir_all(&physics_dir).map_err(|error| error.to_string())?;
    fs::write(
        physics_dir.join("collision__ordinal_000321.json"),
        collision_bytes(),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        physics_dir.join("physics__ordinal_000361.json"),
        physics_bytes(),
    )
    .map_err(|error| error.to_string())?;
    let catalog = json!({
        "schema": "shar.vehicle-catalog.v6",
        "boundary": {},
        "counts": {"vehicles": 1, "physics_sidecars": 2},
        "vehicles": [{
            "vehicle": "sedana",
            "package_id": "extracted-art-cars-sedana",
            "subcategory": "cars/traffic-variants/sedana",
            "fbx": {
                "path": "sedana/sedana.fbx",
                "bytes": declared_size.unwrap_or_else(|| {
                    u64::try_from(bytes.len()).unwrap_or(u64::MAX)
                }),
                "sha256": digest_hex(&bytes)
            },
            "physics_sidecars": [{
                "path": "physics/collision__ordinal_000321.json",
                "package_member_id": "physics-collision",
                "source_path": "extracted/art/cars/sedanA/components/\
simulation_collision_object/sedanA.json",
                "kind": "p3d-collision",
                "source_chunk_kind": "simulation_collision_object",
                "source_ordinal": 321,
                "bytes": collision_bytes().len(),
                "sha256": digest_hex(collision_bytes())
            }, {
                "path": "physics/physics__ordinal_000361.json",
                "package_member_id": "physics-object",
                "source_path": "extracted/art/cars/sedanA/components/\
simulation_physics_object/sedanA.json",
                "kind": "p3d-physics",
                "source_chunk_kind": "simulation_physics_object",
                "source_ordinal": 361,
                "bytes": physics_bytes().len(),
                "sha256": digest_hex(physics_bytes())
            }]
        }]
    });
    fs::write(
        root.join("vehicles.catalog.json"),
        serde_json::to_vec_pretty(&catalog).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())
}

#[test]
fn absent_vehicle_catalog_keeps_specialized_evidence_absent()
-> Result<(), String> {
    let root = TempRoot::new("absent")?;
    if verified_vehicle_fbx_catalog(&root.0)
        .map_err(|error| error.to_string())?
        .is_some()
    {
        return Err("absent vehicle catalog produced evidence".to_owned());
    }
    Ok(())
}

#[test]
fn verifies_vehicle_fbx_without_promoting_other_semantics()
-> Result<(), String> {
    let root = TempRoot::new("valid")?;
    write_catalog(&root.0, None)?;
    let rows = verified_vehicle_fbx_catalog(&root.0)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "vehicle evidence was absent".to_owned())?;
    let [row] = rows.as_slice() else {
        return Err("vehicle catalog returned wrong row count".to_owned());
    };
    let [collision, physics] = row.physics_sidecars.as_slice() else {
        return Err("verified vehicle physics evidence drifted".to_owned());
    };
    if row.evidence.package_id != "extracted-art-cars-sedana"
        || row.evidence.path != "vehicle-assets/sedana/sedana.fbx"
        || row.evidence.fbx_version != FBX_VERSION
        || row.subcategory != "cars/traffic-variants/sedana"
        || collision.source_ordinal != 321
        || physics.source_ordinal != 361
    {
        return Err("verified vehicle evidence drifted".to_owned());
    }
    Ok(())
}

#[test]
fn stale_vehicle_fbx_size_fails_closed() -> Result<(), String> {
    let root = TempRoot::new("stale-size")?;
    write_catalog(&root.0, Some(999))?;
    let error = match verified_vehicle_fbx_catalog(&root.0) {
        Ok(_value) => {
            return Err("stale vehicle size unexpectedly verified".to_owned());
        },
        Err(error) => error,
    };
    if !error.to_string().contains("bytes do not match") {
        return Err("stale size reported the wrong failure".to_owned());
    }
    Ok(())
}

#[test]
fn stale_vehicle_physics_bytes_fail_closed() -> Result<(), String> {
    let root = TempRoot::new("stale-physics")?;
    write_catalog(&root.0, None)?;
    fs::write(
        root.0
            .join("sedana/physics/collision__ordinal_000321.json"),
        br#"{"schema":"simulation_collision_object","value":9}"#,
    )
    .map_err(|error| error.to_string())?;
    let error = match verified_vehicle_fbx_catalog(&root.0) {
        Ok(_value) => {
            return Err(
                "stale vehicle physics unexpectedly verified".to_owned(),
            );
        },
        Err(error) => error,
    };
    if !error.to_string().contains("physics bytes do not match") {
        return Err("stale physics reported the wrong failure".to_owned());
    }
    Ok(())
}

#[test]
fn previous_vehicle_catalog_schema_fails_closed() -> Result<(), String> {
    let root = TempRoot::new("old-schema")?;
    write_catalog(&root.0, None)?;
    let path = root.0.join("vehicles.catalog.json");
    let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    fs::write(&path, text.replace("vehicle-catalog.v6", "vehicle-catalog.v5"))
        .map_err(|error| error.to_string())?;
    let error = match verified_vehicle_fbx_catalog(&root.0) {
        Ok(_value) => {
            return Err(
                "old vehicle catalog schema unexpectedly verified".to_owned(),
            );
        },
        Err(error) => error,
    };
    if !error.to_string().contains("schema is not supported") {
        return Err("old schema reported the wrong failure".to_owned());
    }
    Ok(())
}
