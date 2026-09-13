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

#include "Import/SharVehicleSkeletalAssetBuilder.h"
#include "Import/SharVehicleSkeletalModelBuilder.h"

namespace
{
void RaiseNativeSkeletalError(const FString &Error)
{
    FFrame::KismetExecutionMessage(*Error, ELogVerbosity::Error,
                                   TEXT("SharVehicleSkeletalAssetToolset"));
}
} // namespace

FString USharVehicleSkeletalAssetToolset::CreateVehicleSkeletalMesh(
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
    return Published.MeshObjectPath;
}
