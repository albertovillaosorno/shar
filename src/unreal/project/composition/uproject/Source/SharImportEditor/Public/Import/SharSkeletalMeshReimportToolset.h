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
//   - Revisioned in-place reimport of existing generated SkeletalMesh assets.
// - Must-Not:
//   - Save packages, create replacement identities, or reimport outside SHAR.
// - Allows:
//   - Reimport one clean saved revision and reload it on failed postconditions.
// - Split-When:
//   - Another asset family requires independent replacement semantics.
// - Merge-When:
//   - Another toolset owns the identical generated SkeletalMesh lifecycle.
// - Summary:
//   - Safe generated SkeletalMesh revision reimport toolset.
// - Description:
//   - Keeps canonical object identities while preserving saved rollback state.
// - Usage:
//   - Called only after source, revision, and live capability preflight.
// - Defaults:
//   - Dirty, unsaved, missing, or companion-drifting revisions fail closed.
//

//! Safe generated SkeletalMesh revision reimport toolset.

#pragma once

#include "ToolsetRegistry/ToolsetDefinition.h"

#include "SharSkeletalMeshReimportToolset.generated.h"

UCLASS(BlueprintType)
class SHARIMPORTEDITOR_API USharSkeletalMeshReimportToolset
    : public UToolsetDefinition
{
    GENERATED_BODY()

public:
    /**
     * Reimports one existing generated SkeletalMesh without changing identity.
     * The clean saved revision is reloaded if reimport or validation fails.
     * The caller remains responsible for read-back and explicit package saving.
     * @param SourceFile Absolute verified FBX source path.
     * @param FolderPath Existing generated Unreal content folder.
     * @param AssetName Existing canonical SkeletalMesh asset name.
     * @return Existing SkeletalMesh then Skeleton object paths after reimport.
     */
    UFUNCTION(meta = (AICallable), Category = "SharSkeletalMeshReimportToolset")
    static TArray<FString> ReimportSkeletalMeshRevision(
        const FString& SourceFile,
        const FString& FolderPath,
        const FString& AssetName
    );

    /**
     * Reimports one generated vehicle SkeletalMesh without another axis
     * conversion; the published FBX already owns Unreal vehicle axes.
     */
    UFUNCTION(meta = (AICallable), Category = "SharSkeletalMeshReimportToolset")
    static TArray<FString> ReimportVehicleSkeletalMeshRevision(
        const FString& SourceFile,
        const FString& FolderPath,
        const FString& AssetName
    );
};
