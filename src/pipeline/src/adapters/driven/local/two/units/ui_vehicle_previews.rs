// File:
//   - ui_vehicle_previews.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/ui_vehicle_previews.rs
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
//   - The ui vehicle previews contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute ui vehicle previews.
// - Split-When:
//   - Split when ui vehicle previews contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - UI vehicle preview package classifier.
// - Description:
//   - Defines ui vehicle previews data and behavior for pipeline phase two
//   - minor units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs ui vehicle
//   - previews.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! UI vehicle preview package classifier.
//! UI vehicle preview package classifier.

use super::cars;
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
    if category_from_root(&package.package_root)
        != PackageCategory::UiVehiclePreviews
        && !tokens_identify_preview_package(&tokens)
    {
        return None;
    }
    let car_index = tokens
        .iter()
        .position(|token| *token == "cars")?;
    let subcategory = cars::model_from_tokens(
        &tokens,
        car_index.saturating_add(1),
    )
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
    Some(
        (
            PackageCategory::UiVehiclePreviews,
            subcategory,
        ),
    )
}

/// Supports the `tokens_identify_preview_package` operation within this
/// deterministic classification boundary.
fn tokens_identify_preview_package(tokens: &[&str]) -> bool {
    tokens
        .windows(2)
        .any(
            |window| {
                window
                    == [
                        "dynaload", "cars",
                    ]
            },
        )
}

#[cfg(test)]
mod tests {
    use super::tokens_identify_preview_package;

    #[test]
    fn identifies_frontend_vehicle_tokens() {
        assert!(
            tokens_identify_preview_package(
                &[
                    "extracted",
                    "art",
                    "frontend",
                    "dynaload",
                    "cars",
                    "homer",
                    "v"
                ]
            )
        );
    }
}
