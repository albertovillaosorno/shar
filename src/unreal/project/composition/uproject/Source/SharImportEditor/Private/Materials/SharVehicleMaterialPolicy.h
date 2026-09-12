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
//   - Native selection policy for reviewed vehicle simple material graphs.
// - Must-Not:
//   - Parse plan JSON, promote presentation-special slots, or reuse world-only
//   - culling assumptions.
// - Allows:
//   - Exact verified PDDI fields already classified by the vehicle plan.
// - Split-When:
//   - Lit, sphere-map, environment, glass, light, or VFX materials gain policy.
// - Merge-When:
//   - Another vehicle policy owns the identical reviewed subset.
// - Summary:
//   - Vehicle simple material graph policy.
// - Description:
//   - Converts exact graph-review fields into the shared native kernel while
//   - retaining vehicle-specific readiness boundaries.
// - Usage:
//   - Used by the vehicle material toolset and native automation tests.
// - Defaults:
//   - Anything outside the reviewed simple material classifiers fails.
//

//! Vehicle simple material graph policy.

#pragma once

#include "CoreMinimal.h"
#include "Materials/SharSimpleLitMaterialGraph.h"
#include "Materials/SharSimpleUnlitMaterialGraph.h"

class UMaterial;

struct FSharSimpleUnlitVehicleMasterRecipe
{
    ESharSimpleUnlitBlend Blend = ESharSimpleUnlitBlend::Opaque;
    bool bAlphaTest = false;
    bool bTwoSided = false;
};

struct FSharSimpleLitVehicleMasterRecipe
{
    float SourceShininess = 0.0F;
    bool bTwoSided = false;
};

namespace UE::SharImportEditor::Private
{
bool ResolveSimpleUnlitVehicleMasterRecipe(
    const FString& ShaderFamily,
    bool bLit,
    int32 BlendMode,
    bool bAlphaTest,
    int32 AlphaCompare,
    bool bTwoSided,
    FSharSimpleUnlitVehicleMasterRecipe& OutRecipe,
    FString& OutError
);

bool ResolveSimpleLitVehicleMasterRecipe(
    const FString& ShaderFamily,
    bool bLit,
    int32 BlendMode,
    bool bAlphaTest,
    int32 AlphaCompare,
    bool bTwoSided,
    FLinearColor SourceAmbient,
    FLinearColor SourceSpecular,
    FLinearColor SourceEmissive,
    float SourceShininess,
    FSharSimpleLitVehicleMasterRecipe& OutRecipe,
    FString& OutError
);

bool BuildSimpleLitVehicleMaster(
    UMaterial& Material,
    const FSharSimpleLitVehicleMasterRecipe& Recipe,
    FString& OutError
);

bool ReadBackSimpleLitVehicleMaster(
    const UMaterial& Material,
    const FSharSimpleLitVehicleMasterRecipe& Recipe,
    FString& OutError
);

bool BuildSimpleUnlitVehicleMaster(
    UMaterial& Material,
    const FSharSimpleUnlitVehicleMasterRecipe& Recipe,
    FString& OutError
);

bool ReadBackSimpleUnlitVehicleMaster(
    const UMaterial& Material,
    const FSharSimpleUnlitVehicleMasterRecipe& Recipe,
    FString& OutError
);
}
