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
//   - Bounded editor creation of reviewed SHAR vehicle materials.
// - Must-Not:
//   - Save packages, overwrite assets, parse source catalogs, or claim special
//   - presentation bindings are ready.
// - Allows:
//   - Generated vehicle material masters selected from verified plan fields.
// - Split-When:
//   - Additional shader families or runtime parameter mutation gain policy.
// - Merge-When:
//   - Another editor toolset owns identical vehicle master construction.
// - Summary:
//   - SHAR generated vehicle material toolset.
// - Description:
//   - Creates reviewed bounded simple/unlit and opaque simple/lit graphs.
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

    /**
     * Create one reviewed opaque simple/lit vehicle master.
     * The source family must be simple and lit with PDDI None blend, no alpha
     * test, black ambient/specular/emissive, and finite shininess in the GL
     * range.
     * TwoSided is preserved exactly. The material stays dirty and unsaved.
     */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleMaterialToolset")
    static FString CreateSimpleLitVehicleMaster(
        const FString& FolderPath,
        const FString& AssetName,
        const FString& ShaderFamily,
        bool bLit,
        int32 BlendMode,
        bool bAlphaTest,
        int32 AlphaCompare,
        bool bTwoSided,
        FLinearColor SourceAmbient,
        FLinearColor SourceSpecular,
        FLinearColor SourceEmissive,
        float SourceShininess
    );

    /** Verify one existing simple/unlit master without mutating it. */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleMaterialToolset")
    static bool VerifySimpleUnlitVehicleMaster(
        const FString& ObjectPath,
        const FString& ShaderFamily,
        bool bLit,
        int32 BlendMode,
        bool bAlphaTest,
        int32 AlphaCompare,
        bool bTwoSided
    );

    /** Verify one existing opaque simple/lit master without mutating it. */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleMaterialToolset")
    static bool VerifySimpleLitVehicleMaster(
        const FString& ObjectPath,
        const FString& ShaderFamily,
        bool bLit,
        int32 BlendMode,
        bool bAlphaTest,
        int32 AlphaCompare,
        bool bTwoSided,
        FLinearColor SourceAmbient,
        FLinearColor SourceSpecular,
        FLinearColor SourceEmissive,
        float SourceShininess
    );

    /**
     * Upgrade one clean simple/unlit master for SkeletalMesh rendering.
     * The exact graph recipe is verified before and after the usage-only edit.
     * The package remains unsaved and dirty after a successful mutation.
     */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleMaterialToolset")
    static bool UpgradeSimpleUnlitVehicleMasterSkeletalUsage(
        const FString& ObjectPath,
        const FString& ShaderFamily,
        bool bLit,
        int32 BlendMode,
        bool bAlphaTest,
        int32 AlphaCompare,
        bool bTwoSided
    );

    /**
     * Upgrade one clean opaque simple/lit master for SkeletalMesh rendering.
     * The exact graph recipe is verified before and after the usage-only edit.
     * The package remains unsaved and dirty after a successful mutation.
     */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleMaterialToolset")
    static bool UpgradeSimpleLitVehicleMasterSkeletalUsage(
        const FString& ObjectPath,
        const FString& ShaderFamily,
        bool bLit,
        int32 BlendMode,
        bool bAlphaTest,
        int32 AlphaCompare,
        bool bTwoSided,
        FLinearColor SourceAmbient,
        FLinearColor SourceSpecular,
        FLinearColor SourceEmissive,
        float SourceShininess
    );

    /**
     * Create one vehicle Material Instance from reviewed generated inputs.
     * The parent must be a generated vehicle master. An optional texture must
     * live below the generated vehicle texture root. The instance remains
     * dirty and unsaved for transaction read-back and explicit publication.
     */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleMaterialToolset")
    static FString CreateVehicleMaterialInstance(
        const FString& FolderPath,
        const FString& AssetName,
        const FString& ParentMaterialPath,
        const FString& BaseColorTexturePath,
        FLinearColor BaseColorTint,
        bool bSetAlphaReference,
        float AlphaReference
    );

    /**
     * Read exact material paths from selected generated vehicle mesh slots.
     * Slot indices must be unique and ascending, and every supplied slot name
     * must match the imported Skeletal Mesh. Empty strings represent null
     * materials. The mesh is not modified.
     */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleMaterialToolset")
    static TArray<FString> ReadVehicleMaterialSlots(
        const FString& SkeletalMeshPath,
        const TArray<int32>& SlotIndices,
        const TArray<FString>& ExpectedSlotNames
    );

    /**
     * Compare and exchange selected generated vehicle Skeletal Mesh materials.
     * Current paths must exactly match ExpectedMaterialPaths before mutation.
     * Replacement paths must be generated vehicle Material Instances or empty
     * for rollback. Slot topology and names are preserved. The mesh remains
     * dirty and unsaved for outer transaction read-back and explicit save.
     */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleMaterialToolset")
    static TArray<FString> CompareExchangeVehicleMaterialSlots(
        const FString& SkeletalMeshPath,
        const TArray<int32>& SlotIndices,
        const TArray<FString>& ExpectedSlotNames,
        const TArray<FString>& ExpectedMaterialPaths,
        const TArray<FString>& ReplacementMaterialPaths
    );
};
