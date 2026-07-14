// File:
//   - cars.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/cars.rs
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
//   - The cars contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute cars.
// - Split-When:
//   - Split when cars contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Vehicle package classifier.
// - Description:
//   - Defines cars data and behavior for pipeline phase two minor units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs cars.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Vehicle package classifier.
//!
//! This boundary keeps vehicle package classifier explicit and returns
//! deterministic results to pipeline callers.

use super::index::{
    MinorUnitPackage, PackageCategory, category_from_root, package_id_tokens,
};

/// Supports the `classification_from_package` operation within this
/// deterministic classification boundary.
pub(super) fn classification_from_package(
    package: &MinorUnitPackage
) -> Option<(
    PackageCategory,
    String,
)> {
    let tokens = package_id_tokens(package);
    if category_from_root(&package.package_root) != PackageCategory::Cars
        && !tokens_identify_art_car_package(&tokens)
    {
        return None;
    }
    let car_index = tokens
        .iter()
        .position(|token| *token == "cars")?;
    let model = model_from_tokens(
        &tokens,
        car_index.saturating_add(1),
    )?;
    let family = vehicle_family(&model);
    Some(
        (
            PackageCategory::Cars,
            format!("cars/{family}/{model}"),
        ),
    )
}

/// Supports the `tokens_identify_art_car_package` operation within this
/// deterministic classification boundary.
fn tokens_identify_art_car_package(tokens: &[&str]) -> bool {
    tokens
        .windows(2)
        .any(
            |window| {
                window
                    == [
                        "art", "cars",
                    ]
            },
        )
}

/// Supports the `family_from_model_tokens` operation within this deterministic
/// classification boundary.
pub(super) fn family_from_model_tokens(
    tokens: &[&str],
    model_index: usize,
) -> Option<&'static str> {
    let model_token = model_from_tokens(
        tokens,
        model_index,
    )?;
    Some(vehicle_family(&model_token))
}

/// Supports the `model_from_tokens` operation within this deterministic
/// classification boundary.
pub(super) fn model_from_tokens(
    tokens: &[&str],
    model_index: usize,
) -> Option<String> {
    let model = tokens
        .get(model_index)
        .copied()?;
    if tokens
        .get(model_index.saturating_add(1))
        .is_some_and(|suffix| *suffix == "v")
    {
        Some(format!("{model}-v"))
    } else {
        Some(model.to_owned())
    }
}

/// Supports the `vehicle_family` operation within this deterministic
/// classification boundary.
fn vehicle_family(model_token: &str) -> &'static str {
    if model_token == "common" {
        "runtime-base"
    } else if service_vehicle_token(model_token) {
        "service-vehicles"
    } else if special_vehicle_token(model_token) {
        "special-vehicles"
    } else if traffic_variant_token(model_token) {
        "traffic-variants"
    } else if commercial_vehicle_token(model_token) {
        "commercial-vehicles"
    } else if character_vehicle_token(model_token) {
        "character-rigs"
    } else {
        "traffic-vehicles"
    }
}

/// Supports the `character_vehicle_token` operation within this deterministic
/// classification boundary.
fn character_vehicle_token(model_token: &str) -> bool {
    model_token.ends_with("-v")
}

/// Supports the `service_vehicle_token` operation within this deterministic
/// classification boundary.
fn service_vehicle_token(model_token: &str) -> bool {
    matches!(
        model_token,
        // cspell:disable-next-line -- ambul cfire cpolice schoolbu votetruc
        "ambul" | "cfire-v" | "cpolice" | "garbage" | "schoolbu" | "votetruc"
    )
}

/// Supports the `traffic_variant_token` operation within this deterministic
/// classification boundary.
fn traffic_variant_token(model_token: &str) -> bool {
    const TOKENS: &[&str] = &[
        "compacta", // cspell:disable-line -- compacta
        "minivana", // cspell:disable-line -- minivana
        "pickupa",  // cspell:disable-line -- pickupa
        "sedana",   // cspell:disable-line -- sedana
        "sedanb",   // cspell:disable-line -- sedanb
        "sportsa",  // cspell:disable-line -- sportsa
        "sportsb",  // cspell:disable-line -- sportsb
        "suva",     // cspell:disable-line -- suva
        "taxia",    // cspell:disable-line -- taxia
        "wagona",   // cspell:disable-line -- wagona
    ];
    TOKENS.contains(&model_token)
}

/// Supports the `special_vehicle_token` operation within this deterministic
/// classification boundary.
fn special_vehicle_token(model_token: &str) -> bool {
    #[rustfmt::skip]
    const TOKENS: &[&str] = &[
        "atv-v",
        "burnsarm", // cspell:disable-line -- burnsarm
        "cbone", // cspell:disable-line -- cbone
        "ccube", // cspell:disable-line -- ccube
        "cnonup", // cspell:disable-line -- cnonup
        "coffin",
        "dune-v",
        "hallo",
        "huska", // cspell:disable-line -- huska
        "mono-v",
        "oblit-v", // cspell:disable-line -- oblit
        "redbrick",
        "rocke-v", // cspell:disable-line -- rocke
        "ship",
        "tt",
        "witchcar", // cspell:disable-line -- witchcar
    ];
    TOKENS.contains(&model_token)
}

/// Supports the `commercial_vehicle_token` operation within this deterministic
/// classification boundary.
fn commercial_vehicle_token(model_token: &str) -> bool {
    const TOKENS: &[&str] = &[
        "carmor",   // cspell:disable-line -- carmor
        "ccola",    // cspell:disable-line -- ccola
        "ccurator", // cspell:disable-line -- ccurator
        "cdonut",   // cspell:disable-line -- cdonut
        "cduff",    // cspell:disable-line -- cduff
        "chears",   // cspell:disable-line -- chears
        "cklimo",   // cspell:disable-line -- cklimo
        "climo",    // cspell:disable-line -- climo
        "cmilk",    // cspell:disable-line -- cmilk
        "cvan",     // cspell:disable-line -- cvan
        "fishtruc", // cspell:disable-line -- fishtruc
        "glastruc", // cspell:disable-line -- glastruc
        "icecream", // cspell:disable-line -- icecream
        "istruck",  // cspell:disable-line -- istruck
        "nuctruck", // cspell:disable-line -- nuctruck
        "pizza",
    ];
    // cspell:disable-next-line -- truc
    model_token.contains("truc") || TOKENS.contains(&model_token)
}

#[cfg(test)]
mod tests {
    use super::vehicle_family;

    #[test]
    fn classifies_character_vehicle_rigs() {
        assert_eq!(
            vehicle_family("apu-v"),
            "character-rigs"
        );
        assert_eq!(
            vehicle_family("homer-v"),
            "character-rigs"
        );
    }

    #[test]
    fn classifies_vehicle_support_families() {
        assert_eq!(
            vehicle_family("common"),
            "runtime-base"
        );
        assert_eq!(
            // cspell:disable-next-line -- ambul
            vehicle_family("ambul"),
            "service-vehicles"
        );
        assert_eq!(
            // cspell:disable-next-line -- sedana
            vehicle_family("sedana"),
            "traffic-variants"
        );
        assert_eq!(
            // cspell:disable-next-line -- ccola
            vehicle_family("ccola"),
            "commercial-vehicles"
        );
        assert_eq!(
            vehicle_family("tt"),
            "special-vehicles"
        );
    }
}
