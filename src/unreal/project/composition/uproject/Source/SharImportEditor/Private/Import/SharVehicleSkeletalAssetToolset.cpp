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
//   - SHAR native vehicle skeletal asset toolset implementation.
// - Description:
//   - Publishes a dirty unsaved SkeletalMesh and Skeleton pair without FBX.
// - Usage:
//   - Called after normalized model verification and before physics/materials.
// - Defaults:
//   - Existing outputs and invalid normalized evidence fail closed.
//

//! SHAR native vehicle skeletal asset toolset implementation.

#include "Import/SharVehicleSkeletalAssetToolset.h"

#include "Import/SharVehicleSkeletalAssetBuilder.hpp"
#include "Import/SharVehicleSkeletalModelBuilder.hpp"

#include "Animation/Skeleton.h"
#include "Engine/SkeletalMesh.h"
#include "Misc/PackageName.h"

namespace
{
void RaiseNativeSkeletalError(const FString &Error)
{
    FFrame::KismetExecutionMessage(*Error, ELogVerbosity::Error,
                                   TEXT("SharVehicleSkeletalAssetToolset"));
}
} // namespace

TArray<FString> USharVehicleSkeletalAssetToolset::CreateVehicleSkeletalMesh(
    const FString &SourceFile, const FString &FolderPath,
    const FString &AssetName)
{
    using namespace UE::SharImportEditor::Private;
    FSharNormalizedVehicleSkeletalModel Model;
    FString Error;
    if (!ParseNormalizedVehicleSkeletalModelFile(SourceFile, Model, Error))
    {
        RaiseNativeSkeletalError(Error);
        return {};
    }
    FSharPublishedVehicleSkeletalAssets Published;
    if (!PublishVehicleSkeletalAssetsCreateOnly(Model, FolderPath, AssetName,
                                                AssetName + TEXT("_Skeleton"),
                                                Published, Error))
    {
        RaiseNativeSkeletalError(Error);
        return {};
    }
    return {
        Published.MeshObjectPath,
        Published.SkeletonObjectPath,
    };
}

bool USharVehicleSkeletalAssetToolset::VerifyVehicleSkeletalMesh(
    const FString &SourceFile, const FString &SkeletalMeshPath)
{
    using namespace UE::SharImportEditor::Private;
    const FString PackagePath =
        FPackageName::ObjectPathToPackageName(SkeletalMeshPath);
    const FString ObjectName =
        FPackageName::ObjectPathToObjectName(SkeletalMeshPath);
    if (PackagePath.IsEmpty() || ObjectName.IsEmpty() ||
        !PackagePath.StartsWith(TEXT("/Game/Generated/SHAR/"),
                                ESearchCase::CaseSensitive))
    {
        RaiseNativeSkeletalError(
            TEXT("skeletal_mesh_path is not generated and canonical"));
        return false;
    }
    FSharNormalizedVehicleSkeletalModel Model;
    FString Error;
    if (!ParseNormalizedVehicleSkeletalModelFile(SourceFile, Model, Error))
    {
        RaiseNativeSkeletalError(Error);
        return false;
    }
    USkeletalMesh *Mesh = LoadObject<USkeletalMesh>(nullptr, *SkeletalMeshPath);
    if (Mesh == nullptr)
    {
        RaiseNativeSkeletalError(TEXT("skeletal_mesh_path did not load"));
        return false;
    }
    USkeleton *Skeleton = Mesh->GetSkeleton();
    if (Skeleton == nullptr)
    {
        RaiseNativeSkeletalError(TEXT("native vehicle Skeleton is missing"));
        return false;
    }
    if (!VerifyVehicleSkeletalAssets(Model, *Mesh, *Skeleton, Error))
    {
        RaiseNativeSkeletalError(Error);
        return false;
    }
    return true;
}
