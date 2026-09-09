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
//   - Transient Physics Asset construction from verified vehicle shape recipes.
// - Must-Not:
//   - Read source catalogs, save packages, or approximate unsupported shapes.
// - Allows:
//   - Create analytic sphere and box bodies against an imported skeletal mesh.
// - Split-When:
//   - Persisted Physics Asset publication gains an independent lifecycle.
// - Merge-When:
//   - Another editor boundary owns identical vehicle collision construction.
// - Summary:
//   - Vehicle Physics Asset construction kernel.
// - Description:
//   - Revalidates imported rig identity, scene-unit scale, bones, and geometry.
// - Usage:
//   - Called only after the revision-bound vehicle-physics plan is verified.
// - Defaults:
//   - Invalid or unsupported construction requests fail without mutation.
//

//! Vehicle Physics Asset construction kernel.


#pragma once

#include "CoreMinimal.h"

class UPhysicsAsset;
class USkeletalMesh;

namespace UE::SharImportEditor::Private
{
enum class ESharVehiclePhysicsShapeKind : uint8
{
    Sphere,
    Box,
};

struct FSharVehiclePhysicsShapeRecipe
{
    ESharVehiclePhysicsShapeKind Kind = ESharVehiclePhysicsShapeKind::Sphere;
    FName BoneName;
    FVector Center = FVector::ZeroVector;
    float Radius = 0.0F;
    FVector AxisX = FVector::ForwardVector;
    FVector AxisY = FVector::RightVector;
    FVector AxisZ = FVector::UpVector;
    FVector BoxExtents = FVector::ZeroVector;
};

bool BuildTransientVehiclePhysicsAsset(
    USkeletalMesh& SkeletalMesh,
    const FName RigIdentity,
    int32 JointCount,
    const TArray<FSharVehiclePhysicsShapeRecipe>& Shapes,
    UPhysicsAsset*& OutPhysicsAsset,
    FString& OutError
);
} // namespace UE::SharImportEditor::Private
