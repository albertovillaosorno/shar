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
//   - Canonical Unreal plan hashing and rendering orchestration.
// - Must-Not:
//   - Select source packages, read files, or contact Unreal Editor.
// - Allows:
//   - Validated domain values and shared hashing and JSON adapters.
// - Split-When:
//   - Split when another plan serialization gains an independent lifecycle.
// - Merge-When:
//   - Merge when another composition module owns identical plan assembly.
// - Summary:
//   - Unreal conversion-plan composition.
// - Description:
//   - Assigns stable operation identities and renders the six-plan bundle.
// - Usage:
//   - Invoked through inherent methods on validated domain values.
// - Defaults:
//   - Collisions, dependency drift, and noncanonical values fail closed.
//

//! Canonical Unreal plan hashing and rendering orchestration.

use std::collections::{BTreeMap, BTreeSet};

use shar_sha256::digest_hex;

use crate::domain::{
    ConversionPlan, PlanArtifact, PlanBundle, PlanContext, PlanDependency,
    PlanFamily, SemanticBlockerClass,
};

mod render;

impl ConversionPlan {
    /// Return the deterministic operation identity.
    #[must_use]
    pub fn operation_id(&self) -> String {
        let digest = digest_hex(self.identity_preimage().as_bytes());
        let prefix = digest.get(..16).unwrap_or(digest.as_str());
        format!("operation-{prefix}")
    }
}

impl PlanBundle {
    /// Build and render one complete plan bundle.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid evidence, collisions, or dependency drift.
    pub fn build(
        context: &PlanContext,
        operations: Vec<ConversionPlan>,
    ) -> Result<Self, String> {
        Self::build_with_semantic_blockers(context, operations, Vec::new())
    }

    /// Build a complete plan bundle with unresolved semantic-source evidence.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid evidence, collisions, or dependency drift.
    pub fn build_with_semantic_blockers(
        context: &PlanContext,
        operations: Vec<ConversionPlan>,
        mut semantic_blockers: Vec<SemanticBlockerClass>,
    ) -> Result<Self, String> {
        context.validate()?;
        for blocker in &semantic_blockers {
            blocker.validate()?;
        }
        semantic_blockers.sort();
        if semantic_blockers.windows(2).any(|pair| {
            pair.first().zip(pair.get(1)).is_some_and(|(left, right)| {
                left.category == right.category
                    && left.target_kind == right.target_kind
                    && left.import_profile == right.import_profile
            })
        }) {
            return Err("semantic blocker classes must be unique".to_owned());
        }
        let semantic_blocker_count = semantic_blockers
            .iter()
            .try_fold(0usize, |total, blocker| total.checked_add(blocker.count))
            .ok_or_else(|| "semantic blocker count overflowed".to_owned())?;
        let mut validated = operations;
        for operation in &validated {
            let identity = operation.operation_id();
            operation.validate(&identity)?;
        }
        validated.sort_by_cached_key(ConversionPlan::operation_id);
        validate_operation_set(&validated)?;
        let mut artifacts = Vec::with_capacity(PlanFamily::all().len());
        for family in PlanFamily::all() {
            let dependencies = resolve_plan_dependencies(family, &artifacts)?;
            let family_operations = validated
                .iter()
                .filter(|operation| operation.family() == family)
                .cloned()
                .collect::<Vec<_>>();
            let preimage = render::plan_preimage(
                context,
                family,
                &dependencies,
                &family_operations,
            );
            let revision = digest_hex(preimage.as_bytes());
            artifacts.push(PlanArtifact {
                family,
                filename: family.filename().to_owned(),
                json: render::plan_json(
                    context,
                    family,
                    &revision,
                    &dependencies,
                    &family_operations,
                ),
                dependencies,
                revision,
                operation_count: family_operations.len(),
            });
        }
        let preimage = render::bundle_preimage(
            context,
            semantic_blocker_count,
            &semantic_blockers,
            &artifacts,
        );
        let index_revision = digest_hex(preimage.as_bytes());
        let index_json = render::bundle_json(
            context,
            semantic_blocker_count,
            &semantic_blockers,
            &index_revision,
            &artifacts,
        );
        Ok(Self {
            artifacts,
            semantic_blocker_count,
            semantic_blockers,
            index_revision,
            index_json,
        })
    }
}

fn resolve_plan_dependencies(
    family: PlanFamily,
    artifacts: &[PlanArtifact],
) -> Result<Vec<PlanDependency>, String> {
    family
        .dependency_ids()
        .iter()
        .map(|plan_id| {
            let artifact = artifacts
                .iter()
                .find(|artifact| artifact.family.plan_id() == *plan_id)
                .ok_or_else(|| {
                    format!(
                        "plan {} depends on unavailable plan {plan_id}",
                        family.plan_id()
                    )
                })?;
            Ok(PlanDependency {
                plan_id: (*plan_id).to_owned(),
                revision: artifact.revision.clone(),
            })
        })
        .collect()
}

fn validate_operation_set(operations: &[ConversionPlan]) -> Result<(), String> {
    let mut operation_ids = BTreeSet::new();
    let mut destinations = BTreeSet::new();
    for operation in operations {
        let operation_id = operation.operation_id();
        if !operation_ids.insert(operation_id.clone()) {
            return Err(format!(
                "duplicate operation identity: {operation_id}"
            ));
        }
        if !destinations.insert(operation.destination.to_ascii_lowercase()) {
            return Err(format!(
                "case-insensitive Unreal destination collision: {}",
                operation.destination
            ));
        }
    }
    let operation_families = operations
        .iter()
        .map(|operation| (operation.operation_id(), operation.family()))
        .collect::<BTreeMap<_, _>>();
    for operation in operations {
        for dependency in &operation.dependencies {
            let Some(dependency_family) = operation_families.get(dependency)
            else {
                return Err(format!(
                    "operation {} depends on unknown operation {dependency}",
                    operation.operation_id()
                ));
            };
            if *dependency_family > operation.family() {
                return Err(
                    "operation depends on a later plan family".to_owned()
                );
            }
        }
    }
    validate_acyclic_dependencies(operations)
}

fn validate_acyclic_dependencies(
    operations: &[ConversionPlan],
) -> Result<(), String> {
    let mut remaining = operations
        .iter()
        .map(|operation| {
            (operation.operation_id(), operation.dependencies.as_slice())
        })
        .collect::<BTreeMap<_, _>>();
    let mut completed = BTreeSet::new();
    while !remaining.is_empty() {
        let ready = remaining
            .iter()
            .filter(|(_operation_id, dependencies)| {
                dependencies
                    .iter()
                    .all(|dependency| completed.contains(dependency))
            })
            .map(|(operation_id, _dependencies)| operation_id.clone())
            .collect::<Vec<_>>();
        if ready.is_empty() {
            return Err(
                "operation dependency graph contains a cycle".to_owned()
            );
        }
        for operation_id in ready {
            let _removed = remaining.remove(&operation_id);
            let _inserted = completed.insert(operation_id);
        }
    }
    Ok(())
}
