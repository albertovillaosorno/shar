// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar world definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar world definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar world definition composition module.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharWorldDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharCardinalDirection : uint8
{
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
};

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharNorthUpMapCoordinate
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "World|Orientation")
    double EastingCentimeters = 0.0;

    UPROPERTY(BlueprintReadOnly, Category = "World|Orientation")
    double NorthingCentimeters = 0.0;
};

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharNorthUpScreenCoordinate
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "World|Orientation")
    double HorizontalCentimeters = 0.0;

    UPROPERTY(BlueprintReadOnly, Category = "World|Orientation")
    double VerticalCentimeters = 0.0;
};

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharWorldOrientationDefinition
{
    GENERATED_BODY()

    // jig-ignore-next-line: exact syntax is indivisible
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World|Orientation")
    FVector NorthAxis = FVector(1.0, 0.0, 0.0);

    // jig-ignore-next-line: exact syntax is indivisible
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World|Orientation")
    FVector EastAxis = FVector(0.0, 1.0, 0.0);

    // jig-ignore-next-line: exact syntax is indivisible
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World|Orientation")
    FVector UpAxis = FVector(0.0, 0.0, 1.0);

    // jig-ignore-next-line: exact syntax is indivisible
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World|Orientation")
    FVector MapCenter = FVector(0.0, 0.0, 0.0);

    // jig-ignore-next-line: exact syntax is indivisible
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World|Orientation")
    double SeaLevelZCentimeters = 0.0;

    // jig-ignore-next-line: exact syntax is indivisible
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World|Orientation", meta = (ClampMin = "0.0"))
    double NorthernHarborMinimumNorthingCentimeters = 1.0;

    [[nodiscard]] bool IsCanonical() const;
    void GatherValidationErrors(TArray<FText>& OutErrors) const;
};

UCLASS()
class SHARWORLD_API USharWorldOrientationLibrary final : public UObject
{
    GENERATED_BODY()

public:
    static constexpr double FullCircleDegrees = 360.0;
    static constexpr double CardinalSectorDegrees = 45.0;
    static constexpr double CardinalHalfSectorDegrees = 22.5;

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FSharWorldOrientationDefinition GetCanonicalWorldOrientation();

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FVector GetWorldNorth();

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FVector GetWorldEast();

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FVector GetWorldSouth();

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FVector GetWorldWest();

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FVector GetWorldUp();

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FVector GetMapCenter();

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static double GetSeaLevelZCentimeters();

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static double NormalizeBearingDegrees(double BearingDegrees);

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static bool TryGetBearingDegrees(
        const FVector& FromWorldLocation,
        const FVector& ToWorldLocation,
        double& OutBearingDegrees
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static double GetHeadingDegreesFromYaw(double WorldYawDegrees);

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static double GetSignedHeadingDeltaDegrees(
        double CurrentHeadingDegrees,
        double TargetHeadingDegrees
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static ESharCardinalDirection GetCardinalDirection(double BearingDegrees);

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FVector GetCardinalUnitVector(ESharCardinalDirection Direction);

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FName GetCardinalAbbreviation(ESharCardinalDirection Direction);

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FText GetCardinalDisplayName(ESharCardinalDirection Direction);

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static double GetNorthingCentimeters(const FVector& WorldLocation);

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static double GetEastingCentimeters(const FVector& WorldLocation);

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FSharNorthUpMapCoordinate ProjectWorldToNorthUpMap(
        const FVector& WorldLocation
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static FSharNorthUpScreenCoordinate ProjectWorldToNorthUpScreen(
        const FVector& WorldLocation
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static bool IsAtOrAboveSeaLevel(const FVector& WorldLocation);

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static bool IsNorthOfMapCenter(const FVector& WorldLocation);

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static bool IsValidNorthernHarborPlacement(
        const FVector& WorldLocation
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Orientation")
    static bool IsValidNorthernHarborPlacementWithMinimumNorthing(
        const FVector& WorldLocation,
        double MinimumNorthingCentimeters
    );
};

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharWorldRegionDefinition
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World")
    FName RegionId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Streaming")
    FName RuntimeGridId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Streaming")
    FName HlodProfileId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Streaming")
    bool bAlwaysLoaded = false;
};

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharDataLayerDefinition
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Data Layer")
    FName LayerId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Data Layer")
    TArray<FName> RequiredLayerIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Data Layer")
    bool bRuntime = true;
};

UCLASS(BlueprintType)
class SHARWORLD_API USharWorldDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    static constexpr float DefaultDayLengthSeconds = 1440.0F;

    // jig-ignore-next-line: exact syntax is indivisible
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World|Orientation")
    FSharWorldOrientationDefinition Orientation;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World")
    TArray<FSharWorldRegionDefinition> Regions;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Data Layer")
    TArray<FSharDataLayerDefinition> DataLayers;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Time")
    float DayLengthSeconds = DefaultDayLengthSeconds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Streaming")
    bool bUsesWorldPartition = true;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Streaming")
    bool bSupportsPredictiveStreaming = true;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
