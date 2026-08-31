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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use super::{
    MAX_JSON_NESTING, PackageIntakeError, PackageRole, PhaseThreePackageIndex,
    PhaseThreePackageRow, parse_json_string_at,
};

#[test]
fn package_intake_error_escapes_control_characters() {
    let error = PackageIntakeError::new("invalid\npackage\\evidence");

    assert_eq!(error.to_string(), r"invalid\npackage\evidence");
}

const SAMPLE_MEMBERS_FIELD: &str = concat!(
    "\"members\":[",
    "{\"id\":\"texture-a\",\"role\":\"texture\",",
    "\"path\":\"extracted/texture.p3d\",",
    "\"type\":\"texture\",\"kind\":\"image\",",
    "\"source_chunk_kind\":\"texture\"},",
    "{\"id\":\"model-a\",\"role\":\"model\",",
    "\"path\":\"extracted/model.p3d\",",
    "\"type\":\"model\",\"kind\":\"mesh\",",
    "\"source_chunk_kind\":\"mesh\"}]",
);
const SOURCE_ORDERED_MEMBERS_FIELD: &str = concat!(
    "\"members\":[",
    "{\"id\":\"model-a\",\"role\":\"model\",",
    "\"path\":\"extracted/model.p3d\",",
    "\"type\":\"model\",\"kind\":\"mesh\",",
    "\"source_chunk_kind\":\"mesh\"},",
    "{\"id\":\"texture-a\",\"role\":\"texture\",",
    "\"path\":\"extracted/texture.p3d\",",
    "\"type\":\"texture\",\"kind\":\"image\",",
    "\"source_chunk_kind\":\"texture\"}]",
);
const EMPTY_MEMBERS_FIELD: &str = "\"members\":[]";

fn text_keys_field(
    id: &str,
    source_unit_id: &str,
    subcategory: &str,
) -> String {
    format!(
        concat!(
            "\"text_keys\":[{{",
            "\"id\":\"{}\",",
            "\"key\":\"HELLO\",",
            "\"source_unit_id\":\"{}\",",
            "\"subcategory\":\"{}\"}}]"
        ),
        id, source_unit_id, subcategory,
    )
}

fn error_row() -> &'static str {
    concat!(
        "{\"package_id\":\"pkg-error\",",
        "\"package_root\":\"pkg-error\",",
        "\"package_category\":\"error\",",
        "\"package_subcategory\":\"error\",",
        "\"unit_count\":1,\"text_key_count\":0,",
        "\"unit_ids\":[\"error-a\"],",
        "\"world_ids\":[],\"texture_ids\":[],",
        "\"material_ids\":[],\"model_ids\":[],",
        "\"physics_ids\":[],\"animation_ids\":[],",
        "\"scene_ids\":[],\"locator_ids\":[],",
        "\"camera_ids\":[],\"light_ids\":[],",
        "\"particle_ids\":[],\"controller_ids\":[],",
        "\"audio_ids\":[],\"movie_ids\":[],",
        "\"script_ids\":[],\"text_ids\":[],",
        "\"ui_ids\":[],\"metadata_ids\":[],",
        "\"error_ids\":[\"error-a\"],",
        "\"source_unit_ids\":[],\"text_key_ids\":[],",
        "\"members\":[{\"id\":\"error-a\",",
        "\"role\":\"error\",",
        "\"path\":\"extracted/error.bin\",",
        "\"type\":\"metadata\",",
        "\"kind\":\"unclassified\",",
        "\"source_chunk_kind\":\"none\"}],",
        "\"text_keys\":[]}"
    )
}

fn sample_row() -> &'static str {
    concat!(
        "{\"package_id\":\"pkg-car\",",
        "\"package_root\":\"pkg-car\",",
        "\"package_category\":\"cars\",",
        "\"package_subcategory\":\"cars/character-rigs/homer-v\",",
        "\"unit_count\":2,\"text_key_count\":0,",
        "\"unit_ids\":[\"texture-a\",\"model-a\"],",
        "\"world_ids\":[],\"texture_ids\":[\"texture-a\"],",
        "\"material_ids\":[],\"model_ids\":[\"model-a\"],",
        "\"physics_ids\":[],\"animation_ids\":[],",
        "\"scene_ids\":[],\"locator_ids\":[],",
        "\"camera_ids\":[],\"light_ids\":[],",
        "\"particle_ids\":[],\"controller_ids\":[],",
        "\"audio_ids\":[],\"movie_ids\":[],",
        "\"script_ids\":[],\"text_ids\":[],",
        "\"ui_ids\":[],\"metadata_ids\":[],",
        "\"error_ids\":[],\"source_unit_ids\":[],",
        "\"text_key_ids\":[],",
        "\"members\":[",
        "{\"id\":\"texture-a\",\"role\":\"texture\",",
        "\"path\":\"extracted/texture.p3d\",",
        "\"type\":\"texture\",\"kind\":\"image\",",
        "\"source_chunk_kind\":\"texture\"},",
        "{\"id\":\"model-a\",\"role\":\"model\",",
        "\"path\":\"extracted/model.p3d\",",
        "\"type\":\"model\",\"kind\":\"mesh\",",
        "\"source_chunk_kind\":\"mesh\"}],",
        "\"text_keys\":[]}",
    )
}

#[test]
fn unreal_intake_validates_and_excludes_error_evidence() -> Result<(), String> {
    let contents = format!("{}\n{}", sample_row(), error_row());
    let index = PhaseThreePackageIndex::from_jsonl_for_unreal(&contents)
        .map_err(|error| error.to_string())?;
    if index.packages().len() != 1 || index.find_package("pkg-car").is_none() {
        return Err(
            "Unreal intake should preserve the importable package".to_owned()
        );
    }
    if index.find_package("pkg-error").is_some() {
        return Err(
            "Unreal intake must exclude fail-closed error evidence".to_owned()
        );
    }
    Ok(())
}

#[test]
fn unreal_intake_rejects_malformed_error_evidence() {
    let malformed =
        error_row().replace("\"role\":\"error\"", "\"role\":\"metadata\"");
    assert!(PhaseThreePackageIndex::from_jsonl_for_unreal(&malformed).is_err());
}

#[test]
fn unreal_intake_requires_at_least_one_importable_package() {
    assert!(
        PhaseThreePackageIndex::from_jsonl_for_unreal(error_row()).is_err()
    );
}

#[test]
fn unreal_intake_rejects_dependencies_on_excluded_error_evidence() {
    let derived = sample_row()
        .replace("pkg-car", "pkg-derived")
        .replace(
            "\"package_category\":\"cars\"",
            "\"package_category\":\"language\"",
        )
        .replace("cars/character-rigs/homer-v", "language/derived/text")
        .replace("\"unit_count\":2", "\"unit_count\":0")
        .replace("\"text_key_count\":0", "\"text_key_count\":1")
        .replace(
            "\"unit_ids\":[\"texture-a\",\"model-a\"]",
            "\"unit_ids\":[]",
        )
        .replace(SAMPLE_MEMBERS_FIELD, EMPTY_MEMBERS_FIELD)
        .replace("\"texture_ids\":[\"texture-a\"]", "\"texture_ids\":[]")
        .replace("\"model_ids\":[\"model-a\"]", "\"model_ids\":[]")
        .replace(
            "\"source_unit_ids\":[]",
            "\"source_unit_ids\":[\"error-a\"]",
        )
        .replace("\"text_key_ids\":[]", "\"text_key_ids\":[\"text-a\"]")
        .replace(
            "\"text_keys\":[]",
            &text_keys_field("text-a", "error-a", "language/derived/text"),
        );
    let contents = format!(
        "{derived}
{}",
        error_row()
    );
    assert!(PhaseThreePackageIndex::from_jsonl_for_unreal(&contents).is_err());
}

#[test]
fn reads_one_package_row() -> Result<(), String> {
    let row = PhaseThreePackageRow::from_json_line(sample_row())
        .map_err(|error| error.to_string())?;
    if row.package_id != "pkg-car" {
        return Err("package id should match sample".to_owned());
    }
    if row.category() != "cars"
        || row.subcategory() != "cars/character-rigs/homer-v"
    {
        return Err("package taxonomy getters should match sample".to_owned());
    }
    if row.ids_for_role(PackageRole::Model) != ["model-a".to_owned()] {
        return Err("model bucket should expose model id".to_owned());
    }
    if !row.has_model_components() || row.has_error_ids() {
        return Err("sample row should be model-like and error-free".to_owned());
    }
    Ok(())
}

#[test]
fn decodes_unicode_json_escapes() -> Result<(), String> {
    let input = r#""caf\u00e9 \uD83D\uDE80""#;
    let (value, cursor) =
        parse_json_string_at(input, 0).map_err(|error| error.to_string())?;
    if value != "caf\u{00e9} \u{1f680}" || cursor != input.len() {
        return Err(format!(
            "Unicode JSON escapes were not decoded: {value:?}"
        ));
    }
    Ok(())
}

#[test]
fn rejects_invalid_unicode_surrogates() -> Result<(), String> {
    for input in [
        r#""\uD83D""#,
        r#""\uDE80""#,
        r#""\uD83D\u0041""#,
        r#""\u12x4""#,
    ] {
        if parse_json_string_at(input, 0).is_ok() {
            return Err(format!(
                "invalid Unicode escape was accepted: {input}"
            ));
        }
    }
    Ok(())
}

#[test]
fn preserves_utf8_json_strings() -> Result<(), String> {
    let input = "\"café\"";
    let (value, cursor) =
        parse_json_string_at(input, 0).map_err(|error| error.to_string())?;
    if value != "café" || cursor != input.len() {
        return Err(format!("UTF-8 JSON string was corrupted: {value}"));
    }
    Ok(())
}

#[test]
fn rejects_tokens_appended_to_string_fields() -> Result<(), String> {
    for (field, replacement) in [
        (
            "\"package_id\":\"pkg-car\"",
            "\"package_id\":\"pkg-car\"true",
        ),
        (
            "\"package_category\":\"cars\"",
            "\"package_category\":\"cars\"false",
        ),
    ] {
        let row_text = sample_row().replace(field, replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "string field accepted appended token: {field}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_unescaped_control_in_package_ids() -> Result<(), String> {
    let row_text = sample_row().replace(
        "pkg-car", "pkg-
car",
    );
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err("unescaped control characters must be rejected".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_tokens_appended_to_string_arrays() -> Result<(), String> {
    for (field, replacement) in [
        (
            "\"unit_ids\":[\"texture-a\",\"model-a\"]",
            "\"unit_ids\":[\"texture-a\",\"model-a\"]null",
        ),
        (
            "\"model_ids\":[\"model-a\"]",
            "\"model_ids\":[\"model-a\"]true",
        ),
    ] {
        let row_text = sample_row().replace(field, replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "string array accepted appended token: {field}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_trailing_commas_in_role_arrays() -> Result<(), String> {
    let row_text = sample_row().replace(
        "\"model_ids\":[\"model-a\"]",
        "\"model_ids\":[\"model-a\",]",
    );
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err("trailing array commas must be rejected".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_empty_required_package_fields() -> Result<(), String> {
    for (needle, replacement, label) in [
        (
            "\"package_id\":\"pkg-car\"",
            "\"package_id\":\"\"",
            "package id",
        ),
        (
            "\"package_category\":\"cars\"",
            "\"package_category\":\"\"",
            "package category",
        ),
        (
            "\"package_subcategory\":\"cars/character-rigs/homer-v\"",
            "\"package_subcategory\":\"\"",
            "package subcategory",
        ),
    ] {
        let row_text = sample_row().replace(needle, replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!("empty {label} must be rejected"));
        }
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_package_array_ids() -> Result<(), String> {
    for invalid in [
        "model a", "Model-a", "model/a", "model--a", "-model-a", "model-a-",
        "modél-a",
    ] {
        let row_text = sample_row().replace("model-a", invalid);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "noncanonical identifier was accepted: {invalid}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_empty_identifiers_in_package_arrays() -> Result<(), String> {
    for (needle, replacement, label) in [
        (
            "\"unit_ids\":[\"texture-a\",\"model-a\"]",
            "\"unit_ids\":[\"\",\"texture-a\"]",
            "unit ids",
        ),
        (
            "\"model_ids\":[\"model-a\"]",
            "\"model_ids\":[\"\"]",
            "role ids",
        ),
        (
            "\"text_key_ids\":[]",
            "\"text_key_ids\":[\"\"]",
            "text key ids",
        ),
        (
            "\"source_unit_ids\":[]",
            "\"source_unit_ids\":[\"\"]",
            "source unit ids",
        ),
    ] {
        let row_text = sample_row().replace(needle, replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!("empty {label} must be rejected"));
        }
    }
    Ok(())
}

#[test]
fn rejects_duplicate_identifiers_within_package_arrays() -> Result<(), String> {
    for (needle, replacement, label) in [
        (
            "\"unit_ids\":[\"texture-a\",\"model-a\"]",
            "\"unit_ids\":[\"model-a\",\"model-a\"]",
            "unit ids",
        ),
        (
            "\"model_ids\":[\"model-a\"]",
            "\"model_ids\":[\"model-a\",\"model-a\"]",
            "role ids",
        ),
        (
            "\"text_key_ids\":[]",
            "\"text_key_ids\":[\"text-a\",\"text-a\"]",
            "text key ids",
        ),
        (
            "\"source_unit_ids\":[]",
            "\"source_unit_ids\":[\"source-a\",\"source-a\"]",
            "source unit ids",
        ),
    ] {
        let row_text = sample_row().replace(needle, replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!("duplicate {label} must be rejected"));
        }
    }
    Ok(())
}

#[test]
fn rejects_role_ids_missing_from_physical_members() -> Result<(), String> {
    let row_text = sample_row().replace(
        "\"model_ids\":[\"model-a\"]",
        "\"model_ids\":[\"orphan-model\"]",
    );
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err("role ids absent from unit_ids must be rejected".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_physical_members_without_roles() -> Result<(), String> {
    let row_text =
        sample_row().replace("\"model_ids\":[\"model-a\"]", "\"model_ids\":[]");
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err("physical members absent from every role must be rejected"
            .to_owned());
    }
    Ok(())
}

#[test]
fn rejects_members_assigned_to_multiple_roles() -> Result<(), String> {
    let row_text = sample_row().replace(
        "\"texture_ids\":[\"texture-a\"]",
        "\"texture_ids\":[\"texture-a\",\"model-a\"]",
    );
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err(
            "one physical member must not occupy multiple roles".to_owned()
        );
    }
    Ok(())
}

#[test]
fn rejects_packages_with_error_role_members() -> Result<(), String> {
    let row_text = sample_row()
        .replace("\"model_ids\":[\"model-a\"]", "\"model_ids\":[]")
        .replace("\"error_ids\":[]", "\"error_ids\":[\"model-a\"]");
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err("error-role packages must fail intake".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_packages_without_physical_or_derived_members() -> Result<(), String>
{
    let row_text = sample_row()
        .replace(
            "\"unit_ids\":[\"texture-a\",\"model-a\"]",
            "\"unit_ids\":[]",
        )
        .replace(SAMPLE_MEMBERS_FIELD, EMPTY_MEMBERS_FIELD)
        .replace("\"model_ids\":[\"model-a\"]", "\"model_ids\":[]")
        .replace("\"texture_ids\":[\"texture-a\"]", "\"texture_ids\":[]");
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err("empty packages must fail intake".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_derived_text_keys_without_source_units() -> Result<(), String> {
    let row_text = sample_row()
        .replace(
            "\"unit_ids\":[\"texture-a\",\"model-a\"]",
            "\"unit_ids\":[]",
        )
        .replace(SAMPLE_MEMBERS_FIELD, EMPTY_MEMBERS_FIELD)
        .replace("\"model_ids\":[\"model-a\"]", "\"model_ids\":[]")
        .replace("\"texture_ids\":[\"texture-a\"]", "\"texture_ids\":[]")
        .replace("\"text_key_count\":0", "\"text_key_count\":1")
        .replace("\"text_key_ids\":[]", "\"text_key_ids\":[\"text-a\"]")
        .replace(
            "\"text_keys\":[]",
            &text_keys_field(
                "text-a",
                "model-a",
                "cars/character-rigs/homer-v",
            ),
        );
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err("derived text keys without source units must be rejected"
            .to_owned());
    }
    Ok(())
}

#[test]
fn rejects_blank_jsonl_records() -> Result<(), String> {
    for contents in [
        format!("\n{}\n", sample_row()),
        format!("{}\n \t\n", sample_row()),
    ] {
        if PhaseThreePackageIndex::from_jsonl(&contents).is_ok() {
            return Err(
                "blank package-index records must be rejected".to_owned()
            );
        }
    }
    Ok(())
}

#[test]
fn rejects_empty_package_indexes() -> Result<(), String> {
    let whitespace_only = ["", " ", ""].join(
        "
",
    );
    if PhaseThreePackageIndex::from_jsonl(&whitespace_only).is_ok() {
        return Err("empty package indexes must be rejected".to_owned());
    }
    Ok(())
}

#[test]
fn accepts_source_ordered_member_mirrors() -> Result<(), String> {
    let row_text = sample_row()
        .replace(
            "\"unit_ids\":[\"texture-a\",\"model-a\"]",
            "\"unit_ids\":[\"model-a\",\"texture-a\"]",
        )
        .replace(SAMPLE_MEMBERS_FIELD, SOURCE_ORDERED_MEMBERS_FIELD);
    let row = PhaseThreePackageRow::from_json_line(&row_text)
        .map_err(|error| error.to_string())?;
    let ids = row
        .members()
        .iter()
        .map(|member| member.id.as_str())
        .collect::<Vec<_>>();
    if ids != ["model-a", "texture-a"] {
        return Err("phase-three intake changed source member order".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_package_order() -> Result<(), String> {
    let row_z = sample_row()
        .replace("pkg-car", "pkg-z")
        .replace("model-a", "model-z")
        .replace("texture-a", "texture-z");
    let row_a = sample_row()
        .replace("pkg-car", "pkg-a")
        .replace("model-a", "model-a2")
        .replace("texture-a", "texture-a2");
    let contents = format!(
        "{row_z}
{row_a}"
    );
    if PhaseThreePackageIndex::from_jsonl(&contents).is_ok() {
        return Err("descending package ids must be rejected".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_physical_ids_claimed_by_multiple_packages() -> Result<(), String> {
    let row_a = sample_row().replace("pkg-car", "pkg-a");
    let row_z = sample_row().replace("pkg-car", "pkg-z");
    let contents = format!(
        "{row_a}
{row_z}"
    );
    if PhaseThreePackageIndex::from_jsonl(&contents).is_ok() {
        return Err(
            "physical ids claimed by multiple packages must be rejected"
                .to_owned(),
        );
    }
    Ok(())
}

#[test]
fn rejects_physical_and_derived_id_collisions() -> Result<(), String> {
    let derived = sample_row()
        .replace("pkg-car", "aaa-derived")
        .replace(
            "\"package_category\":\"cars\"",
            "\"package_category\":\"language\"",
        )
        .replace("cars/character-rigs/homer-v", "language/derived/text")
        .replace("\"unit_count\":2", "\"unit_count\":0")
        .replace("\"text_key_count\":0", "\"text_key_count\":1")
        .replace(
            "\"unit_ids\":[\"texture-a\",\"model-a\"]",
            "\"unit_ids\":[]",
        )
        .replace(SAMPLE_MEMBERS_FIELD, EMPTY_MEMBERS_FIELD)
        .replace("\"texture_ids\":[\"texture-a\"]", "\"texture_ids\":[]")
        .replace("\"model_ids\":[\"model-a\"]", "\"model_ids\":[]")
        .replace(
            "\"source_unit_ids\":[]",
            "\"source_unit_ids\":[\"model-a\"]",
        )
        .replace("\"text_key_ids\":[]", "\"text_key_ids\":[\"model-a\"]")
        .replace(
            "\"text_keys\":[]",
            &text_keys_field("model-a", "model-a", "language/derived/text"),
        );
    let contents = format!("{derived}\n{}", sample_row());
    if PhaseThreePackageIndex::from_jsonl(&contents).is_ok() {
        return Err("one id must not be both physical and derived".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_derived_ids_claimed_by_multiple_packages() -> Result<(), String> {
    let derived = |package_id: &str| {
        sample_row()
            .replace("pkg-car", package_id)
            .replace("\"unit_count\":2", "\"unit_count\":0")
            .replace("\"text_key_count\":0", "\"text_key_count\":1")
            .replace(
                "\"unit_ids\":[\"texture-a\",\"model-a\"]",
                "\"unit_ids\":[]",
            )
            .replace(SAMPLE_MEMBERS_FIELD, EMPTY_MEMBERS_FIELD)
            .replace("\"model_ids\":[\"model-a\"]", "\"model_ids\":[]")
            .replace("\"texture_ids\":[\"texture-a\"]", "\"texture_ids\":[]")
            .replace("\"text_key_ids\":[]", "\"text_key_ids\":[\"text-a\"]")
            .replace(
                "\"source_unit_ids\":[]",
                "\"source_unit_ids\":[\"source-a\"]",
            )
            .replace(
                "\"text_keys\":[]",
                &text_keys_field(
                    "text-a",
                    "source-a",
                    "cars/character-rigs/homer-v",
                ),
            )
    };
    let contents = format!(
        "{}
{}",
        derived("pkg-a"),
        derived("pkg-z")
    );
    if PhaseThreePackageIndex::from_jsonl(&contents).is_ok() {
        return Err(
            "derived ids claimed by multiple packages must be rejected"
                .to_owned(),
        );
    }
    Ok(())
}

#[test]
fn rejects_hyphen_edge_cases_in_package_slugs() -> Result<(), String> {
    for invalid in ["-pkg-car", "pkg-car-", "pkg--car"] {
        let row_text = sample_row()
            .replace(
                "\"package_id\":\"pkg-car\"",
                &format!("\"package_id\":\"{invalid}\""),
            )
            .replace(
                "\"package_root\":\"pkg-car\"",
                &format!("\"package_root\":\"{invalid}\""),
            );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "noncanonical package slug was accepted: {invalid}"
            ));
        }
    }
    for invalid in ["-character-rigs", "character-rigs-", "character--rigs"] {
        let row_text = sample_row().replace(
            "cars/character-rigs/homer-v",
            &format!("cars/{invalid}/homer-v"),
        );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "noncanonical subcategory slug was accepted: {invalid}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_package_id_characters() -> Result<(), String> {
    for invalid in ["Pkg-Car", "pkg_car", "pkg/car", "pkg-café"] {
        let row_text = sample_row().replace("pkg-car", invalid);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "noncanonical package id must be rejected: {invalid}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_unknown_or_error_package_categories() -> Result<(), String> {
    for invalid in ["unknown", "error"] {
        let row_text = sample_row().replace(
            "\"package_category\":\"cars\"",
            &format!("\"package_category\":\"{invalid}\""),
        );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "non-success package category must be rejected: \
                         {invalid}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_invalid_package_root_paths() -> Result<(), String> {
    for invalid_root in [
        "/pkg/car",
        "pkg/../car",
        r"pkg\car",
        " pkg/car",
        "pkg/car ",
        "pkg//car",
        r"pkg\u0000car",
    ] {
        let row = sample_row().replace(
            "\"package_root\":\"pkg-car\"",
            &format!("\"package_root\":\"{invalid_root}\""),
        );
        if PhaseThreePackageRow::from_json_line(&row).is_ok() {
            return Err(format!(
                "invalid package root was accepted: {invalid_root:?}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_subcategory_paths() -> Result<(), String> {
    for invalid in [
        "Cars/character-rigs/homer-v",
        "cars/character_rigs/homer-v",
        "cars//homer-v",
        r"cars\homer-v",
        "cars/café",
    ] {
        let row_text =
            sample_row().replace("cars/character-rigs/homer-v", invalid);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "noncanonical subcategory must be rejected: {invalid}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_placeholder_subcategory_segments() -> Result<(), String> {
    for placeholder in
        ["unknown", "generic", "misc", "context", "shared", "global"]
    {
        let row_text = sample_row().replace(
            "cars/character-rigs/homer-v",
            &format!("cars/{placeholder}"),
        );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "placeholder subcategory must be rejected: \
                         {placeholder}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_missing_or_mismatched_package_roots() -> Result<(), String> {
    let missing = sample_row().replace("\"package_root\":\"pkg-car\",", "");
    let empty = sample_row()
        .replace("\"package_root\":\"pkg-car\"", "\"package_root\":\"\"");
    let mismatched = sample_row().replace(
        "\"package_root\":\"pkg-car\"",
        "\"package_root\":\"different-root\"",
    );
    for (label, row_text) in [
        ("missing", missing),
        ("empty", empty),
        ("mismatched", mismatched),
    ] {
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!("{label} package root must be rejected"));
        }
    }
    Ok(())
}

#[test]
fn rejects_leading_zero_declared_counts() -> Result<(), String> {
    for (field, replacement) in [
        ("\"unit_count\":2", "\"unit_count\":02"),
        ("\"text_key_count\":0", "\"text_key_count\":00"),
    ] {
        let row_text = sample_row().replace(field, replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!("leading-zero count was accepted: {field}"));
        }
    }
    Ok(())
}

#[test]
fn rejects_mismatched_or_malformed_declared_counts() -> Result<(), String> {
    for (needle, replacement, label) in [
        (
            "\"unit_count\":2",
            "\"unit_count\":1",
            "unit count mismatch",
        ),
        (
            "\"text_key_count\":0",
            "\"text_key_count\":1",
            "text key count mismatch",
        ),
        (
            "\"unit_count\":2",
            "\"unit_count\":-2",
            "negative unit count",
        ),
        (
            "\"text_key_count\":0",
            "\"text_key_count\":null",
            "nonnumeric text key count",
        ),
    ] {
        let row_text = sample_row().replace(needle, replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!("{label} must be rejected"));
        }
    }
    Ok(())
}

#[test]
fn rejects_dangling_derived_source_units() -> Result<(), String> {
    let derived = sample_row()
        .replace("pkg-car", "pkg-derived")
        .replace(
            "\"unit_ids\":[\"texture-a\",\"model-a\"]",
            "\"unit_ids\":[]",
        )
        .replace(SAMPLE_MEMBERS_FIELD, EMPTY_MEMBERS_FIELD)
        .replace("\"unit_count\":2", "\"unit_count\":0")
        .replace("\"model_ids\":[\"model-a\"]", "\"model_ids\":[]")
        .replace("\"texture_ids\":[\"texture-a\"]", "\"texture_ids\":[]")
        .replace("\"text_key_count\":0", "\"text_key_count\":1")
        .replace("\"text_key_ids\":[]", "\"text_key_ids\":[\"text-a\"]")
        .replace(
            "\"source_unit_ids\":[]",
            "\"source_unit_ids\":[\"missing-source\"]",
        )
        .replace(
            "\"text_keys\":[]",
            &text_keys_field(
                "text-a",
                "missing-source",
                "cars/character-rigs/homer-v",
            ),
        );
    if PhaseThreePackageIndex::from_jsonl(&derived).is_ok() {
        return Err(
            "derived source ids absent from physical coverage must fail"
                .to_owned(),
        );
    }
    Ok(())
}

#[test]
fn rejects_empty_physical_member_mirrors() -> Result<(), String> {
    let row_text =
        sample_row().replace(SAMPLE_MEMBERS_FIELD, EMPTY_MEMBERS_FIELD);
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err(
            "physical unit ids require member mirror records".to_owned()
        );
    }
    Ok(())
}

#[test]
fn rejects_empty_derived_text_key_mirrors() -> Result<(), String> {
    let row_text = sample_row()
        .replace("\"text_key_count\":0", "\"text_key_count\":1")
        .replace(
            "\"source_unit_ids\":[]",
            "\"source_unit_ids\":[\"model-a\"]",
        )
        .replace("\"text_key_ids\":[]", "\"text_key_ids\":[\"text-a\"]");
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err(
            "derived text_key_ids require text_keys mirror records".to_owned()
        );
    }
    Ok(())
}

#[test]
fn rejects_non_array_structured_mirrors() -> Result<(), String> {
    for (field, replacement) in [
        (SAMPLE_MEMBERS_FIELD, "\"members\":true"),
        ("\"text_keys\":[]", "\"text_keys\":{}"),
    ] {
        let row_text = sample_row().replace(field, replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!("non-array mirror was accepted: {field}"));
        }
    }
    Ok(())
}

#[test]
fn rejects_missing_structured_mirror_fields() -> Result<(), String> {
    for field in [
        format!(",{SAMPLE_MEMBERS_FIELD}"),
        ",\"text_keys\":[]".to_owned(),
    ] {
        let row_text = sample_row().replace(&field, "");
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!("missing mirror field was accepted: {field}"));
        }
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_top_level_field_order() -> Result<(), String> {
    let canonical_tail = format!("{SAMPLE_MEMBERS_FIELD},\"text_keys\":[]");
    let swapped_tail = format!("\"text_keys\":[],{SAMPLE_MEMBERS_FIELD}");
    for row_text in [
        sample_row().replacen(
            "\"package_id\":\"pkg-car\",\"package_root\":\"pkg-car\"",
            "\"package_root\":\"pkg-car\",\"package_id\":\"pkg-car\"",
            1,
        ),
        sample_row().replace(
            "\"material_ids\":[],\"model_ids\":[\"model-a\"]",
            "\"model_ids\":[\"model-a\"],\"material_ids\":[]",
        ),
        sample_row().replace(&canonical_tail, &swapped_tail),
    ] {
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err("noncanonical package field order must fail".to_owned());
        }
    }
    Ok(())
}

#[test]
fn rejects_unknown_top_level_fields() -> Result<(), String> {
    let row_text = sample_row()
        .replace("\"text_keys\":[]}", "\"text_keys\":[],\"unexpected\":true}");
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err("unknown package-index fields must be rejected".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_invalid_required_text_key_values() -> Result<(), String> {
    for invalid_key in ["", " PADDED", r"HELLO\u0000"] {
        let text_keys =
            text_keys_field("text-a", "model-a", "cars/character-rigs/homer-v")
                .replace(
                    "\"key\":\"HELLO\"",
                    &format!("\"key\":\"{invalid_key}\""),
                );
        let row_text = sample_row()
            .replace("\"text_key_count\":0", "\"text_key_count\":1")
            .replace(
                "\"source_unit_ids\":[]",
                "\"source_unit_ids\":[\"model-a\"]",
            )
            .replace("\"text_key_ids\":[]", "\"text_key_ids\":[\"text-a\"]")
            .replace("\"text_keys\":[]", &text_keys);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "invalid text key must be rejected: {invalid_key:?}"
            ));
        }
    }
    Ok(())
}

#[test]
fn accepts_canonical_structured_text_key_json() -> Result<(), String> {
    let row_text = sample_row()
        .replace("\"text_key_count\":0", "\"text_key_count\":1")
        .replace(
            "\"source_unit_ids\":[]",
            "\"source_unit_ids\":[\"model-a\"]",
        )
        .replace("\"text_key_ids\":[]", "\"text_key_ids\":[\"text-a\"]")
        .replace(
            "\"text_keys\":[]",
            concat!(
                "\"text_keys\":[{",
                "\"id\":\"text-a\",",
                "\"key\":\"HELLO\",",
                "\"source_unit_id\":\"model-a\",",
                "\"subcategory\":\"cars/character-rigs/homer-v\"}]",
            ),
        );
    let row = PhaseThreePackageRow::from_json_line(&row_text)
        .map_err(|error| error.to_string())?;
    if row.text_key_ids != ["text-a".to_owned()] {
        return Err("canonical text-key mirror changed id intake".to_owned());
    }
    let [key] = row.text_keys() else {
        return Err("canonical text-key mirror was not preserved".to_owned());
    };
    if key.id != "text-a"
        || key.key != "HELLO"
        || key.source_unit_id != "model-a"
        || key.subcategory != "cars/character-rigs/homer-v"
    {
        return Err("canonical text-key mirror evidence drifted".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_inconsistent_structured_text_key_mirrors() -> Result<(), String> {
    let canonical = concat!(
        "\"text_keys\":[{",
        "\"id\":\"text-a\",",
        "\"key\":\"HELLO\",",
        "\"source_unit_id\":\"model-a\",",
        "\"subcategory\":\"cars/character-rigs/homer-v\"}]",
    );
    for replacement in [
        canonical.replace("\"id\":\"text-a\"", "\"id\":\"other\""),
        canonical.replace(
            "\"source_unit_id\":\"model-a\"",
            "\"source_unit_id\":\"missing\"",
        ),
        canonical.replace("cars/character-rigs/homer-v", "language/other"),
        "\"text_keys\":[{\"id\":\"text-a\"}]".to_owned(),
    ] {
        let row_text = sample_row()
            .replace("\"text_key_count\":0", "\"text_key_count\":1")
            .replace(
                "\"source_unit_ids\":[]",
                "\"source_unit_ids\":[\"model-a\"]",
            )
            .replace("\"text_key_ids\":[]", "\"text_key_ids\":[\"text-a\"]")
            .replace("\"text_keys\":[]", &replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "invalid text-key mirror was accepted: {replacement}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_inconsistent_structured_member_mirrors() -> Result<(), String> {
    let full_member = concat!(
        "\"members\":[",
        "{\"id\":\"other\",\"role\":\"model\",",
        "\"path\":\"extracted/model.p3d\",",
        "\"type\":\"model\",\"kind\":\"mesh\",",
        "\"source_chunk_kind\":\"mesh\"},",
        "{\"id\":\"texture-a\",\"role\":\"texture\",",
        "\"path\":\"extracted/texture.p3d\",",
        "\"type\":\"texture\",\"kind\":\"image\",",
        "\"source_chunk_kind\":\"texture\"}]",
    );
    let unknown_role =
        full_member.replace("\"role\":\"model\"", "\"role\":\"unknown\"");
    for replacement in [
        full_member.to_owned(),
        unknown_role,
        "\"members\":[{\"id\":\"model-a\"}]".to_owned(),
    ] {
        let row_text = sample_row().replace(SAMPLE_MEMBERS_FIELD, &replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "invalid member mirror was accepted: {replacement}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_invalid_source_chunk_ordinal() -> Result<(), String> {
    for invalid in ["0", "01x", " padded"] {
        let row_text = sample_row().replacen(
            r#""source_chunk_kind":"texture""#,
            &format!(
                concat!(
                    r#""source_chunk_kind":"texture","#,
                    r#""source_chunk_ordinal":"{}""#,
                ),
                invalid,
            ),
            1,
        );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "invalid source chunk ordinal was accepted: {invalid:?}"
            ));
        }
    }
    Ok(())
}

#[test]
fn accepts_source_chunk_ordinal_member_provenance() -> Result<(), String> {
    let row_text = sample_row().replacen(
        r#""source_chunk_kind":"texture""#,
        concat!(
            r#""source_chunk_kind":"texture","#,
            r#""source_chunk_ordinal":"5311""#,
        ),
        1,
    );
    let row = PhaseThreePackageRow::from_json_line(&row_text)
        .map_err(|error| error.to_string())?;
    let member = row
        .members()
        .first()
        .ok_or_else(|| "source ordinal fixture member is missing".to_owned())?;
    if member.source_chunk_ordinal != Some(5311) {
        return Err(
            "source chunk ordinal provenance was not preserved".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn accepts_canonical_structured_mirror_json() -> Result<(), String> {
    let row_text = sample_row().replace(
        SAMPLE_MEMBERS_FIELD,
        concat!(
            "\"members\":[",
            "{\"id\":\"texture-a\",\"role\":\"texture\",",
            "\"path\":\"extracted/texture.p3d\",",
            "\"type\":\"texture\",\"kind\":\"image\",",
            "\"source_chunk_kind\":\"texture\"},",
            "{\"id\":\"model-a\",\"role\":\"model\",",
            "\"path\":\"extracted/model.p3d\",",
            "\"type\":\"model\",\"kind\":\"mesh\",",
            "\"source_chunk_kind\":\"mesh\"}]",
        ),
    );
    let row = PhaseThreePackageRow::from_json_line(&row_text)
        .map_err(|error| error.to_string())?;
    if row.unit_ids.len() != 2 {
        return Err(
            "canonical structured mirrors changed unit intake".to_owned()
        );
    }
    Ok(())
}

#[test]
fn rejects_excessive_structured_mirror_nesting() -> Result<(), String> {
    let nested = format!(
        "\"members\":{}null{}",
        "[".repeat(MAX_JSON_NESTING.saturating_add(1)),
        "]".repeat(MAX_JSON_NESTING.saturating_add(1)),
    );
    let row_text = sample_row().replace(SAMPLE_MEMBERS_FIELD, &nested);
    if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
        return Err("excessive structured-mirror nesting must fail".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_malformed_structured_mirror_json() -> Result<(), String> {
    for replacement in [
        "\"members\":[1,]",
        "\"members\":[{\"id\":1,}]",
        "\"members\":[true false]",
    ] {
        let row_text = sample_row().replace(SAMPLE_MEMBERS_FIELD, replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "malformed mirror JSON was accepted: {replacement}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_malformed_strings_in_structured_mirrors() -> Result<(), String> {
    for (field, replacement) in [
        (
            SAMPLE_MEMBERS_FIELD,
            r#"\"members\":[{\"value\":\"bad\q\"}]"#,
        ),
        (
            "\"text_keys\":[]",
            r#"\"text_keys\":[{\"value\":\"bad\u12x4\"}]"#,
        ),
    ] {
        let row_text = sample_row().replace(field, replacement);
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "malformed mirror string was accepted: {field}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_outer_row_whitespace() -> Result<(), String> {
    for row_text in [
        format!(" {}", sample_row()),
        format!("{} ", sample_row()),
        format!("\t{}", sample_row()),
    ] {
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(
                "package rows with outer whitespace must fail".to_owned()
            );
        }
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_top_level_json_structure() -> Result<(), String> {
    let unframed = sample_row()
        .strip_prefix('{')
        .and_then(|row| row.strip_suffix('}'))
        .ok_or_else(|| "sample row framing is invalid".to_owned())?
        .to_owned();
    let trailing_garbage = format!("{}garbage", sample_row());
    let nested_identity = sample_row().replace(
        "\"package_id\":\"pkg-car\",",
        "\"metadata\":{\"package_id\":\"pkg-car\"},",
    );
    let duplicate_category = sample_row().replace(
        "\"package_category\":\"cars\",",
        concat!(
            "\"package_category\":\"cars\",",
            "\"package_category\":\"cars\",",
        ),
    );
    let trailing_comma =
        sample_row().replace("\"text_keys\":[]}", "\"text_keys\":[],}");
    for (label, row_text) in [
        ("unframed", unframed),
        ("trailing garbage", trailing_garbage),
        ("nested identity", nested_identity),
        ("duplicate key", duplicate_category),
        ("trailing comma", trailing_comma),
    ] {
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(format!(
                "noncanonical top-level JSON must be rejected: {label}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_nonportable_member_path_segments() -> Result<(), String> {
    for invalid_path in [
        "extracted/con/model.p3d",
        "extracted/PRN.txt/model.p3d",
        "extracted/com1/model.p3d",
        "extracted/lpt9.log/model.p3d",
        "extracted/folder./model.p3d",
        "extracted/folder /model.p3d",
    ] {
        let row = sample_row().replacen("extracted/model.p3d", invalid_path, 1);
        if PhaseThreePackageRow::from_json_line(&row).is_ok() {
            return Err(format!(
                "Windows-incompatible member path was accepted: \
                         {invalid_path}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_invalid_required_member_classification_fields() -> Result<(), String>
{
    for (field, canonical_value) in [
        ("type", "model"),
        ("kind", "mesh"),
        ("source_chunk_kind", "mesh"),
    ] {
        for invalid_value in ["", " padded", "control\u{0}"] {
            let canonical = format!("\"{field}\":\"{canonical_value}\"");
            let invalid = format!("\"{field}\":\"{invalid_value}\"");
            let row = sample_row().replacen(&canonical, &invalid, 1);
            if PhaseThreePackageRow::from_json_line(&row).is_ok() {
                let detail = format!("{field}={invalid_value:?}");
                return Err(format!(
                    "invalid member field must be rejected: {detail}"
                ));
            }
        }
    }
    Ok(())
}

#[test]
fn indexes_package_ids_and_prefixes() -> Result<(), String> {
    let index = PhaseThreePackageIndex::from_jsonl(sample_row())
        .map_err(|error| error.to_string())?;
    let package = index
        .require_package("pkg-car")
        .map_err(|error| error.to_string())?;
    if package.member_refs().len() != 2 {
        return Err("sample package should expose two role refs".to_owned());
    }
    if index.packages_by_category("cars").len() != 1 {
        return Err("category lookup should find sample package".to_owned());
    }
    if index
        .packages_by_subcategory_prefix("cars/character-rigs")
        .len()
        != 1
    {
        return Err("prefix lookup should find sample package".to_owned());
    }
    Ok(())
}
