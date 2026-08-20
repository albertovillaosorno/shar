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
//   - Synthetic Unreal import-plan fixture integrity validation.
// - Must-Not:
//   - Contact Unreal Editor, mutate fixtures, or inspect private game content.
// - Allows:
//   - Read repository-owned fixture bytes and compare declared public evidence.
// - Split-When:
//   - Split when another fixture family gains an independent schema lifecycle.
// - Merge-When:
//   - Merge when another test owns identical import-plan evidence validation.
// - Summary:
//   - Synthetic Unreal fixture contract test.
// - Description:
//   - Proves canonical JSON, hashes, safe paths, and read-back linkage.
// - Usage:
//   - Runs as the asset-conversion crate's external fixture contract test.
// - Defaults:
//   - Missing, stale, unsafe, or inconsistent fixture evidence fails closed.
//

//! Synthetic Unreal fixture contract test.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value};
use shar_json_text as _;
use shar_sha256::digest_hex;
use shar_unreal_conversion as _;

const FIXTURE_DIRECTORY: &str = "tests/fixtures/unreal/character_triangle";
const PLAN_FILE: &str = "unreal-import-plan.json";
const READ_BACK_FILE: &str = "expected-native-read-back.json";
const README_FILE: &str = "README.md";
const PLAN_SCHEMA: &str = "shar.unreal.import-plan.v1";
const GENERATED_TEST_ROOT: &str = "/Game/SHAR/Tests/Generated/";

#[test]
fn fixture_matches_import_and_read_back_contract() -> Result<(), String> {
    let fixture_root = repository_root().join(FIXTURE_DIRECTORY);
    let (plan_bytes, plan) = read_canonical_json(
        &fixture_root.join(PLAN_FILE),
        "synthetic Unreal import plan",
    )?;
    let (read_back_bytes, read_back) = read_canonical_json(
        &fixture_root.join(READ_BACK_FILE),
        "synthetic Unreal read-back contract",
    )?;
    let plan = object(&plan, "import plan")?;
    let read_back = object(&read_back, "read-back contract")?;

    equal_string(plan, "schema", PLAN_SCHEMA, "import plan")?;
    revision(plan, "transaction_id", "import plan")?;
    revision(plan, "package_revision", "import plan")?;
    canonical_field(plan, "canonical_id", "import plan")?;
    canonical_field(plan, "asset_family", "import plan")?;
    canonical_field(plan, "owning_feature", "import plan")?;
    canonical_field(plan, "validation_profile", "import plan")?;
    package_identity(string(plan, "package_id", "import plan")?)?;
    coordinate_contract(plan)?;

    let artifacts = array_field(plan, "artifacts", "import plan")?;
    let artifact_ids = validate_artifacts(&fixture_root, artifacts)?;
    validate_target(plan, read_back, &artifact_ids)?;
    documented_digests(&fixture_root, artifacts, &plan_bytes, &read_back_bytes)
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn read_canonical_json(
    path: &Path,
    label: &str,
) -> Result<(Vec<u8>, Value), String> {
    let bytes = fs::read(path)
        .map_err(|error| format!("failed to read {label}: {error}"))?;
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(format!("{label} contains a UTF-8 byte-order mark"));
    }
    if !bytes.ends_with(b"\n") || bytes.contains(&b'\r') {
        return Err(format!("{label} must use LF and end in one newline"));
    }
    let text = std::str::from_utf8(&bytes)
        .map_err(|error| format!("{label} is not UTF-8: {error}"))?;
    let value = serde_json::from_str::<Value>(text)
        .map_err(|error| format!("{label} is invalid JSON: {error}"))?;
    let mut canonical = serde_json::to_string_pretty(&value)
        .map_err(|error| format!("failed to render {label}: {error}"))?;
    canonical.push('\n');
    if canonical.as_bytes() != bytes {
        return Err(format!("{label} is not canonical sorted pretty JSON"));
    }
    Ok((bytes, value))
}

fn validate_artifacts(
    fixture_root: &Path,
    artifacts: &[Value],
) -> Result<BTreeSet<String>, String> {
    if artifacts.is_empty() {
        return Err("synthetic import plan has no artifacts".to_owned());
    }
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for artifact in artifacts {
        let artifact = object(artifact, "artifact")?;
        let id = string(artifact, "artifact_id", "artifact")?;
        canonical_identifier(id, "artifact identity")?;
        if !ids.insert(id.to_owned()) {
            return Err(format!("duplicate artifact identity: {id}"));
        }
        let relative_path = string(artifact, "relative_path", "artifact")?;
        relative_path_contract(relative_path)?;
        if !paths.insert(relative_path.to_owned()) {
            return Err(format!("duplicate artifact path: {relative_path}"));
        }
        if !boolean(artifact, "required", "artifact")? {
            return Err(format!("fixture artifact is not required: {id}"));
        }
        canonical_identifier(
            string(artifact, "role", "artifact")?,
            "artifact role",
        )?;
        media_type(string(artifact, "media_type", "artifact")?)?;
        let bytes =
            fs::read(fixture_root.join(relative_path)).map_err(|error| {
                format!("failed to read artifact {id}: {error}")
            })?;
        let actual_length = u64::try_from(bytes.len())
            .map_err(|error| format!("artifact length overflow: {error}"))?;
        let expected_length = unsigned(artifact, "byte_length", "artifact")?;
        if actual_length != expected_length {
            return Err(format!("artifact length mismatch for {id}"));
        }
        let expected_digest = string(artifact, "sha256", "artifact")?;
        sha256(expected_digest, "artifact digest")?;
        if digest_hex(&bytes) != expected_digest {
            return Err(format!("artifact digest mismatch for {id}"));
        }
    }
    Ok(ids)
}

fn validate_target(
    plan: &Map<String, Value>,
    read_back: &Map<String, Value>,
    artifact_ids: &BTreeSet<String>,
) -> Result<(), String> {
    let targets = array_field(plan, "targets", "import plan")?;
    let [target] = targets else {
        return Err(
            "synthetic import plan must contain exactly one target".to_owned()
        );
    };
    let target = object(target, "target")?;
    let object_path = string(target, "object_path", "target")?;
    test_object_path(object_path)?;
    equal_string(read_back, "object_path", object_path, "read-back contract")?;
    let native_class = string(target, "native_class", "target")?;
    equal_string(read_back, "asset_class", native_class, "read-back contract")?;
    canonical_field(target, "promotion_group", "target")?;
    let properties = object_field(target, "properties", "target")?;
    equal_string_arrays(
        properties,
        "material_slots",
        read_back,
        "material_slot_names",
    )?;
    validate_native_read_back(read_back)?;

    let mut referenced = BTreeSet::new();
    for source_id in array_field(target, "source_artifact_ids", "target")? {
        let source_id = source_id.as_str().ok_or_else(|| {
            "target source artifact identity must be a string".to_owned()
        })?;
        if !artifact_ids.contains(source_id) {
            return Err(format!(
                "target references unknown artifact: {source_id}"
            ));
        }
        if !referenced.insert(source_id) {
            return Err(format!("target repeats artifact: {source_id}"));
        }
    }
    if referenced.len() != artifact_ids.len() {
        return Err(
            "target does not reference every fixture artifact".to_owned()
        );
    }

    let verification = object_field(target, "verification", "target")?;
    equal_unsigned(
        verification,
        "expected_triangle_count_lod0",
        read_back,
        "triangle_count_lod0",
    )?;
    equal_unsigned(
        verification,
        "expected_uv_channel_count",
        read_back,
        "uv_channel_count",
    )?;
    let expected_materials =
        unsigned(verification, "expected_material_slot_count", "verification")?;
    let material_count = u64::try_from(
        array_field(read_back, "material_slot_names", "read-back contract")?
            .len(),
    )
    .map_err(|error| format!("material-slot count overflow: {error}"))?;
    if expected_materials != material_count {
        return Err(
            "material-slot expectation disagrees with read-back".to_owned()
        );
    }
    Ok(())
}

fn validate_native_read_back(
    read_back: &Map<String, Value>,
) -> Result<(), String> {
    let bounds = array_field(
        read_back,
        "bounds_extent_centimeters",
        "read-back contract",
    )?;
    let expected_bounds = [50., 0., 50.];
    if bounds.len() != expected_bounds.len()
        || bounds.iter().zip(expected_bounds).any(|(value, expected)| {
            value.as_f64().is_none_or(|actual| {
                !actual.is_finite() || (actual - expected).abs() > 1e-6
            })
        })
    {
        return Err("read-back bounds disagree with native evidence".to_owned());
    }
    if unsigned(read_back, "vertex_count_lod0", "read-back contract")? != 3 {
        return Err(
            "read-back vertex count disagrees with native evidence".to_owned()
        );
    }
    if unsigned(read_back, "lod_count", "read-back contract")? != 1 {
        return Err(
            "read-back LOD count disagrees with native evidence".to_owned()
        );
    }
    Ok(())
}

fn equal_string_arrays(
    left: &Map<String, Value>,
    left_field: &str,
    right: &Map<String, Value>,
    right_field: &str,
) -> Result<(), String> {
    let left = array_field(left, left_field, "target properties")?;
    let right = array_field(right, right_field, "read-back contract")?;
    if left.len() != right.len()
        || left.iter().zip(right).any(|(left, right)| {
            left.as_str()
                .zip(right.as_str())
                .is_none_or(|(left, right)| left != right)
        })
    {
        return Err("target material slots disagree with read-back".to_owned());
    }
    Ok(())
}

fn coordinate_contract(plan: &Map<String, Value>) -> Result<(), String> {
    let source = object_field(plan, "source_coordinate_system", "import plan")?;
    equal_string(source, "distance_unit", "centimeter", "source coordinates")?;
    equal_string(source, "forward_axis", "+x", "source coordinates")?;
    equal_string(source, "up_axis", "+z", "source coordinates")?;
    equal_string(source, "handedness", "right", "source coordinates")?;
    let target = object_field(plan, "target_coordinate_system", "import plan")?;
    equal_string(target, "distance_unit", "centimeter", "target coordinates")?;
    equal_string(target, "forward_axis", "+x", "target coordinates")?;
    equal_string(target, "up_axis", "+z", "target coordinates")?;
    equal_string(target, "handedness", "unreal", "target coordinates")
}

fn documented_digests(
    fixture_root: &Path,
    artifacts: &[Value],
    plan_bytes: &[u8],
    read_back_bytes: &[u8],
) -> Result<(), String> {
    let readme = fs::read_to_string(fixture_root.join(README_FILE))
        .map_err(|error| format!("failed to read fixture README: {error}"))?;
    documented_digest(&readme, PLAN_FILE, plan_bytes)?;
    documented_digest(&readme, READ_BACK_FILE, read_back_bytes)?;
    for artifact in artifacts {
        let artifact = object(artifact, "artifact")?;
        let filename = string(artifact, "relative_path", "artifact")?;
        let bytes = fs::read(fixture_root.join(filename)).map_err(|error| {
            format!("failed to read documented artifact: {error}")
        })?;
        documented_digest(&readme, filename, &bytes)?;
    }
    Ok(())
}

fn documented_digest(
    readme: &str,
    filename: &str,
    bytes: &[u8],
) -> Result<(), String> {
    let expected = format!("{}  {filename}", digest_hex(bytes));
    if !readme.lines().any(|line| line == expected) {
        return Err(format!("fixture README has stale digest for {filename}"));
    }
    Ok(())
}

fn relative_path_contract(path: &str) -> Result<(), String> {
    if path.is_empty()
        || path.starts_with('/')
        || path.contains(char::from(92))
        || path.contains(':')
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(format!("unsafe fixture artifact path: {path}"));
    }
    Ok(())
}

fn test_object_path(path: &str) -> Result<(), String> {
    if !path.starts_with(GENERATED_TEST_ROOT) || path.contains("//") {
        return Err("fixture target escapes the generated test root".to_owned());
    }
    let Some((package, object)) = path.rsplit_once('.') else {
        return Err("fixture target has no object name".to_owned());
    };
    let Some((_parent, asset)) = package.rsplit_once('/') else {
        return Err("fixture target has no package name".to_owned());
    };
    if object != asset || !unreal_name(object) {
        return Err(
            "fixture target package and object names disagree".to_owned()
        );
    }
    if package
        .split('/')
        .filter(|segment| !segment.is_empty())
        .any(|segment| !unreal_name(segment))
    {
        return Err("fixture target contains a noncanonical segment".to_owned());
    }
    Ok(())
}

fn unreal_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn package_identity(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value
            .split('.')
            .any(|part| canonical_identifier(part, "package identity").is_err())
    {
        return Err("invalid synthetic package identity".to_owned());
    }
    Ok(())
}

fn canonical_identifier(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty()
        || value.starts_with('_')
        || value.ends_with('_')
        || value.contains("__")
        || !value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'
        })
    {
        return Err(format!("invalid canonical {label}: {value}"));
    }
    Ok(())
}

fn media_type(value: &str) -> Result<(), String> {
    if !matches!(value, "application/vnd.autodesk.fbx" | "image/png") {
        return Err(format!("unsupported fixture media type: {value}"));
    }
    Ok(())
}

fn sha256(value: &str, label: &str) -> Result<(), String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(format!("invalid {label}"));
    }
    Ok(())
}

fn revision(
    object: &Map<String, Value>,
    field: &str,
    label: &str,
) -> Result<(), String> {
    let value = string(object, field, label)?;
    let Some(digest) = value.strip_prefix("sha256:") else {
        return Err(format!("{label} field {field} has no SHA-256 prefix"));
    };
    sha256(digest, field)
}

fn canonical_field(
    object: &Map<String, Value>,
    field: &str,
    label: &str,
) -> Result<(), String> {
    canonical_identifier(string(object, field, label)?, field)
}

fn equal_string(
    object: &Map<String, Value>,
    field: &str,
    expected: &str,
    label: &str,
) -> Result<(), String> {
    if string(object, field, label)? != expected {
        return Err(format!(
            "{label} field {field} disagrees with its contract"
        ));
    }
    Ok(())
}

fn equal_unsigned(
    left: &Map<String, Value>,
    left_field: &str,
    right: &Map<String, Value>,
    right_field: &str,
) -> Result<(), String> {
    if unsigned(left, left_field, "verification")?
        != unsigned(right, right_field, "read-back contract")?
    {
        return Err(format!(
            concat!(
                "verification field {} disagrees with ",
                "read-back field {}"
            ),
            left_field, right_field
        ));
    }
    Ok(())
}

fn object<'value>(
    value: &'value Value,
    label: &str,
) -> Result<&'value Map<String, Value>, String> {
    value
        .as_object()
        .ok_or_else(|| format!("{label} must be an object"))
}

fn object_field<'value>(
    object: &'value Map<String, Value>,
    field: &str,
    label: &str,
) -> Result<&'value Map<String, Value>, String> {
    object
        .get(field)
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{label} field {field} must be an object"))
}

fn array_field<'value>(
    object: &'value Map<String, Value>,
    field: &str,
    label: &str,
) -> Result<&'value [Value], String> {
    object
        .get(field)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| format!("{label} field {field} must be an array"))
}

fn string<'value>(
    object: &'value Map<String, Value>,
    field: &str,
    label: &str,
) -> Result<&'value str, String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{label} field {field} must be a string"))
}

fn unsigned(
    object: &Map<String, Value>,
    field: &str,
    label: &str,
) -> Result<u64, String> {
    object.get(field).and_then(Value::as_u64).ok_or_else(|| {
        format!("{label} field {field} must be an unsigned integer")
    })
}

fn boolean(
    object: &Map<String, Value>,
    field: &str,
    label: &str,
) -> Result<bool, String> {
    object
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("{label} field {field} must be a Boolean"))
}
