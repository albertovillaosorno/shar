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
//   - Shared native graph mechanics for reviewed Pure3D simple/lit materials.
// - Must-Not:
//   - Infer source colours, translucency, metallic response, or save assets.
// - Allows:
//   - Opaque DefaultLit texture/diffuse modulation with source shininess.
// - Split-When:
//   - Translucent lit or coloured-specular materials gain reviewed policy.
// - Merge-When:
//   - Another private builder owns the identical simple/lit graph mechanics.
// - Summary:
//   - Shared reviewed simple/lit material graph kernel.
// - Description:
//   - Maps the bounded Pure3D fixed-function lit subset to Unreal DefaultLit.
// - Usage:
//   - Called after vehicle-specific source-state validation.
// - Defaults:
//   - The reviewed subset is dielectric and has no source specular response.
//

//! Shared reviewed simple/lit material graph kernel.

#pragma once

#include "CoreMinimal.h"

class UMaterial;

struct FSharSimpleLitMaterialGraphRecipe
{
    float SourceShininess = 0.0F;
    bool bTwoSided = false;
};

namespace UE::SharImportEditor::Private
{
float SourceShininessToRoughness(float SourceShininess);

bool BuildSimpleLitMaterialGraph(
    UMaterial& Material,
    const FSharSimpleLitMaterialGraphRecipe& Recipe,
    FString& OutError
);

bool ReadBackSimpleLitMaterialGraph(
    const UMaterial& Material,
    const FSharSimpleLitMaterialGraphRecipe& Recipe,
    FString& OutError
);
}
