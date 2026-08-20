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
//   - Material binding test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Material binding test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Material binding test module.

use fbx::domain::texture::{MaterialBinding, MaterialBindingError};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

#[test]
fn rejects_blank_material_binding_identities() {
    assert_eq!(
        MaterialBinding::new("   ", None),
        Err(MaterialBindingError::MissingMaterialName)
    );
    assert_eq!(
        MaterialBinding::new("material", Some("   ".to_owned())),
        Err(MaterialBindingError::BlankTextureFileName)
    );
}

#[test]
fn classifies_shared_glass_mirror_and_light_identities() -> Result<(), String> {
    let glass = MaterialBinding::new(
        "windsheild_glass_m",
        Some("headlight_lens.png".to_owned()),
    )
    .map_err(|error| format!("glass material failed: {error:?}"))?;
    if !glass.semantics.is_transparent()
        || !glass.semantics.is_glass()
        || !glass.semantics.is_light_emitter()
        || glass.semantics.is_mirror()
    {
        return Err(format!("glass semantics were incomplete: {glass:?}"));
    }
    let police_lights = MaterialBinding::new(
        "cPoliceLights_m",
        Some("police_lightbar.png".to_owned()),
    )
    .map_err(|error| format!("police lights failed: {error:?}"))?;
    if !police_lights.semantics.is_light_emitter()
        || police_lights.semantics.is_glass()
    {
        return Err(format!(
            "plural light semantics were incomplete: {police_lights:?}"
        ));
    }
    let mirror = MaterialBinding::new(
        "rearview_mirror_m",
        Some("rearview.png".to_owned()),
    )
    .map_err(|error| format!("mirror material failed: {error:?}"))?;
    if !mirror.semantics.is_mirror()
        || !mirror.semantics.is_reflective()
        || mirror.semantics.is_glass()
        || mirror.semantics.is_light_emitter()
    {
        return Err(format!("mirror semantics were incomplete: {mirror:?}"));
    }
    let chrome = MaterialBinding::new(
        "vehicle_chrome_m",
        Some("vehicle_chrome.png".to_owned()),
    )
    .map_err(|error| format!("reflective material failed: {error:?}"))?;
    if !chrome.semantics.is_reflective() || chrome.semantics.is_light_emitter()
    {
        return Err(format!(
            "reflective semantics were incomplete: {chrome:?}"
        ));
    }
    let fire_truck = MaterialBinding::new(
        "cFire_vBackNorm_m",
        Some("cFire_vBackNorm.png".to_owned()),
    )
    .map_err(|error| format!("fire-truck material failed: {error:?}"))?;
    if fire_truck.semantics.is_visual_effect() {
        return Err(format!("vehicle identity became VFX: {fire_truck:?}"));
    }
    let flame = MaterialBinding::new("flame_m", Some("fireseq.png".to_owned()))
        .map_err(|error| format!("VFX material failed: {error:?}"))?;
    if !flame.semantics.is_visual_effect() || flame.semantics.is_light_emitter()
    {
        return Err(format!("VFX semantics were incomplete: {flame:?}"));
    }
    Ok(())
}

#[test]
fn classifies_world_lamp_identities_without_false_light_matches()
-> Result<(), String> {
    for name in [
        "l1-parkinglight-shape",
        "l3-globelight-shape",
        "l4-light-shape",
        "downtown-streetlight",
    ] {
        let material = MaterialBinding::new(name, None)
            .map_err(|error| format!("world light failed: {error:?}"))?;
        if !material.semantics.is_light_emitter() {
            return Err(format!("world light was not emissive: {name}"));
        }
    }
    for name in [
        "dontlight-trackshape",
        "gens-relight-dayshape",
        "lighthouse-wall",
    ] {
        let material = MaterialBinding::new(name, None)
            .map_err(|error| format!("non-light identity failed: {error:?}"))?;
        if material.semantics.is_light_emitter() {
            return Err(format!("non-light identity became emissive: {name}"));
        }
    }
    Ok(())
}

#[test]
fn classifies_kwik_e_mart_ceiling_fixture_as_emissive() -> Result<(), String> {
    for (material_name, texture_name) in [
        ("int_kwik_light_m", None),
        ("material-content-addressed", Some("int_kwik_light.png")),
    ] {
        let material = MaterialBinding::new(
            material_name,
            texture_name.map(str::to_owned),
        )
        .map_err(|error| format!("Kwik-E-Mart light failed: {error:?}"))?;
        if !material.semantics.is_light_emitter() {
            return Err(format!(
                "Kwik-E-Mart ceiling fixture was not emissive: \
                     {material_name}"
            ));
        }
    }
    Ok(())
}
