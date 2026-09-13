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
//   - Validated native vehicle skeletal-model decoding and basis conversion.
// - Must-Not:
//   - Create assets, save packages, or infer missing normalized evidence.
// - Allows:
//   - Decode the versioned normalized model and convert source basis once.
// - Split-When:
//   - Asset publication or animation construction gains its own lifecycle.
// - Merge-When:
//   - Another builder owns identical normalized skeletal-model decoding.
// - Summary:
//   - Vehicle normalized skeletal-model construction input.
// - Description:
//   - Produces Unreal-basis mesh and rig recipes without FBX mediation.
// - Usage:
//   - Consumed by native vehicle SkeletalMesh and Skeleton construction.
// - Defaults:
//   - Invalid schema, basis, topology, rig, or skin evidence fails closed.
//

//! Vehicle normalized skeletal-model construction input.

#pragma once

#include "CoreMinimal.h"

#include <array>

namespace UE::SharImportEditor::Private
{
struct FSharVehicleBoneRecipe
{
    FName BoneName;
    int32 ParentIndex = INDEX_NONE;
    std::array<double, 16> LocalRestMatrix{};
};

struct FSharVehicleSkinInfluenceRecipe
{
    int32 VertexIndex = INDEX_NONE;
    int32 BoneIndex = INDEX_NONE;
    float Weight = 0.0F;
};

struct FSharVehiclePrimitiveGroupRecipe
{
    FString MaterialId;
    TArray<FVector3f> PositionsCm;
    TArray<FVector3f> Normals;
    TArray<FVector2f> UV0;
    TArray<FVector4f> Colors;
    TArray<FIntVector> Triangles;
    TArray<FSharVehicleSkinInfluenceRecipe> SkinInfluences;
};

struct FSharVehiclePartRecipe
{
    FString MeshId;
    TArray<FSharVehiclePrimitiveGroupRecipe> Groups;
};

struct FSharNormalizedVehicleSkeletalModel
{
    FString ModelId;
    TArray<FSharVehicleBoneRecipe> Bones;
    TArray<FSharVehiclePartRecipe> Parts;
    int32 AnimationCount = 0;
};

FVector3f ConvertSharVehiclePositionToUnrealCm(const FVector3f &Source);

FVector3f ConvertSharVehicleDirectionToUnreal(const FVector3f &Source);

std::array<double, 16> ConvertSharVehicleRestMatrixToUnreal(
    const std::array<double, 16> &Source);

bool ParseNormalizedVehicleSkeletalModel(
    const FString &JsonText, FSharNormalizedVehicleSkeletalModel &OutModel,
    FString &OutError);

bool ParseNormalizedVehicleSkeletalModelFile(
    const FString &SourceFile, FSharNormalizedVehicleSkeletalModel &OutModel,
    FString &OutError);
} // namespace UE::SharImportEditor::Private
