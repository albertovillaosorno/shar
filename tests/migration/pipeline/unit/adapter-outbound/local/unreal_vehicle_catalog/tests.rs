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
    br#"{"schema":"simulation_collision_object","name":"sedanA","value":1}"#
}

fn physics_bytes() -> &'static [u8] {
    br#"{"schema":"simulation_physics_object","name":"sedanA","value":2}"#
}

fn shader_bytes() -> &'static [u8] {
    br#"{"schema":"shader","name":"sedanA_m","version":0,
"pddi_shader_name":"simple\u0000\u0000","has_translucency":0,
"vertex_needs":17,"vertex_mask":1,"num_params":11,"params":[
{"kind":"int","param":"LIT","value":1},
{"kind":"int","param":"2SID","value":1},
{"kind":"int","param":"BLMD","value":0},
{"kind":"int","param":"A\u0043M\u0050","value":4},
{"kind":"int","param":"A\u0054S\u0054","value":0},
{"kind":"float","param":"ACTH","value":0.5},
{"kind":"colour","param":"DIFF","value":4294967295},
{"kind":"colour","param":"A\u004dB\u0049","value":4278190080},
{"kind":"colour","param":"EMIS","value":4278190080},
{"kind":"colour","param":"SPEC","value":4278190080},
{"kind":"float","param":"SHIN","value":10.0}]}"#
}

fn texture_bytes() -> &'static [u8] {
    b"png-material-fixture"
}

fn headlight_shader_bytes() -> Vec<u8> {
    String::from_utf8_lossy(shader_bytes())
        .replace("sedanA_m", "headlight_m")
        .replace(
            r#""param":"LIT","value":1"#,
            r#""param":"LIT","value":0"#,
        )
        .into_bytes()
}

fn headlight_bytes() -> &'static [u8] {
    br#"{"schema":"quad_group","version":0,"name":"headlightShape",
"shader":"headlight_m","z_test":1,"z_write":0,"fog":0,"num_quads":1,
"quads":[{"name":"quad","version":2,"billboard_mode":"ALL_AXIS",
"translation":[0.0,0.0,0.0],"colour":4294967295,
"uvs":[[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,1.0]],
"width":1.0,"height":1.0,"distance":0.0,"uv_offset":[0.0,0.0],
"display_info_version":null,"rotation_wxyz":[1.0,0.0,0.0,0.0],
"cutoff_mode":"none","uv_offset_range":[0.0,0.0],"source_range":0.0,
"edge_range":0.0,"perspective_info_version":null,"perspective":false}]}"#
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
    let shader_dir = vehicle_dir.join("shaders");
    fs::create_dir_all(&shader_dir).map_err(|error| error.to_string())?;
    fs::write(shader_dir.join("sedana-m.json"), shader_bytes())
        .map_err(|error| error.to_string())?;
    let headlight_shader = headlight_shader_bytes();
    fs::write(shader_dir.join("headlight-m.json"), &headlight_shader)
        .map_err(|error| error.to_string())?;
    let texture_dir = vehicle_dir.join("textures");
    fs::create_dir_all(&texture_dir).map_err(|error| error.to_string())?;
    fs::write(texture_dir.join("sedanA.png"), texture_bytes())
        .map_err(|error| error.to_string())?;
    let headlight_dir = vehicle_dir.join("presentation").join("headlights");
    fs::create_dir_all(&headlight_dir).map_err(|error| error.to_string())?;
    fs::write(headlight_dir.join("headlight.json"), headlight_bytes())
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
        "schema": "shar.vehicle-catalog.v8",
        "boundary": {},
        "counts": {
            "vehicles": 1,
            "material_slots": 1,
            "parts": 1,
            "headlight_billboard_sidecars": 1,
            "physics_sidecars": 2,
            "physics_rigs": 1,
            "physics_primitives": 1
        },
        "vehicles": [{
            "vehicle": "sedana",
            "package_id": "extracted-art-cars-sedana",
            "subcategory": "cars/traffic-variants/sedana",
            "fbx": {
                "path": "sedana/sedana.fbx",
                "bytes": declared_size.unwrap_or_else(|| {
                    u64::try_from(bytes.len()).unwrap_or(u64::MAX)
                }),
                "sha256": digest_hex(&bytes),
                "materials": 1
            },
            "material_slots": [{
                "slot_name": "sedanA_m",
                "source_material_name": "sedanA_m",
                "base_color_rgba8": [255, 255, 255, 255],
                "surface_semantics": {
                    "transparent": false,
                    "glass": false,
                    "mirror": false,
                    "reflective": false,
                    "light_emitter": false,
                    "visual_effect": false
                },
                "shader": {
                    "path": "shaders/sedana-m.json",
                    "bytes": shader_bytes().len(),
                    "sha256": digest_hex(shader_bytes())
                },
                "texture": {
                    "path": "textures/sedanA.png",
                    "bytes": texture_bytes().len(),
                    "sha256": digest_hex(texture_bytes())
                }
            }],
            "parts": [{
                "name": "vehicle-body-part",
                "source_mesh": "vehicle-body-mesh",
                "role": "body",
                "surface_semantics": [],
                "shader": "sedanA_m",
                "bones": ["sedanA"]
            }],
            "headlight_billboard_sidecars": [{
                "path": "presentation/headlights/headlight.json",
                "identity": "headlightShape",
                "shader_identity": "headlight_m",
                "bones": ["hll", "hlr"],
                "material": {
                    "source_material_name": "headlight_m",
                    "base_color_rgba8": [255, 255, 255, 255],
                    "surface_semantics": {
                        "transparent": true,
                        "glass": false,
                        "mirror": false,
                        "reflective": false,
                        "light_emitter": true,
                        "visual_effect": false
                    },
                    "shader": {
                        "path": "shaders/headlight-m.json",
                        "bytes": headlight_shader.len(),
                        "sha256": digest_hex(&headlight_shader)
                    },
                    "texture": null
                },
                "bytes": headlight_bytes().len(),
                "sha256": digest_hex(headlight_bytes())
            }],
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
            }],
            "physics_rigs": [{
                "identity": "sedanA",
                "coordinate_space": "source-bone-local",
                "unit": "meter",
                "joint_count": 2,
                "primitives": [{
                    "kind": "sphere",
                    "bone_name": "w0",
                    "center_m": [0.0, 0.0, 0.0],
                    "radius_m": 0.5
                }]
            }]
        }]
    });
    fs::write(
        root.join("vehicles.catalog.json"),
        serde_json::to_vec_pretty(&catalog).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())
}


fn rewrite_headlight_bones(root: &Path, bones: &[&str]) -> Result<(), String> {
    let path = root.join("vehicles.catalog.json");
    let bytes = fs::read(&path).map_err(|error| error.to_string())?;
    let mut catalog: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    let vehicles = catalog
        .get_mut("vehicles")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "fixture vehicles are missing".to_owned())?;
    let vehicle = vehicles
        .first_mut()
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| "fixture vehicle is missing".to_owned())?;
    let sidecars = vehicle
        .get_mut("headlight_billboard_sidecars")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "fixture headlight sidecars are missing".to_owned())?;
    if bones.is_empty() {
        sidecars.clear();
    } else {
        let sidecar = sidecars
            .first_mut()
            .and_then(serde_json::Value::as_object_mut)
            .ok_or_else(|| "fixture headlight sidecar is missing".to_owned())?;
        let _previous = sidecar.insert("bones".to_owned(), json!(bones));
    }
    let counts = catalog
        .get_mut("counts")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| "fixture counts are missing".to_owned())?;
    let _previous = counts.insert(
        "headlight_billboard_sidecars".to_owned(),
        json!(usize::from(!bones.is_empty())),
    );
    let rendered =
        serde_json::to_vec_pretty(&catalog).map_err(|error| error.to_string())?;
    fs::write(path, rendered).map_err(|error| error.to_string())
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
    let [headlight] = row.headlight_billboard_sidecars.as_slice() else {
        return Err("verified headlight billboard evidence drifted".to_owned());
    };
    let [collision, physics] = row.physics_sidecars.as_slice() else {
        return Err("verified vehicle physics evidence drifted".to_owned());
    };
    let [material] = row.material_slots.as_slice() else {
        return Err("verified vehicle material evidence drifted".to_owned());
    };
    if row.evidence.package_id != "extracted-art-cars-sedana"
        || row.evidence.path != "vehicle-assets/sedana/sedana.fbx"
        || row.evidence.fbx_version != FBX_VERSION
        || row.subcategory != "cars/traffic-variants/sedana"
        || headlight.identity != "headlightShape"
        || headlight.shader_identity != "headlight_m"
        || headlight.bones != ["hll", "hlr"]
        || headlight.material.source_material_name != "headlight_m"
        || headlight.material.raster.lit
        || !headlight.material.semantics.light_emitter
        || collision.source_ordinal != 321
        || physics.source_ordinal != 361
        || material.source_material_name != "sedanA_m"
        || material.raster.shader_family != "simple"
        || !material.raster.lit
        || !material.raster.two_sided
        || material.raster.blend_mode != 0
        || material.raster.alpha_compare != 4
        || material.raster.diffuse_rgba8 != [255, 255, 255, 255]
        || material.raster.ambient_rgba8 != [0, 0, 0, 255]
        || material.raster.shininess_bits != 10.0_f32.to_bits()
    {
        return Err("verified vehicle evidence drifted".to_owned());
    }
    Ok(())
}


#[test]
fn accepts_single_authored_headlight_hardpoint() -> Result<(), String> {
    for bones in [["hll"].as_slice(), ["hlr"].as_slice()] {
        let root = TempRoot::new("single-headlight-hardpoint")?;
        write_catalog(&root.0, None)?;
        rewrite_headlight_bones(&root.0, bones)?;
        let rows = verified_vehicle_fbx_catalog(&root.0)
            .map_err(|error| error.to_string())?
            .ok_or_else(|| "vehicle evidence was absent".to_owned())?;
        let [row] = rows.as_slice() else {
            return Err("single-hardpoint fixture row count drifted".to_owned());
        };
        let [headlight] = row.headlight_billboard_sidecars.as_slice() else {
            return Err("single-hardpoint sidecar was not retained".to_owned());
        };
        if headlight.bones != bones {
            return Err(format!(
                "single hardpoint changed: {:?}",
                headlight.bones
            ));
        }
    }
    Ok(())
}

#[test]
fn accepts_vehicle_without_headlight_hardpoints() -> Result<(), String> {
    let root = TempRoot::new("no-headlight-hardpoints")?;
    write_catalog(&root.0, None)?;
    rewrite_headlight_bones(&root.0, &[])?;
    let rows = verified_vehicle_fbx_catalog(&root.0)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "vehicle evidence was absent".to_owned())?;
    let [row] = rows.as_slice() else {
        return Err("no-hardpoint fixture row count drifted".to_owned());
    };
    if !row.headlight_billboard_sidecars.is_empty() {
        return Err("headlight sidecar survived without hardpoints".to_owned());
    }
    Ok(())
}

#[test]
fn noncanonical_headlight_hardpoints_fail_closed() -> Result<(), String> {
    for bones in [
        ["hlr", "hll"].as_slice(),
        ["hll", "hll"].as_slice(),
        ["hll", "unknown"].as_slice(),
    ] {
        let root = TempRoot::new("bad-headlight-hardpoints")?;
        write_catalog(&root.0, None)?;
        rewrite_headlight_bones(&root.0, bones)?;
        let error = match verified_vehicle_fbx_catalog(&root.0) {
            Ok(_value) => {
                return Err(format!(
                    "noncanonical hardpoints unexpectedly verified: {bones:?}"
                ));
            },
            Err(error) => error,
        };
        if !error.to_string().contains("bones are not canonical") {
            return Err(format!(
                "noncanonical hardpoints reported wrong failure: {error}"
            ));
        }
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
fn stale_headlight_billboard_bytes_fail_closed() -> Result<(), String> {
    let root = TempRoot::new("stale-headlight")?;
    write_catalog(&root.0, None)?;
    fs::write(
        root.0.join("sedana/presentation/headlights/headlight.json"),
        b"{}",
    )
    .map_err(|error| error.to_string())?;
    let error = match verified_vehicle_fbx_catalog(&root.0) {
        Ok(_value) => {
            return Err(
                "stale headlight billboard unexpectedly verified".to_owned(),
            );
        },
        Err(error) => error,
    };
    if !error.to_string().contains("headlight billboard bytes") {
        return Err("stale headlight reported the wrong failure".to_owned());
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
fn stale_vehicle_material_shader_bytes_fail_closed() -> Result<(), String> {
    let root = TempRoot::new("stale-material")?;
    write_catalog(&root.0, None)?;
    fs::write(
        root.0.join("sedana/shaders/sedana-m.json"),
        br#"{"schema":"shader","name":"sedanA_m","value":9}"#,
    )
    .map_err(|error| error.to_string())?;
    let error = match verified_vehicle_fbx_catalog(&root.0) {
        Ok(_value) => {
            return Err(
                "stale vehicle material unexpectedly verified".to_owned(),
            );
        },
        Err(error) => error,
    };
    if !error.to_string().contains("material artifact bytes do not match") {
        return Err("stale material reported the wrong failure".to_owned());
    }
    Ok(())
}

#[test]
fn previous_vehicle_catalog_schema_fails_closed() -> Result<(), String> {
    let root = TempRoot::new("old-schema")?;
    write_catalog(&root.0, None)?;
    let path = root.0.join("vehicles.catalog.json");
    let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    fs::write(&path, text.replace("vehicle-catalog.v8", "vehicle-catalog.v7"))
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
