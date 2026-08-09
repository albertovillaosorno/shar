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
//   - Lossless syntax typing for one Dyna Load Data string.
// - Must-Not:
//   - Infer package lifetime, runtime ordering, or mission-stage precedence.
// - Allows:
//   - Distinguish region, interior, and World Sphere postfix operations.
// - Split-When:
//   - Runtime package lifetime gains an independent transition model.
// - Merge-When:
//   - Dyna Load Data syntax is owned by a broader script grammar.
// - Summary:
//   - Typed Dyna Load Data operation parser.
// - Description:
//   - Preserves authored targets while assigning only documented postfix syntax.
// - Usage:
//   - Shared by mission initialization and DynamicZone locator preflight.
// - Defaults:
//   - Blank, unterminated, unsafe P3D, or empty operations fail closed.
//

//! Typed syntax for Dyna Load Data strings.

/// One documented Dyna Load Data postfix operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynaLoadOperationKind {
    /// Load one region P3D.
    RegionLoad,
    /// Unload one region P3D.
    RegionUnload,
    /// Load one interior P3D.
    InteriorLoad,
    /// Unload one interior P3D.
    InteriorUnload,
    /// Enable one World Sphere chunk.
    WorldSphereEnable,
    /// Disable one World Sphere chunk.
    WorldSphereDisable,
}

impl DynaLoadOperationKind {
    const fn from_symbol(symbol: char) -> Option<Self> {
        match symbol {
            ';' => Some(Self::RegionLoad),
            ':' => Some(Self::RegionUnload),
            '@' => Some(Self::InteriorLoad),
            '$' => Some(Self::InteriorUnload),
            '*' => Some(Self::WorldSphereEnable),
            '&' => Some(Self::WorldSphereDisable),
            _ => None,
        }
    }

    /// Return true when this operation loads a P3D package.
    #[must_use]
    pub const fn is_p3d_load(self) -> bool {
        matches!(self, Self::RegionLoad | Self::InteriorLoad)
    }

    /// Return true when this operation unloads a P3D package.
    #[must_use]
    pub const fn is_p3d_unload(self) -> bool {
        matches!(self, Self::RegionUnload | Self::InteriorUnload)
    }

    const fn targets_p3d(self) -> bool {
        self.is_p3d_load() || self.is_p3d_unload()
    }
}

/// One target plus its exact documented postfix operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynaLoadOperation {
    target: String,
    kind: DynaLoadOperationKind,
}

impl DynaLoadOperation {
    /// Return the exact authored target preceding the postfix operator.
    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Return the typed postfix operation.
    #[must_use]
    pub const fn kind(&self) -> DynaLoadOperationKind {
        self.kind
    }
}

/// Lossless typed projection of one Dyna Load Data string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynaLoadData {
    source: String,
    operations: Vec<DynaLoadOperation>,
}

impl DynaLoadData {
    /// Return the exact authored Dyna Load Data string.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Return operations in authored order.
    #[must_use]
    pub fn operations(&self) -> &[DynaLoadOperation] {
        &self.operations
    }
}

/// Parse documented postfix operations without assigning runtime lifetime.
///
/// # Errors
///
/// Rejects blank/control-bearing data, missing postfix operators, empty targets,
/// and unsafe/non-P3D targets for region or interior operations.
pub fn parse_dyna_load_data(source: &str) -> Result<DynaLoadData, String> {
    if source.is_empty() || source != source.trim() || source.chars().any(char::is_control) {
        return Err("Dyna Load Data source is blank, padded, or control-bearing".to_owned());
    }

    let mut start = 0usize;
    let mut operations = Vec::new();
    for (offset, symbol) in source.char_indices() {
        let Some(kind) = DynaLoadOperationKind::from_symbol(symbol) else {
            continue;
        };
        let target = source
            .get(start..offset)
            .ok_or_else(|| "Dyna Load Data target boundary is invalid".to_owned())?;
        validate_target(target, kind)?;
        operations.push(DynaLoadOperation {
            target: target.to_owned(),
            kind,
        });
        start = offset
            .checked_add(symbol.len_utf8())
            .ok_or_else(|| "Dyna Load Data offset overflowed".to_owned())?;
    }
    if operations.is_empty() || start != source.len() {
        return Err("Dyna Load Data operation is missing a postfix symbol".to_owned());
    }
    Ok(DynaLoadData {
        source: source.to_owned(),
        operations,
    })
}

fn validate_target(target: &str, kind: DynaLoadOperationKind) -> Result<(), String> {
    if target.is_empty() || target != target.trim() || target.chars().any(char::is_control) {
        return Err("Dyna Load Data target is blank, padded, or control-bearing".to_owned());
    }
    if kind.targets_p3d() {
        validate_p3d_target(target)?;
    }
    Ok(())
}

fn validate_p3d_target(target: &str) -> Result<(), String> {
    let normalized = target.replace(char::from(92), "/");
    if !normalized.to_ascii_lowercase().ends_with(".p3d")
        || normalized.starts_with('/')
        || normalized.contains(':')
        || normalized.split('/').any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err("Dyna Load Data P3D target is malformed".to_owned());
    }
    Ok(())
}

#[cfg(test)]
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/dyna_load_data/tests.rs"]
mod tests;
