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
//   - Level-scoped binding of typed mission camera and multicontroller names.
// - Must-Not:
//   - Infer cross-level precedence, camera timing, blending, or playback
//     policy.
// - Allows:
//   - Resolve exact decoded component names using source-script level
//     provenance.
// - Split-When:
//   - Camera runtime behavior requires independent lifecycle state.
// - Merge-When:
//   - Final mission-definition compilation owns this exact reference boundary.
// - Summary:
//   - Mission camera package-reference catalog and preflight.
// - Description:
//   - Preserves globally repeated camera names as distinct level-scoped source
//     identities and binds reviewed initialization directives fail closed.
// - Usage:
//   - Built from decoded mission camera packages before Unreal mission intake.
// - Defaults:
//   - Missing, duplicate, malformed, or cross-level references fail closed.
//

//! Level-scoped package binding for mission cameras and multicontrollers.

use std::collections::BTreeMap;

use super::{
    MissionInitializationDirective as InitDirective,
    MissionInitializationReport,
};

/// Physical component family referenced by one camera initialization command.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MissionCameraComponentKind {
    /// One decoded P3D camera component.
    Camera,
    /// One decoded P3D multi-controller component.
    MultiController,
}

/// One decoded camera-component identity available within a source level.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionCameraCatalogEntry {
    level: String,
    name: String,
    kind: MissionCameraComponentKind,
    member_id: String,
    package_id: String,
    package_root: String,
    source_path: String,
}

impl MissionCameraCatalogEntry {
    /// Build one validated level-scoped camera component entry.
    ///
    /// # Errors
    ///
    /// Returns an error when identity fields are malformed or the package root
    /// does not carry matching mission-level provenance.
    pub fn new(
        name: String,
        kind: MissionCameraComponentKind,
        member_id: String,
        package_id: String,
        package_root: String,
        source_path: String,
    ) -> Result<Self, String> {
        for (label, value) in [
            ("camera name", name.as_str()),
            ("member id", member_id.as_str()),
            ("package id", package_id.as_str()),
            ("package root", package_root.as_str()),
            ("source path", source_path.as_str()),
        ] {
            validate_identity(value, label)?;
        }
        let level = mission_level_from_package_root(&package_root)?;
        Ok(Self {
            level,
            name,
            kind,
            member_id,
            package_id,
            package_root,
            source_path,
        })
    }
}

/// Exact level-scoped camera component catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionCameraCatalog {
    by_key: BTreeMap<
        (String, MissionCameraComponentKind, String),
        Vec<MissionCameraCatalogEntry>,
    >,
}

impl MissionCameraCatalog {
    /// Build the catalog while preserving every level-scoped candidate.
    #[must_use]
    pub fn from_entries(entries: Vec<MissionCameraCatalogEntry>) -> Self {
        let mut by_key = BTreeMap::new();
        for entry in entries {
            let key = (entry.level.clone(), entry.kind, entry.name.clone());
            by_key.entry(key).or_insert_with(Vec::new).push(entry);
        }
        Self { by_key }
    }

    fn resolve(
        &self,
        level: &str,
        kind: MissionCameraComponentKind,
        name: &str,
    ) -> Result<&MissionCameraCatalogEntry, String> {
        let candidates = self
            .by_key
            .get(&(level.to_owned(), kind, name.to_owned()))
            .map_or(&[][..], Vec::as_slice);
        match candidates {
            [entry] => Ok(entry),
            [] => Err(concat!(
                "mission camera reference has no exact level-scoped ",
                "component"
            )
            .to_owned()),
            _ => Err(concat!(
                "mission camera reference has ambiguous level-scoped ",
                "components"
            )
            .to_owned()),
        }
    }
}

/// Semantic command role that owns one camera package reference.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MissionCameraReferenceRole {
    /// `SetAnimCamName` camera component.
    AnimatedCamera,
    /// `SetAnimCamMulticontName` multi-controller component.
    AnimatedCameraMulticont,
    /// `SetMissionStartCameraName` camera component.
    MissionStartCamera,
    /// `SetMissionStartMulticontName` multi-controller component.
    MissionStartMulticont,
}

/// One typed camera reference bound to exact package/member provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionCameraReferenceBinding {
    source_ordinal: usize,
    role: MissionCameraReferenceRole,
    source_name: String,
    member_id: String,
    package_id: String,
    package_root: String,
    source_path: String,
}

impl MissionCameraReferenceBinding {
    /// Return the source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the typed camera-reference role.
    #[must_use]
    pub const fn role(&self) -> MissionCameraReferenceRole {
        self.role
    }

    /// Return the exact authored component name.
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Return the canonical member id.
    #[must_use]
    pub fn member_id(&self) -> &str {
        &self.member_id
    }

    /// Return the canonical package id.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return the canonical package root.
    #[must_use]
    pub fn package_root(&self) -> &str {
        &self.package_root
    }

    /// Return the decoded component source path.
    #[must_use]
    pub fn source_path(&self) -> &str {
        &self.source_path
    }
}

/// Bound camera references for one normalized mission source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionCameraReferenceReport {
    bindings: Vec<MissionCameraReferenceBinding>,
}

impl MissionCameraReferenceReport {
    /// Return bindings in source ordinal and semantic-role order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionCameraReferenceBinding] {
        &self.bindings
    }
}

/// Bind typed initialization camera references by exact source level.
///
/// # Errors
///
/// Returns an error when source-level provenance is malformed or one camera or
/// multi-controller name is absent from that exact level.
pub fn preflight_mission_camera_references(
    source_path: &str,
    catalog: &MissionCameraCatalog,
    initialization: &MissionInitializationReport,
) -> Result<MissionCameraReferenceReport, String> {
    let level = mission_level_from_script_path(source_path)?;
    let mut bindings = Vec::new();
    for mission in initialization.missions() {
        for directive in mission.directives() {
            let reference = match directive {
                InitDirective::AnimatedCamera {
                    source_ordinal,
                    camera_id,
                } => Some((
                    *source_ordinal,
                    MissionCameraReferenceRole::AnimatedCamera,
                    MissionCameraComponentKind::Camera,
                    camera_id.as_str(),
                )),
                InitDirective::AnimatedCameraMulticont {
                    source_ordinal,
                    multicont_id,
                } => Some((
                    *source_ordinal,
                    MissionCameraReferenceRole::AnimatedCameraMulticont,
                    MissionCameraComponentKind::MultiController,
                    multicont_id.as_str(),
                )),
                InitDirective::MissionStartCamera {
                    source_ordinal,
                    camera_id,
                } => Some((
                    *source_ordinal,
                    MissionCameraReferenceRole::MissionStartCamera,
                    MissionCameraComponentKind::Camera,
                    camera_id.as_str(),
                )),
                InitDirective::MissionStartMulticont {
                    source_ordinal,
                    multicont_id,
                } => Some((
                    *source_ordinal,
                    MissionCameraReferenceRole::MissionStartMulticont,
                    MissionCameraComponentKind::MultiController,
                    multicont_id.as_str(),
                )),
                _ => None,
            };
            if let Some((ordinal, role, kind, name)) = reference {
                push_binding(
                    &mut bindings,
                    catalog,
                    &level,
                    ordinal,
                    role,
                    kind,
                    name,
                )?;
            }
        }
    }
    bindings.sort_by_key(|binding| (binding.source_ordinal, binding.role));
    Ok(MissionCameraReferenceReport { bindings })
}

fn push_binding(
    bindings: &mut Vec<MissionCameraReferenceBinding>,
    catalog: &MissionCameraCatalog,
    level: &str,
    source_ordinal: usize,
    role: MissionCameraReferenceRole,
    kind: MissionCameraComponentKind,
    source_name: &str,
) -> Result<(), String> {
    let entry = catalog.resolve(level, kind, source_name)?;
    bindings.push(MissionCameraReferenceBinding {
        source_ordinal,
        role,
        source_name: source_name.to_owned(),
        member_id: entry.member_id.clone(),
        package_id: entry.package_id.clone(),
        package_root: entry.package_root.clone(),
        source_path: entry.source_path.clone(),
    });
    Ok(())
}

fn mission_level_from_script_path(path: &str) -> Result<String, String> {
    mission_level_from_path(path, "extracted/game/scripts/missions/")
}

fn mission_level_from_package_root(root: &str) -> Result<String, String> {
    mission_level_from_path(root, "extracted/art/missions/")
}

fn mission_level_from_path(path: &str, prefix: &str) -> Result<String, String> {
    validate_identity(path, "mission camera provenance path")?;
    if path.contains(char::from(92)) || path.contains(':') {
        return Err(
            "mission camera provenance path is not portable".to_owned()
        );
    }
    let tail = path.strip_prefix(prefix).ok_or_else(|| {
        "mission camera provenance path is outside mission levels".to_owned()
    })?;
    let level = tail.split('/').next().unwrap_or_default();
    let Some(digits) = level.strip_prefix("level") else {
        return Err(
            "mission camera level provenance is malformed".to_owned()
        );
    };
    if digits.len() != 2
        || !digits.bytes().all(|value| value.is_ascii_digit())
    {
        return Err(
            "mission camera level provenance is malformed".to_owned()
        );
    }
    Ok(level.to_owned())
}

fn validate_identity(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty()
        || value != value.trim()
        || value.chars().any(char::is_control)
    {
        return Err(format!("{label} is malformed"));
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_camera_reference/tests.rs"]
mod tests;
