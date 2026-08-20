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
//   - Ui images outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Ui images outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Ui images outbound adapter.

use super::index::{
    MinorUnitPackage, PackageCategory, category_from_root, package_id_tokens,
};

/// Supports the `classification_from_package` operation within this
/// deterministic classification boundary.
pub(super) fn classification_from_package(
    package: &MinorUnitPackage,
) -> Option<(PackageCategory, String)> {
    let tokens = package_id_tokens(package);
    if category_from_root(&package.package_root) != PackageCategory::UiImages
        && !tokens.contains(&"images")
    {
        return None;
    }
    let image_index = tokens.iter().position(|token| *token == "images")?;
    let tail = tokens.get(image_index.saturating_add(1)..)?;
    let subcategory = if tail.is_empty() {
        "ui-images/source-metadata".to_owned()
    } else {
        subcategory_from_tail(tail)
            .unwrap_or_else(|| image_metadata_subcategory(tail))
    };
    Some((PackageCategory::UiImages, subcategory))
}

/// Supports the `image_metadata_subcategory` operation within this
/// deterministic classification boundary.
fn image_metadata_subcategory(tail: &[&str]) -> String {
    match tail.first().copied() {
        None => "ui-images/source-metadata/root".to_owned(),
        Some("cars2d") => "ui-images/vehicle-icons/source-metadata".to_owned(),
        Some("msnicons") => {
            "ui-images/mission-icons/source-metadata".to_owned()
        },
        Some("skins2d") => {
            "ui-images/character-skins/source-metadata".to_owned()
        },
        Some(head) => format!("ui-images/source-metadata/{head}"),
    }
}

/// Supports the `subcategory_from_tail` operation within this deterministic
/// classification boundary.
fn subcategory_from_tail(tail: &[&str]) -> Option<String> {
    let head = tail.first().copied()?;
    if head == "cars2d" {
        return vehicle_icon_subcategory(tail);
    }
    if head == "msnicons" {
        return mission_icon_subcategory(tail);
    }
    if head == "scrapbook" {
        return scrapbook_subcategory(tail);
    }
    if head == "skins2d" {
        return character_skin_subcategory(tail);
    }
    if head == "license" {
        return Some(license_subcategory(tail));
    }
    if head == "loading" {
        return Some(loading_subcategory(tail));
    }
    if let Some(level) = mission_image_level(head) {
        return Some(format!(
            "ui-images/mission-briefing/{level}/{}",
            mission_image_detail(tail)
        ));
    }
    if let Some(level) = loading_screen_level(head) {
        return Some(format!("ui-images/loading/{level}"));
    }
    if matches!(head, "kang" | "kodos") {
        return Some(format!("ui-images/character-icons/aliens/{head}"));
    }
    if head == "mouse" {
        return Some("ui-images/controls/pointer".to_owned());
    }
    None
}

/// Supports the `scrapbook_subcategory` operation within this deterministic
/// classification boundary.
fn scrapbook_subcategory(tail: &[&str]) -> Option<String> {
    let level = level_tail(tail)?;
    Some(format!(
        "ui-images/scrapbook/{level}/{}",
        mission_image_detail(tail.get(1..).unwrap_or(&[]),),
    ))
}

/// Supports the `loading_subcategory` operation within this deterministic
/// classification boundary.
fn loading_subcategory(tail: &[&str]) -> String {
    let locale = localized_scope(tail);
    let level = tail
        .iter()
        .find_map(|token| loading_screen_level(token))
        .unwrap_or_else(|| "source-metadata".to_owned());
    format!("ui-images/loading/{locale}/{level}")
}

/// Supports the `license_subcategory` operation within this deterministic
/// classification boundary.
fn license_subcategory(tail: &[&str]) -> String {
    let locale = localized_scope(tail);
    let detail = tail
        .iter()
        .rev()
        .copied()
        .find(|token| token.starts_with("license"))
        .unwrap_or("source-metadata");
    format!("ui-images/licenses/{locale}/{detail}")
}

/// Supports the `mission_image_detail` operation within this deterministic
/// classification boundary.
fn mission_image_detail(tail: &[&str]) -> String {
    if tail.is_empty() {
        "source-metadata".to_owned()
    } else {
        tail.join("-")
    }
}

/// Supports the `vehicle_icon_subcategory` operation within this deterministic
/// classification boundary.
fn vehicle_icon_subcategory(tail: &[&str]) -> Option<String> {
    let vehicle = tail.get(1).copied()?;
    Some(format!(
        "ui-images/vehicle-icons/{}/{}",
        vehicle_icon_state(tail),
        vehicle,
    ))
}

/// Supports the `character_skin_subcategory` operation within this
/// deterministic classification boundary.
fn character_skin_subcategory(tail: &[&str]) -> Option<String> {
    let character = tail.get(1).copied()?;
    let skin = tail.get(2).copied().unwrap_or("base");
    Some(format!("ui-images/character-skins/{character}/{skin}"))
}
/// Supports the `vehicle_icon_state` operation within this deterministic
/// classification boundary.
fn vehicle_icon_state(tail: &[&str]) -> &'static str {
    if tail
        .iter()
        .skip(1)
        .any(|token| *token == "vd" || token.ends_with('d'))
    {
        "damaged"
    } else {
        "normal"
    }
}

/// Supports the `mission_icon_subcategory` operation within this deterministic
/// classification boundary.
fn mission_icon_subcategory(tail: &[&str]) -> Option<String> {
    let family = tail.get(1).copied()?;
    let role = match family {
        "char" => "characters",
        "location" => "locations",
        "object" => "objects",
        "vehicle" => "vehicles",
        _ => return None,
    };
    let Some(icon) = tail.get(2).copied() else {
        return Some(format!("ui-images/mission-icons/{role}/source-metadata"));
    };
    Some(format!("ui-images/mission-icons/{role}/{icon}"))
}

/// Supports the `level_tail` operation within this deterministic classification
/// boundary.
fn level_tail(tail: &[&str]) -> Option<String> {
    tail.iter().find_map(|token| mission_image_level(token))
}

/// Supports the `mission_image_level` operation within this deterministic
/// classification boundary.
fn mission_image_level(token: &str) -> Option<String> {
    let raw = token.strip_prefix("mis")?;
    if raw == "xx" {
        return Some("bonus".to_owned());
    }
    raw.parse::<u8>()
        .ok()
        .filter(|level| (1..=7).contains(level))
        .map(|level| format!("level-{level:02}"))
}

/// Supports the `localized_scope` operation within this deterministic
/// classification boundary.
fn localized_scope(tail: &[&str]) -> &'static str {
    if tail.contains(&"french") {
        "french"
    } else if tail.contains(&"german") {
        "german"
    } else if tail.contains(&"spanish") {
        "spanish"
    } else {
        "default"
    }
}

/// Supports the `loading_screen_level` operation within this deterministic
/// classification boundary.
fn loading_screen_level(token: &str) -> Option<String> {
    token
        .strip_prefix("loading")
        .and_then(|suffix| suffix.parse::<u8>().ok())
        .filter(|level| (1..=7).contains(level))
        .map(|level| format!("level-{level:02}"))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/units/ui_images/tests.rs"]
mod tests;
