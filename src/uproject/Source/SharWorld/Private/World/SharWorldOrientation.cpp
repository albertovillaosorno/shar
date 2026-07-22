// File: SharWorldOrientation.cpp
// Path: src/uproject/Source/SharWorld/Private/World/SharWorldOrientation.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic cardinal and map-orientation math only; no world, actor, asset, or widget access.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#include "World/SharWorldDefinition.h"

#include <cmath>

#define LOCTEXT_NAMESPACE "SharWorldOrientation"

namespace
{
constexpr double AxisTolerance = 0.00000001;
constexpr double DirectionToleranceSquared = 0.000000000001;
constexpr double DegreesPerRadian = 57.295779513082320876;
constexpr double InverseSquareRootTwo = 0.707106781186547524;
constexpr double HalfCircleDegrees = 180.0;
constexpr int32 CardinalDirectionCount = 8;
} // namespace

static bool IsFiniteVector(const FVector& Value)
{
    return std::isfinite(Value.X)
        && std::isfinite(Value.Y)
        && std::isfinite(Value.Z);
}

static bool NearlyEqual(const double Left, const double Right)
{
    return std::abs(Left - Right) <= AxisTolerance;
}

static bool NearlyEquals(const FVector& Left, const FVector& Right)
{
    return NearlyEqual(Left.X, Right.X)
        && NearlyEqual(Left.Y, Right.Y)
        && NearlyEqual(Left.Z, Right.Z);
}

static void AddOrientationError(TArray<FText>& OutErrors, const TCHAR* Message)
{
    OutErrors.Add(FText::FromString(Message));
}

bool FSharWorldOrientationDefinition::IsCanonical() const
{
    return IsFiniteVector(NorthAxis)
        && IsFiniteVector(EastAxis)
        && IsFiniteVector(UpAxis)
        && IsFiniteVector(MapCenter)
        && std::isfinite(SeaLevelZCentimeters)
        && std::isfinite(NorthernHarborMinimumNorthingCentimeters)
        && NearlyEquals(NorthAxis, FVector(1.0, 0.0, 0.0))
        && NearlyEquals(EastAxis, FVector(0.0, 1.0, 0.0))
        && NearlyEquals(UpAxis, FVector(0.0, 0.0, 1.0))
        && NearlyEquals(MapCenter, FVector(0.0, 0.0, 0.0))
        && NearlyEqual(SeaLevelZCentimeters, 0.0)
        && NorthernHarborMinimumNorthingCentimeters > 0.0;
}

void FSharWorldOrientationDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    if (!IsCanonical())
    {
        AddOrientationError(
            OutErrors,
            TEXT("World orientation must use +X north, +Y east, +Z up, origin map center, Z=0 sea level, and a positive northern-harbor northing threshold.")
        );
    }
}

FSharWorldOrientationDefinition
USharWorldOrientationLibrary::GetCanonicalWorldOrientation()
{
    return {};
}

FVector USharWorldOrientationLibrary::GetWorldNorth()
{
    return {1.0, 0.0, 0.0};
}

FVector USharWorldOrientationLibrary::GetWorldEast()
{
    return {0.0, 1.0, 0.0};
}

FVector USharWorldOrientationLibrary::GetWorldSouth()
{
    return {-1.0, 0.0, 0.0};
}

FVector USharWorldOrientationLibrary::GetWorldWest()
{
    return {0.0, -1.0, 0.0};
}

FVector USharWorldOrientationLibrary::GetWorldUp()
{
    return {0.0, 0.0, 1.0};
}

FVector USharWorldOrientationLibrary::GetMapCenter()
{
    return {0.0, 0.0, 0.0};
}

double USharWorldOrientationLibrary::GetSeaLevelZCentimeters()
{
    return 0.0;
}

double USharWorldOrientationLibrary::NormalizeBearingDegrees(
    const double BearingDegrees
)
{
    if (!std::isfinite(BearingDegrees))
    {
        return 0.0;
    }

    const double Remainder = std::fmod(BearingDegrees, FullCircleDegrees);
    return Remainder < 0.0 ? Remainder + FullCircleDegrees : Remainder;
}

bool USharWorldOrientationLibrary::TryGetBearingDegrees(
    const FVector& FromWorldLocation,
    const FVector& ToWorldLocation,
    double& OutBearingDegrees
)
{
    OutBearingDegrees = 0.0;
    if (!IsFiniteVector(FromWorldLocation)
        || !IsFiniteVector(ToWorldLocation))
    {
        return false;
    }

    const double DeltaNorth = ToWorldLocation.X - FromWorldLocation.X;
    const double DeltaEast = ToWorldLocation.Y - FromWorldLocation.Y;
    const double DistanceSquared =
        (DeltaNorth * DeltaNorth) + (DeltaEast * DeltaEast);
    if (DistanceSquared <= DirectionToleranceSquared)
    {
        return false;
    }

    OutBearingDegrees = NormalizeBearingDegrees(
        std::atan2(DeltaEast, DeltaNorth) * DegreesPerRadian
    );
    return true;
}

double USharWorldOrientationLibrary::GetHeadingDegreesFromYaw(
    const double WorldYawDegrees
)
{
    return NormalizeBearingDegrees(WorldYawDegrees);
}

double USharWorldOrientationLibrary::GetSignedHeadingDeltaDegrees(
    const double CurrentHeadingDegrees,
    const double TargetHeadingDegrees
)
{
    if (!std::isfinite(CurrentHeadingDegrees)
        || !std::isfinite(TargetHeadingDegrees))
    {
        return 0.0;
    }

    const double ClockwiseDelta = NormalizeBearingDegrees(
        TargetHeadingDegrees - CurrentHeadingDegrees
    );
    return ClockwiseDelta > HalfCircleDegrees
        ? ClockwiseDelta - FullCircleDegrees
        : ClockwiseDelta;
}

ESharCardinalDirection USharWorldOrientationLibrary::GetCardinalDirection(
    const double BearingDegrees
)
{
    const double Shifted = NormalizeBearingDegrees(BearingDegrees)
        + CardinalHalfSectorDegrees;
    const int32 Sector = static_cast<int32>(
        std::floor(Shifted / CardinalSectorDegrees)
    ) % CardinalDirectionCount;
    return static_cast<ESharCardinalDirection>(Sector);
}

FVector USharWorldOrientationLibrary::GetCardinalUnitVector(
    const ESharCardinalDirection Direction
)
{
    switch (Direction)
    {
        case ESharCardinalDirection::North:
            return {1.0, 0.0, 0.0};
        case ESharCardinalDirection::NorthEast:
            return {
                InverseSquareRootTwo,
                InverseSquareRootTwo,
                0.0
            };
        case ESharCardinalDirection::East:
            return {0.0, 1.0, 0.0};
        case ESharCardinalDirection::SouthEast:
            return {
                -InverseSquareRootTwo,
                InverseSquareRootTwo,
                0.0
            };
        case ESharCardinalDirection::South:
            return {-1.0, 0.0, 0.0};
        case ESharCardinalDirection::SouthWest:
            return {
                -InverseSquareRootTwo,
                -InverseSquareRootTwo,
                0.0
            };
        case ESharCardinalDirection::West:
            return {0.0, -1.0, 0.0};
        case ESharCardinalDirection::NorthWest:
            return {
                InverseSquareRootTwo,
                -InverseSquareRootTwo,
                0.0
            };
        default:
            return {0.0, 0.0, 0.0};
    }
}

FName USharWorldOrientationLibrary::GetCardinalAbbreviation(
    const ESharCardinalDirection Direction
)
{
    switch (Direction)
    {
        case ESharCardinalDirection::North:
            return FName(TEXT("N"));
        case ESharCardinalDirection::NorthEast:
            return FName(TEXT("NE"));
        case ESharCardinalDirection::East:
            return FName(TEXT("E"));
        case ESharCardinalDirection::SouthEast:
            return FName(TEXT("SE"));
        case ESharCardinalDirection::South:
            return FName(TEXT("S"));
        case ESharCardinalDirection::SouthWest:
            return FName(TEXT("SW"));
        case ESharCardinalDirection::West:
            return FName(TEXT("W"));
        case ESharCardinalDirection::NorthWest:
            return FName(TEXT("NW"));
        default:
            return NAME_None;
    }
}

FText USharWorldOrientationLibrary::GetCardinalDisplayName(
    const ESharCardinalDirection Direction
)
{
    switch (Direction)
    {
        case ESharCardinalDirection::North:
            return LOCTEXT("North", "North");
        case ESharCardinalDirection::NorthEast:
            return LOCTEXT("NorthEast", "Northeast");
        case ESharCardinalDirection::East:
            return LOCTEXT("East", "East");
        case ESharCardinalDirection::SouthEast:
            return LOCTEXT("SouthEast", "Southeast");
        case ESharCardinalDirection::South:
            return LOCTEXT("South", "South");
        case ESharCardinalDirection::SouthWest:
            return LOCTEXT("SouthWest", "Southwest");
        case ESharCardinalDirection::West:
            return LOCTEXT("West", "West");
        case ESharCardinalDirection::NorthWest:
            return LOCTEXT("NorthWest", "Northwest");
        default:
            return {};
    }
}

double USharWorldOrientationLibrary::GetNorthingCentimeters(
    const FVector& WorldLocation
)
{
    return WorldLocation.X - GetMapCenter().X;
}

double USharWorldOrientationLibrary::GetEastingCentimeters(
    const FVector& WorldLocation
)
{
    return WorldLocation.Y - GetMapCenter().Y;
}

FSharNorthUpMapCoordinate
USharWorldOrientationLibrary::ProjectWorldToNorthUpMap(
    const FVector& WorldLocation
)
{
    FSharNorthUpMapCoordinate Coordinate;
    Coordinate.EastingCentimeters = GetEastingCentimeters(WorldLocation);
    Coordinate.NorthingCentimeters = GetNorthingCentimeters(WorldLocation);
    return Coordinate;
}

FSharNorthUpScreenCoordinate
USharWorldOrientationLibrary::ProjectWorldToNorthUpScreen(
    const FVector& WorldLocation
)
{
    const FSharNorthUpMapCoordinate MapCoordinate =
        ProjectWorldToNorthUpMap(WorldLocation);
    FSharNorthUpScreenCoordinate ScreenCoordinate;
    ScreenCoordinate.HorizontalCentimeters =
        MapCoordinate.EastingCentimeters;
    ScreenCoordinate.VerticalCentimeters =
        -MapCoordinate.NorthingCentimeters;
    return ScreenCoordinate;
}

bool USharWorldOrientationLibrary::IsAtOrAboveSeaLevel(
    const FVector& WorldLocation
)
{
    return IsFiniteVector(WorldLocation)
        && WorldLocation.Z >= GetSeaLevelZCentimeters();
}

bool USharWorldOrientationLibrary::IsNorthOfMapCenter(
    const FVector& WorldLocation
)
{
    return IsFiniteVector(WorldLocation)
        && GetNorthingCentimeters(WorldLocation) > 0.0;
}

bool USharWorldOrientationLibrary::IsValidNorthernHarborPlacement(
    const FVector& WorldLocation
)
{
    const FSharWorldOrientationDefinition Orientation =
        GetCanonicalWorldOrientation();
    return IsValidNorthernHarborPlacementWithMinimumNorthing(
        WorldLocation,
        Orientation.NorthernHarborMinimumNorthingCentimeters
    );
}

bool USharWorldOrientationLibrary::
IsValidNorthernHarborPlacementWithMinimumNorthing(
    const FVector& WorldLocation,
    const double MinimumNorthingCentimeters
)
{
    return IsFiniteVector(WorldLocation)
        && std::isfinite(MinimumNorthingCentimeters)
        && MinimumNorthingCentimeters > 0.0
        && GetNorthingCentimeters(WorldLocation)
            >= MinimumNorthingCentimeters;
}

#undef LOCTEXT_NAMESPACE
