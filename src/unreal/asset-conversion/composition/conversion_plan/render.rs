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
//   - Canonical Unreal conversion-plan JSON serialization.
// - Must-Not:
//   - Select conversion policy, read files, or contact Unreal Editor.
// - Allows:
//   - Validated plan contexts, operations, and artifact revisions.
// - Split-When:
//   - Split when another serialization format gains a lifecycle.
// - Merge-When:
//   - Merge when another module owns identical canonical rendering.
// - Summary:
//   - Unreal conversion-plan renderer.
// - Description:
//   - Produces stable JSON and revision preimages with fixed key ordering.
// - Usage:
//   - Called only by the conversion-plan domain module.
// - Defaults:
//   - Arrays preserve validated canonical order and output ends in LF.
//

//! Canonical Unreal conversion-plan JSON serialization.

use shar_json_text::domain::escape;

use crate::domain::{
    ConversionPlan, PlanArtifact, PlanContext, PlanDependency, PlanFamily,
    UNREAL_PLAN_BUNDLE_SCHEMA, UNREAL_PLAN_SCHEMA,
};

pub(super) fn plan_preimage(
    context: &PlanContext,
    family: PlanFamily,
    dependencies: &[PlanDependency],
    operations: &[ConversionPlan],
) -> String {
    render_plan(context, family, "", dependencies, operations, false)
}

pub(super) fn plan_json(
    context: &PlanContext,
    family: PlanFamily,
    revision: &str,
    dependencies: &[PlanDependency],
    operations: &[ConversionPlan],
) -> String {
    render_plan(context, family, revision, dependencies, operations, true)
}

fn render_plan(
    context: &PlanContext,
    family: PlanFamily,
    revision: &str,
    dependencies: &[PlanDependency],
    operations: &[ConversionPlan],
    trailing_newline: bool,
) -> String {
    let outputs = operations
        .iter()
        .map(|operation| operation.destination.clone())
        .collect::<Vec<_>>();
    let mut output = format!(
        concat!(
            "{{\"schema\":\"{}\",\"plan_id\":\"{}\",",
            "\"revision\":\"{}\",",
            "\"source_manifest_revision\":\"{}\",",
            "\"engine_contract_revision\":\"{}\",",
            "\"target_engine_version\":\"{}\",",
            "\"target_platform\":\"{}\",",
            "\"dependencies\":{},\"outputs\":{},",
            "\"operations\":{},\"validation\":{}}}"
        ),
        UNREAL_PLAN_SCHEMA,
        family.plan_id(),
        revision,
        context.source_manifest_revision,
        escape(&context.engine_contract_revision),
        escape(&context.target_engine_version),
        escape(&context.target_platform),
        dependencies_json(dependencies),
        strings(&outputs),
        operations_json(operations),
        validation_json(family, operations.len()),
    );
    if trailing_newline {
        output.push('\n');
    }
    output
}

fn dependencies_json(dependencies: &[PlanDependency]) -> String {
    let mut output = String::from("[");
    for (index, dependency) in dependencies.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"plan_id\":\"");
        output.push_str(&escape(&dependency.plan_id));
        output.push_str("\",\"revision\":\"");
        output.push_str(&dependency.revision);
        output.push_str("\"}");
    }
    output.push(']');
    output
}

fn operations_json(operations: &[ConversionPlan]) -> String {
    let mut output = String::from("[");
    for (index, operation) in operations.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&operation_json(operation));
    }
    output.push(']');
    output
}

fn operation_json(operation: &ConversionPlan) -> String {
    format!(
        concat!(
            "{{\"operation_id\":\"{}\",",
            "\"package_identity\":\"{}\",",
            "\"source_identity\":\"{}\",",
            "\"source_format\":\"{}\",",
            "\"target_family\":\"{}\",",
            "\"source_path\":\"{}\",",
            "\"source_revision\":\"{}\",",
            "\"destination\":\"{}\",",
            "\"target_class\":\"{}\",",
            "\"importer\":\"{}\",",
            "\"import_profile\":\"{}\",",
            "\"dependencies\":{},",
            "\"readiness\":\"{}\",",
            "\"world_owned\":{},\"runtime_bound\":{}}}"
        ),
        operation.operation_id(),
        escape(&operation.package_identity),
        escape(&operation.source_identity),
        operation.source_format.as_str(),
        operation.target_family.as_str(),
        escape(&operation.source_path),
        operation.source_revision,
        escape(&operation.destination),
        escape(&operation.target_class),
        escape(&operation.importer),
        escape(&operation.import_profile),
        strings(&operation.dependencies),
        operation.readiness.as_str(),
        operation.world_owned,
        operation.runtime_bound,
    )
}

fn validation_json(family: PlanFamily, operation_count: usize) -> String {
    let mut requirements = vec![
        "case-insensitive-destinations-unique".to_owned(),
        "generated-root-confined".to_owned(),
        "revision-matches-canonical-body".to_owned(),
        "schema-supported".to_owned(),
    ];
    match family {
        PlanFamily::AssetImport => requirements.extend([
            "import-settings-match-profile".to_owned(),
            "saved-class-matches-plan".to_owned(),
            "source-bytes-match-revision".to_owned(),
        ]),
        PlanFamily::AssetConstruction => requirements.extend([
            "native-factory-available".to_owned(),
            "normalized-json-schema-valid".to_owned(),
            "saved-class-matches-plan".to_owned(),
        ]),
        PlanFamily::WorldAssembly => requirements.extend([
            "authored-transforms-preserved".to_owned(),
            "streaming-ownership-valid".to_owned(),
        ]),
        PlanFamily::RuntimeBinding => {
            requirements.push("all-stable-references-resolve".to_owned());
        },
        PlanFamily::Validation => requirements.extend([
            "dependencies-resolve-after-restart".to_owned(),
            "semantic-digest-reproducible".to_owned(),
        ]),
        PlanFamily::Package => requirements.extend([
            "cook-succeeds".to_owned(),
            "packaged-build-loads-generated-assets".to_owned(),
        ]),
    }
    requirements.sort();
    requirements.dedup();
    format!(
        "{{\"operation_count\":{operation_count},\"requirements\":{}}}",
        strings(&requirements)
    )
}

pub(super) fn bundle_preimage(
    context: &PlanContext,
    artifacts: &[PlanArtifact],
) -> String {
    render_bundle(context, "", artifacts, false)
}

pub(super) fn bundle_json(
    context: &PlanContext,
    revision: &str,
    artifacts: &[PlanArtifact],
) -> String {
    render_bundle(context, revision, artifacts, true)
}

fn render_bundle(
    context: &PlanContext,
    revision: &str,
    artifacts: &[PlanArtifact],
    trailing_newline: bool,
) -> String {
    let mut plans = String::from("[");
    for (index, artifact) in artifacts.iter().enumerate() {
        if index > 0 {
            plans.push(',');
        }
        plans.push_str("{\"plan_id\":\"");
        plans.push_str(artifact.family.plan_id());
        plans.push_str("\",\"revision\":\"");
        plans.push_str(&artifact.revision);
        plans.push_str("\",\"filename\":\"");
        plans.push_str(&artifact.filename);
        plans.push_str("\",\"operation_count\":");
        plans.push_str(&artifact.operation_count.to_string());
        plans.push('}');
    }
    plans.push(']');
    let mut output = format!(
        concat!(
            "{{\"schema\":\"{}\",\"revision\":\"{}\",",
            "\"source_manifest_revision\":\"{}\",",
            "\"engine_contract_revision\":\"{}\",",
            "\"target_engine_version\":\"{}\",",
            "\"target_platform\":\"{}\",\"plans\":{}}}"
        ),
        UNREAL_PLAN_BUNDLE_SCHEMA,
        revision,
        context.source_manifest_revision,
        escape(&context.engine_contract_revision),
        escape(&context.target_engine_version),
        escape(&context.target_platform),
        plans,
    );
    if trailing_newline {
        output.push('\n');
    }
    output
}

fn strings(values: &[String]) -> String {
    let mut output = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push('"');
        output.push_str(&escape(value));
        output.push('"');
    }
    output.push(']');
    output
}
