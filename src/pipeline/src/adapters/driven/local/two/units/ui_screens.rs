// File:
//   - ui_screens.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/ui_screens.rs
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
//   - The ui screens contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute ui screens.
// - Split-When:
//   - Split when ui screens contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - UI screen package classifier.
// - Description:
//   - Defines ui screens data and behavior for pipeline phase two minor
//   - units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs ui screens.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! UI screen package classifier.
//! UI screen package classifier.

use super::index::{
    MinorUnitPackage, MinorUnitRole, PackageCategory, category_from_root,
    package_id_tokens,
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
    if tokens.contains(&"resource") || tokens.contains(&"dynaload") {
        return None;
    }
    if category_from_root(&package.package_root) != PackageCategory::UiScreens
        && !tokens_identify_ui_screen(&tokens)
    {
        return None;
    }
    let scope = screen_scope(
        package, &tokens,
    )?;
    Some(
        (
            PackageCategory::UiScreens,
            format!("ui-screens/{scope}"),
        ),
    )
}

/// Supports the `tokens_identify_ui_screen` operation within this deterministic
/// classification boundary.
fn tokens_identify_ui_screen(tokens: &[&str]) -> bool {
    tokens
        .iter()
        .any(|token| token.starts_with("scrooby"))
}

/// Supports the `screen_scope` operation within this deterministic
/// classification boundary.
fn screen_scope(
    package: &MinorUnitPackage,
    tokens: &[&str],
) -> Option<String> {
    if tokens.contains(&"pages") {
        return Some("layout-index/pages".to_owned());
    }
    if tokens.contains(&"screens") {
        return Some("layout-index/screens".to_owned());
    }
    if tokens
        .last()
        .is_some_and(|token| *token == "scrooby2")
    {
        return Some("source-metadata".to_owned());
    }
    let screen_tokens = screen_tokens(tokens)?;
    let surface = screen_surface(screen_tokens)?;
    let family = if package
        .members
        .iter()
        .any(|member| member.role == MinorUnitRole::Scene)
    {
        "scene-layouts"
    } else {
        "sprite-layouts"
    };
    Some(format!("{family}/{surface}"))
}

/// Supports the `screen_tokens` operation within this deterministic
/// classification boundary.
fn screen_tokens<'a>(tokens: &'a [&str]) -> Option<&'a [&'a str]> {
    let index = tokens
        .iter()
        .position(|token| token.starts_with("scrooby"))?;
    tokens.get(index.saturating_add(1)..)
}

/// Supports the `screen_surface` operation within this deterministic
/// classification boundary.
fn screen_surface(tokens: &[&str]) -> Option<String> {
    if let Some(level) = tokens
        .iter()
        .find_map(|token| screen_level(token))
    {
        return Some(format!("in-game/{level}"));
    }
    tokens
        .iter()
        .find_map(
            |token| match *token {
                "backend" => Some("backend".to_owned()),
                "bootup" => Some("bootup".to_owned()),
                "frontend" => Some("frontend".to_owned()),
                "ingame" => Some("in-game".to_owned()),
                "minigame" => Some("mini-game".to_owned()),
                _ => None,
            },
        )
}

/// Supports the `screen_level` operation within this deterministic
/// classification boundary.
fn screen_level(token: &str) -> Option<String> {
    token
        .strip_prefix("ingamel")
        .and_then(
            |raw| {
                raw.parse::<u8>()
                    .ok()
            },
        )
        .filter(|level| (1..=7).contains(level))
        .map(|level| format!("level-{level:02}"))
}

#[cfg(test)]
mod tests {
    use super::{screen_level, screen_surface};

    #[test]
    fn classifies_screen_surfaces() {
        assert_eq!(
            screen_surface(
                &[
                    "extracted",
                    "scrooby",
                    "ingamel4"
                ]
            ),
            Some("in-game/level-04".to_owned())
        );
        assert_eq!(
            screen_surface(
                &[
                    "extracted",
                    "scrooby",
                    "minigame"
                ]
            ),
            Some("mini-game".to_owned())
        );
        assert_eq!(
            screen_level("ingamel7"),
            Some("level-07".to_owned())
        );
    }
}
