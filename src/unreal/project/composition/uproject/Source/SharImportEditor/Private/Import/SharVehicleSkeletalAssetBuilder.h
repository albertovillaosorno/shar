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
//   - Transient native vehicle Skeleton and SkeletalMesh shell construction.
// - Must-Not:
//   - Save packages, publish content, or repeat source-basis conversion.
// - Allows:
//   - Construct transient UObjects and render data from a validated recipe.
// - Split-When:
//   - Render geometry publication gains its own construction lifecycle.
// - Merge-When:
//   - Another builder owns identical transient skeletal UObject construction.
// - Summary:
//   - Native vehicle skeletal UObject shell builder.
// - Description:
//   - Materializes rig identity, rest transforms, and material slots natively.
// - Usage:
//   - Runs after normalized model decoding and before package publication.
// - Defaults:
//   - Invalid recipes fail before output objects are returned.
//

//! Native vehicle skeletal UObject shell builder.

#pragma once

#include "CoreMinimal.h"

class USkeletalMesh;
class USkeleton;

namespace UE::SharImportEditor::Private
{
struct FSharNormalizedVehicleSkeletalModel;

bool BuildTransientVehicleSkeletalAssetShell(
    const FSharNormalizedVehicleSkeletalModel &Model, USkeletalMesh *&OutMesh,
    USkeleton *&OutSkeleton, FString &OutError);

struct FSharPublishedVehicleSkeletalAssets
{
    USkeletalMesh *Mesh = nullptr;
    USkeleton *Skeleton = nullptr;
    FString MeshObjectPath;
    FString SkeletonObjectPath;
};

bool PublishVehicleSkeletalAssetsCreateOnly(
    const FSharNormalizedVehicleSkeletalModel &Model, const FString &FolderPath,
    const FString &MeshAssetName, const FString &SkeletonAssetName,
    FSharPublishedVehicleSkeletalAssets &OutAssets, FString &OutError);

bool VerifyVehicleSkeletalAssets(
    const FSharNormalizedVehicleSkeletalModel &Model, const USkeletalMesh &Mesh,
    const USkeleton &Skeleton, FString &OutError);
} // namespace UE::SharImportEditor::Private
