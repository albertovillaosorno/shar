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
//   - Bounded editor creation of reviewed SHAR vehicle master materials.
// - Must-Not:
//   - Save packages, overwrite assets, parse source catalogs, or claim special
//   - presentation bindings are ready.
// - Allows:
//   - Generated vehicle material masters selected from verified plan fields.
// - Split-When:
//   - Vehicle Material Instances or additional shader families gain policy.
// - Merge-When:
//   - Another editor toolset owns identical vehicle master construction.
// - Summary:
//   - SHAR generated vehicle material toolset.
// - Description:
//   - Creates only the reviewed simple/unlit graph candidate subset.
// - Usage:
//   - Called after vehicle material plan verification and native planning.
// - Defaults:
//   - Presentation-special semantics remain blocked outside this graph toolset.
//

//! SHAR generated vehicle material toolset.

#pragma once

#include "ToolsetRegistry/ToolsetDefinition.h"

#include "SharVehicleMaterialToolset.generated.h"

UCLASS(BlueprintType)
class SHARIMPORTEDITOR_API USharVehicleMaterialToolset
    : public UToolsetDefinition
{
    GENERATED_BODY()

public:
    /**
     * Create one reviewed simple/unlit vehicle master from exact plan fields.
     * ShaderFamily must be simple, Lit must be false, BlendMode must be PDDI
     * None/Alpha/Add (0/1/2), and AlphaCompare must be Greater (4). TwoSided is
     * preserved exactly. The material remains dirty and unsaved for read-back.
     */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleMaterialToolset")
    static FString CreateSimpleUnlitVehicleMaster(
        const FString& FolderPath,
        const FString& AssetName,
        const FString& ShaderFamily,
        bool bLit,
        int32 BlendMode,
        bool bAlphaTest,
        int32 AlphaCompare,
        bool bTwoSided
    );
};
