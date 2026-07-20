// File: SharSpatialPlacementDefinition.h
// Path: src/uproject/Source/SharWorld/Public/Spatial/SharSpatialPlacementDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable authored placement, volume, filter, and observation policy metadata only.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharWorld; reason=cohesive reflected spatial placement schema;
// split=extract volume assets if shape policies become independently versioned;
// validation=validate.sh SharWorld plus Unreal automation; review=2027-01.

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
    FVector Location;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Transform")
    FVector RotationEulerDegrees;

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
    FVector Dimensions;

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
