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
//   - Grounding tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Grounding tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Grounding tests unit tests.

use fbx::domain::animation::{BoneAnimationTrack, LocalTransformSample};
use fbx::domain::mesh::PrimitiveGroup;
use fbx::domain::skeleton::Bone;
use fbx::domain::skin::SkinInfluence;

use super::*;

fn role(mesh: &str, shader: &str) -> &'static str {
    let semantics =
        vehicle_part_semantics(mesh, shader, MaterialSemantics::default());
    vehicle_part_role(mesh, shader, semantics)
}

fn wheel_part(
    name: &str,
    bone: &str,
    minimum_y: f32,
) -> Result<SkinnedPart, String> {
    let group = PrimitiveGroup::new(
        0,
        "wheel_m",
        vec![[0., minimum_y, 0.], [1., minimum_y + 1., 0.], [
            0.,
            minimum_y + 1.,
            1.,
        ]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("wheel group failed: {error:?}"))?;
    let mesh = MeshAsset::new(name, vec![group])
        .map_err(|error| format!("wheel mesh failed: {error:?}"))?;
    Ok(SkinnedPart {
        mesh,
        group_influences: vec![
            (0_u32..3)
                .map(|vertex_index| SkinInfluence {
                    vertex_index,
                    bone_id: bone.to_owned(),
                    weight: 1.,
                })
                .collect(),
        ],
    })
}

fn grounded_fixture() -> Result<CharacterAsset, String> {
    let mut bones = vec![Bone {
        id: "root".to_owned(),
        parent_id: None,
        rest_matrix: [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ],
        source_identity: None,
        source_rig: None,
    }];
    for wheel in ["w0", "w1", "w2", "w3"] {
        bones.push(Bone {
            id: wheel.to_owned(),
            parent_id: Some("root".to_owned()),
            rest_matrix: [
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
            source_identity: None,
            source_rig: None,
        });
    }
    let parts = ["w0", "w1", "w2", "w3"]
        .into_iter()
        .enumerate()
        .map(|(index, wheel)| {
            wheel_part(&format!("wheel-{index}"), wheel, -0.75)
        })
        .collect::<Result<Vec<_>, _>>()?;
    CharacterAsset::new("vehicle", bones, parts)
        .map_err(|error| format!("vehicle fixture failed: {error:?}"))
}

#[test]
fn selects_only_explicit_nonvisual_wheel_proxy_vehicles() -> Result<(), String>
{
    let asset = grounded_fixture()?;
    let frink_count = hidden_wheel_proxy_indices(&asset, "frink-v").len();
    let monorail_count = hidden_wheel_proxy_indices(&asset, "mono-v").len();
    let ordinary = hidden_wheel_proxy_indices(&asset, "snake-v");
    if frink_count != 4 || monorail_count != 4 || !ordinary.is_empty() {
        return Err(format!(
            concat!(
                "wheel proxy counts differ: frink={} ",
                "mono={} ordinary={}"
            ),
            frink_count,
            monorail_count,
            ordinary.len()
        ));
    }
    Ok(())
}

/// Return the minimum Y coordinate across selected parts.
fn minimum_y(parts: &[SkinnedPart]) -> f32 {
    parts
        .iter()
        .flat_map(|part| &part.mesh.groups)
        .flat_map(|group| &group.positions)
        .map(|position| position[1])
        .fold(f32::INFINITY, f32::min)
}

#[test]
fn monorail_body_grounding_preserves_proxy_surfaces() -> Result<(), String> {
    let mut asset = grounded_fixture()?;
    asset.parts.push(wheel_part("monorail-body", "root", 2.)?);
    let proxies = hidden_wheel_proxy_indices(&asset, "mono-v");
    let (grounded, offset, root) = ground_monorail_asset(asset, &proxies)
        .map_err(|error| format!("monorail grounding failed: {error:?}"))?;
    if root != "root" || (offset + 2f64).abs() > f64::EPSILON {
        return Err(format!(
            "unexpected monorail root result: {root} {offset}"
        ));
    }
    let proxy_parts = grounded.parts.get(..4).ok_or_else(|| {
        "monorail fixture has fewer than four proxies".to_owned()
    })?;
    let body = grounded
        .parts
        .get(4)
        .ok_or_else(|| "monorail fixture has no body part".to_owned())?;
    let proxy_minimum = minimum_y(proxy_parts);
    let body_minimum = minimum_y(std::slice::from_ref(body));
    if (proxy_minimum + 0.75_f32).abs() > f32::EPSILON
        || body_minimum.abs() > f32::EPSILON
    {
        return Err(format!(
            "monorail grounding changed surfaces: \
                 proxies={proxy_minimum} body={body_minimum}"
        ));
    }
    Ok(())
}

#[test]
fn grounds_all_vehicle_parts_and_root_from_road_wheels() -> Result<(), String> {
    let asset = grounded_fixture()?;
    let (grounded, offset, root) = ground_vehicle_asset(asset)
        .map_err(|error| format!("grounding failed: {error:?}"))?;
    if root != "root" || (offset - 0.75).abs() > f64::EPSILON {
        return Err(format!("unexpected grounding result: {root} {offset}"));
    }
    let minimum = grounded
        .parts
        .iter()
        .flat_map(|part| part.mesh.groups.iter())
        .flat_map(|group| group.positions.iter())
        .map(|position| position[1])
        .fold(f32::INFINITY, f32::min);
    if minimum.abs() > f32::EPSILON {
        return Err(format!("vehicle wheels are not grounded: {minimum}"));
    }
    let root_translation = grounded
        .bones
        .first()
        .and_then(|bone| bone.rest_matrix.get(13))
        .copied()
        .ok_or_else(|| "grounded fixture has no root translation".to_owned())?;
    if (root_translation - 0.75_f32).abs() > f32::EPSILON {
        return Err("vehicle root did not receive grounding offset".to_owned());
    }
    Ok(())
}

#[test]
fn grounds_root_animation_samples() -> Result<(), String> {
    let mut clips = vec![
        AnimationClip::new(
            "idle",
            30.,
            true,
            1,
            vec![BoneAnimationTrack {
                bone_id: "root".to_owned(),
                samples: vec![LocalTransformSample {
                    translation: [0f64, 0f64, 0f64],
                    rotation_wxyz: [1f64, 0f64, 0f64, 0f64],
                }],
            }],
            Vec::new(),
        )
        .map_err(|error| format!("animation fixture failed: {error:?}"))?,
    ];
    ground_vehicle_animations(&mut clips, "root", 0.75)
        .map_err(|error| format!("animation grounding failed: {error:?}"))?;
    let grounded_y = clips
        .first()
        .and_then(|clip| clip.tracks.first())
        .and_then(|track| track.samples.first())
        .and_then(|sample| sample.translation.get(1))
        .copied()
        .ok_or_else(|| {
            "grounded animation fixture has no Y sample".to_owned()
        })?;
    if (grounded_y - 0.75_f64).abs() > f64::EPSILON {
        return Err(
            "root animation did not receive grounding offset".to_owned()
        );
    }
    Ok(())
}

#[test]
fn classifies_vehicle_shader_automation_roles() {
    assert_eq!(role("DoorDRotShape", "window_glass_m"), "glass");
    assert_eq!(role("mirrorShape", "body_m"), "mirror");
    assert_eq!(role("brake1Shape", "brakeFlareA_m"), "light-emitter");
    assert_eq!(role("lightsShape", "cPoliceLights_m"), "light-emitter");
    assert_eq!(role("smokeShape__transparent-source", "smoke_m"), "vfx");
    assert_eq!(role("cFire_vShape", "cFire_vBackNorm_m"), "body");
    assert_eq!(role("chromeShape", "vehicle_chrome_m"), "reflective");
    assert_ne!(role("steeringwheelShape", "interior_m"), "wheel");
}

#[test]
fn hidden_proxy_sidecar_preserves_part_order() -> Result<(), String> {
    let mut asset = grounded_fixture()?;
    asset.parts.reverse();
    let proxies = hidden_wheel_proxy_indices(&asset, "hbike-v");
    let root = std::env::temp_dir().join(format!(
        "shar-hidden-proxy-order-{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    let result = mark_hidden_wheel_proxies(asset, &root, &proxies)
        .map_err(|error| format!("proxy sidecar failed: {error:?}"));
    let (_asset, _sidecars, count) = result?;
    if count != 4 {
        return Err(format!("unexpected hidden proxy count: {count}"));
    }
    let path = root.join("geometry/hidden-wheel-proxies.json");
    let value: Value = serde_json::from_slice(
        &fs::read(&path).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let bones = value
        .get("proxies")
        .and_then(Value::as_array)
        .ok_or_else(|| "proxy sidecar has no proxy array".to_owned())?
        .iter()
        .map(|proxy| {
            proxy
                .get("bones")
                .and_then(Value::as_array)
                .and_then(|bones| bones.first())
                .and_then(Value::as_str)
                .ok_or_else(|| "proxy sidecar has no bone identity".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let cleanup = fs::remove_dir_all(&root);
    cleanup.map_err(|error| error.to_string())?;
    if bones != ["w3", "w2", "w1", "w0"] {
        return Err(format!("hidden proxy source order changed: {bones:?}"));
    }
    Ok(())
}
