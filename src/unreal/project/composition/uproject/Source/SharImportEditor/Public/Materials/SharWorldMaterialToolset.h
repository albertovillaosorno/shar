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
//   - Bounded editor creation of reviewed SHAR world materials.
// - Must-Not:
//   - Save packages, overwrite assets, or infer source shader state.
// - Allows:
//   - Generated-root material assets selected by a verified conversion plan.
// - Split-When:
//   - Additional shader families gain separate construction policy.
// - Merge-When:
//   - Another editor toolset owns identical world master construction.
// - Summary:
//   - SHAR generated world material toolset.
// - Description:
//   - Creates reviewed masters and instances without reparsing source packages.
// - Usage:
//   - Called after world material catalog verification and native planning.
// - Defaults:
//   - Only reviewed simple-unlit world families are currently supported.
//

//! SHAR generated world material toolset.

#pragma once

#include "ToolsetRegistry/ToolsetDefinition.h"

#include "SharWorldMaterialToolset.generated.h"

UCLASS(BlueprintType)
class SHARIMPORTEDITOR_API USharWorldMaterialToolset : public UToolsetDefinition
{
    GENERATED_BODY()

public:
    /**
     * Create one regular-world simple/unlit master under the generated root.
     * BlendFamily must be exactly opaque, alpha, or additive. Alpha-test and
     * blending remain independent so the original combined state is retained.
     * The material is left dirty and unsaved for transaction-owned read-back.
     */
    UFUNCTION(meta = (AICallable), Category = "SharWorldMaterialToolset")
    static FString CreateSimpleUnlitWorldMaster(
        const FString& FolderPath,
        const FString& AssetName,
        const FString& BlendFamily,
        bool bAlphaTest
    );

    /**
     * Create one simple/unlit Material Instance from verified plan values.
     * Parent and optional texture must already exist under generated roots.
     * Tint and alpha reference are normalized plan values, never source parse
     * inputs. The instance is left dirty and unsaved for transaction read-back.
     */
    UFUNCTION(meta = (AICallable), Category = "SharWorldMaterialToolset")
    static FString CreateSimpleUnlitWorldMaterialInstance(
        const FString& FolderPath,
        const FString& AssetName,
        const FString& ParentMaterialPath,
        const FString& BaseColorTexturePath,
        FLinearColor BaseColorTint,
        bool bSetAlphaReference,
        float AlphaReference
    );
};
