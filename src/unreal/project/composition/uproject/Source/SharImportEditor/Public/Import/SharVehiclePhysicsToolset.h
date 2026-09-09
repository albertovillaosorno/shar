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
//   - Dirty, unsaved publication of verified vehicle Physics Assets.
// - Must-Not:
//   - Parse source catalogs, save packages, overwrite assets, or approximate
//   - unsupported source shapes.
// - Allows:
//   - Already projected sphere and box fields from prepare-unreal evidence.
// - Split-When:
//   - Physics Asset persistence gains transaction-owned save policy.
// - Merge-When:
//   - Another editor toolset owns identical generated vehicle publication.
// - Summary:
//   - SHAR generated vehicle Physics Asset toolset.
// - Description:
//   - Publishes one reviewed native Physics Asset from verified recipe fields.
// - Usage:
//   - Called after skeletal import and vehicle-physics plan verification.
// - Defaults:
//   - Creation is create-only, generated-root-only, dirty, and unsaved.
//

//! SHAR generated vehicle Physics Asset toolset.

#pragma once

#include "CoreMinimal.h"
#include "ToolsetRegistry/ToolsetDefinition.h"

#include "SharVehiclePhysicsToolset.generated.h"

UENUM(BlueprintType)
enum class ESharVehiclePhysicsShapeInputKind : uint8
{
    Sphere,
    Box,
};

USTRUCT(BlueprintType)
struct SHARIMPORTEDITOR_API FSharVehiclePhysicsShapeInput
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Vehicle Physics")
    ESharVehiclePhysicsShapeInputKind Kind =
        ESharVehiclePhysicsShapeInputKind::Sphere;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Vehicle Physics")
    FName BoneName;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Vehicle Physics")
    FVector Center = FVector::ZeroVector;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Vehicle Physics")
    double Radius = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Vehicle Physics")
    FVector AxisX = FVector::ForwardVector;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Vehicle Physics")
    FVector AxisY = FVector::RightVector;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Vehicle Physics")
    FVector AxisZ = FVector::UpVector;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Vehicle Physics")
    FVector BoxExtents = FVector::ZeroVector;
};

UCLASS(BlueprintType)
class SHARIMPORTEDITOR_API USharVehiclePhysicsToolset
    : public UToolsetDefinition
{
    GENERATED_BODY()

public:
    /**
     * Create one generated-root Physics Asset from verified plan fields.
     * The result is registered, dirty, and intentionally not saved.
     */
    UFUNCTION(meta = (AICallable), Category = "SharVehiclePhysicsToolset")
    static FString CreateVehiclePhysicsAsset(
        const FString& FolderPath,
        const FString& AssetName,
        const FString& SkeletalMeshPath,
        FName RigIdentity,
        int32 SourceJointCount,
        const TArray<FSharVehiclePhysicsShapeInput>& Shapes
    );
};
