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
//   - Unreal asset-conversion domain facade.
// - Must-Not:
//   - Own persistence, transport, or live editor effects.
// - Allows:
//   - Public deterministic conversion-plan values.
// - Split-When:
//   - Split when another domain responsibility gains a lifecycle.
// - Merge-When:
//   - Merge when another module owns identical domain exports.
// - Summary:
//   - Unreal asset-conversion domain facade.
// - Description:
//   - Exposes canonical plan schemas, values, and bundle construction.
// - Usage:
//   - Consumed by pipeline orchestration and plan application adapters.
// - Defaults:
//   - Only validated plan bundles are public.
//

//! Unreal asset-conversion domain facade.

mod conversion_plan;

pub use conversion_plan::{
    ConversionPlan, NativeAssetFamily, OperationReadiness, PlanArtifact,
    PlanBundle, PlanContext, PlanDependency, PlanFamily, SourceFormat,
    UNREAL_PLAN_BUNDLE_SCHEMA, UNREAL_PLAN_SCHEMA,
};
