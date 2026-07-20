// File: SharProgressionCatalogProfileTests.cpp
// Path: src/uproject/Source/SharProgression/Private/Tests/SharProgressionCatalogProfileTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient progression catalog validation, profile snapshot validation, and immutable baseline projection tests only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharProgression; reason=two cohesive catalog and initial-profile validation scenarios;
// split=separate projection tests when additional query families are implemented;
// validation=validate.sh SharProgression plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharProgressionTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "Progression/SharProgressionCatalogDefinition.h"
#include "Progression/SharProgressionContracts.h"
#include "Progression/SharProgressionState.h"
#include "Progression/SharProgressionSubsystem.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharProgressionCatalogValidationTest,
    "SHAR.Progression.Catalog.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharProgressionProfileSnapshotValidationTest,
    "SHAR.Progression.Profile.SnapshotValidation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharProgressionCatalogValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharProgressionCatalogDefinition* Catalog =
        MakeProgressionCatalogDefinition();
    TArray<FText> Errors;
    Catalog->GatherValidationErrors(Errors);
    TestTrue(TEXT("Complete progression catalog is valid"), Errors.IsEmpty());

    for (FSharProgressionOperationDefinition& Operation : Catalog->Operations)
    {
        if (Operation.OperationId == FName(TEXT("unlock_character")))
        {
            Operation.ValuePolicy = ESharProgressionValuePolicy::Additive;
        }
    }
    Errors.Reset();
    Catalog->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Catalog cannot reinterpret set-once state as additive"),
        Errors.IsEmpty()
    );
    return true;
}

bool FSharProgressionProfileSnapshotValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharProgressionRuntimeFixture Runtime = MakeProgressionRuntime();
    const FSharProgressionObservation Observation =
        Runtime.ProgressionSubsystem->GetObservation();
    TestTrue(
        TEXT("Valid initial snapshot makes profile ready"),
        Observation.ProfileState == ESharProfileLifecycleState::Ready
    );
    TestTrue(
        TEXT("Initial currency value is queryable"),
        Runtime.ProgressionSubsystem->GetQuantity(
            FName(TEXT("grant_currency")),
            FName(TEXT("coins"))
        ) == InitialCoinQuantity
    );

    FSharProgressionCountProjection Projection;
    const FSharProgressionCountQuery Query{
        .OperationId = FName(TEXT("grant_collectible")),
        .RequiredTargetIds = {
            FName(TEXT("collector_card_level_01_01")),
            FName(TEXT("collector_card_level_01_02")),
        },
        .ExcludedTargetIds = {},
    };
    TestTrue(
        TEXT("Count projection is derived without mutation"),
        Runtime.ProgressionSubsystem->ProjectCount(Query, Projection)
    );
    TestTrue(TEXT("No cards are initially collected"), Projection.Numerator == 0);
    TestTrue(TEXT("Projection preserves exact denominator"), Projection.Denominator == ExpectedCollectibleCount);
    TestTrue(
        TEXT("Projection carries active snapshot revision"),
        Projection.SnapshotRevision == TEXT("sha256:progression_v1")
    );

    auto* InvalidSubsystem = NewObject<USharProgressionSubsystem>(
        Runtime.GameInstance
    );
    FSharProgressionSnapshot InvalidSnapshot = MakeInitialProgressionSnapshot();
    FSharProgressionValue DuplicateValue;
    for (const FSharProgressionValue& Value : InvalidSnapshot.Values)
    {
        DuplicateValue = Value;
        break;
    }
    InvalidSnapshot.Values.Add(DuplicateValue);
    TestFalse(
        TEXT("Duplicate snapshot values fail closed"),
        InvalidSubsystem->Configure(Runtime.CatalogSubsystem, InvalidSnapshot)
    );
    return true;
}

#endif
