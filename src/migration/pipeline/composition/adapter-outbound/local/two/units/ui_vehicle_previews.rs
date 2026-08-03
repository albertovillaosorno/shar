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
//   - Ui vehicle previews outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Ui vehicle previews outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Ui vehicle previews outbound adapter.

use super::cars;
use super::index::{
    MinorUnitPackage, PackageCategory, category_from_root, package_id_tokens,
};

/// Supports the `classification_from_package` operation within this
/// deterministic classification boundary.
pub(super) fn classification_from_package(
    package: &MinorUnitPackage,
) -> Option<(PackageCategory, String)> {
    let tokens = package_id_tokens(package);
    if category_from_root(&package.package_root)
        != PackageCategory::UiVehiclePreviews
        && !tokens_identify_preview_package(&tokens)
    {
        return None;
    }
    let car_index = tokens.iter().position(|token| *token == "cars")?;
    let subcategory =
        cars::model_from_tokens(&tokens, car_index.saturating_add(1))
            .map_or_else(
                || "ui-vehicle-previews/source-metadata".to_owned(),
                |model| {
                    let family = cars::family_from_model_tokens(
                        &tokens,
                        car_index.saturating_add(1),
                    )
                    .unwrap_or("source-metadata");
                    format!("ui-vehicle-previews/{family}/{model}")
                },
            );
    Some((PackageCategory::UiVehiclePreviews, subcategory))
}

/// Supports the `tokens_identify_preview_package` operation within this
/// deterministic classification boundary.
fn tokens_identify_preview_package(tokens: &[&str]) -> bool {
    tokens
        .windows(2)
        .any(|window| window == ["dynaload", "cars"])
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/units/ui_vehicle_previews/tests.rs"]
mod tests;
