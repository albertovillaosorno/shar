// File:
//   - binary_uv_policy.rs
// Path:
//   - src/fbx/src/adapters/driven/binary_uv_policy.rs
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
//   - Conservative Pure3D-to-FBX horizontal UV correction selection.
// - Must-Not:
//   - Read image pixels, mutate geometry, or infer unnamed artistic intent.
// - Allows:
//   - Exact semantic-role and authored identity evidence.
// - Summary:
//   - Mirrors U only for source-backed decals, liveries, signs, and displays.
//
// Large file:
//   - false
//

//! Conservative horizontal UV correction for binary FBX geometry groups.

/// Return whether one geometry group requires the horizontal decal correction.
#[must_use]
pub(super) fn mirrors_u(
    mesh_name: &str,
    material_name: &str,
    texture_file_name: Option<&str>,
) -> bool {
    let mut evidence = format!(
        "{} {}",
        mesh_name.to_ascii_lowercase(),
        material_name.to_ascii_lowercase(),
    );
    if let Some(texture) = texture_file_name {
        evidence.push(' ');
        evidence.push_str(&texture.to_ascii_lowercase());
    }

    if contains_any(
        &evidence,
        &[
            "hidden-wheel-proxy",
            "__wheel",
            "wheel",
            "tire",
            "tyre",
            "rim",
            "hubcap",
            "__glass",
            "windshield",
            "windsheild",
            "windscreen",
            "lens",
            "__light-emitter",
            "headlight",
            "taillight",
            "brakelight",
            "reverse-light",
            "flare",
            "glow",
            "__vfx",
            "smoke",
            "flame",
            "backfire",
            "exhaust",
            "particle",
            "__interior",
            "char_swatches",
            "eyeball",
            "pupil",
        ],
    ) {
        return false;
    }

    contains_any(
        &evidence,
        &[
            "decal",
            "logo",
            "sign",
            "poster",
            "billboard",
            "banner",
            "label",
            "license",
            "licence",
            "plate",
            "advert",
            "graffiti",
            "newspaper",
            "magazine",
            "menu",
            "lettering",
            "text",
            "sticker",
            "mural",
            "picture",
            "screen",
            "display",
            "monitor",
            "phone",
            "card",
            "photo",
            "photograph",
            "portrait",
            "comic",
            "map-sign",
            "map_label",
            "icon",
            "livery",
            "police",
            "ambul",
            "taxi",
            "schoolbus",
            "school-bus",
            "news-van",
            "cola",
            "duff",
            "pizza",
        ],
    )
}

/// Return whether normalized evidence contains any exact conservative token.
fn contains_any(
    evidence: &str,
    tokens: &[&str],
) -> bool {
    tokens
        .iter()
        .any(|token| evidence.contains(token))
}

#[cfg(test)]
mod tests {
    use super::mirrors_u;

    #[test]
    fn mirrors_vehicle_liveries_but_not_plain_paint_wheels_or_effects() {
        assert!(
            mirrors_u(
                "ambulshape__body",
                "ambul_m",
                Some("ambul.png"),
            )
        );
        assert!(
            mirrors_u(
                "cColaDoorDShape__driver-door",
                "cColaDoorDNorm_m",
                Some("cColaDoorDNorm.png"),
            )
        );
        assert!(
            !mirrors_u(
                "snake-vshape__body",
                "snake_vPaint_m",
                Some("snake_vPaint.png"),
            )
        );
        assert!(
            !mirrors_u(
                "wshape__wheel",
                "cCola_Wheel_m",
                Some("cCola_Wheel.png"),
            )
        );
        assert!(
            !mirrors_u(
                "honor-vshape__body",
                "honor_vWheel_m",
                Some("honor_vWheel.png"),
            )
        );
        assert!(
            !mirrors_u(
                "backfireflashgroupshape__vfx",
                "brakeFlareA_m",
                Some("brakeFlareA.png"),
            )
        );
    }

    #[test]
    fn mirrors_named_world_and_prop_graphics_only() {
        assert!(
            mirrors_u(
                "kwik-e-mart-sign",
                "store_sign_m",
                Some("store_sign.png"),
            )
        );
        assert!(
            mirrors_u(
                "phone-screen",
                "phone_icon_m",
                Some("phone_icon.png"),
            )
        );
        assert!(
            !mirrors_u(
                "terrain-patch",
                "grass_m",
                Some("grass.png"),
            )
        );
        assert!(
            !mirrors_u(
                "frink_h_merged_3__glass",
                "eyeglass3_m",
                Some("eyeglass3.png"),
            )
        );
    }
}
