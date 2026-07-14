// File:
//   - index.rs
// Path:
//   - src/pipeline/src/domain/package/index.rs
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
//   - The index contract for pipeline phase three package.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute index.
// - Split-When:
//   - Split when index contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Phase-three package-index reader.
// - Description:
//   - Defines index data and behavior for pipeline phase three package.
// - Usage:
//   - Used by pipeline phase three package code that needs index.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Phase-three package-index reader keeps tightly coupled
//   - validation, ordering, and deterministic transformation invariants
//   - together; split when a stable independently testable sub-boundary is
//   - identified.
//

//! Phase-three package-index reader.
//!
//! This boundary keeps phase-three package-index reader explicit and returns
//! deterministic results to pipeline callers.

use std::collections::BTreeMap;
use std::path::Path;
use std::{fmt, fs};

/// Package-index read or parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackageIntakeError {
    /// Human-readable error message.
    message: String,
}

impl PackageIntakeError {
    /// Build a package intake error.
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for PackageIntakeError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.message)
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
    line: &str
) -> Result<BTreeMap<PackageRole, Vec<String>>, PackageIntakeError> {
    let mut role_ids = BTreeMap::new();
    for role in PackageRole::all() {
        let previous = role_ids.insert(
            role,
            extract_string_array(
                line,
                role.id_field(),
            )?,
        );
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
    Ok(
        PhaseThreePackageRow {
            package_id: extract_string_field(
                line,
                "package_id",
            )?,
            package_root: extract_string_field(
                line,
                "package_root",
            )?,
            category: extract_string_field(
                line,
                "package_category",
            )?,
            subcategory: extract_string_field(
                line,
                "package_subcategory",
            )?,
            unit_ids: extract_string_array(
                line, "unit_ids",
            )?,
            text_key_ids: extract_string_array(
                line,
                "text_key_ids",
            )?,
            source_unit_ids: extract_string_array(
                line,
                "source_unit_ids",
            )?,
            role_ids,
            members: Vec::new(),
        },
    )
}

/// Require canonical structured mirror fields with array values.
fn validate_mirror_fields(line: &str) -> Result<(), PackageIntakeError> {
    let bytes = line.as_bytes();
    for field in [
        "members",
        "text_keys",
    ] {
        let cursor = value_cursor(
            line, field,
        )?;
        if bytes.get(cursor) != Some(&b'[') {
            return Err(
                PackageIntakeError::new(
                    format!("field {field} is not an array"),
                ),
            );
        }
    }
    Ok(())
}

/// Decode one canonical object whose fields are ordered JSON strings.
fn parse_string_object_fields(
    row: &str,
    start: usize,
    fields: &[&str],
) -> Result<
    (
        Vec<String>,
        usize,
    ),
    PackageIntakeError,
> {
    let bytes = row.as_bytes();
    if bytes.get(start) != Some(&b'{') {
        return Err(
            PackageIntakeError::new(
                "structured mirror member is not an object",
            ),
        );
    }
    let mut cursor = skip_json_ws(
        row,
        start.saturating_add(1),
    );
    let mut values = Vec::with_capacity(fields.len());
    for (index, expected_field) in fields
        .iter()
        .enumerate()
    {
        if bytes.get(cursor) != Some(&b'"') {
            return Err(
                PackageIntakeError::new(
                    "structured mirror object has a malformed key",
                ),
            );
        }
        let (field, field_end) = parse_json_string_at(
            row, cursor,
        )?;
        if field != *expected_field {
            return Err(
                PackageIntakeError::new(
                    format!(
                        "structured mirror expected field {expected_field}, \
                         found {field}"
                    ),
                ),
            );
        }
        cursor = skip_json_ws(
            row, field_end,
        );
        if bytes.get(cursor) != Some(&b':') {
            return Err(
                PackageIntakeError::new(
                    "structured mirror field is missing a colon",
                ),
            );
        }
        cursor = skip_json_ws(
            row,
            cursor.saturating_add(1),
        );
        if bytes.get(cursor) != Some(&b'"') {
            return Err(
                PackageIntakeError::new(
                    format!("structured mirror field {field} is not a string"),
                ),
            );
        }
        let (value, value_end) = parse_json_string_at(
            row, cursor,
        )?;
        values.push(value);
        cursor = skip_json_ws(
            row, value_end,
        );
        let last = index.saturating_add(1) == fields.len();
        match (
            last,
            bytes.get(cursor),
        ) {
            (false, Some(b',')) => {
                cursor = skip_json_ws(
                    row,
                    cursor.saturating_add(1),
                );
            }
            (true, Some(b'}')) => {
                return Ok(
                    (
                        values,
                        cursor.saturating_add(1),
                    ),
                );
            }
            _ => {
                return Err(
                    PackageIntakeError::new(
                        "structured mirror object has noncanonical fields",
                    ),
                );
            }
        }
    }
    Err(PackageIntakeError::new("structured mirror object has no fields"))
}

/// Canonical ordered fields for one physical package member mirror.
const MEMBER_MIRROR_FIELDS: [&str; 6] = [
    "id",
    "role",
    "path",
    "type",
    "kind",
    "source_chunk_kind",
];

/// Return whether a path segment uses a host-reserved filename.
fn is_reserved_portable_path_segment(segment: &str) -> bool {
    let stem = segment
        .split('.')
        .next()
        .unwrap_or(segment);
    if [
        "con", "prn", "aux", "nul", "clock$", "conin$", "conout$",
    ]
    .iter()
    .any(|reserved| stem.eq_ignore_ascii_case(reserved))
    {
        return true;
    }
    let Some(prefix) = stem.get(..3) else {
        return false;
    };
    stem.as_bytes()
        .get(3)
        .is_some_and(
            |number| {
                matches!(
                    number,
                    b'1'..=b'9'
                ) && (prefix.eq_ignore_ascii_case("com")
                    || prefix.eq_ignore_ascii_case("lpt"))
            },
        )
        && stem.len() == 4
}

/// Validate one member path as safe, relative, and traversal-free.
fn validate_member_path(path: &str) -> Result<(), PackageIntakeError> {
    if path
        .trim()
        .is_empty()
        || path != path.trim()
    {
        return Err(
            PackageIntakeError::new("member mirror has a blank padded path"),
        );
    }
    if path.contains('\\') || path.contains(':') {
        return Err(
            PackageIntakeError::new(
                format!("member mirror path is not portable: {path}"),
            ),
        );
    }
    if path
        .chars()
        .any(char::is_control)
    {
        return Err(
            PackageIntakeError::new(
                "member mirror path contains control characters",
            ),
        );
    }
    if path.starts_with('/') {
        return Err(
            PackageIntakeError::new(
                format!("member mirror path is not relative: {path}"),
            ),
        );
    }
    for segment in path.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." {
            return Err(
                PackageIntakeError::new(
                    format!("member mirror path allows traversal: {path}"),
                ),
            );
        }
        if segment.ends_with('.')
            || segment.ends_with(' ')
            || is_reserved_portable_path_segment(segment)
        {
            return Err(
                PackageIntakeError::new(
                    format!("member mirror path is not portable: {path}"),
                ),
            );
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
        return Err(
            PackageIntakeError::new(
                format!("member mirror has an invalid {field}: {value:?}"),
            ),
        );
    }
    if value
        .chars()
        .any(char::is_control)
    {
        return Err(
            PackageIntakeError::new(
                format!("member mirror {field} contains control characters"),
            ),
        );
    }
    Ok(())
}

/// Convert one ordered member mirror field list into the intake record.
fn member_from_values(
    parsed_values: Vec<String>
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
        .ok_or_else(
            || {
                PackageIntakeError::new(
                    format!("member mirror has an unknown role: {role_text}"),
                )
            },
        )?;
    let path = values
        .next()
        .ok_or_else(|| PackageIntakeError::new("member mirror has no path"))?;
    validate_member_path(&path)?;
    let unit_type = values
        .next()
        .ok_or_else(|| PackageIntakeError::new("member mirror has no type"))?;
    validate_required_member_field(
        "type", &unit_type,
    )?;
    let kind = values
        .next()
        .ok_or_else(|| PackageIntakeError::new("member mirror has no kind"))?;
    validate_required_member_field(
        "kind", &kind,
    )?;
    let source_chunk_kind = values
        .next()
        .ok_or_else(
            || {
                PackageIntakeError::new(
                    "member mirror has no source chunk kind",
                )
            },
        )?;
    validate_required_member_field(
        "source chunk kind",
        &source_chunk_kind,
    )?;
    Ok(
        PhaseThreePackageMember {
            id,
            role,
            path,
            unit_type,
            kind,
            source_chunk_kind,
        },
    )
}

/// Decode physical member mirrors from one canonical package row.
fn parse_member_mirrors(
    line: &str
) -> Result<Vec<PhaseThreePackageMember>, PackageIntakeError> {
    let bytes = line.as_bytes();
    let mut cursor = value_cursor(
        line, "members",
    )?;
    cursor = skip_json_ws(
        line,
        cursor.saturating_add(1),
    );
    if bytes.get(cursor) == Some(&b']') {
        return Ok(Vec::new());
    }
    let mut members = Vec::new();
    loop {
        let (parsed_values, object_end) = parse_string_object_fields(
            line,
            cursor,
            &MEMBER_MIRROR_FIELDS,
        )?;
        members.push(member_from_values(parsed_values)?);
        cursor = skip_json_ws(
            line, object_end,
        );
        match bytes.get(cursor) {
            Some(b',') => {
                cursor = skip_json_ws(
                    line,
                    cursor.saturating_add(1),
                );
                if bytes.get(cursor) == Some(&b']') {
                    return Err(
                        PackageIntakeError::new(
                            "members mirror has a trailing array comma",
                        ),
                    );
                }
            }
            Some(b']') => return Ok(members),
            _ => {
                return Err(
                    PackageIntakeError::new(
                        "members mirror has a malformed array delimiter",
                    ),
                );
            }
        }
    }
}

/// Require the exact member ordering emitted by phase two.
fn validate_member_order(
    members: &[PhaseThreePackageMember]
) -> Result<(), PackageIntakeError> {
    for (left, right) in members
        .iter()
        .zip(
            members
                .iter()
                .skip(1),
        )
    {
        let ordering = left
            .role
            .cmp(&right.role)
            .then_with(
                || {
                    left.path
                        .cmp(&right.path)
                },
            )
            .then_with(
                || {
                    left.id
                        .cmp(&right.id)
                },
            );
        if ordering.is_gt() {
            return Err(
                PackageIntakeError::new(
                    "members mirror is not in canonical phase-two order",
                ),
            );
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
        if row
            .unit_ids
            .is_empty()
        {
            return Ok(members);
        }
        return Err(
            PackageIntakeError::new(
                "members mirror is empty for physical unit_ids",
            ),
        );
    }
    let member_ids: Vec<_> = members
        .iter()
        .map(
            |member| {
                member
                    .id
                    .clone()
            },
        )
        .collect();
    if member_ids != row.unit_ids {
        return Err(
            PackageIntakeError::new("members mirror ids do not match unit_ids"),
        );
    }
    for role in PackageRole::all() {
        let role_ids: Vec<_> = members
            .iter()
            .filter(|member| member.role == role)
            .map(
                |member| {
                    member
                        .id
                        .clone()
                },
            )
            .collect();
        if role_ids != row.ids_for_role(role) {
            return Err(
                PackageIntakeError::new(
                    format!(
                        "members mirror role {} does not match {}",
                        role.as_str(),
                        role.id_field()
                    ),
                ),
            );
        }
    }
    Ok(members)
}

/// Canonical ordered fields for one derived text-key mirror.
const TEXT_KEY_MIRROR_FIELDS: [&str; 4] = [
    "id",
    "key",
    "source_unit_id",
    "subcategory",
];

/// Reject blank, padded, or control-bearing localization keys.
fn validate_text_key_value(key: &str) -> Result<(), PackageIntakeError> {
    if key.is_empty() || key != key.trim() {
        return Err(
            PackageIntakeError::new(
                format!("text-key mirror has an invalid key: {key:?}"),
            ),
        );
    }
    if key
        .chars()
        .any(char::is_control)
    {
        return Err(
            PackageIntakeError::new(
                "text-key mirror key contains control characters",
            ),
        );
    }
    Ok(())
}

/// Decode derived text-key mirrors from one canonical package row.
fn parse_text_key_mirrors(
    line: &str
) -> Result<
    Vec<(
        String,
        String,
        String,
    )>,
    PackageIntakeError,
> {
    let bytes = line.as_bytes();
    let mut cursor = value_cursor(
        line,
        "text_keys",
    )?;
    cursor = skip_json_ws(
        line,
        cursor.saturating_add(1),
    );
    if bytes.get(cursor) == Some(&b']') {
        return Ok(Vec::new());
    }
    let mut keys = Vec::new();
    loop {
        let (parsed_values, object_end) = parse_string_object_fields(
            line,
            cursor,
            &TEXT_KEY_MIRROR_FIELDS,
        )?;
        let mut value_iter = parsed_values.into_iter();
        let id = value_iter
            .next()
            .ok_or_else(
                || PackageIntakeError::new("text-key mirror has no id"),
            )?;
        let key = value_iter
            .next()
            .ok_or_else(
                || PackageIntakeError::new("text-key mirror has no key"),
            )?;
        validate_text_key_value(&key)?;
        let source_unit_id = value_iter
            .next()
            .ok_or_else(
                || PackageIntakeError::new("text-key mirror has no source id"),
            )?;
        let subcategory = value_iter
            .next()
            .ok_or_else(
                || {
                    PackageIntakeError::new(
                        "text-key mirror has no subcategory",
                    )
                },
            )?;
        keys.push(
            (
                id,
                source_unit_id,
                subcategory,
            ),
        );
        cursor = skip_json_ws(
            line, object_end,
        );
        match bytes.get(cursor) {
            Some(b',') => {
                cursor = skip_json_ws(
                    line,
                    cursor.saturating_add(1),
                );
                if bytes.get(cursor) == Some(&b']') {
                    return Err(
                        PackageIntakeError::new(
                            "text_keys mirror has a trailing array comma",
                        ),
                    );
                }
            }
            Some(b']') => return Ok(keys),
            _ => {
                return Err(
                    PackageIntakeError::new(
                        "text_keys mirror has a malformed array delimiter",
                    ),
                );
            }
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
        if row
            .text_key_ids
            .is_empty()
        {
            return Ok(());
        }
        return Err(
            PackageIntakeError::new(
                "text_keys mirror is empty for derived text_key_ids",
            ),
        );
    }
    let key_ids: Vec<_> = keys
        .iter()
        .map(|(id, _, _)| id.clone())
        .collect();
    if key_ids != row.text_key_ids {
        return Err(
            PackageIntakeError::new(
                "text_keys mirror ids do not match text_key_ids",
            ),
        );
    }
    for (_, source_unit_id, subcategory) in keys {
        if !row
            .source_unit_ids
            .contains(&source_unit_id)
        {
            return Err(
                PackageIntakeError::new(
                    format!(
                        "text-key mirror source is not declared: \
                         {source_unit_id}"
                    ),
                ),
            );
        }
        if subcategory != row.subcategory {
            return Err(
                PackageIntakeError::new(
                    "text-key mirror subcategory does not match package",
                ),
            );
        }
    }
    Ok(())
}

/// Verify declared counts against their decoded identifier arrays.
fn validate_declared_counts(
    line: &str,
    row: &PhaseThreePackageRow,
) -> Result<(), PackageIntakeError> {
    let unit_count = extract_usize_field(
        line,
        "unit_count",
    )?;
    if unit_count
        != row
            .unit_ids
            .len()
    {
        return Err(
            PackageIntakeError::new(
                format!(
                    "unit_count {unit_count} does not match {} unit ids",
                    row.unit_ids
                        .len()
                ),
            ),
        );
    }
    let text_key_count = extract_usize_field(
        line,
        "text_key_count",
    )?;
    if text_key_count
        != row
            .text_key_ids
            .len()
    {
        return Err(
            PackageIntakeError::new(
                format!(
                    "text_key_count {text_key_count} does not match {} text \
                     key ids",
                    row.text_key_ids
                        .len()
                ),
            ),
        );
    }
    Ok(())
}

/// Validate package identity and routing taxonomy.
fn validate_package_identity(
    row: &PhaseThreePackageRow
) -> Result<(), PackageIntakeError> {
    validate_required_scalars(row)?;
    validate_package_root(&row.package_root)?;
    validate_root_identity(row)?;
    validate_category(row)?;
    validate_subcategory(&row.subcategory)?;
    if !is_canonical_slug(&row.package_id) {
        return Err(
            PackageIntakeError::new(
                "package_id contains noncanonical characters",
            ),
        );
    }
    Ok(())
}

/// Reject empty package identity scalars.
fn validate_required_scalars(
    row: &PhaseThreePackageRow
) -> Result<(), PackageIntakeError> {
    for (field, value) in [
        (
            "package_id",
            row.package_id
                .as_str(),
        ),
        (
            "package_root",
            row.package_root
                .as_str(),
        ),
        (
            "package_category",
            row.category
                .as_str(),
        ),
        (
            "package_subcategory",
            row.subcategory
                .as_str(),
        ),
    ] {
        if value.is_empty() {
            return Err(
                PackageIntakeError::new(
                    format!("field {field} must not be empty"),
                ),
            );
        }
    }
    Ok(())
}

/// Reject package roots that cannot be safe relative manifest paths.
fn validate_package_root(root: &str) -> Result<(), PackageIntakeError> {
    if root != root.trim()
        || root.starts_with('/')
        || root.ends_with('/')
        || root
            .as_bytes()
            .contains(&92)
        || root.contains(':')
        || root
            .chars()
            .any(char::is_control)
    {
        return Err(
            PackageIntakeError::new(
                format!(
                    "package_root is not a portable relative path: {root:?}"
                ),
            ),
        );
    }
    for segment in root.split('/') {
        if segment.is_empty()
            || segment == "."
            || segment == ".."
            || segment.ends_with('.')
            || segment.ends_with(' ')
            || is_reserved_portable_path_segment(segment)
        {
            return Err(
                PackageIntakeError::new(
                    format!("package_root has an invalid segment: {root:?}"),
                ),
            );
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
    row: &PhaseThreePackageRow
) -> Result<(), PackageIntakeError> {
    let mut expected = String::with_capacity(
        row.package_root
            .len(),
    );
    let mut separator_pending = false;
    for character in row
        .package_root
        .chars()
    {
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
        return Err(
            PackageIntakeError::new(
                format!(
                    "package_id {} does not match package_root {}",
                    row.package_id, row.package_root
                ),
            ),
        );
    }
    Ok(())
}

/// Reject unresolved or unsupported package categories.
fn validate_category(
    row: &PhaseThreePackageRow
) -> Result<(), PackageIntakeError> {
    if is_supported_category(&row.category) {
        return Ok(());
    }
    Err(
        PackageIntakeError::new(
            format!(
                "unsupported package category: {}",
                row.category
            ),
        ),
    )
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
    let segments = subcategory
        .split('/')
        .collect::<Vec<_>>();
    if !segments
        .iter()
        .copied()
        .all(is_canonical_slug)
    {
        return Err(
            PackageIntakeError::new(
                "package_subcategory is not canonical kebab-case",
            ),
        );
    }
    if segments
        .iter()
        .any(
            |segment| {
                matches!(
                    *segment,
                    "unknown"
                        | "generic"
                        | "misc"
                        | "context"
                        | "shared"
                        | "global"
                )
            },
        )
    {
        return Err(
            PackageIntakeError::new(
                "package_subcategory contains a placeholder segment",
            ),
        );
    }
    Ok(())
}

/// Return whether a stable identity token is lowercase ASCII kebab-case.
fn is_canonical_slug(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
        && bytes
            .last()
            .is_some_and(u8::is_ascii_alphanumeric)
        && !bytes
            .windows(2)
            .any(|pair| pair == b"--")
        && bytes
            .iter()
            .copied()
            .all(
                |byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || byte == b'-'
                },
            )
}

/// Validate empty and duplicate identifiers in every package array.
fn validate_identifier_arrays(
    row: &PhaseThreePackageRow
) -> Result<(), PackageIntakeError> {
    for (field, ids) in [
        (
            "unit_ids",
            row.unit_ids
                .as_slice(),
        ),
        (
            "text_key_ids",
            row.text_key_ids
                .as_slice(),
        ),
        (
            "source_unit_ids",
            row.source_unit_ids
                .as_slice(),
        ),
    ] {
        reject_empty_ids(
            field, ids,
        )?;
        reject_duplicate_ids(
            field, ids,
        )?;
    }
    Ok(())
}

/// Verify exact one-role coverage for every physical package member.
fn validate_role_assignments(
    row: &PhaseThreePackageRow
) -> Result<(), PackageIntakeError> {
    let physical_ids = row
        .unit_ids
        .iter()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    let mut assigned_ids = std::collections::BTreeSet::new();
    for role in PackageRole::all() {
        let ids = row.ids_for_role(role);
        reject_empty_ids(
            role.id_field(),
            ids,
        )?;
        reject_duplicate_ids(
            role.id_field(),
            ids,
        )?;
        for id in ids {
            if !physical_ids.contains(id.as_str()) {
                return Err(
                    PackageIntakeError::new(
                        format!(
                            "field {} references absent unit id {id}",
                            role.id_field()
                        ),
                    ),
                );
            }
            if !assigned_ids.insert(id.as_str()) {
                return Err(
                    PackageIntakeError::new(
                        format!("unit id {id} is assigned to multiple roles"),
                    ),
                );
            }
        }
    }
    if assigned_ids != physical_ids {
        return Err(
            PackageIntakeError::new(
                "unit_ids contains a member without a role",
            ),
        );
    }
    Ok(())
}

/// Enforce fail-closed member presence and derived provenance.
fn validate_package_members(
    row: &PhaseThreePackageRow
) -> Result<(), PackageIntakeError> {
    if row.has_error_ids() {
        return Err(
            PackageIntakeError::new("package row contains error-role members"),
        );
    }
    if row
        .unit_ids
        .is_empty()
        && row
            .text_key_ids
            .is_empty()
    {
        return Err(
            PackageIntakeError::new(
                "package row contains no physical or derived members",
            ),
        );
    }
    if !row
        .text_key_ids
        .is_empty()
        && row
            .source_unit_ids
            .is_empty()
    {
        return Err(
            PackageIntakeError::new(
                "derived text keys require source_unit_ids",
            ),
        );
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
        if line.trim() != line {
            return Err(
                PackageIntakeError::new(
                    "package row contains outer whitespace",
                ),
            );
        }
        validate_mirror_fields(line)?;
        let role_ids = parse_role_ids(line)?;
        let mut row = parse_package_fields(
            line, role_ids,
        )?;
        row.members = validate_member_mirrors(
            line, &row,
        )?;
        validate_declared_counts(
            line, &row,
        )?;
        validate_package_identity(&row)?;
        validate_identifier_arrays(&row)?;
        validate_text_key_mirrors(
            line, &row,
        )?;
        validate_role_assignments(&row)?;
        validate_package_members(&row)?;
        Ok(row)
    }

    /// Return ids belonging to a role bucket.
    #[must_use]
    pub fn ids_for_role(
        &self,
        role: PackageRole,
    ) -> &[String] {
        self.role_ids
            .get(&role)
            .map_or(
                &[],
                Vec::as_slice,
            )
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
                refs.push(
                    PackageMemberRef {
                        id: id.clone(),
                        role,
                    },
                );
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
        .any(
            |role| {
                !self
                    .ids_for_role(role)
                    .is_empty()
            },
        )
    }

    /// True when this package contains ids that phase three must reject before
    /// generating a conversion plan.
    #[must_use]
    pub fn has_error_ids(&self) -> bool {
        !self
            .ids_for_role(PackageRole::Error)
            .is_empty()
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
        self.claim_ids(
            "physical unit id",
            &row.unit_ids,
            true,
        )?;
        self.claim_ids(
            "derived text key id",
            &row.text_key_ids,
            false,
        )?;
        let previous = self
            .by_id
            .insert(
                row.package_id
                    .clone(),
                self.packages
                    .len(),
            );
        debug_assert!(
            previous.is_none(),
            "package id was checked before insertion"
        );
        self.packages
            .push(row);
        Ok(())
    }

    /// Finish the index after all source references can be resolved globally.
    fn finish(self) -> Result<PhaseThreePackageIndex, PackageIntakeError> {
        if self
            .packages
            .is_empty()
        {
            return Err(
                PackageIntakeError::new("package index contains no rows"),
            );
        }
        validate_source_coverage(
            &self.packages,
            &self.physical_ids,
        )?;
        Ok(
            PhaseThreePackageIndex {
                packages: self.packages,
                by_id: self.by_id,
            },
        )
    }

    /// Reject duplicate ids and descending canonical row order.
    fn validate_package_id(
        &self,
        row: &PhaseThreePackageRow,
    ) -> Result<(), PackageIntakeError> {
        if self
            .by_id
            .contains_key(&row.package_id)
        {
            return Err(
                PackageIntakeError::new(
                    format!(
                        "duplicate package id: {}",
                        row.package_id
                    ),
                ),
            );
        }
        if let Some(previous) = self
            .packages
            .last()
            && previous.package_id > row.package_id
        {
            return Err(
                PackageIntakeError::new(
                    format!(
                        "package ids are not canonically ordered: {} before {}",
                        previous.package_id, row.package_id
                    ),
                ),
            );
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
                self.derived_ids
                    .contains(id)
            } else {
                self.physical_ids
                    .contains(id)
            };
            if opposite_claimed {
                return Err(
                    PackageIntakeError::new(
                        format!(
                            "identifier is claimed as both physical and \
                             derived: {id}"
                        ),
                    ),
                );
            }
            let inserted = if physical {
                self.physical_ids
                    .insert(id.clone())
            } else {
                self.derived_ids
                    .insert(id.clone())
            };
            if !inserted {
                return Err(
                    PackageIntakeError::new(
                        format!(
                            "{label} is claimed by multiple packages: {id}"
                        ),
                    ),
                );
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
                return Err(
                    PackageIntakeError::new(
                        format!(
                            "derived source unit id is absent from physical \
                             coverage: {source_unit_id}"
                        ),
                    ),
                );
            }
        }
    }
    Ok(())
}

impl PhaseThreePackageIndex {
    /// Read the generated package index JSONL file from disk.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read, a row cannot be parsed,
    /// or duplicate package ids are encountered.
    pub fn read(path: &Path) -> Result<Self, PackageIntakeError> {
        let contents = fs::read_to_string(path).map_err(
            |error| {
                PackageIntakeError::new(
                    format!(
                        "failed to read package index {}: {error}",
                        path.display()
                    ),
                )
            },
        )?;
        Self::from_jsonl(&contents)
    }

    /// Parse generated package index JSONL contents.
    ///
    /// # Errors
    ///
    /// Returns an error when any row is malformed or package ids are
    /// duplicated.
    pub fn from_jsonl(contents: &str) -> Result<Self, PackageIntakeError> {
        let mut builder = PackageIndexBuilder::default();
        for (line_index, line) in contents
            .lines()
            .enumerate()
        {
            if line
                .trim()
                .is_empty()
            {
                return Err(
                    PackageIntakeError::new(
                        format!(
                            "package index contains a blank row at line {}",
                            line_index.saturating_add(1)
                        ),
                    ),
                );
            }
            let row = PhaseThreePackageRow::from_json_line(line).map_err(
                |error| {
                    PackageIntakeError::new(
                        format!(
                            "failed to parse package row {}: {error}",
                            line_index.saturating_add(1)
                        ),
                    )
                },
            )?;
            builder.push(row)?;
        }
        builder.finish()
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
            .and_then(
                |index| {
                    self.packages
                        .get(*index)
                },
            )
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
        self.find_package(package_id)
            .ok_or_else(
                || {
                    PackageIntakeError::new(
                        format!("package id not found: {package_id}"),
                    )
                },
            )
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
            .filter(
                |package| {
                    package
                        .subcategory
                        .starts_with(prefix)
                },
            )
            .collect()
    }
}

/// Return whether one stable unit identifier is canonical ASCII kebab-case.
fn is_canonical_identifier(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
        && bytes
            .last()
            .is_some_and(u8::is_ascii_alphanumeric)
        && !bytes
            .windows(2)
            .any(|pair| pair == b"--")
        && bytes
            .iter()
            .copied()
            .all(
                |byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || byte == b'-'
                },
            )
}

/// Rejects identifiers that cannot name a package member or derived record.
fn reject_empty_ids(
    field: &str,
    ids: &[String],
) -> Result<(), PackageIntakeError> {
    if let Some(id) = ids
        .iter()
        .find(|id| !is_canonical_identifier(id))
    {
        return Err(
            PackageIntakeError::new(
                format!(
                    "field {field} contains a noncanonical identifier: {id}"
                ),
            ),
        );
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
            return Err(
                PackageIntakeError::new(
                    format!("field {field} duplicates identifier {id}"),
                ),
            );
        }
    }
    Ok(())
}

/// Reads one required nonnegative integer with canonical delimiters.
fn extract_usize_field(
    row: &str,
    field: &str,
) -> Result<usize, PackageIntakeError> {
    let start = value_cursor(
        row, field,
    )?;
    let bytes = row.as_bytes();
    let mut end = start;
    while bytes
        .get(end)
        .is_some_and(u8::is_ascii_digit)
    {
        end = end.saturating_add(1);
    }
    if end == start {
        return Err(
            PackageIntakeError::new(
                format!("field {field} is not a nonnegative integer"),
            ),
        );
    }
    if end.saturating_sub(start) > 1 && bytes.get(start) == Some(&b'0') {
        return Err(
            PackageIntakeError::new(
                format!("field {field} has a leading zero"),
            ),
        );
    }
    let delimiter = skip_json_ws(
        row, end,
    );
    if !matches!(
        bytes.get(delimiter),
        Some(b',' | b'}')
    ) {
        return Err(
            PackageIntakeError::new(
                format!("field {field} has malformed integer syntax"),
            ),
        );
    }
    row.get(start..end)
        .ok_or_else(|| PackageIntakeError::new("invalid integer field range"))?
        .parse::<usize>()
        .map_err(
            |error| {
                PackageIntakeError::new(
                    format!("field {field} integer overflow: {error}"),
                )
            },
        )
}

/// Require one decoded field value to end at an object delimiter.
fn validate_field_value_end(
    row: &str,
    end: usize,
    field: &str,
) -> Result<(), PackageIntakeError> {
    let delimiter = skip_json_ws(
        row, end,
    );
    if matches!(
        row.as_bytes()
            .get(delimiter),
        Some(b',' | b'}')
    ) {
        return Ok(());
    }
    Err(
        PackageIntakeError::new(
            format!("field {field} has trailing JSON content"),
        ),
    )
}

/// Reads one required JSON string without accepting alternate field shapes.
fn extract_string_field(
    row: &str,
    field: &str,
) -> Result<String, PackageIntakeError> {
    let cursor = value_cursor(
        row, field,
    )?;
    let bytes = row.as_bytes();
    if bytes.get(cursor) != Some(&b'"') {
        return Err(
            PackageIntakeError::new(format!("field {field} is not a string")),
        );
    }
    let (value, end) = parse_json_string_at(
        row, cursor,
    )?;
    validate_field_value_end(
        row, end, field,
    )?;
    Ok(value)
}

/// Reads one required JSON string array with fail-closed delimiter checks.
fn extract_string_array(
    row: &str,
    field: &str,
) -> Result<Vec<String>, PackageIntakeError> {
    let mut cursor = value_cursor(
        row, field,
    )?;
    let bytes = row.as_bytes();
    if bytes.get(cursor) != Some(&b'[') {
        return Err(
            PackageIntakeError::new(
                format!("field {field} is not a string array"),
            ),
        );
    }
    cursor = cursor.saturating_add(1);
    let mut values = Vec::new();
    loop {
        cursor = skip_json_ws(
            row, cursor,
        );
        match bytes.get(cursor) {
            Some(b']') => {
                validate_field_value_end(
                    row,
                    cursor.saturating_add(1),
                    field,
                )?;
                return Ok(values);
            }
            Some(b'"') => {
                let (value, next_cursor) = parse_json_string_at(
                    row, cursor,
                )?;
                values.push(value);
                cursor = skip_json_ws(
                    row,
                    next_cursor,
                );
                match bytes.get(cursor) {
                    Some(b',') => {
                        cursor = skip_json_ws(
                            row,
                            cursor.saturating_add(1),
                        );
                        if bytes.get(cursor) == Some(&b']') {
                            return Err(
                                PackageIntakeError::new(
                                    format!(
                                        "field {field} has a trailing array \
                                         comma"
                                    ),
                                ),
                            );
                        }
                    }
                    Some(b']') => {
                        validate_field_value_end(
                            row,
                            cursor.saturating_add(1),
                            field,
                        )?;
                        return Ok(values);
                    }
                    _ => {
                        return Err(
                            PackageIntakeError::new(
                                format!(
                                    "field {field} has malformed string array"
                                ),
                            ),
                        );
                    }
                }
            }
            _ => {
                return Err(
                    PackageIntakeError::new(
                        format!("field {field} has malformed string array"),
                    ),
                );
            }
        }
    }
}

/// Number of fields in one canonical package-index row.
const CANONICAL_PACKAGE_FIELD_COUNT: usize = 30;

/// Return one field's canonical package-index position.
fn canonical_package_field_position(field: &str) -> Option<usize> {
    match field {
        "package_id" => Some(0),
        "package_root" => Some(1),
        "package_category" => Some(2),
        "package_subcategory" => Some(3),
        "unit_count" => Some(4),
        "text_key_count" => Some(5),
        "unit_ids" => Some(6),
        "source_unit_ids" => Some(26),
        "text_key_ids" => Some(27),
        "members" => Some(28),
        "text_keys" => Some(29),
        _ => PackageRole::all()
            .into_iter()
            .position(|role| role.id_field() == field)
            .map(|position| position.saturating_add(7)),
    }
}

/// Return whether one field belongs to the canonical package-index schema.
fn is_known_package_field(field: &str) -> bool {
    canonical_package_field_position(field).is_some()
}

/// Locates one unique top-level field while validating the complete object.
// One scanner owns field order, uniqueness, framing, and delimiters.
#[expect(
    clippy::too_many_lines,
    reason = "The strict scanner validates one canonical JSON object pass."
)]
fn value_cursor(
    row: &str,
    field: &str,
) -> Result<usize, PackageIntakeError> {
    let bytes = row.as_bytes();
    let mut cursor = skip_json_ws(
        row, 0,
    );
    if bytes.get(cursor) != Some(&b'{') {
        return Err(
            PackageIntakeError::new("package row is not a JSON object"),
        );
    }
    cursor = cursor.saturating_add(1);
    let mut seen = std::collections::BTreeSet::new();
    let mut expected_position = 0usize;
    let mut found = None;
    loop {
        cursor = skip_json_ws(
            row, cursor,
        );
        if bytes.get(cursor) == Some(&b'}') {
            let end = skip_json_ws(
                row,
                cursor.saturating_add(1),
            );
            if end != row.len() {
                return Err(
                    PackageIntakeError::new(
                        "package row has trailing JSON content",
                    ),
                );
            }
            if expected_position != CANONICAL_PACKAGE_FIELD_COUNT {
                return Err(
                    PackageIntakeError::new(
                        "package row has an incomplete canonical field set",
                    ),
                );
            }
            return found.ok_or_else(
                || PackageIntakeError::new(format!("missing field: {field}")),
            );
        }
        if bytes.get(cursor) != Some(&b'"') {
            return Err(
                PackageIntakeError::new(
                    "package row has a malformed top-level key",
                ),
            );
        }
        let (key, next) = parse_json_string_at(
            row, cursor,
        )?;
        if !seen.insert(key.clone()) {
            return Err(
                PackageIntakeError::new(
                    format!("package row duplicates top-level field: {key}"),
                ),
            );
        }
        if !is_known_package_field(&key) {
            return Err(
                PackageIntakeError::new(
                    format!(
                        "package row contains unknown top-level field: {key}"
                    ),
                ),
            );
        }
        let position = canonical_package_field_position(&key).ok_or_else(
            || PackageIntakeError::new("package field position is unavailable"),
        )?;
        if position != expected_position {
            return Err(
                PackageIntakeError::new(
                    format!(
                        "package row field {key} is out of canonical order"
                    ),
                ),
            );
        }
        expected_position = expected_position.saturating_add(1);
        cursor = skip_json_ws(
            row, next,
        );
        if bytes.get(cursor) != Some(&b':') {
            return Err(
                PackageIntakeError::new("package row key is missing a colon"),
            );
        }
        let value_start = skip_json_ws(
            row,
            cursor.saturating_add(1),
        );
        if key == field {
            found = Some(value_start);
        }
        cursor = skip_top_level_value(
            row,
            value_start,
        )?;
        cursor = skip_json_ws(
            row, cursor,
        );
        match bytes.get(cursor) {
            Some(b',') => {
                cursor = skip_json_ws(
                    row,
                    cursor.saturating_add(1),
                );
                if bytes.get(cursor) == Some(&b'}') {
                    return Err(
                        PackageIntakeError::new(
                            "package row has a trailing object comma",
                        ),
                    );
                }
            }
            Some(b'}') => {}
            _ => {
                return Err(
                    PackageIntakeError::new(
                        "package row has a malformed top-level delimiter",
                    ),
                );
            }
        }
    }
}

/// Maximum accepted JSON container depth for generated package rows.
const MAX_JSON_NESTING: usize = 128;

/// Skip one complete top-level JSON value with strict nested grammar.
fn skip_top_level_value(
    row: &str,
    start: usize,
) -> Result<usize, PackageIntakeError> {
    skip_json_value(
        row, start, 0,
    )
}

/// Skip one complete JSON value and return the first byte after it.
fn skip_json_value(
    row: &str,
    start: usize,
    depth: usize,
) -> Result<usize, PackageIntakeError> {
    if depth > MAX_JSON_NESTING {
        return Err(
            PackageIntakeError::new(
                "package row exceeds the JSON nesting limit",
            ),
        );
    }
    let cursor = skip_json_ws(
        row, start,
    );
    match row
        .as_bytes()
        .get(cursor)
    {
        Some(b'"') => parse_json_string_at(
            row, cursor,
        )
        .map(|(_, end)| end),
        Some(b'{') => skip_json_object(
            row,
            cursor,
            depth.saturating_add(1),
        ),
        Some(b'[') => skip_json_array(
            row,
            cursor,
            depth.saturating_add(1),
        ),
        Some(b't') => skip_json_literal(
            row, cursor, b"true",
        ),
        Some(b'f') => skip_json_literal(
            row, cursor, b"false",
        ),
        Some(b'n') => skip_json_literal(
            row, cursor, b"null",
        ),
        Some(b'-' | b'0'..=b'9') => skip_json_number(
            row, cursor,
        ),
        _ => Err(
            PackageIntakeError::new(
                "package row contains an invalid JSON value",
            ),
        ),
    }
}

/// Skip one strict JSON object without accepting trailing commas.
fn skip_json_object(
    row: &str,
    start: usize,
    depth: usize,
) -> Result<usize, PackageIntakeError> {
    let bytes = row.as_bytes();
    let mut cursor = skip_json_ws(
        row,
        start.saturating_add(1),
    );
    if bytes.get(cursor) == Some(&b'}') {
        return Ok(cursor.saturating_add(1));
    }
    loop {
        if bytes.get(cursor) != Some(&b'"') {
            return Err(
                PackageIntakeError::new(
                    "package row has a malformed nested object key",
                ),
            );
        }
        let (_, next) = parse_json_string_at(
            row, cursor,
        )?;
        cursor = skip_json_ws(
            row, next,
        );
        if bytes.get(cursor) != Some(&b':') {
            return Err(
                PackageIntakeError::new(
                    "package row nested object key is missing a colon",
                ),
            );
        }
        cursor = skip_json_value(
            row,
            skip_json_ws(
                row,
                cursor.saturating_add(1),
            ),
            depth,
        )?;
        cursor = skip_json_ws(
            row, cursor,
        );
        match bytes.get(cursor) {
            Some(b',') => {
                cursor = skip_json_ws(
                    row,
                    cursor.saturating_add(1),
                );
                if bytes.get(cursor) == Some(&b'}') {
                    return Err(
                        PackageIntakeError::new(
                            "package row has a trailing nested object comma",
                        ),
                    );
                }
            }
            Some(b'}') => return Ok(cursor.saturating_add(1)),
            _ => {
                return Err(
                    PackageIntakeError::new(
                        "package row has a malformed nested object delimiter",
                    ),
                );
            }
        }
    }
}

/// Skip one strict JSON array without accepting trailing commas.
fn skip_json_array(
    row: &str,
    start: usize,
    depth: usize,
) -> Result<usize, PackageIntakeError> {
    let bytes = row.as_bytes();
    let mut cursor = skip_json_ws(
        row,
        start.saturating_add(1),
    );
    if bytes.get(cursor) == Some(&b']') {
        return Ok(cursor.saturating_add(1));
    }
    loop {
        cursor = skip_json_value(
            row, cursor, depth,
        )?;
        cursor = skip_json_ws(
            row, cursor,
        );
        match bytes.get(cursor) {
            Some(b',') => {
                cursor = skip_json_ws(
                    row,
                    cursor.saturating_add(1),
                );
                if bytes.get(cursor) == Some(&b']') {
                    return Err(
                        PackageIntakeError::new(
                            "package row has a trailing nested array comma",
                        ),
                    );
                }
            }
            Some(b']') => return Ok(cursor.saturating_add(1)),
            _ => {
                return Err(
                    PackageIntakeError::new(
                        "package row has a malformed nested array delimiter",
                    ),
                );
            }
        }
    }
}

/// Skip one exact JSON literal.
fn skip_json_literal(
    row: &str,
    start: usize,
    literal: &[u8],
) -> Result<usize, PackageIntakeError> {
    let end = start.saturating_add(literal.len());
    if row
        .as_bytes()
        .get(start..end)
        != Some(literal)
    {
        return Err(
            PackageIntakeError::new(
                "package row contains a malformed JSON literal",
            ),
        );
    }
    Ok(end)
}

/// Skip one JSON number with canonical integer, fraction, and exponent grammar.
fn skip_json_number(
    row: &str,
    start: usize,
) -> Result<usize, PackageIntakeError> {
    let bytes = row.as_bytes();
    let mut cursor = start;
    if bytes.get(cursor) == Some(&b'-') {
        cursor = cursor.saturating_add(1);
    }
    match bytes.get(cursor) {
        Some(b'0') => {
            cursor = cursor.saturating_add(1);
            if bytes
                .get(cursor)
                .is_some_and(u8::is_ascii_digit)
            {
                return Err(
                    PackageIntakeError::new(
                        "package row JSON number has a leading zero",
                    ),
                );
            }
        }
        Some(b'1'..=b'9') => {
            cursor = cursor.saturating_add(1);
            while bytes
                .get(cursor)
                .is_some_and(u8::is_ascii_digit)
            {
                cursor = cursor.saturating_add(1);
            }
        }
        _ => {
            return Err(
                PackageIntakeError::new(
                    "package row contains a malformed JSON number",
                ),
            );
        }
    }
    if bytes.get(cursor) == Some(&b'.') {
        cursor = cursor.saturating_add(1);
        let fraction_start = cursor;
        while bytes
            .get(cursor)
            .is_some_and(u8::is_ascii_digit)
        {
            cursor = cursor.saturating_add(1);
        }
        if cursor == fraction_start {
            return Err(
                PackageIntakeError::new(
                    "package row JSON fraction has no digits",
                ),
            );
        }
    }
    if matches!(
        bytes.get(cursor),
        Some(b'e' | b'E')
    ) {
        cursor = cursor.saturating_add(1);
        if matches!(
            bytes.get(cursor),
            Some(b'+' | b'-')
        ) {
            cursor = cursor.saturating_add(1);
        }
        let exponent_start = cursor;
        while bytes
            .get(cursor)
            .is_some_and(u8::is_ascii_digit)
        {
            cursor = cursor.saturating_add(1);
        }
        if cursor == exponent_start {
            return Err(
                PackageIntakeError::new(
                    "package row JSON exponent has no digits",
                ),
            );
        }
    }
    Ok(cursor)
}

/// Advances across JSON whitespace so structural parsers share one rule.
fn skip_json_ws(
    row: &str,
    mut cursor: usize,
) -> usize {
    while matches!(
        row.as_bytes()
            .get(cursor),
        Some(b' ' | b'\n' | b'\r' | b'\t')
    ) {
        cursor = cursor.saturating_add(1);
    }
    cursor
}

/// Decode one JSON Unicode escape, including a required surrogate pair.
fn parse_json_unicode_escape(
    row: &str,
    escape_cursor: usize,
) -> Result<
    (
        char,
        usize,
    ),
    PackageIntakeError,
> {
    let first_start = escape_cursor.saturating_add(1);
    let first = parse_json_hex_quad(
        row,
        first_start,
    )?;
    let first_end = first_start.saturating_add(4);
    if (0xd800..=0xdbff).contains(&first) {
        let bytes = row.as_bytes();
        if bytes.get(first_end) != Some(&b'\\')
            || bytes.get(first_end.saturating_add(1)) != Some(&b'u')
        {
            return Err(
                PackageIntakeError::new(
                    "high JSON surrogate is missing a low surrogate",
                ),
            );
        }
        let second_start = first_end.saturating_add(2);
        let second = parse_json_hex_quad(
            row,
            second_start,
        )?;
        if !(0xdc00..=0xdfff).contains(&second) {
            return Err(
                PackageIntakeError::new(
                    "high JSON surrogate is followed by an invalid low \
                     surrogate",
                ),
            );
        }
        let high = u32::from(first)
            .checked_sub(0xd800_u32)
            .ok_or_else(|| PackageIntakeError::new("invalid high surrogate"))?;
        let low = u32::from(second)
            .checked_sub(0xdc00_u32)
            .ok_or_else(|| PackageIntakeError::new("invalid low surrogate"))?;
        let shifted_high = high
            .checked_shl(10)
            .ok_or_else(
                || PackageIntakeError::new("surrogate shift overflow"),
            )?;
        let code_point = 0x1_0000_u32
            .checked_add(shifted_high)
            .and_then(|value| value.checked_add(low))
            .ok_or_else(|| PackageIntakeError::new("surrogate sum overflow"))?;
        let character = char::from_u32(code_point).ok_or_else(
            || PackageIntakeError::new("invalid JSON surrogate code point"),
        )?;
        return Ok(
            (
                character,
                second_start.saturating_add(4),
            ),
        );
    }
    if (0xdc00..=0xdfff).contains(&first) {
        return Err(
            PackageIntakeError::new(
                "low JSON surrogate has no leading high surrogate",
            ),
        );
    }
    let character = char::from_u32(u32::from(first)).ok_or_else(
        || PackageIntakeError::new("invalid JSON Unicode code point"),
    )?;
    Ok(
        (
            character, first_end,
        ),
    )
}

/// Parse exactly four hexadecimal digits from one JSON Unicode escape.
fn parse_json_hex_quad(
    row: &str,
    start: usize,
) -> Result<u16, PackageIntakeError> {
    let bytes = row.as_bytes();
    let mut value = 0u16;
    for offset in 0..4usize {
        let byte = bytes
            .get(start.saturating_add(offset))
            .copied()
            .ok_or_else(
                || PackageIntakeError::new("incomplete JSON Unicode escape"),
            )?;
        let nibble = match byte {
            b'0'..=b'9' => byte
                .checked_sub(b'0')
                .map(u16::from),
            b'a'..=b'f' => byte
                .checked_sub(b'a')
                .map(u16::from)
                .and_then(|nibble_base| nibble_base.checked_add(10_u16)),
            b'A'..=b'F' => byte
                .checked_sub(b'A')
                .map(u16::from)
                .and_then(|nibble_base| nibble_base.checked_add(10_u16)),
            _ => None,
        }
        .ok_or_else(
            || {
                PackageIntakeError::new(
                    "invalid hex digit in JSON Unicode escape",
                )
            },
        )?;
        value = value
            .checked_shl(4)
            .map(|shifted| shifted | nibble)
            .ok_or_else(
                || PackageIntakeError::new("Unicode escape overflow"),
            )?;
    }
    Ok(value)
}

/// Decodes one JSON string while rejecting malformed or incomplete escapes.
fn parse_json_string_at(
    row: &str,
    start: usize,
) -> Result<
    (
        String,
        usize,
    ),
    PackageIntakeError,
> {
    let bytes = row.as_bytes();
    if bytes.get(start) != Some(&b'"') {
        return Err(PackageIntakeError::new("expected JSON string"));
    }
    let mut cursor = start.saturating_add(1);
    let mut output = String::new();
    while let Some(byte) = bytes
        .get(cursor)
        .copied()
    {
        match byte {
            b'"' => {
                return Ok(
                    (
                        output,
                        cursor.saturating_add(1),
                    ),
                );
            }
            b'\\' => {
                cursor = cursor.saturating_add(1);
                let Some(escaped) = bytes
                    .get(cursor)
                    .copied()
                else {
                    return Err(
                        PackageIntakeError::new("unterminated JSON escape"),
                    );
                };
                match escaped {
                    b'"' => output.push('"'),
                    b'\\' => output.push('\\'),
                    b'/' => output.push('/'),
                    b'b' => output.push('\u{0008}'),
                    b'f' => output.push('\u{000C}'),
                    b'n' => output.push('\n'),
                    b'r' => output.push('\r'),
                    b't' => output.push('\t'),
                    b'u' => {
                        let (character, next_cursor) =
                            parse_json_unicode_escape(
                                row, cursor,
                            )?;
                        output.push(character);
                        cursor = next_cursor;
                        continue;
                    }
                    _ => {
                        return Err(
                            PackageIntakeError::new(
                                "unsupported JSON escape in package index",
                            ),
                        );
                    }
                }
            }
            control if control <= 0x1f => {
                return Err(
                    PackageIntakeError::new(
                        "unescaped control character in JSON string",
                    ),
                );
            }
            _ if byte.is_ascii() => output.push(char::from(byte)),
            _ => {
                let tail = row
                    .get(cursor..)
                    .ok_or_else(
                        || {
                            PackageIntakeError::new(
                                "invalid UTF-8 string cursor",
                            )
                        },
                    )?;
                let character = tail
                    .chars()
                    .next()
                    .ok_or_else(
                        || {
                            PackageIntakeError::new(
                                "invalid UTF-8 package string",
                            )
                        },
                    )?;
                output.push(character);
                cursor = cursor.saturating_add(character.len_utf8());
                continue;
            }
        }
        cursor = cursor.saturating_add(1);
    }
    Err(PackageIntakeError::new("unterminated JSON string"))
}

#[cfg(test)]
mod tests {
    use super::{
        MAX_JSON_NESTING, PackageRole, PhaseThreePackageIndex,
        PhaseThreePackageRow, parse_json_string_at,
    };

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
    const NONCANONICAL_MEMBERS_FIELD: &str = concat!(
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
    fn reads_one_package_row() -> Result<(), String> {
        let row = PhaseThreePackageRow::from_json_line(sample_row())
            .map_err(|error| error.to_string())?;
        if row.package_id != "pkg-car" {
            return Err("package id should match sample".to_owned());
        }
        if row.category != "cars" {
            return Err("category should match sample".to_owned());
        }
        if row.ids_for_role(PackageRole::Model) != ["model-a".to_owned()] {
            return Err("model bucket should expose model id".to_owned());
        }
        if !row.has_model_components() || row.has_error_ids() {
            return Err(
                "sample row should be model-like and error-free".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn decodes_unicode_json_escapes() -> Result<(), String> {
        // cspell:disable-next-line -- caf
        let input = r#""caf\u00e9 \uD83D\uDE80""#;
        let (value, cursor) = parse_json_string_at(
            input, 0,
        )
        .map_err(|error| error.to_string())?;
        // cspell:disable-next-line -- caf
        if value != "caf\u{00e9} \u{1f680}" || cursor != input.len() {
            return Err(
                format!("Unicode JSON escapes were not decoded: {value:?}"),
            );
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
            if parse_json_string_at(
                input, 0,
            )
            .is_ok()
            {
                return Err(
                    format!("invalid Unicode escape was accepted: {input}"),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn preserves_utf8_json_strings() -> Result<(), String> {
        let input = "\"café\"";
        let (value, cursor) = parse_json_string_at(
            input, 0,
        )
        .map_err(|error| error.to_string())?;
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
            let row_text = sample_row().replace(
                field,
                replacement,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!("string field accepted appended token: {field}"),
                );
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
            return Err(
                "unescaped control characters must be rejected".to_owned(),
            );
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
            let row_text = sample_row().replace(
                field,
                replacement,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!("string array accepted appended token: {field}"),
                );
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
            let row_text = sample_row().replace(
                needle,
                replacement,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(format!("empty {label} must be rejected"));
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_noncanonical_package_array_ids() -> Result<(), String> {
        for invalid in [
            "model a", "Model-a", "model/a", "model--a", "-model-a",
            // cspell:disable-next-line -- modél
            "model-a-", "modél-a",
        ] {
            let row_text = sample_row().replace(
                "model-a", invalid,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!("noncanonical identifier was accepted: {invalid}"),
                );
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
            let row_text = sample_row().replace(
                needle,
                replacement,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(format!("empty {label} must be rejected"));
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_duplicate_identifiers_within_package_arrays()
    -> Result<(), String> {
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
            let row_text = sample_row().replace(
                needle,
                replacement,
            );
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
            return Err(
                "role ids absent from unit_ids must be rejected".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_physical_members_without_roles() -> Result<(), String> {
        let row_text = sample_row().replace(
            "\"model_ids\":[\"model-a\"]",
            "\"model_ids\":[]",
        );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(
                "physical members absent from every role must be rejected"
                    .to_owned(),
            );
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
                "one physical member must not occupy multiple roles".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_packages_with_error_role_members() -> Result<(), String> {
        let row_text = sample_row()
            .replace(
                "\"model_ids\":[\"model-a\"]",
                "\"model_ids\":[]",
            )
            .replace(
                "\"error_ids\":[]",
                "\"error_ids\":[\"model-a\"]",
            );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err("error-role packages must fail intake".to_owned());
        }
        Ok(())
    }

    #[test]
    fn rejects_packages_without_physical_or_derived_members()
    -> Result<(), String> {
        let row_text = sample_row()
            .replace(
                "\"unit_ids\":[\"texture-a\",\"model-a\"]",
                "\"unit_ids\":[]",
            )
            .replace(
                SAMPLE_MEMBERS_FIELD,
                EMPTY_MEMBERS_FIELD,
            )
            .replace(
                "\"model_ids\":[\"model-a\"]",
                "\"model_ids\":[]",
            )
            .replace(
                "\"texture_ids\":[\"texture-a\"]",
                "\"texture_ids\":[]",
            );
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
            .replace(
                SAMPLE_MEMBERS_FIELD,
                EMPTY_MEMBERS_FIELD,
            )
            .replace(
                "\"model_ids\":[\"model-a\"]",
                "\"model_ids\":[]",
            )
            .replace(
                "\"texture_ids\":[\"texture-a\"]",
                "\"texture_ids\":[]",
            )
            .replace(
                "\"text_key_count\":0",
                "\"text_key_count\":1",
            )
            .replace(
                "\"text_key_ids\":[]",
                "\"text_key_ids\":[\"text-a\"]",
            )
            .replace(
                "\"text_keys\":[]",
                &text_keys_field(
                    "text-a",
                    "model-a",
                    "cars/character-rigs/homer-v",
                ),
            );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(
                "derived text keys without source units must be rejected"
                    .to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_blank_jsonl_records() -> Result<(), String> {
        for contents in [
            format!(
                "\n{}\n",
                sample_row()
            ),
            format!(
                "{}\n \t\n",
                sample_row()
            ),
        ] {
            if PhaseThreePackageIndex::from_jsonl(&contents).is_ok() {
                return Err(
                    "blank package-index records must be rejected".to_owned(),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_empty_package_indexes() -> Result<(), String> {
        let whitespace_only = [
            "", " ", "",
        ]
        .join(
            "
",
        );
        if PhaseThreePackageIndex::from_jsonl(&whitespace_only).is_ok() {
            return Err("empty package indexes must be rejected".to_owned());
        }
        Ok(())
    }

    #[test]
    fn rejects_noncanonical_member_order() -> Result<(), String> {
        let row_text = sample_row()
            .replace(
                "\"unit_ids\":[\"texture-a\",\"model-a\"]",
                "\"unit_ids\":[\"model-a\",\"texture-a\"]",
            )
            .replace(
                SAMPLE_MEMBERS_FIELD,
                NONCANONICAL_MEMBERS_FIELD,
            );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err("noncanonical member order was accepted".to_owned());
        }
        Ok(())
    }

    #[test]
    fn rejects_noncanonical_package_order() -> Result<(), String> {
        let row_z = sample_row()
            .replace(
                "pkg-car", "pkg-z",
            )
            .replace(
                "model-a", "model-z",
            )
            .replace(
                "texture-a",
                "texture-z",
            );
        let row_a = sample_row()
            .replace(
                "pkg-car", "pkg-a",
            )
            .replace(
                "model-a", "model-a2",
            )
            .replace(
                "texture-a",
                "texture-a2",
            );
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
    fn rejects_physical_ids_claimed_by_multiple_packages() -> Result<(), String>
    {
        let row_a = sample_row().replace(
            "pkg-car", "pkg-a",
        );
        let row_z = sample_row().replace(
            "pkg-car", "pkg-z",
        );
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
            .replace(
                "pkg-car",
                // cspell:disable-next-line -- aaa
                "aaa-derived",
            )
            .replace(
                "\"package_category\":\"cars\"",
                "\"package_category\":\"language\"",
            )
            .replace(
                "cars/character-rigs/homer-v",
                "language/derived/text",
            )
            .replace(
                "\"unit_count\":2",
                "\"unit_count\":0",
            )
            .replace(
                "\"text_key_count\":0",
                "\"text_key_count\":1",
            )
            .replace(
                "\"unit_ids\":[\"texture-a\",\"model-a\"]",
                "\"unit_ids\":[]",
            )
            .replace(
                SAMPLE_MEMBERS_FIELD,
                EMPTY_MEMBERS_FIELD,
            )
            .replace(
                "\"texture_ids\":[\"texture-a\"]",
                "\"texture_ids\":[]",
            )
            .replace(
                "\"model_ids\":[\"model-a\"]",
                "\"model_ids\":[]",
            )
            .replace(
                "\"source_unit_ids\":[]",
                "\"source_unit_ids\":[\"model-a\"]",
            )
            .replace(
                "\"text_key_ids\":[]",
                "\"text_key_ids\":[\"model-a\"]",
            )
            .replace(
                "\"text_keys\":[]",
                &text_keys_field(
                    "model-a",
                    "model-a",
                    "language/derived/text",
                ),
            );
        let contents = format!(
            "{derived}\n{}",
            sample_row()
        );
        if PhaseThreePackageIndex::from_jsonl(&contents).is_ok() {
            return Err(
                "one id must not be both physical and derived".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_derived_ids_claimed_by_multiple_packages() -> Result<(), String>
    {
        let derived = |package_id: &str| {
            sample_row()
                .replace(
                    "pkg-car", package_id,
                )
                .replace(
                    "\"unit_count\":2",
                    "\"unit_count\":0",
                )
                .replace(
                    "\"text_key_count\":0",
                    "\"text_key_count\":1",
                )
                .replace(
                    "\"unit_ids\":[\"texture-a\",\"model-a\"]",
                    "\"unit_ids\":[]",
                )
                .replace(
                    SAMPLE_MEMBERS_FIELD,
                    EMPTY_MEMBERS_FIELD,
                )
                .replace(
                    "\"model_ids\":[\"model-a\"]",
                    "\"model_ids\":[]",
                )
                .replace(
                    "\"texture_ids\":[\"texture-a\"]",
                    "\"texture_ids\":[]",
                )
                .replace(
                    "\"text_key_ids\":[]",
                    "\"text_key_ids\":[\"text-a\"]",
                )
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
        for invalid in [
            "-pkg-car", "pkg-car-", "pkg--car",
        ] {
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
                return Err(
                    format!(
                        "noncanonical package slug was accepted: {invalid}"
                    ),
                );
            }
        }
        for invalid in [
            "-character-rigs",
            "character-rigs-",
            "character--rigs",
        ] {
            let row_text = sample_row().replace(
                "cars/character-rigs/homer-v",
                &format!("cars/{invalid}/homer-v"),
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!(
                        "noncanonical subcategory slug was accepted: {invalid}"
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_noncanonical_package_id_characters() -> Result<(), String> {
        for invalid in [
            "Pkg-Car",
            "pkg_car",
            "pkg/car",
            "pkg-café",
        ] {
            let row_text = sample_row().replace(
                "pkg-car", invalid,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!(
                        "noncanonical package id must be rejected: {invalid}"
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_unknown_or_error_package_categories() -> Result<(), String> {
        for invalid in [
            "unknown", "error",
        ] {
            let row_text = sample_row().replace(
                "\"package_category\":\"cars\"",
                &format!("\"package_category\":\"{invalid}\""),
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!(
                        "non-success package category must be rejected: \
                         {invalid}"
                    ),
                );
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
                return Err(
                    format!(
                        "invalid package root was accepted: {invalid_root:?}"
                    ),
                );
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
            let row_text = sample_row().replace(
                "cars/character-rigs/homer-v",
                invalid,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!(
                        "noncanonical subcategory must be rejected: {invalid}"
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_placeholder_subcategory_segments() -> Result<(), String> {
        for placeholder in [
            "unknown", "generic", "misc", "context", "shared", "global",
        ] {
            let row_text = sample_row().replace(
                "cars/character-rigs/homer-v",
                &format!("cars/{placeholder}"),
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!(
                        "placeholder subcategory must be rejected: \
                         {placeholder}"
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_missing_or_mismatched_package_roots() -> Result<(), String> {
        let missing = sample_row().replace(
            "\"package_root\":\"pkg-car\",",
            "",
        );
        let empty = sample_row().replace(
            "\"package_root\":\"pkg-car\"",
            "\"package_root\":\"\"",
        );
        let mismatched = sample_row().replace(
            "\"package_root\":\"pkg-car\"",
            "\"package_root\":\"different-root\"",
        );
        for (label, row_text) in [
            (
                "missing", missing,
            ),
            (
                "empty", empty,
            ),
            (
                "mismatched",
                mismatched,
            ),
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
            (
                "\"unit_count\":2",
                "\"unit_count\":02",
            ),
            (
                "\"text_key_count\":0",
                "\"text_key_count\":00",
            ),
        ] {
            let row_text = sample_row().replace(
                field,
                replacement,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!("leading-zero count was accepted: {field}"),
                );
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
            let row_text = sample_row().replace(
                needle,
                replacement,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(format!("{label} must be rejected"));
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_dangling_derived_source_units() -> Result<(), String> {
        let derived = sample_row()
            .replace(
                "pkg-car",
                "pkg-derived",
            )
            .replace(
                "\"unit_ids\":[\"texture-a\",\"model-a\"]",
                "\"unit_ids\":[]",
            )
            .replace(
                SAMPLE_MEMBERS_FIELD,
                EMPTY_MEMBERS_FIELD,
            )
            .replace(
                "\"unit_count\":2",
                "\"unit_count\":0",
            )
            .replace(
                "\"model_ids\":[\"model-a\"]",
                "\"model_ids\":[]",
            )
            .replace(
                "\"texture_ids\":[\"texture-a\"]",
                "\"texture_ids\":[]",
            )
            .replace(
                "\"text_key_count\":0",
                "\"text_key_count\":1",
            )
            .replace(
                "\"text_key_ids\":[]",
                "\"text_key_ids\":[\"text-a\"]",
            )
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
        let row_text = sample_row().replace(
            SAMPLE_MEMBERS_FIELD,
            EMPTY_MEMBERS_FIELD,
        );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(
                "physical unit ids require member mirror records".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_empty_derived_text_key_mirrors() -> Result<(), String> {
        let row_text = sample_row()
            .replace(
                "\"text_key_count\":0",
                "\"text_key_count\":1",
            )
            .replace(
                "\"source_unit_ids\":[]",
                "\"source_unit_ids\":[\"model-a\"]",
            )
            .replace(
                "\"text_key_ids\":[]",
                "\"text_key_ids\":[\"text-a\"]",
            );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(
                "derived text_key_ids require text_keys mirror records"
                    .to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_non_array_structured_mirrors() -> Result<(), String> {
        for (field, replacement) in [
            (
                SAMPLE_MEMBERS_FIELD,
                "\"members\":true",
            ),
            (
                "\"text_keys\":[]",
                "\"text_keys\":{}",
            ),
        ] {
            let row_text = sample_row().replace(
                field,
                replacement,
            );
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
            let row_text = sample_row().replace(
                &field, "",
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!("missing mirror field was accepted: {field}"),
                );
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
            sample_row().replace(
                &canonical_tail,
                &swapped_tail,
            ),
        ] {
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    "noncanonical package field order must fail".to_owned(),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_unknown_top_level_fields() -> Result<(), String> {
        let row_text = sample_row().replace(
            "\"text_keys\":[]}",
            "\"text_keys\":[],\"unexpected\":true}",
        );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(
                "unknown package-index fields must be rejected".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_invalid_required_text_key_values() -> Result<(), String> {
        for invalid_key in [
            "",
            " PADDED",
            r"HELLO\u0000",
        ] {
            let text_keys = text_keys_field(
                "text-a",
                "model-a",
                "cars/character-rigs/homer-v",
            )
            .replace(
                "\"key\":\"HELLO\"",
                &format!("\"key\":\"{invalid_key}\""),
            );
            let row_text = sample_row()
                .replace(
                    "\"text_key_count\":0",
                    "\"text_key_count\":1",
                )
                .replace(
                    "\"source_unit_ids\":[]",
                    "\"source_unit_ids\":[\"model-a\"]",
                )
                .replace(
                    "\"text_key_ids\":[]",
                    "\"text_key_ids\":[\"text-a\"]",
                )
                .replace(
                    "\"text_keys\":[]",
                    &text_keys,
                );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!(
                        "invalid text key must be rejected: {invalid_key:?}"
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn accepts_canonical_structured_text_key_json() -> Result<(), String> {
        let row_text = sample_row()
            .replace(
                "\"text_key_count\":0",
                "\"text_key_count\":1",
            )
            .replace(
                "\"source_unit_ids\":[]",
                "\"source_unit_ids\":[\"model-a\"]",
            )
            .replace(
                "\"text_key_ids\":[]",
                "\"text_key_ids\":[\"text-a\"]",
            )
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
            return Err(
                "canonical text-key mirror changed id intake".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_inconsistent_structured_text_key_mirrors() -> Result<(), String>
    {
        let canonical = concat!(
            "\"text_keys\":[{",
            "\"id\":\"text-a\",",
            "\"key\":\"HELLO\",",
            "\"source_unit_id\":\"model-a\",",
            "\"subcategory\":\"cars/character-rigs/homer-v\"}]",
        );
        for replacement in [
            canonical.replace(
                "\"id\":\"text-a\"",
                "\"id\":\"other\"",
            ),
            canonical.replace(
                "\"source_unit_id\":\"model-a\"",
                "\"source_unit_id\":\"missing\"",
            ),
            canonical.replace(
                "cars/character-rigs/homer-v",
                "language/other",
            ),
            "\"text_keys\":[{\"id\":\"text-a\"}]".to_owned(),
        ] {
            let row_text = sample_row()
                .replace(
                    "\"text_key_count\":0",
                    "\"text_key_count\":1",
                )
                .replace(
                    "\"source_unit_ids\":[]",
                    "\"source_unit_ids\":[\"model-a\"]",
                )
                .replace(
                    "\"text_key_ids\":[]",
                    "\"text_key_ids\":[\"text-a\"]",
                )
                .replace(
                    "\"text_keys\":[]",
                    &replacement,
                );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!(
                        "invalid text-key mirror was accepted: {replacement}"
                    ),
                );
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
        let unknown_role = full_member.replace(
            "\"role\":\"model\"",
            "\"role\":\"unknown\"",
        );
        for replacement in [
            full_member.to_owned(),
            unknown_role,
            "\"members\":[{\"id\":\"model-a\"}]".to_owned(),
        ] {
            let row_text = sample_row().replace(
                SAMPLE_MEMBERS_FIELD,
                &replacement,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!(
                        "invalid member mirror was accepted: {replacement}"
                    ),
                );
            }
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
        if row
            .unit_ids
            .len()
            != 2
        {
            return Err(
                "canonical structured mirrors changed unit intake".to_owned(),
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
        let row_text = sample_row().replace(
            SAMPLE_MEMBERS_FIELD,
            &nested,
        );
        if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
            return Err(
                "excessive structured-mirror nesting must fail".to_owned(),
            );
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
            let row_text = sample_row().replace(
                SAMPLE_MEMBERS_FIELD,
                replacement,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!(
                        "malformed mirror JSON was accepted: {replacement}"
                    ),
                );
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
            let row_text = sample_row().replace(
                field,
                replacement,
            );
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!("malformed mirror string was accepted: {field}"),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_outer_row_whitespace() -> Result<(), String> {
        for row_text in [
            format!(
                " {}",
                sample_row()
            ),
            format!(
                "{} ",
                sample_row()
            ),
            format!(
                "\t{}",
                sample_row()
            ),
        ] {
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    "package rows with outer whitespace must fail".to_owned(),
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
        let trailing_garbage = format!(
            "{}garbage",
            sample_row()
        );
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
        let trailing_comma = sample_row().replace(
            "\"text_keys\":[]}",
            "\"text_keys\":[],}",
        );
        for (label, row_text) in [
            (
                "unframed", unframed,
            ),
            (
                "trailing garbage",
                trailing_garbage,
            ),
            (
                "nested identity",
                nested_identity,
            ),
            (
                "duplicate key",
                duplicate_category,
            ),
            (
                "trailing comma",
                trailing_comma,
            ),
        ] {
            if PhaseThreePackageRow::from_json_line(&row_text).is_ok() {
                return Err(
                    format!(
                        "noncanonical top-level JSON must be rejected: {label}"
                    ),
                );
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
            let row = sample_row().replacen(
                "extracted/model.p3d",
                invalid_path,
                1,
            );
            if PhaseThreePackageRow::from_json_line(&row).is_ok() {
                return Err(
                    format!(
                        "Windows-incompatible member path was accepted: \
                         {invalid_path}"
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_invalid_required_member_classification_fields()
    -> Result<(), String> {
        for (field, canonical_value) in [
            (
                "type", "model",
            ),
            (
                "kind", "mesh",
            ),
            (
                "source_chunk_kind",
                "mesh",
            ),
        ] {
            for invalid_value in [
                "",
                " padded",
                "control\u{0}",
            ] {
                let canonical = format!("\"{field}\":\"{canonical_value}\"");
                let invalid = format!("\"{field}\":\"{invalid_value}\"");
                let row = sample_row().replacen(
                    &canonical, &invalid, 1,
                );
                if PhaseThreePackageRow::from_json_line(&row).is_ok() {
                    let detail = format!("{field}={invalid_value:?}");
                    return Err(
                        format!(
                            "invalid member field must be rejected: {detail}"
                        ),
                    );
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
        if package
            .member_refs()
            .len()
            != 2
        {
            return Err(
                "sample package should expose two role refs".to_owned(),
            );
        }
        if index
            .packages_by_category("cars")
            .len()
            != 1
        {
            return Err(
                "category lookup should find sample package".to_owned(),
            );
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
}
