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
//   - Shared native graph mechanics for reviewed Pure3D simple/unlit materials.
// - Must-Not:
//   - Select source shader families, infer culling policy, save assets, or
//   - parse generated plans.
// - Allows:
//   - Build and read back the common texture, vertex-colour, blend, and alpha
//   - graph selected by a family-specific policy.
// - Split-When:
//   - Another reviewed source shader family requires a different graph kernel.
// - Merge-When:
//   - Another private builder owns the identical simple/unlit graph mechanics.
// - Summary:
//   - Shared reviewed simple/unlit material graph kernel.
// - Description:
//   - Keeps world and vehicle selection policy separate while sharing only the
//   - native graph that both policies have independently reviewed.
// - Usage:
//   - Called by world and vehicle material policy wrappers after validation.
// - Defaults:
//   - No family-specific culling or readiness assumption is supplied here.
//

//! Shared reviewed simple/unlit material graph kernel.

#pragma once

#include "CoreMinimal.h"

class UMaterial;

enum class ESharSimpleUnlitBlend : uint8
{
    Opaque,
    SourceAlpha,
    Additive,
};

struct FSharSimpleUnlitMaterialGraphRecipe
{
    ESharSimpleUnlitBlend Blend = ESharSimpleUnlitBlend::Opaque;
    bool bAlphaTest = false;
    bool bTwoSided = false;
};

namespace UE::SharImportEditor::Private
{
bool BuildSimpleUnlitMaterialGraph(
    UMaterial& Material,
    const FSharSimpleUnlitMaterialGraphRecipe& Recipe,
    FString& OutError
);

bool ReadBackSimpleUnlitMaterialGraph(
    const UMaterial& Material,
    const FSharSimpleUnlitMaterialGraphRecipe& Recipe,
    FString& OutError
);
}
