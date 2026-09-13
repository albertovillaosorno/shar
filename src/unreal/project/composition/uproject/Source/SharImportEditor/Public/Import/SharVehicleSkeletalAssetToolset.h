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
//   - Create-only native vehicle SkeletalMesh and Skeleton publication.
// - Must-Not:
//   - Save packages, overwrite assets, or accept pre-rebased model evidence.
// - Allows:
//   - Verified normalized model JSON beneath the native construction boundary.
// - Split-When:
//   - Native skeletal reimport gains an independent identity lifecycle.
// - Merge-When:
//   - Another toolset owns identical normalized vehicle skeletal publication.
// - Summary:
//   - SHAR native vehicle skeletal asset toolset.
// - Description:
//   - Publishes a dirty unsaved SkeletalMesh and Skeleton pair without FBX.
// - Usage:
//   - Called after normalized model verification and before physics/materials.
// - Defaults:
//   - Existing outputs and invalid normalized evidence fail closed.
//

//! SHAR native vehicle skeletal asset toolset.

#pragma once

#include "CoreMinimal.h"
#include "ToolsetRegistry/ToolsetDefinition.h"

#include "SharVehicleSkeletalAssetToolset.generated.h"

UCLASS(BlueprintType)
class SHARIMPORTEDITOR_API USharVehicleSkeletalAssetToolset
    : public UToolsetDefinition
{
    GENERATED_BODY()

  public:
    /**
     * Create one native generated-root SkeletalMesh and companion Skeleton.
     * The companion asset is named `${asset_name}_Skeleton`.
     * Both outputs stay dirty and unsaved for caller-owned persistence.
     */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleSkeletalAssetToolset")
    static TArray<FString> CreateVehicleSkeletalMesh(
        const FString &SourceFile,
        const FString &FolderPath,
        const FString &AssetName);

    /** Verify one generated native vehicle SkeletalMesh against source JSON. */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleSkeletalAssetToolset")
    static bool VerifyVehicleSkeletalMesh(const FString &SourceFile,
                                          const FString &SkeletalMeshPath);
};
