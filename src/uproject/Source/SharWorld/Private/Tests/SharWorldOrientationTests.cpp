// File: SharWorldOrientationTests.cpp
// Path: src/uproject/Source/SharWorld/Private/Tests/SharWorldOrientationTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic world-orientation contract tests; no map, actor, asset, or widget loading.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#if WITH_DEV_AUTOMATION_TESTS

#include "World/SharWorldDefinition.h"

#include "Misc/AutomationTest.h"

#include <cmath>

namespace
{
constexpr double BearingTolerance = 0.001;
constexpr double VectorTolerance = 0.000001;
constexpr double HarborMinimumNorthingCentimeters = 100.0;
constexpr double HarborEastingCentimeters = 250.0;
constexpr double WestYawDegrees = -90.0;
constexpr double WestBearingDegrees = 270.0;
constexpr double EastHeadingDegrees = 90.0;
constexpr double ClockwiseNearNorthHeadingDegrees = 350.0;
constexpr double CounterclockwiseNearNorthHeadingDegrees = 10.0;
constexpr double NorthCrossingDeltaDegrees = 20.0;
constexpr double ProjectionNorthingCentimeters = 3000.0;
constexpr double ProjectionEastingCentimeters = 2000.0;
constexpr double ProjectionElevationCentimeters = 500.0;
constexpr double BelowSeaLevelCentimeters = -1.0;

struct FSharBearingCase
{
    FSharBearingCase(
        const TCHAR* InLabel,
        const FVector& InTarget,
        const double InBearingDegrees,
        const ESharCardinalDirection InDirection,
        const TCHAR* InAbbreviation
    )
        : Label(InLabel),
          Target(InTarget),
          BearingDegrees(InBearingDegrees),
          Direction(InDirection),
          Abbreviation(InAbbreviation)
    {
    }

    const TCHAR* Label;
    FVector Target;
    double BearingDegrees;
    ESharCardinalDirection Direction;
    FName Abbreviation;
};

struct FSharObservedBearing
{
    bool bValid = false;
    double BearingDegrees = 0.0;
};

} // namespace

static bool NearlyEqual(
    const double Left,
    const double Right,
    const double Tolerance
)
{
    return std::abs(Left - Right) <= Tolerance;
}

static bool VectorsEqual(
    const FVector& Left,
    const FVector& Right,
    const double Tolerance
)
{
    return NearlyEqual(Left.X, Right.X, Tolerance)
        && NearlyEqual(Left.Y, Right.Y, Tolerance)
        && NearlyEqual(Left.Z, Right.Z, Tolerance);
}

static const TArray<FSharBearingCase>& GetBearingCases()
{
    static const TArray<FSharBearingCase> Cases = {
        {
            TEXT("North"),
            FVector(100.0, 0.0, 0.0),
            0.0,
            ESharCardinalDirection::North,
            TEXT("N"),
        },
        {
            TEXT("Northeast"),
            FVector(100.0, 100.0, 0.0),
            45.0,
            ESharCardinalDirection::NorthEast,
            TEXT("NE"),
        },
        {
            TEXT("East"),
            FVector(0.0, 100.0, 0.0),
            90.0,
            ESharCardinalDirection::East,
            TEXT("E"),
        },
        {
            TEXT("Southeast"),
            FVector(-100.0, 100.0, 0.0),
            135.0,
            ESharCardinalDirection::SouthEast,
            TEXT("SE"),
        },
        {
            TEXT("South"),
            FVector(-100.0, 0.0, 0.0),
            180.0,
            ESharCardinalDirection::South,
            TEXT("S"),
        },
        {
            TEXT("Southwest"),
            FVector(-100.0, -100.0, 0.0),
            225.0,
            ESharCardinalDirection::SouthWest,
            TEXT("SW"),
        },
        {
            TEXT("West"),
            FVector(0.0, -100.0, 0.0),
            270.0,
            ESharCardinalDirection::West,
            TEXT("W"),
        },
        {
            TEXT("Northwest"),
            FVector(100.0, -100.0, 0.0),
            315.0,
            ESharCardinalDirection::NorthWest,
            TEXT("NW"),
        },
    };
    return Cases;
}

static FSharWorldOrientationDefinition MakeMirroredOrientation()
{
    FSharWorldOrientationDefinition Orientation;
    Orientation.NorthAxis = FVector(-1.0, 0.0, 0.0);
    return Orientation;
}

static FSharObservedBearing ObserveBearing(
    const FVector& FromWorldLocation,
    const FVector& ToWorldLocation
)
{
    FSharObservedBearing Result;
    Result.bValid = USharWorldOrientationLibrary::TryGetBearingDegrees(
        FromWorldLocation,
        ToWorldLocation,
        Result.BearingDegrees
    );
    return Result;
}

static void CheckBearingCase(
    FAutomationTestBase& Test,
    const FSharBearingCase& Case
)
{
    const FSharObservedBearing Observed = ObserveBearing(
        FVector(0.0, 0.0, 0.0),
        Case.Target
    );
    Test.TestTrue(
        Case.Label,
        Observed.bValid
    );
    Test.TestTrue(
        Case.Label,
        NearlyEqual(
            Observed.BearingDegrees,
            Case.BearingDegrees,
            BearingTolerance
        )
    );
    Test.TestEqual(
        Case.Label,
        USharWorldOrientationLibrary::GetCardinalDirection(
            Observed.BearingDegrees
        ),
        Case.Direction
    );
    Test.TestEqual(
        Case.Label,
        USharWorldOrientationLibrary::GetCardinalAbbreviation(Case.Direction),
        Case.Abbreviation
    );

    const FSharObservedBearing VectorObserved = ObserveBearing(
        FVector(0.0, 0.0, 0.0),
        USharWorldOrientationLibrary::GetCardinalUnitVector(Case.Direction)
    );
    Test.TestTrue(
        Case.Label,
        VectorObserved.bValid
    );
    Test.TestTrue(
        Case.Label,
        NearlyEqual(
            VectorObserved.BearingDegrees,
            Case.BearingDegrees,
            BearingTolerance
        )
    );
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldOrientationAxesTest,
    "SHAR.World.Orientation.CanonicalAxes",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldOrientationBearingTest,
    "SHAR.World.Orientation.Bearings",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldOrientationHeadingTest,
    "SHAR.World.Orientation.Headings",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldOrientationProjectionTest,
    "SHAR.World.Orientation.MapProjection",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldOrientationHarborTest,
    "SHAR.World.Orientation.NorthernHarbor",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

} // namespace

bool FSharWorldOrientationAxesTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharWorldOrientationDefinition Orientation =
        USharWorldOrientationLibrary::GetCanonicalWorldOrientation();

    TestTrue(TEXT("Canonical orientation validates"), Orientation.IsCanonical());
    TestTrue(
        TEXT("World north is +X"),
        VectorsEqual(
            USharWorldOrientationLibrary::GetWorldNorth(),
            FVector(1.0, 0.0, 0.0),
            VectorTolerance
        )
    );
    TestTrue(
        TEXT("World east is +Y"),
        VectorsEqual(
            USharWorldOrientationLibrary::GetWorldEast(),
            FVector(0.0, 1.0, 0.0),
            VectorTolerance
        )
    );
    TestTrue(
        TEXT("World south is -X"),
        VectorsEqual(
            USharWorldOrientationLibrary::GetWorldSouth(),
            FVector(-1.0, 0.0, 0.0),
            VectorTolerance
        )
    );
    TestTrue(
        TEXT("World west is -Y"),
        VectorsEqual(
            USharWorldOrientationLibrary::GetWorldWest(),
            FVector(0.0, -1.0, 0.0),
            VectorTolerance
        )
    );
    TestTrue(
        TEXT("World up is +Z"),
        VectorsEqual(
            USharWorldOrientationLibrary::GetWorldUp(),
            FVector(0.0, 0.0, 1.0),
            VectorTolerance
        )
    );
    TestTrue(
        TEXT("Map center is the world origin"),
        VectorsEqual(
            USharWorldOrientationLibrary::GetMapCenter(),
            FVector(0.0, 0.0, 0.0),
            VectorTolerance
        )
    );
    TestTrue(
        TEXT("Sea level is Z=0"),
        NearlyEqual(
            USharWorldOrientationLibrary::GetSeaLevelZCentimeters(),
            0.0,
            VectorTolerance
        )
    );

    const FSharWorldOrientationDefinition Invalid =
        MakeMirroredOrientation();
    TArray<FText> Errors;
    Invalid.GatherValidationErrors(Errors);
    TestFalse(TEXT("Mirrored world orientation is rejected"), Errors.IsEmpty());
    return true;
}

bool FSharWorldOrientationBearingTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    for (const FSharBearingCase& Case : GetBearingCases())
    {
        CheckBearingCase(*this, Case);
    }
    TestFalse(
        TEXT("Coincident points do not invent a bearing"),
        ObserveBearing(
            FVector(0.0, 0.0, 0.0),
            FVector(0.0, 0.0, 0.0)
        ).bValid
    );
    return true;
}

bool FSharWorldOrientationHeadingTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    TestTrue(
        TEXT("Negative bearings normalize into the canonical range"),
        NearlyEqual(
            USharWorldOrientationLibrary::NormalizeBearingDegrees(WestYawDegrees),
            WestBearingDegrees,
            BearingTolerance
        )
    );
    TestTrue(
        TEXT("Unreal yaw is the canonical compass heading"),
        NearlyEqual(
            USharWorldOrientationLibrary::GetHeadingDegreesFromYaw(EastHeadingDegrees),
            EastHeadingDegrees,
            BearingTolerance
        )
    );
    TestTrue(
        TEXT("Heading delta crosses north clockwise"),
        NearlyEqual(
            USharWorldOrientationLibrary::GetSignedHeadingDeltaDegrees(
                ClockwiseNearNorthHeadingDegrees,
                CounterclockwiseNearNorthHeadingDegrees
            ),
            NorthCrossingDeltaDegrees,
            BearingTolerance
        )
    );
    TestTrue(
        TEXT("Heading delta crosses north counterclockwise"),
        NearlyEqual(
            USharWorldOrientationLibrary::GetSignedHeadingDeltaDegrees(
                CounterclockwiseNearNorthHeadingDegrees,
                ClockwiseNearNorthHeadingDegrees
            ),
            -NorthCrossingDeltaDegrees,
            BearingTolerance
        )
    );
    return true;
}

bool FSharWorldOrientationProjectionTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FVector WorldLocation(
        ProjectionNorthingCentimeters,
        ProjectionEastingCentimeters,
        ProjectionElevationCentimeters
    );
    const FSharNorthUpMapCoordinate NorthUp =
        USharWorldOrientationLibrary::ProjectWorldToNorthUpMap(
            WorldLocation
        );
    const FSharNorthUpScreenCoordinate Screen =
        USharWorldOrientationLibrary::ProjectWorldToNorthUpScreen(
            WorldLocation
        );

    TestTrue(
        TEXT("North-up map X is world easting"),
        NearlyEqual(NorthUp.EastingCentimeters,
            ProjectionEastingCentimeters,
            VectorTolerance)
    );
    TestTrue(
        TEXT("North-up map Y is world northing"),
        NearlyEqual(NorthUp.NorthingCentimeters,
            ProjectionNorthingCentimeters,
            VectorTolerance)
    );
    TestTrue(
        TEXT("Screen projection preserves easting"),
        NearlyEqual(Screen.HorizontalCentimeters,
            ProjectionEastingCentimeters,
            VectorTolerance)
    );
    TestTrue(
        TEXT("Screen projection maps north upward"),
        NearlyEqual(Screen.VerticalCentimeters,
            -ProjectionNorthingCentimeters,
            VectorTolerance)
    );
    TestTrue(
        TEXT("Sea level accepts Z=0"),
        USharWorldOrientationLibrary::IsAtOrAboveSeaLevel(
            FVector(0.0, 0.0, 0.0)
        )
    );
    TestFalse(
        TEXT("Submerged position is below sea level"),
        USharWorldOrientationLibrary::IsAtOrAboveSeaLevel(
            FVector(0.0, 0.0, BelowSeaLevelCentimeters)
        )
    );
    return true;
}

bool FSharWorldOrientationHarborTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    TestTrue(
        TEXT("Harbor north of Springfield satisfies the contract"),
        USharWorldOrientationLibrary::
        IsValidNorthernHarborPlacementWithMinimumNorthing(
            FVector(
                HarborMinimumNorthingCentimeters,
                HarborEastingCentimeters,
                0.0
            ),
            HarborMinimumNorthingCentimeters
        )
    );
    TestFalse(
        TEXT("Harbor at the map center is rejected"),
        USharWorldOrientationLibrary::
        IsValidNorthernHarborPlacementWithMinimumNorthing(
            FVector(0.0, 0.0, 0.0),
            HarborMinimumNorthingCentimeters
        )
    );
    TestFalse(
        TEXT("Harbor south of Springfield is rejected"),
        USharWorldOrientationLibrary::
        IsValidNorthernHarborPlacementWithMinimumNorthing(
            FVector(-HarborMinimumNorthingCentimeters, 0.0, 0.0),
            HarborMinimumNorthingCentimeters
        )
    );
    TestFalse(
        TEXT("Non-positive harbor threshold is rejected"),
        USharWorldOrientationLibrary::
        IsValidNorthernHarborPlacementWithMinimumNorthing(
            FVector(HarborMinimumNorthingCentimeters, 0.0, 0.0),
            0.0
        )
    );
    return true;
}

#endif
