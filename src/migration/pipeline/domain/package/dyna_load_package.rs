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
//   - Deterministic package-set effects of typed Dyna Load Data operations.
// - Must-Not:
//   - Infer DynamicZone trigger order, mission progress, or locator precedence.
// - Allows:
//   - Normalize P3D targets and apply authored load/unload effects in order.
// - Split-When:
//   - Trigger history or package residency becomes an independent runtime
//     model.
// - Merge-When:
//   - Dyna Load Data syntax owns package lifetime directly.
// - Summary:
//   - Pure Dyna Load Data package transition model.
// - Description:
//   - Projects P3D postfix operations onto normalized extracted package roots.
// - Usage:
//   - Shared by DynamicZone preflight and future mission package-lifetime
//     logic.
// - Defaults:
//   - World Sphere operations are preserved but do not alter P3D package roots.
//

//! Pure package-set transitions for typed Dyna Load Data.

use std::collections::BTreeMap;

use super::{DynaLoadData, DynaLoadOperationKind};

/// One authored Dyna Load Data effect projected onto package identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynaLoadPackageEffect {
    kind: DynaLoadOperationKind,
    source_target: String,
    package_root: Option<String>,
}

impl DynaLoadPackageEffect {
    /// Return the exact authored target preceding the postfix operator.
    #[must_use]
    pub fn source_target(&self) -> &str {
        &self.source_target
    }

    /// Return the typed authored operation.
    #[must_use]
    pub const fn kind(&self) -> DynaLoadOperationKind {
        self.kind
    }

    /// Return the normalized extracted P3D package root when applicable.
    #[must_use]
    pub fn package_root(&self) -> Option<&str> {
        self.package_root.as_deref()
    }
}

/// Ordered package effects for one already-validated Dyna Load Data string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynaLoadPackageTransition {
    source: String,
    effects: Vec<DynaLoadPackageEffect>,
}

impl DynaLoadPackageTransition {
    /// Return the exact authored Dyna Load Data source.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Return package effects in authored order.
    #[must_use]
    pub fn effects(&self) -> &[DynaLoadPackageEffect] {
        &self.effects
    }

    /// Apply P3D effects whose final set is independent of runtime ordering.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed active roots or when one package is both
    /// loaded and unloaded by the same transition and runtime ordering matters.
    pub fn apply_order_independent_package_roots(
        &self,
        active: &[String],
    ) -> Result<Vec<String>, String> {
        validate_order_independent_p3d_effects(&self.effects)?;
        let mut roots = BTreeMap::new();
        for root in active {
            let normalized = normalize_active_package_root(root)?;
            let _ = roots.entry(normalized).or_insert_with(|| root.clone());
        }
        for effect in &self.effects {
            let Some(root) = effect.package_root() else {
                continue;
            };
            if effect.kind().is_p3d_load() {
                drop(roots.insert(root.to_owned(), root.to_owned()));
            } else if effect.kind().is_p3d_unload() {
                drop(roots.remove(root));
            }
        }
        Ok(roots.into_values().collect())
    }
}

/// Project typed Dyna Load Data onto normalized package-set effects.
///
/// # Errors
///
/// Returns an error if a P3D target cannot be normalized into extracted package
/// identity. The syntax parser is expected to have rejected unsafe targets
/// first.
pub fn compile_dyna_load_package_transition(
    data: &DynaLoadData,
) -> Result<DynaLoadPackageTransition, String> {
    let mut effects = Vec::with_capacity(data.operations().len());
    for operation in data.operations() {
        let package_root = if operation.kind().is_p3d_load()
            || operation.kind().is_p3d_unload()
        {
            Some(normalize_p3d_target(operation.target())?)
        } else {
            None
        };
        effects.push(DynaLoadPackageEffect {
            kind: operation.kind(),
            source_target: operation.target().to_owned(),
            package_root,
        });
    }
    Ok(DynaLoadPackageTransition {
        source: data.source().to_owned(),
        effects,
    })
}

fn validate_order_independent_p3d_effects(
    effects: &[DynaLoadPackageEffect],
) -> Result<(), String> {
    let mut directions = BTreeMap::<&str, bool>::new();
    for effect in effects {
        let Some(root) = effect.package_root() else {
            continue;
        };
        let is_load = effect.kind().is_p3d_load();
        if let Some(previous) = directions.insert(root, is_load)
            && previous != is_load
        {
            return Err(format!(
                                // jig-ignore-next-line: literal
                                "Dyna Load Data package `{root}` has conflicting load/unload effects"
            ));
        }
    }
    Ok(())
}

fn normalize_p3d_target(target: &str) -> Result<String, String> {
    let normalized = target.replace(char::from(92), "/").to_ascii_lowercase();
    let Some(without_extension) = normalized.strip_suffix(".p3d") else {
        return Err(
            "Dyna Load Data package target does not end in .p3d".to_owned()
        );
    };
    let relative = without_extension
        .strip_prefix("art/")
        .unwrap_or(without_extension);
    if relative.is_empty()
        || relative.starts_with('/')
        || relative.ends_with('/')
        || relative.contains(':')
        || relative.split('/').any(|segment| {
            segment.is_empty() || segment == "." || segment == ".."
        })
    {
        return Err("Dyna Load Data package target is malformed".to_owned());
    }
    Ok(format!("extracted/art/{relative}"))
}

pub(super) fn validate_active_package_roots(
    roots: &[String],
) -> Result<(), String> {
    for root in roots {
        drop(normalize_active_package_root(root)?);
    }
    Ok(())
}

fn normalize_active_package_root(root: &str) -> Result<String, String> {
    if root.is_empty()
        || root != root.trim()
        || root.chars().any(char::is_control)
    {
        return Err(
            "Dyna Load Data active package root is malformed".to_owned()
        );
    }
    let normalized = root.replace(char::from(92), "/").to_ascii_lowercase();
    if !normalized.starts_with("extracted/")
        || normalized.starts_with('/')
        || normalized.ends_with('/')
        || normalized.contains(':')
        || normalized.split('/').any(|segment| {
            segment.is_empty() || segment == "." || segment == ".."
        })
    {
        return Err("Dyna Load Data active package root is unsafe".to_owned());
    }
    Ok(normalized)
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/dyna_load_package/tests.rs"]
mod tests;
