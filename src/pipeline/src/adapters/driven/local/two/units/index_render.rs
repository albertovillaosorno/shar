// File:
//   - index_render.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/index_render.rs
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
//   - The index render contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute index render.
// - Split-When:
//   - Split when index render contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - JSONL rendering for the minor-unit package index.
// - Description:
//   - Defines index render data and behavior for pipeline phase two minor
//   - units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs index render.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: JSONL rendering for the minor-unit package index keeps tightly
//   - coupled validation, ordering, and deterministic transformation
//   - invariants together; split when a stable independently testable sub-
//   - boundary is identified.
//

//! JSONL rendering for the minor-unit package index.
//! JSONL rendering for the minor-unit package index.

use super::index::{MinorUnitPackage, MinorUnitRole, PackageCategory};
use crate::domain::escape_json as json_escape;

/// Supports the `render_index_jsonl` operation within this deterministic
/// classification boundary.
pub(super) fn render_index_jsonl(packages: &[MinorUnitPackage]) -> String {
    let mut output = String::new();
    for package in packages {
        output.push_str(&render_package(package));
        output.push('\n');
    }
    output
}

/// Render one package row as canonical JSONL.
// Canonical JSONL ordering and omission rules stay visible together.
#[expect(
    clippy::too_many_lines,
    reason = "Canonical JSONL field ordering and omission rules must remain \
              visible in one renderer."
)]
fn render_package(package: &MinorUnitPackage) -> String {
    let fields = [
        (
            "unit_ids",
            ids_json(
                package, None,
            ),
        ),
        (
            "world_ids",
            ids_json(
                package,
                Some(MinorUnitRole::World),
            ),
        ),
        (
            "texture_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Texture),
            ),
        ),
        (
            "material_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Material),
            ),
        ),
        (
            "model_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Model),
            ),
        ),
        (
            "physics_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Physics),
            ),
        ),
        (
            "animation_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Animation),
            ),
        ),
        (
            "scene_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Scene),
            ),
        ),
        (
            "locator_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Locator),
            ),
        ),
        (
            "camera_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Camera),
            ),
        ),
        (
            "light_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Light),
            ),
        ),
        (
            "particle_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Particle),
            ),
        ),
        (
            "controller_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Controller),
            ),
        ),
        (
            "audio_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Audio),
            ),
        ),
        (
            "movie_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Movie),
            ),
        ),
        (
            "script_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Script),
            ),
        ),
        (
            "text_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Text),
            ),
        ),
        (
            "ui_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Ui),
            ),
        ),
        (
            "metadata_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Metadata),
            ),
        ),
        (
            "error_ids",
            ids_json(
                package,
                Some(MinorUnitRole::Error),
            ),
        ),
        (
            "source_unit_ids",
            string_array_json(
                &package
                    .source_unit_ids
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
            ),
        ),
        (
            "text_key_ids",
            text_key_ids_json(package),
        ),
        (
            "members",
            members_json(package),
        ),
        (
            "text_keys",
            text_keys_json(package),
        ),
    ];
    let mut row = format!(
        concat!(
            "{{\"package_id\":\"{}\",",
            "\"package_root\":\"{}\",",
            "\"package_category\":\"{}\",",
            "\"package_subcategory\":\"{}\",",
            "\"unit_count\":{}",
            ",\"text_key_count\":{}"
        ),
        json_escape(
            package
                .package_id
                .as_str()
        ),
        json_escape(&package.package_root),
        package
            .category
            .as_str(),
        json_escape(&package.subcategory),
        package
            .members
            .len(),
        package
            .text_keys
            .len()
    );
    for (field, value) in fields {
        row.push_str(",\"");
        row.push_str(field);
        row.push_str("\":");
        row.push_str(&value);
    }
    row.push('}');
    row
}

/// Render the id array for one optional role filter.
fn ids_json(
    package: &MinorUnitPackage,
    role: Option<MinorUnitRole>,
) -> String {
    let ids = package
        .members
        .iter()
        .filter(
            |member| {
                if role == Some(MinorUnitRole::Error)
                    && package.category == PackageCategory::Error
                {
                    true
                } else {
                    role.is_none_or(|wanted| member.role == wanted)
                }
            },
        )
        .map(
            |member| {
                member
                    .id
                    .as_str()
            },
        )
        .collect::<Vec<_>>();
    string_array_json(&ids)
}

/// Render the derived text key id array.
fn text_key_ids_json(package: &MinorUnitPackage) -> String {
    let ids = package
        .text_keys
        .iter()
        .map(
            |key| {
                key.id
                    .as_str()
            },
        )
        .collect::<Vec<_>>();
    string_array_json(&ids)
}

/// Render full package member objects for downstream tooling.
fn members_json(package: &MinorUnitPackage) -> String {
    let mut output = String::from("[");
    for (index, member) in package
        .members
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        let member_json = format!(
            concat!(
                "{{\"id\":\"{}\",\"role\":\"{}\",",
                "\"path\":\"{}\",\"type\":\"{}\",",
                "\"kind\":\"{}\",\"source_chunk_kind\":\"{}\"}}"
            ),
            json_escape(
                member
                    .id
                    .as_str()
            ),
            member
                .role
                .as_str(),
            json_escape(&member.path),
            json_escape(&member.type_),
            json_escape(&member.kind),
            json_escape(&member.source_chunk_kind),
        );
        output.push_str(&member_json);
    }
    output.push(']');
    output
}

/// Render derived text key records for language package importers.
fn text_keys_json(package: &MinorUnitPackage) -> String {
    let mut output = String::from("[");
    for (index, key) in package
        .text_keys
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        let key_json = format!(
            concat!(
                "{{\"id\":\"{}\",\"key\":\"{}\",",
                "\"source_unit_id\":\"{}\",",
                "\"subcategory\":\"{}\"}}"
            ),
            json_escape(&key.id),
            json_escape(&key.key),
            json_escape(&key.source_unit_id),
            json_escape(&key.subcategory),
        );
        output.push_str(&key_json);
    }
    output.push(']');
    output
}

/// Render a JSON string array with deterministic ordering.
fn string_array_json(values: &[&str]) -> String {
    let mut output = String::from("[");
    for (index, value) in values
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        output.push('"');
        output.push_str(&json_escape(value));
        output.push('"');
    }
    output.push(']');
    output
}
