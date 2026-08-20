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
//   - Shar spatial placement definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar spatial placement definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar spatial placement definition composition module.

#pragma once

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"

#include "SharSpatialPlacementDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharSpatialVolumeShape : uint8
{
    Point,
    Sphere,
    OrientedBox,
    Capsule,
};

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharSpatialTransformDefinition
{
    GENERATED_BODY()

    static constexpr double DefaultScaleComponent = 1.0;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Transform")
    FVector Location = FVector::ZeroVector;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Transform")
    FVector RotationEulerDegrees = FVector::ZeroVector;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Transform")
    FVector Scale = FVector(
        DefaultScaleComponent,
        DefaultScaleComponent,
        DefaultScaleComponent
    );
};

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharSpatialVolumeDefinition
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Identity")
    FName VolumeId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Shape")
    ESharSpatialVolumeShape Shape = ESharSpatialVolumeShape::Point;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Shape")
    FSharSpatialTransformDefinition LocalTransform;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Shape")
    FVector Dimensions = FVector::ZeroVector;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Query")
    FName QueryChannelId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Filter")
    FName ParticipantFilterId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Observation")
    FName ObservationPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Observation")
    float BoundaryToleranceCentimeters = 0.0F;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Observation")
    float HysteresisCentimeters = 0.0F;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Observation")
    float DwellSeconds = 0.0F;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Observation")
    float CooldownSeconds = 0.0F;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Observation")
    int32 Priority = 0;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Activation")
    bool bEnabled = true;
};

UCLASS(BlueprintType)
class SHARWORLD_API USharSpatialPlacementDefinition final : public UDataAsset
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Identity")
    FName PlacementId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Identity")
    FName OwnerId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Role")
    TArray<FName> RoleIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Transform")
    FSharSpatialTransformDefinition Transform;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Volume")
    TArray<FSharSpatialVolumeDefinition> Volumes;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Activation")
    FName ActivationPredicateId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Filter")
    FName ParticipantFilterId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Observation")
    FName ObservationPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Streaming")
    TArray<FName> DataLayerIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Bundles")
    TArray<FName> BundleIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Revision")
    FString RevisionToken;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Provenance")
    TArray<FName> SourceAliases;

    UFUNCTION(BlueprintCallable, Category = "SHAR|World|Spatial")
    void GatherValidationErrors(TArray<FText>& OutErrors) const;
};
