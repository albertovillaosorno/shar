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
//   - Cars outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Cars outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Cars outbound adapter.

use super::index::{
    MinorUnitPackage, PackageCategory, category_from_root, package_id_tokens,
};

/// Supports the `classification_from_package` operation within this
/// deterministic classification boundary.
pub(super) fn classification_from_package(
    package: &MinorUnitPackage,
) -> Option<(PackageCategory, String)> {
    let tokens = package_id_tokens(package);
    if category_from_root(&package.package_root) != PackageCategory::Cars
        && !tokens_identify_art_car_package(&tokens)
    {
        return None;
    }
    let car_index = tokens.iter().position(|token| *token == "cars")?;
    let model = model_from_tokens(&tokens, car_index.saturating_add(1))?;
    let family = vehicle_family(&model);
    Some((PackageCategory::Cars, format!("cars/{family}/{model}")))
}

/// Supports the `tokens_identify_art_car_package` operation within this
/// deterministic classification boundary.
fn tokens_identify_art_car_package(tokens: &[&str]) -> bool {
    tokens.windows(2).any(|window| window == ["art", "cars"])
}

/// Supports the `family_from_model_tokens` operation within this deterministic
/// classification boundary.
pub(super) fn family_from_model_tokens(
    tokens: &[&str],
    model_index: usize,
) -> Option<&'static str> {
    let model_token = model_from_tokens(tokens, model_index)?;
    Some(vehicle_family(&model_token))
}

/// Supports the `model_from_tokens` operation within this deterministic
/// classification boundary.
pub(super) fn model_from_tokens(
    tokens: &[&str],
    model_index: usize,
) -> Option<String> {
    let model = tokens.get(model_index).copied()?;
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
        "ambul" | "cfire-v" | "cpolice" | "garbage" | "schoolbu" | "votetruc"
    )
}

/// Supports the `traffic_variant_token` operation within this deterministic
/// classification boundary.
fn traffic_variant_token(model_token: &str) -> bool {
    const TOKENS: &[&str] = &[
        "compacta", "minivana", "pickupa", "sedana", "sedanb", "sportsa",
        "sportsb", "suva", "taxia", "wagona",
    ];
    TOKENS.contains(&model_token)
}

/// Supports the `special_vehicle_token` operation within this deterministic
/// classification boundary.
fn special_vehicle_token(model_token: &str) -> bool {
    #[rustfmt::skip]
    const TOKENS: &[&str] = &[
        "atv-v",
        "burnsarm",
        "cbone",
        "ccube",
        "cnonup",
        "coffin",
        "dune-v",
        "hallo",
        "huska",
        "mono-v",
        "oblit-v",
        "redbrick",
        "rocke-v",
        "ship",
        "tt",
        "witchcar",
    ];
    TOKENS.contains(&model_token)
}

/// Supports the `commercial_vehicle_token` operation within this deterministic
/// classification boundary.
fn commercial_vehicle_token(model_token: &str) -> bool {
    const TOKENS: &[&str] = &[
        "carmor", "ccola", "ccurator", "cdonut", "cduff", "chears", "cklimo",
        "climo", "cmilk", "cvan", "fishtruc", "glastruc", "icecream",
        "istruck", "nuctruck", "pizza",
    ];
    model_token.contains("truc") || TOKENS.contains(&model_token)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/units/cars/tests.rs"]
mod tests;
