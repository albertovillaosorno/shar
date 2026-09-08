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
//   - Native simple-unlit world master-material graph construction policy.
// - Must-Not:
//   - Parse source catalogs, select unsupported shader families, or save
//   - assets.
// - Allows:
//   - Reviewed regular-world raster state and Unreal editor material graphs.
// - Split-When:
//   - Lit, environment, or runtime-error materials gain reviewed native policy.
// - Merge-When:
//   - Another material builder owns the identical regular-world graph contract.
// - Summary:
//   - Source-backed simple-unlit world master material policy.
// - Description:
//   - Reproduces reviewed texture modulation, blending, alpha test, and
//   - culling.
// - Usage:
//   - Used by the native world-material toolset and automation tests.
// - Defaults:
//   - Unsupported family requests fail closed without modifying the material.
//

//! Source-backed simple-unlit world master material policy.

#pragma once

#include "CoreMinimal.h"

class UMaterial;

enum class ESharWorldBlendFamily : uint8
{
    Opaque,
    SourceAlpha,
    Additive,
};

struct FSharSimpleUnlitWorldMasterRecipe
{
    ESharWorldBlendFamily Blend = ESharWorldBlendFamily::Opaque;
    bool bAlphaTest = false;
};

namespace UE::SharImportEditor::Private
{
bool ResolveSimpleUnlitWorldMasterRecipe(
    const FString& BlendFamily,
    bool bAlphaTest,
    FSharSimpleUnlitWorldMasterRecipe& OutRecipe,
    FString& OutError
);

bool BuildSimpleUnlitWorldMaster(
    UMaterial& Material,
    const FSharSimpleUnlitWorldMasterRecipe& Recipe,
    FString& OutError
);
}
