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
//   - Binary uv policy outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Binary uv policy outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Binary uv policy outbound adapter.

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

    if contains_any(&evidence, &[
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
    ]) {
        return false;
    }

    contains_any(&evidence, &[
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
    ])
}

/// Return whether normalized evidence contains any exact conservative token.
fn contains_any(evidence: &str, tokens: &[&str]) -> bool {
    tokens.iter().any(|token| evidence.contains(token))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/fbx/unit/adapter-outbound/binary_uv_policy/tests.rs"]
mod tests;
