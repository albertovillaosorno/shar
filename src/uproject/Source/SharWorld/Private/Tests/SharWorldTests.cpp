// File: SharWorldTests.cpp
// Path: src/uproject/Source/SharWorld/Private/Tests/SharWorldTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient world-definition and clock tests; no map or actor loading.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharWorld; reason=two cohesive world-contract scenarios;
// split=separate clock tests when more time-of-day behaviors exist;
// validation=validate.sh SharWorld plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "World/SharWorldClock.h"
#include "World/SharWorldDefinition.h"

#include "Misc/AutomationTest.h"

static constexpr float NoonHour = 12.0F;
static constexpr float SixHoursOfRealTime = 360.0F;
static constexpr float ExpectedEveningHour = 18.0F;
static constexpr float HourTolerance = 0.001F;
static constexpr float PausedAdvanceSeconds = 60.0F;

static void FillWorldBase(USharWorldDefinition& World)
{
    World.CanonicalId = FName(TEXT("open_world"));
    World.DisplayName = FText::FromString(TEXT("Open world"));
    World.SourcePackageIds = {FName(TEXT("world_contract"))};
    World.RevisionToken = TEXT("sha256:world_definition_v1");
    World.ValidationProfile = FName(TEXT("world_definition_v1"));
    World.OwningFeature = FName(TEXT("base"));

    FSharWorldRegionDefinition Region;
    Region.RegionId = FName(TEXT("springfield_core"));
    Region.RuntimeGridId = FName(TEXT("open_world_grid"));
    Region.HlodProfileId = FName(TEXT("desktop_hlod_v1"));
    Region.bAlwaysLoaded = true;
    World.Regions.Add(Region);

    FSharDataLayerDefinition ChapterLayer;
    ChapterLayer.LayerId = FName(TEXT("chapter_01_content"));
    ChapterLayer.RequiredLayerIds.Add(FName(TEXT("base_geography")));
    World.DataLayers.Add(ChapterLayer);

    FSharDataLayerDefinition BaseLayer;
    BaseLayer.LayerId = FName(TEXT("base_geography"));
    World.DataLayers.Add(BaseLayer);
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldDefinitionValidationTest,
    "SHAR.World.Definition.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldClockTest,
    "SHAR.World.Clock.DayCycle",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharWorldDefinitionValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* World = NewObject<USharWorldDefinition>();
    FillWorldBase(*World);

    TArray<FText> Errors;
    World->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid connected world passes"), Errors.IsEmpty());

    World->Orientation.EastAxis = FVector(0.0, -1.0, 0.0);
    Errors.Reset();
    World->GatherValidationErrors(Errors);
    TestFalse(TEXT("Mirrored world orientation is rejected"), Errors.IsEmpty());
    World->Orientation = {};

    World->DataLayers.Last().RequiredLayerIds.Add(
        FName(TEXT("chapter_01_content"))
    );
    Errors.Reset();
    World->GatherValidationErrors(Errors);
    TestFalse(TEXT("Data Layer cycle is rejected"), Errors.IsEmpty());
    return true;
}

bool FSharWorldClockTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Clock = NewObject<USharWorldClock>();
    TestTrue(
        TEXT("Clock accepts the canonical day length"),
        Clock->Configure(
            USharWorldClock::DefaultDayLengthSeconds,
            NoonHour
        )
    );
    TestTrue(
        TEXT("Clock advances by world-time ratio"),
        Clock->AdvanceRealSeconds(SixHoursOfRealTime)
    );
    TestTrue(
        TEXT("Six real minutes advance six world hours"),
        FMath::Abs(Clock->GetWorldHour() - ExpectedEveningHour)
            <= HourTolerance
    );

    Clock->SetPaused(true);
    TestTrue(TEXT("Paused clock accepts elapsed time"), Clock->AdvanceRealSeconds(PausedAdvanceSeconds));
    TestTrue(
        TEXT("Paused clock does not advance"),
        FMath::Abs(Clock->GetWorldHour() - ExpectedEveningHour)
            <= HourTolerance
    );
    return true;
}

#endif
