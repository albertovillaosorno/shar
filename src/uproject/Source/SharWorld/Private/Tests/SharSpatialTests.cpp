// File: SharSpatialTests.cpp
// Path: src/uproject/Source/SharWorld/Private/Tests/SharSpatialTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient spatial definition and observation tests; no collision scene or downstream adapters.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharWorld; reason=three cohesive spatial-runtime scenarios;
// split=separate shape tests when convex or spline volumes are introduced;
// validation=validate.sh SharWorld plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "Spatial/SharSpatialPlacementDefinition.h"
#include "Spatial/SharWorldSpatialObservationSubsystem.h"

#include "CoreMinimal.h"
#include "Misc/AutomationTest.h"

static constexpr int32 EnterSequence = 1;
static constexpr int32 StaySequence = 2;
static constexpr int32 ExitSequence = 3;
static constexpr double SphereRadiusCentimeters = 200.0;

static void FillPlacementBase(USharSpatialPlacementDefinition& Definition)
{
    Definition.PlacementId = FName(TEXT("kwik_e_mart_mission_start"));
    Definition.OwnerId = FName(TEXT("chapter_01"));
    Definition.RoleIds = {FName(TEXT("mission_start"))};
    Definition.ActivationPredicateId = FName(TEXT("chapter_01_active"));
    Definition.ParticipantFilterId = FName(TEXT("local_player"));
    Definition.ObservationPolicyId = FName(TEXT("enter_exit_v1"));
    Definition.DataLayerIds = {FName(TEXT("base_geography"))};
    Definition.BundleIds = {FName(TEXT("gameplay"))};
    Definition.RevisionToken = TEXT("sha256:placement_v1");
    Definition.SourceAliases = {FName(TEXT("legacy_kwik_mission_start"))};

    FSharSpatialVolumeDefinition Volume;
    Volume.VolumeId = FName(TEXT("mission_start_volume"));
    Volume.Shape = ESharSpatialVolumeShape::Sphere;
    Volume.Dimensions = FVector(SphereRadiusCentimeters, 0.0, 0.0);
    Volume.QueryChannelId = FName(TEXT("pawn_overlap"));
    Volume.ParticipantFilterId = FName(TEXT("local_player"));
    Volume.ObservationPolicyId = FName(TEXT("enter_exit_v1"));
    Definition.Volumes.Add(Volume);
}

static FSharSpatialObservation MakeObservation(
    const ESharSpatialObservationKind Kind,
    const int32 SequenceNumber
)
{
    FSharSpatialObservation Observation;
    Observation.PlacementId = FName(TEXT("kwik_e_mart_mission_start"));
    Observation.VolumeId = FName(TEXT("mission_start_volume"));
    Observation.ParticipantId = FName(TEXT("player_01"));
    Observation.WorldRevision = TEXT("sha256:world_v1");
    Observation.DefinitionRevision = TEXT("sha256:placement_v1");
    Observation.Kind = Kind;
    Observation.SequenceNumber = SequenceNumber;
    return Observation;
}

static USharWorldSpatialObservationSubsystem* MakeConfiguredSubsystem()
{
    auto* Subsystem = NewObject<USharWorldSpatialObservationSubsystem>();
    Subsystem->ConfigureWorld(
        FName(TEXT("springfield_world")),
        TEXT("sha256:world_v1")
    );
    Subsystem->RegisterPlacement(
        FName(TEXT("kwik_e_mart_mission_start")),
        TEXT("sha256:placement_v1")
    );
    return Subsystem;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharSpatialDefinitionValidationTest,
    "SHAR.World.Spatial.Definition.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharSpatialObservationLifecycleTest,
    "SHAR.World.Spatial.Observation.Lifecycle",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharSpatialRevisionFenceTest,
    "SHAR.World.Spatial.Observation.RevisionFences",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharSpatialDefinitionValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Definition = NewObject<USharSpatialPlacementDefinition>();
    FillPlacementBase(*Definition);

    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid spatial placement passes"), Errors.IsEmpty());

    Definition->Volumes.Last().Dimensions.X = -SphereRadiusCentimeters;
    Errors.Reset();
    Definition->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Negative sphere radius is rejected"),
        Errors.IsEmpty()
    );
    return true;
}

bool FSharSpatialObservationLifecycleTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharWorldSpatialObservationSubsystem* Subsystem =
        MakeConfiguredSubsystem();
    const FSharSpatialObservation Enter = MakeObservation(
        ESharSpatialObservationKind::Enter,
        EnterSequence
    );
    const FSharSpatialObservation Stay = MakeObservation(
        ESharSpatialObservationKind::Stay,
        StaySequence
    );
    const FSharSpatialObservation Exit = MakeObservation(
        ESharSpatialObservationKind::Exit,
        ExitSequence
    );

    TestTrue(
        TEXT("Enter observation is accepted"),
        Subsystem->Observe(Enter) == ESharSpatialObservationResult::Accepted
    );
    TestTrue(
        TEXT("Accepted enter creates occupancy"),
        Subsystem->IsOccupied(
            Enter.PlacementId,
            Enter.VolumeId,
            Enter.ParticipantId
        )
    );
    TestTrue(
        TEXT("Stay observation is accepted while occupied"),
        Subsystem->Observe(Stay) == ESharSpatialObservationResult::Accepted
    );
    TestTrue(
        TEXT("Exit observation is accepted"),
        Subsystem->Observe(Exit) == ESharSpatialObservationResult::Accepted
    );
    TestFalse(
        TEXT("Accepted exit releases occupancy"),
        Subsystem->IsOccupied(
            Exit.PlacementId,
            Exit.VolumeId,
            Exit.ParticipantId
        )
    );
    return true;
}

bool FSharSpatialRevisionFenceTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharWorldSpatialObservationSubsystem* Subsystem =
        MakeConfiguredSubsystem();
    FSharSpatialObservation Observation = MakeObservation(
        ESharSpatialObservationKind::Enter,
        EnterSequence
    );

    Observation.WorldRevision = TEXT("sha256:stale_world");
    TestTrue(
        TEXT("Stale world revision is rejected"),
        Subsystem->Observe(Observation)
            == ESharSpatialObservationResult::StaleWorld
    );

    Observation.WorldRevision = TEXT("sha256:world_v1");
    Observation.DefinitionRevision = TEXT("sha256:stale_placement");
    TestTrue(
        TEXT("Stale definition revision is rejected"),
        Subsystem->Observe(Observation)
            == ESharSpatialObservationResult::StaleDefinition
    );

    Observation.DefinitionRevision = TEXT("sha256:placement_v1");
    TestTrue(
        TEXT("Placement release succeeds"),
        Subsystem->ReleasePlacement(Observation.PlacementId)
    );
    TestTrue(
        TEXT("Late observation after release is rejected"),
        Subsystem->Observe(Observation)
            == ESharSpatialObservationResult::UnknownPlacement
    );
    return true;
}

#endif
