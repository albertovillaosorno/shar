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
//   - Index domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Index domain module.
// - Description:
//   - Implements the declared domain module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Index domain module.

use std::collections::BTreeMap;
use std::fmt;

mod json;

#[cfg(test)]
use json::MAX_JSON_NESTING;
use json::{
    extract_string_array, extract_string_field, extract_usize_field,
    parse_json_string_at, skip_json_ws, value_cursor,
};

/// Package-index read or parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackageIntakeError {
    /// Human-readable error message.
    message: String,
}

impl PackageIntakeError {
    /// Build a package intake error.
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

/// Returns untrusted diagnostic text without raw control characters.
fn escaped_diagnostic_text(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        if character.is_control() {
            output.extend(character.escape_default());
        } else {
            output.push(character);
        }
    }
    output
}

impl fmt::Display for PackageIntakeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered_message = escaped_diagnostic_text(&self.message);
        formatter.write_str(&rendered_message)
    }
}

impl std::error::Error for PackageIntakeError {}

/// Stable role bucket exposed by a package row.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PackageRole {
    /// World geometry, roads, interiors, or world records.
    World,
    /// Texture image payload.
    Texture,
    /// Material or shader payload.
    Material,
    /// Mesh or model payload.
    Model,
    /// Collision or physics payload.
    Physics,
    /// Animation or skeleton payload.
    Animation,
    /// Scene graph payload.
    Scene,
    /// Locator payload.
    Locator,
    /// Camera payload.
    Camera,
    /// Light payload.
    Light,
    /// Particle payload.
    Particle,
    /// Controller payload.
    Controller,
    /// Audio payload.
    Audio,
    /// Movie payload.
    Movie,
    /// Script payload.
    Script,
    /// Text or localization payload.
    Text,
    /// UI layout or UI resource payload.
    Ui,
    /// Metadata payload that is not imported directly.
    Metadata,
    /// Error payload. Phase three should normally reject these rows.
    Error,
}

impl PackageRole {
    /// Stable role label used in generated index rows.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::World => "world",
            Self::Texture => "texture",
            Self::Material => "material",
            Self::Model => "model",
            Self::Physics => "physics",
            Self::Animation => "animation",
            Self::Scene => "scene",
            Self::Locator => "locator",
            Self::Camera => "camera",
            Self::Light => "light",
            Self::Particle => "particle",
            Self::Controller => "controller",
            Self::Audio => "audio",
            Self::Movie => "movie",
            Self::Script => "script",
            Self::Text => "text",
            Self::Ui => "ui",
            Self::Metadata => "metadata",
            Self::Error => "error",
        }
    }

    /// Stable id bucket field name in the package-index JSONL schema.
    const fn id_field(self) -> &'static str {
        match self {
            Self::World => "world_ids",
            Self::Texture => "texture_ids",
            Self::Material => "material_ids",
            Self::Model => "model_ids",
            Self::Physics => "physics_ids",
            Self::Animation => "animation_ids",
            Self::Scene => "scene_ids",
            Self::Locator => "locator_ids",
            Self::Camera => "camera_ids",
            Self::Light => "light_ids",
            Self::Particle => "particle_ids",
            Self::Controller => "controller_ids",
            Self::Audio => "audio_ids",
            Self::Movie => "movie_ids",
            Self::Script => "script_ids",
            Self::Text => "text_ids",
            Self::Ui => "ui_ids",
            Self::Metadata => "metadata_ids",
            Self::Error => "error_ids",
        }
    }

    /// All stable roles in canonical output order.
    #[must_use]
    pub const fn all() -> [Self; 19] {
        [
            Self::World,
            Self::Texture,
            Self::Material,
            Self::Model,
            Self::Physics,
            Self::Animation,
            Self::Scene,
            Self::Locator,
            Self::Camera,
            Self::Light,
            Self::Particle,
            Self::Controller,
            Self::Audio,
            Self::Movie,
            Self::Script,
            Self::Text,
            Self::Ui,
            Self::Metadata,
            Self::Error,
        ]
    }
}

/// One package member id annotated with its phase-three role bucket.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageMemberRef {
    /// Minor-unit id.
    pub id: String,
    /// Role bucket that made the id relevant to the plan.
    pub role: PackageRole,
}

/// One physical package member with its published extraction evidence.
// The phase-qualified name keeps this intake record distinct from the
// role-only reference and from phase-two writer records.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhaseThreePackageMember {
    /// Minor-unit id.
    pub id: String,
    /// Role bucket assigned by the generated index.
    pub role: PackageRole,
    /// Safe relative extraction path published by the generated index.
    pub path: String,
    /// Controlled unit type published by the generated index.
    pub unit_type: String,
    /// Controlled unit kind published by the generated index.
    pub kind: String,
    /// Source chunk kind published by the generated index.
    pub source_chunk_kind: String,
}

/// Controls whether package-index intake accepts fail-closed evidence rows.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PackageIntakeMode {
    /// Every row must be directly eligible for phase-three planning.
    ImportableOnly,
    /// Error rows are validated but excluded from the resulting index.
    UnrealEvidence,
}

impl PackageIntakeMode {
    /// Return whether canonical error evidence is permitted during parsing.
    const fn allows_error_evidence(self) -> bool {
        matches!(self, Self::UnrealEvidence)
    }
}

/// One phase-three package row read from the generated package index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhaseThreePackageRow {
    /// Stable package id used by future phase-three callers.
    pub package_id: String,
    /// Exact phase-two package root used to derive the stable package id.
    pub package_root: String,
    /// Stable high-level package category.
    pub category: String,
    /// Exact package subcategory produced by phase two.
    pub subcategory: String,
    /// Manifest member ids in package order.
    pub unit_ids: Vec<String>,
    /// Derived text key ids for text packages.
    pub text_key_ids: Vec<String>,
    /// Source units referenced by derived packages.
    pub source_unit_ids: Vec<String>,
    /// Ids grouped by stable role bucket.
    role_ids: BTreeMap<PackageRole, Vec<String>>,
    /// Physical members with published extraction evidence.
    members: Vec<PhaseThreePackageMember>,
}

/// Decode every canonical role bucket once.
fn parse_role_ids(
    line: &str,
) -> Result<BTreeMap<PackageRole, Vec<String>>, PackageIntakeError> {
    let mut role_ids = BTreeMap::new();
    for role in PackageRole::all() {
        let previous =
            role_ids.insert(role, extract_string_array(line, role.id_field())?);
        if previous.is_some() {
            return Err(PackageIntakeError::new("duplicated role bucket"));
        }
    }
    Ok(role_ids)
}

/// Decode scalar and identifier fields after the role map is available.
fn parse_package_fields(
    line: &str,
    role_ids: BTreeMap<PackageRole, Vec<String>>,
) -> Result<PhaseThreePackageRow, PackageIntakeError> {
    Ok(PhaseThreePackageRow {
        package_id: extract_string_field(line, "package_id")?,
        package_root: extract_string_field(line, "package_root")?,
        category: extract_string_field(line, "package_category")?,
        subcategory: extract_string_field(line, "package_subcategory")?,
        unit_ids: extract_string_array(line, "unit_ids")?,
        text_key_ids: extract_string_array(line, "text_key_ids")?,
        source_unit_ids: extract_string_array(line, "source_unit_ids")?,
        role_ids,
        members: Vec::new(),
    })
}

/// Require canonical structured mirror fields with array values.
fn validate_mirror_fields(line: &str) -> Result<(), PackageIntakeError> {
    let bytes = line.as_bytes();
    for field in ["members", "text_keys"] {
        let cursor = value_cursor(line, field)?;
        if bytes.get(cursor) != Some(&b'[') {
            return Err(PackageIntakeError::new(format!(
                "field {field} is not an array"
            )));
        }
    }
    Ok(())
}

/// Decode one canonical object whose fields are ordered JSON strings.
fn parse_string_object_fields(
    row: &str,
    start: usize,
    fields: &[&str],
) -> Result<(Vec<String>, usize), PackageIntakeError> {
    let bytes = row.as_bytes();
    if bytes.get(start) != Some(&b'{') {
        return Err(PackageIntakeError::new(
            "structured mirror member is not an object",
        ));
    }
    let mut cursor = skip_json_ws(row, start.saturating_add(1));
    let mut values = Vec::with_capacity(fields.len());
    for (index, expected_field) in fields.iter().enumerate() {
        if bytes.get(cursor) != Some(&b'"') {
            return Err(PackageIntakeError::new(
                "structured mirror object has a malformed key",
            ));
        }
        let (field, field_end) = parse_json_string_at(row, cursor)?;
        if field != *expected_field {
            return Err(PackageIntakeError::new(format!(
                "structured mirror expected field {expected_field}, \
                         found {field}"
            )));
        }
        cursor = skip_json_ws(row, field_end);
        if bytes.get(cursor) != Some(&b':') {
            return Err(PackageIntakeError::new(
                "structured mirror field is missing a colon",
            ));
        }
        cursor = skip_json_ws(row, cursor.saturating_add(1));
        if bytes.get(cursor) != Some(&b'"') {
            return Err(PackageIntakeError::new(format!(
                "structured mirror field {field} is not a string"
            )));
        }
        let (value, value_end) = parse_json_string_at(row, cursor)?;
        values.push(value);
        cursor = skip_json_ws(row, value_end);
        let last = index.saturating_add(1) == fields.len();
        match (last, bytes.get(cursor)) {
            (false, Some(b',')) => {
                cursor = skip_json_ws(row, cursor.saturating_add(1));
            },
            (true, Some(b'}')) => {
                return Ok((values, cursor.saturating_add(1)));
            },
            _ => {
                return Err(PackageIntakeError::new(
                    "structured mirror object has noncanonical fields",
                ));
            },
        }
    }
    Err(PackageIntakeError::new(
        "structured mirror object has no fields",
    ))
}

/// Canonical ordered fields for one physical package member mirror.
const MEMBER_MIRROR_FIELDS: [&str; 6] =
    ["id", "role", "path", "type", "kind", "source_chunk_kind"];

/// Return whether a path segment uses a host-reserved filename.
fn is_reserved_portable_path_segment(segment: &str) -> bool {
    let stem = segment.split('.').next().unwrap_or(segment);
    if ["con", "prn", "aux", "nul", "clock$", "conin$", "conout$"]
        .iter()
        .any(|reserved| stem.eq_ignore_ascii_case(reserved))
    {
        return true;
    }
    let Some(prefix) = stem.get(..3) else {
        return false;
    };
    stem.as_bytes().get(3).is_some_and(|number| {
        matches!(number, b'1'..=b'9')
            && (prefix.eq_ignore_ascii_case("com")
                || prefix.eq_ignore_ascii_case("lpt"))
    }) && stem.len() == 4
}

/// Validate one member path as safe, relative, and traversal-free.
fn validate_member_path(path: &str) -> Result<(), PackageIntakeError> {
    if path.trim().is_empty() || path != path.trim() {
        return Err(PackageIntakeError::new(
            "member mirror has a blank padded path",
        ));
    }
    if path.contains('\\') || path.contains(':') {
        return Err(PackageIntakeError::new(format!(
            "member mirror path is not portable: {path}"
        )));
    }
    if path.chars().any(char::is_control) {
        return Err(PackageIntakeError::new(
            "member mirror path contains control characters",
        ));
    }
    if path.starts_with('/') {
        return Err(PackageIntakeError::new(format!(
            "member mirror path is not relative: {path}"
        )));
    }
    for segment in path.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." {
            return Err(PackageIntakeError::new(format!(
                "member mirror path allows traversal: {path}"
            )));
        }
        if segment.ends_with('.')
            || segment.ends_with(' ')
            || is_reserved_portable_path_segment(segment)
        {
            return Err(PackageIntakeError::new(format!(
                "member mirror path is not portable: {path}"
            )));
        }
    }
    Ok(())
}

/// Reject missing or padded member classification evidence.
fn validate_required_member_field(
    field: &str,
    value: &str,
) -> Result<(), PackageIntakeError> {
    if value.is_empty() || value != value.trim() {
        return Err(PackageIntakeError::new(format!(
            "member mirror has an invalid {field}: {value:?}"
        )));
    }
    if value.chars().any(char::is_control) {
        return Err(PackageIntakeError::new(format!(
            "member mirror {field} contains control characters"
        )));
    }
    Ok(())
}

/// Convert one ordered member mirror field list into the intake record.
fn member_from_values(
    parsed_values: Vec<String>,
) -> Result<PhaseThreePackageMember, PackageIntakeError> {
    let mut values = parsed_values.into_iter();
    let id = values
        .next()
        .ok_or_else(|| PackageIntakeError::new("member mirror has no id"))?;
    let role_text = values
        .next()
        .ok_or_else(|| PackageIntakeError::new("member mirror has no role"))?;
    let role = PackageRole::all()
        .into_iter()
        .find(|candidate| candidate.as_str() == role_text)
        .ok_or_else(|| {
            PackageIntakeError::new(format!(
                "member mirror has an unknown role: {role_text}"
            ))
        })?;
    let path = values
        .next()
        .ok_or_else(|| PackageIntakeError::new("member mirror has no path"))?;
    validate_member_path(&path)?;
    let unit_type = values
        .next()
        .ok_or_else(|| PackageIntakeError::new("member mirror has no type"))?;
    validate_required_member_field("type", &unit_type)?;
    let kind = values
        .next()
        .ok_or_else(|| PackageIntakeError::new("member mirror has no kind"))?;
    validate_required_member_field("kind", &kind)?;
    let source_chunk_kind = values.next().ok_or_else(|| {
        PackageIntakeError::new("member mirror has no source chunk kind")
    })?;
    validate_required_member_field("source chunk kind", &source_chunk_kind)?;
    Ok(PhaseThreePackageMember {
        id,
        role,
        path,
        unit_type,
        kind,
        source_chunk_kind,
    })
}

/// Decode physical member mirrors from one canonical package row.
fn parse_member_mirrors(
    line: &str,
) -> Result<Vec<PhaseThreePackageMember>, PackageIntakeError> {
    let bytes = line.as_bytes();
    let mut cursor = value_cursor(line, "members")?;
    cursor = skip_json_ws(line, cursor.saturating_add(1));
    if bytes.get(cursor) == Some(&b']') {
        return Ok(Vec::new());
    }
    let mut members = Vec::new();
    loop {
        let (parsed_values, object_end) =
            parse_string_object_fields(line, cursor, &MEMBER_MIRROR_FIELDS)?;
        members.push(member_from_values(parsed_values)?);
        cursor = skip_json_ws(line, object_end);
        match bytes.get(cursor) {
            Some(b',') => {
                cursor = skip_json_ws(line, cursor.saturating_add(1));
                if bytes.get(cursor) == Some(&b']') {
                    return Err(PackageIntakeError::new(
                        "members mirror has a trailing array comma",
                    ));
                }
            },
            Some(b']') => return Ok(members),
            _ => {
                return Err(PackageIntakeError::new(
                    "members mirror has a malformed array delimiter",
                ));
            },
        }
    }
}

/// Require the exact member ordering emitted by phase two.
fn validate_member_order(
    members: &[PhaseThreePackageMember],
) -> Result<(), PackageIntakeError> {
    for (left, right) in members.iter().zip(members.iter().skip(1)) {
        let ordering = left
            .role
            .cmp(&right.role)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.id.cmp(&right.id));
        if ordering.is_gt() {
            return Err(PackageIntakeError::new(
                "members mirror is not in canonical phase-two order",
            ));
        }
    }
    Ok(())
}

/// Validate non-empty physical member mirrors against canonical id buckets.
fn validate_member_mirrors(
    line: &str,
    row: &PhaseThreePackageRow,
) -> Result<Vec<PhaseThreePackageMember>, PackageIntakeError> {
    let members = parse_member_mirrors(line)?;
    validate_member_order(&members)?;
    if members.is_empty() {
        if row.unit_ids.is_empty() {
            return Ok(members);
        }
        return Err(PackageIntakeError::new(
            "members mirror is empty for physical unit_ids",
        ));
    }
    let member_ids: Vec<_> =
        members.iter().map(|member| member.id.clone()).collect();
    if member_ids != row.unit_ids {
        return Err(PackageIntakeError::new(
            "members mirror ids do not match unit_ids",
        ));
    }
    for role in PackageRole::all() {
        let role_ids: Vec<_> = members
            .iter()
            .filter(|member| member.role == role)
            .map(|member| member.id.clone())
            .collect();
        if role_ids != row.ids_for_role(role) {
            return Err(PackageIntakeError::new(format!(
                "members mirror role {} does not match {}",
                role.as_str(),
                role.id_field()
            )));
        }
    }
    Ok(members)
}

/// Canonical ordered fields for one derived text-key mirror.
const TEXT_KEY_MIRROR_FIELDS: [&str; 4] =
    ["id", "key", "source_unit_id", "subcategory"];

/// Reject blank, padded, or control-bearing localization keys.
fn validate_text_key_value(key: &str) -> Result<(), PackageIntakeError> {
    if key.is_empty() || key != key.trim() {
        return Err(PackageIntakeError::new(format!(
            "text-key mirror has an invalid key: {key:?}"
        )));
    }
    if key.chars().any(char::is_control) {
        return Err(PackageIntakeError::new(
            "text-key mirror key contains control characters",
        ));
    }
    Ok(())
}

/// Decode derived text-key mirrors from one canonical package row.
fn parse_text_key_mirrors(
    line: &str,
) -> Result<Vec<(String, String, String)>, PackageIntakeError> {
    let bytes = line.as_bytes();
    let mut cursor = value_cursor(line, "text_keys")?;
    cursor = skip_json_ws(line, cursor.saturating_add(1));
    if bytes.get(cursor) == Some(&b']') {
        return Ok(Vec::new());
    }
    let mut keys = Vec::new();
    loop {
        let (parsed_values, object_end) =
            parse_string_object_fields(line, cursor, &TEXT_KEY_MIRROR_FIELDS)?;
        let mut value_iter = parsed_values.into_iter();
        let id = value_iter.next().ok_or_else(|| {
            PackageIntakeError::new("text-key mirror has no id")
        })?;
        let key = value_iter.next().ok_or_else(|| {
            PackageIntakeError::new("text-key mirror has no key")
        })?;
        validate_text_key_value(&key)?;
        let source_unit_id = value_iter.next().ok_or_else(|| {
            PackageIntakeError::new("text-key mirror has no source id")
        })?;
        let subcategory = value_iter.next().ok_or_else(|| {
            PackageIntakeError::new("text-key mirror has no subcategory")
        })?;
        keys.push((id, source_unit_id, subcategory));
        cursor = skip_json_ws(line, object_end);
        match bytes.get(cursor) {
            Some(b',') => {
                cursor = skip_json_ws(line, cursor.saturating_add(1));
                if bytes.get(cursor) == Some(&b']') {
                    return Err(PackageIntakeError::new(
                        "text_keys mirror has a trailing array comma",
                    ));
                }
            },
            Some(b']') => return Ok(keys),
            _ => {
                return Err(PackageIntakeError::new(
                    "text_keys mirror has a malformed array delimiter",
                ));
            },
        }
    }
}

/// Validate non-empty derived text-key mirrors against canonical evidence.
fn validate_text_key_mirrors(
    line: &str,
    row: &PhaseThreePackageRow,
) -> Result<(), PackageIntakeError> {
    let keys = parse_text_key_mirrors(line)?;
    if keys.is_empty() {
        if row.text_key_ids.is_empty() {
            return Ok(());
        }
        return Err(PackageIntakeError::new(
            "text_keys mirror is empty for derived text_key_ids",
        ));
    }
    let key_ids: Vec<_> = keys.iter().map(|(id, _, _)| id.clone()).collect();
    if key_ids != row.text_key_ids {
        return Err(PackageIntakeError::new(
            "text_keys mirror ids do not match text_key_ids",
        ));
    }
    for (_, source_unit_id, subcategory) in keys {
        if !row.source_unit_ids.contains(&source_unit_id) {
            return Err(PackageIntakeError::new(format!(
                "text-key mirror source is not declared: \
                         {source_unit_id}"
            )));
        }
        if subcategory != row.subcategory {
            return Err(PackageIntakeError::new(
                "text-key mirror subcategory does not match package",
            ));
        }
    }
    Ok(())
}

/// Verify declared counts against their decoded identifier arrays.
fn validate_declared_counts(
    line: &str,
    row: &PhaseThreePackageRow,
) -> Result<(), PackageIntakeError> {
    let unit_count = extract_usize_field(line, "unit_count")?;
    if unit_count != row.unit_ids.len() {
        return Err(PackageIntakeError::new(format!(
            "unit_count {unit_count} does not match {} unit ids",
            row.unit_ids.len()
        )));
    }
    let text_key_count = extract_usize_field(line, "text_key_count")?;
    if text_key_count != row.text_key_ids.len() {
        return Err(PackageIntakeError::new(format!(
            "text_key_count {text_key_count} does not match {} text \
                     key ids",
            row.text_key_ids.len()
        )));
    }
    Ok(())
}

/// Validate package identity and routing taxonomy.
fn validate_package_identity(
    row: &PhaseThreePackageRow,
    mode: PackageIntakeMode,
) -> Result<(), PackageIntakeError> {
    validate_required_scalars(row)?;
    validate_package_root(&row.package_root)?;
    validate_root_identity(row)?;
    validate_category(row, mode)?;
    validate_subcategory(&row.subcategory)?;
    if !is_canonical_slug(&row.package_id) {
        return Err(PackageIntakeError::new(
            "package_id contains noncanonical characters",
        ));
    }
    Ok(())
}

/// Reject empty package identity scalars.
fn validate_required_scalars(
    row: &PhaseThreePackageRow,
) -> Result<(), PackageIntakeError> {
    for (field, value) in [
        ("package_id", row.package_id.as_str()),
        ("package_root", row.package_root.as_str()),
        ("package_category", row.category.as_str()),
        ("package_subcategory", row.subcategory.as_str()),
    ] {
        if value.is_empty() {
            return Err(PackageIntakeError::new(format!(
                "field {field} must not be empty"
            )));
        }
    }
    Ok(())
}

/// Reject package roots that cannot be safe relative manifest paths.
fn validate_package_root(root: &str) -> Result<(), PackageIntakeError> {
    if root != root.trim()
        || root.starts_with('/')
        || root.ends_with('/')
        || root.as_bytes().contains(&92)
        || root.contains(':')
        || root.chars().any(char::is_control)
    {
        return Err(PackageIntakeError::new(format!(
            "package_root is not a portable relative path: {root:?}"
        )));
    }
    for segment in root.split('/') {
        if segment.is_empty()
            || segment == "."
            || segment == ".."
            || segment.ends_with('.')
            || segment.ends_with(' ')
            || is_reserved_portable_path_segment(segment)
        {
            return Err(PackageIntakeError::new(format!(
                "package_root has an invalid segment: {root:?}"
            )));
        }
    }
    Ok(())
}

/// Bind a stable package id to its exact phase-two root transform.
///
/// The transform mirrors the phase-two package-id writer exactly: separator
/// runs collapse into one dash and leading or trailing separators vanish, so
/// roots such as `pure3d/_stubs` bind to `pure3d-stubs` rather than a
/// double-dash id that phase two never emits.
fn validate_root_identity(
    row: &PhaseThreePackageRow,
) -> Result<(), PackageIntakeError> {
    let mut expected = String::with_capacity(row.package_root.len());
    let mut separator_pending = false;
    for character in row.package_root.chars() {
        if character.is_ascii_alphanumeric() {
            if separator_pending && !expected.is_empty() {
                expected.push('-');
            }
            expected.push(character.to_ascii_lowercase());
            separator_pending = false;
        } else if !expected.is_empty() {
            separator_pending = true;
        }
    }
    if row.package_id != expected {
        return Err(PackageIntakeError::new(format!(
            "package_id {} does not match package_root {}",
            row.package_id, row.package_root
        )));
    }
    Ok(())
}

/// Reject unresolved or unsupported package categories.
fn validate_category(
    row: &PhaseThreePackageRow,
    mode: PackageIntakeMode,
) -> Result<(), PackageIntakeError> {
    if is_supported_category(&row.category)
        || (mode.allows_error_evidence() && row.category == "error")
    {
        return Ok(());
    }
    Err(PackageIntakeError::new(format!(
        "unsupported package category: {}",
        row.category
    )))
}

/// Return whether phase two can emit this successful category.
fn is_supported_category(category: &str) -> bool {
    matches!(
        category,
        "characters"
            | "cars"
            | "terrain-world"
            | "missions"
            | "cards"
            | "ui-screens"
            | "ui-images"
            | "ui-resources"
            | "ui-vehicle-previews"
            | "ui-components"
            | "language"
            | "cinematics"
            | "music"
            | "dialog"
            | "sound-effects"
            | "movies"
            | "mission-scripts"
            | "vehicle-tuning"
            | "sound-scripts"
            | "props"
            | "extraction-reports"
            | "game-icons"
    )
}

/// Validate slash-separated kebab-case taxonomy without placeholders.
fn validate_subcategory(subcategory: &str) -> Result<(), PackageIntakeError> {
    let segments = subcategory.split('/').collect::<Vec<_>>();
    if !segments.iter().copied().all(is_canonical_slug) {
        return Err(PackageIntakeError::new(
            "package_subcategory is not canonical kebab-case",
        ));
    }
    if segments.iter().any(|segment| {
        matches!(
            *segment,
            "unknown" | "generic" | "misc" | "context" | "shared" | "global"
        )
    }) {
        return Err(PackageIntakeError::new(
            "package_subcategory contains a placeholder segment",
        ));
    }
    Ok(())
}

/// Return whether a stable identity token is lowercase ASCII kebab-case.
fn is_canonical_slug(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        && bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        && !bytes.windows(2).any(|pair| pair == b"--")
        && bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
}

/// Validate empty and duplicate identifiers in every package array.
fn validate_identifier_arrays(
    row: &PhaseThreePackageRow,
) -> Result<(), PackageIntakeError> {
    for (field, ids) in [
        ("unit_ids", row.unit_ids.as_slice()),
        ("text_key_ids", row.text_key_ids.as_slice()),
        ("source_unit_ids", row.source_unit_ids.as_slice()),
    ] {
        reject_empty_ids(field, ids)?;
        reject_duplicate_ids(field, ids)?;
    }
    Ok(())
}

/// Verify exact one-role coverage for every physical package member.
fn validate_role_assignments(
    row: &PhaseThreePackageRow,
) -> Result<(), PackageIntakeError> {
    let physical_ids = row
        .unit_ids
        .iter()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    let mut assigned_ids = std::collections::BTreeSet::new();
    for role in PackageRole::all() {
        let ids = row.ids_for_role(role);
        reject_empty_ids(role.id_field(), ids)?;
        reject_duplicate_ids(role.id_field(), ids)?;
        for id in ids {
            if !physical_ids.contains(id.as_str()) {
                return Err(PackageIntakeError::new(format!(
                    "field {} references absent unit id {id}",
                    role.id_field()
                )));
            }
            if !assigned_ids.insert(id.as_str()) {
                return Err(PackageIntakeError::new(format!(
                    "unit id {id} is assigned to multiple roles"
                )));
            }
        }
    }
    if assigned_ids != physical_ids {
        return Err(PackageIntakeError::new(
            "unit_ids contains a member without a role",
        ));
    }
    Ok(())
}

/// Enforce fail-closed member presence and derived provenance.
fn validate_package_members(
    row: &PhaseThreePackageRow,
    mode: PackageIntakeMode,
) -> Result<(), PackageIntakeError> {
    if row.category == "error" {
        if !mode.allows_error_evidence() {
            return Err(PackageIntakeError::new(
                "package row contains error-role members",
            ));
        }
        if !row.has_error_ids()
            || row.ids_for_role(PackageRole::Error) != row.unit_ids
        {
            return Err(PackageIntakeError::new(concat!(
                "error package must route every physical member ",
                "through error_ids"
            )));
        }
        if !row.text_key_ids.is_empty() || !row.source_unit_ids.is_empty() {
            return Err(PackageIntakeError::new(
                "error package must not publish derived members",
            ));
        }
        if row.subcategory != "error" && !row.subcategory.starts_with("error/")
        {
            return Err(PackageIntakeError::new(
                "error package must use the error taxonomy",
            ));
        }
        return Ok(());
    }
    if row.has_error_ids() {
        return Err(PackageIntakeError::new(
            "successful package row contains error-role members",
        ));
    }
    if row.unit_ids.is_empty() && row.text_key_ids.is_empty() {
        return Err(PackageIntakeError::new(
            "package row contains no physical or derived members",
        ));
    }
    if !row.text_key_ids.is_empty() && row.source_unit_ids.is_empty() {
        return Err(PackageIntakeError::new(
            "derived text keys require source_unit_ids",
        ));
    }
    Ok(())
}

impl PhaseThreePackageRow {
    /// Parse one canonical package-index JSONL row.
    ///
    /// # Errors
    ///
    /// Returns an error when the row is not the canonical package-index JSONL
    /// schema emitted by phase two.
    pub fn from_json_line(line: &str) -> Result<Self, PackageIntakeError> {
        Self::from_json_line_with_mode(line, PackageIntakeMode::ImportableOnly)
    }

    /// Parse one canonical row under a caller-selected intake mode.
    fn from_json_line_with_mode(
        line: &str,
        mode: PackageIntakeMode,
    ) -> Result<Self, PackageIntakeError> {
        if line.trim() != line {
            return Err(PackageIntakeError::new(
                "package row contains outer whitespace",
            ));
        }
        validate_mirror_fields(line)?;
        let role_ids = parse_role_ids(line)?;
        let mut row = parse_package_fields(line, role_ids)?;
        row.members = validate_member_mirrors(line, &row)?;
        validate_declared_counts(line, &row)?;
        validate_package_identity(&row, mode)?;
        validate_identifier_arrays(&row)?;
        validate_text_key_mirrors(line, &row)?;
        validate_role_assignments(&row)?;
        validate_package_members(&row, mode)?;
        Ok(row)
    }

    /// Return ids belonging to a role bucket.
    #[must_use]
    pub fn ids_for_role(&self, role: PackageRole) -> &[String] {
        self.role_ids.get(&role).map_or(&[], Vec::as_slice)
    }

    /// Return physical members with their published extraction evidence.
    #[must_use]
    pub fn members(&self) -> &[PhaseThreePackageMember] {
        &self.members
    }

    /// Return role-annotated member ids for all non-empty role buckets.
    #[must_use]
    pub fn member_refs(&self) -> Vec<PackageMemberRef> {
        let mut refs = Vec::new();
        for role in PackageRole::all() {
            if role == PackageRole::Error {
                continue;
            }
            for id in self.ids_for_role(role) {
                refs.push(PackageMemberRef { id: id.clone(), role });
            }
        }
        refs
    }

    /// True when this package has model-like component evidence that can enter
    /// FBX planning before Unreal import.
    #[must_use]
    pub fn has_model_components(&self) -> bool {
        [
            PackageRole::Model,
            PackageRole::Animation,
            PackageRole::Scene,
            PackageRole::World,
            PackageRole::Physics,
            PackageRole::Locator,
            PackageRole::Camera,
        ]
        .into_iter()
        .any(|role| !self.ids_for_role(role).is_empty())
    }

    /// True when this package contains ids that phase three must reject before
    /// generating a conversion plan.
    #[must_use]
    pub fn has_error_ids(&self) -> bool {
        !self.ids_for_role(PackageRole::Error).is_empty()
    }
}

/// Loaded phase-three package index.
// The phase-qualified name prevents callers from confusing this strict reader
// with the separate phase-two index model that generates the consumed rows.
#[expect(
    clippy::module_name_repetitions,
    reason = "The phase-qualified public name distinguishes this strict \
              intake               model from the phase-two package index \
              that produces its rows."
)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PhaseThreePackageIndex {
    /// Canonically ordered package rows preserved for deterministic iteration.
    packages: Vec<PhaseThreePackageRow>,
    /// Package-id lookup offsets avoid rebuilding maps for repeated selectors.
    by_id: BTreeMap<String, usize>,
}

/// Incremental fail-closed package-index assembly state.
#[derive(Default)]
struct PackageIndexBuilder {
    /// Canonically ordered rows accumulated so far.
    packages: Vec<PhaseThreePackageRow>,
    /// Package identifier to row offset lookup.
    by_id: BTreeMap<String, usize>,
    /// Physical identifiers already claimed by prior packages.
    physical_ids: std::collections::BTreeSet<String>,
    /// Derived identifiers already claimed by prior packages.
    derived_ids: std::collections::BTreeSet<String>,
}

impl PackageIndexBuilder {
    /// Add one validated row while enforcing index-wide uniqueness and order.
    fn push(
        &mut self,
        row: PhaseThreePackageRow,
    ) -> Result<(), PackageIntakeError> {
        self.validate_package_id(&row)?;
        self.claim_ids("physical unit id", &row.unit_ids, true)?;
        self.claim_ids("derived text key id", &row.text_key_ids, false)?;
        let previous = self
            .by_id
            .insert(row.package_id.clone(), self.packages.len());
        debug_assert!(
            previous.is_none(),
            "package id was checked before insertion"
        );
        self.packages.push(row);
        Ok(())
    }

    /// Finish the index after all source references can be resolved globally.
    fn finish(self) -> Result<PhaseThreePackageIndex, PackageIntakeError> {
        if self.packages.is_empty() {
            return Err(PackageIntakeError::new(
                "package index contains no rows",
            ));
        }
        validate_source_coverage(&self.packages, &self.physical_ids)?;
        Ok(PhaseThreePackageIndex {
            packages: self.packages,
            by_id: self.by_id,
        })
    }

    /// Finish an Unreal-facing index after excluding canonical error evidence.
    fn finish_unreal_evidence(
        self,
    ) -> Result<PhaseThreePackageIndex, PackageIntakeError> {
        let packages = self
            .packages
            .into_iter()
            .filter(|package| package.category != "error")
            .collect::<Vec<_>>();
        if packages.is_empty() {
            return Err(PackageIntakeError::new(
                "package index contains no importable rows",
            ));
        }
        let physical_ids = packages
            .iter()
            .flat_map(|package| package.unit_ids.iter().cloned())
            .collect::<std::collections::BTreeSet<_>>();
        validate_source_coverage(&packages, &physical_ids)?;
        let by_id = packages
            .iter()
            .enumerate()
            .map(|(index, package)| (package.package_id.clone(), index))
            .collect();
        Ok(PhaseThreePackageIndex { packages, by_id })
    }

    /// Reject duplicate ids and descending canonical row order.
    fn validate_package_id(
        &self,
        row: &PhaseThreePackageRow,
    ) -> Result<(), PackageIntakeError> {
        if self.by_id.contains_key(&row.package_id) {
            return Err(PackageIntakeError::new(format!(
                "duplicate package id: {}",
                row.package_id
            )));
        }
        if let Some(previous) = self.packages.last()
            && previous.package_id > row.package_id
        {
            return Err(PackageIntakeError::new(format!(
                "package ids are not canonically ordered: {} before {}",
                previous.package_id, row.package_id
            )));
        }
        Ok(())
    }

    /// Claim one index-wide identifier namespace exactly once.
    fn claim_ids(
        &mut self,
        label: &str,
        ids: &[String],
        physical: bool,
    ) -> Result<(), PackageIntakeError> {
        for id in ids {
            let opposite_claimed = if physical {
                self.derived_ids.contains(id)
            } else {
                self.physical_ids.contains(id)
            };
            if opposite_claimed {
                return Err(PackageIntakeError::new(format!(
                    "identifier is claimed as both physical and \
                             derived: {id}"
                )));
            }
            let inserted = if physical {
                self.physical_ids.insert(id.clone())
            } else {
                self.derived_ids.insert(id.clone())
            };
            if !inserted {
                return Err(PackageIntakeError::new(format!(
                    "{label} is claimed by multiple packages: {id}"
                )));
            }
        }
        Ok(())
    }
}

/// Resolve every derived source id against final physical coverage.
fn validate_source_coverage(
    packages: &[PhaseThreePackageRow],
    physical_ids: &std::collections::BTreeSet<String>,
) -> Result<(), PackageIntakeError> {
    for package in packages {
        for source_unit_id in &package.source_unit_ids {
            if !physical_ids.contains(source_unit_id) {
                return Err(PackageIntakeError::new(format!(
                    "derived source unit id is absent from physical \
                             coverage: {source_unit_id}"
                )));
            }
        }
    }
    Ok(())
}

impl PhaseThreePackageIndex {
    /// Parse generated package index JSONL contents.
    ///
    /// # Errors
    ///
    /// Returns an error when any row is malformed or package ids are
    /// duplicated.
    pub fn from_jsonl(contents: &str) -> Result<Self, PackageIntakeError> {
        Self::from_jsonl_with_mode(contents, PackageIntakeMode::ImportableOnly)
    }

    /// Parse an Unreal-facing index while retaining fail-closed rows only as
    /// validation evidence.
    pub(crate) fn from_jsonl_for_unreal(
        contents: &str,
    ) -> Result<Self, PackageIntakeError> {
        Self::from_jsonl_with_mode(contents, PackageIntakeMode::UnrealEvidence)
    }

    /// Parse generated JSONL under one explicit intake mode.
    fn from_jsonl_with_mode(
        contents: &str,
        mode: PackageIntakeMode,
    ) -> Result<Self, PackageIntakeError> {
        let mut builder = PackageIndexBuilder::default();
        for (line_index, line) in contents.lines().enumerate() {
            if line.trim().is_empty() {
                return Err(PackageIntakeError::new(format!(
                    "package index contains a blank row at line {}",
                    line_index.saturating_add(1)
                )));
            }
            let row =
                PhaseThreePackageRow::from_json_line_with_mode(line, mode)
                    .map_err(|error| {
                        PackageIntakeError::new(format!(
                            "failed to parse package row {}: {error}",
                            line_index.saturating_add(1)
                        ))
                    })?;
            builder.push(row)?;
        }
        if mode.allows_error_evidence() {
            builder.finish_unreal_evidence()
        } else {
            builder.finish()
        }
    }

    /// Return all packages in deterministic index order.
    #[must_use]
    pub fn packages(&self) -> &[PhaseThreePackageRow] {
        &self.packages
    }

    /// Find one package by id.
    #[must_use]
    pub fn find_package(
        &self,
        package_id: &str,
    ) -> Option<&PhaseThreePackageRow> {
        self.by_id
            .get(package_id)
            .and_then(|index| self.packages.get(*index))
    }

    /// Require one package by id.
    ///
    /// # Errors
    ///
    /// Returns an error when the package id is not present in the index.
    pub fn require_package(
        &self,
        package_id: &str,
    ) -> Result<&PhaseThreePackageRow, PackageIntakeError> {
        self.find_package(package_id).ok_or_else(|| {
            PackageIntakeError::new(format!(
                "package id not found: {package_id}"
            ))
        })
    }

    /// List packages by exact category.
    #[must_use]
    pub fn packages_by_category(
        &self,
        category: &str,
    ) -> Vec<&PhaseThreePackageRow> {
        self.packages
            .iter()
            .filter(|package| package.category == category)
            .collect()
    }

    /// List packages whose exact subcategory begins with a prefix.
    #[must_use]
    pub fn packages_by_subcategory_prefix(
        &self,
        prefix: &str,
    ) -> Vec<&PhaseThreePackageRow> {
        self.packages
            .iter()
            .filter(|package| package.subcategory.starts_with(prefix))
            .collect()
    }
}

/// Return whether one stable unit identifier is canonical ASCII kebab-case.
fn is_canonical_identifier(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        && bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        && !bytes.windows(2).any(|pair| pair == b"--")
        && bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
}

/// Rejects identifiers that cannot name a package member or derived record.
fn reject_empty_ids(
    field: &str,
    ids: &[String],
) -> Result<(), PackageIntakeError> {
    if let Some(id) = ids.iter().find(|id| !is_canonical_identifier(id)) {
        return Err(PackageIntakeError::new(format!(
            "field {field} contains a noncanonical identifier: {id}"
        )));
    }
    Ok(())
}

/// Rejects repeated identifiers inside one canonical package array.
fn reject_duplicate_ids(
    field: &str,
    ids: &[String],
) -> Result<(), PackageIntakeError> {
    let mut seen = std::collections::BTreeSet::new();
    for id in ids {
        if !seen.insert(id.as_str()) {
            return Err(PackageIntakeError::new(format!(
                "field {field} duplicates identifier {id}"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/index/tests.rs"]
mod tests;
