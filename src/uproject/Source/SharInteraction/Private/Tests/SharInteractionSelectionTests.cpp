// File: SharInteractionSelectionTests.cpp
// Path: src/uproject/Source/SharInteraction/Private/Tests/SharInteractionSelectionTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic candidate ordering and eligibility-parity tests only.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharInteraction; reason=two cohesive selection scenarios;
// split=separate eligibility reasons if policy evaluation expands;
// validation=validate.sh SharInteraction plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharInteractionTestFixtures.h"

#include "Interaction/SharInteractionSubsystem.h"
#include "Misc/AutomationTest.h"

static constexpr int32 EqualPriority = 10;
static constexpr double EqualDistanceSquared = 400.0;

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharInteractionSelectionOrderingTest,
    "SHAR.Interaction.Selection.DeterministicOrdering",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharInteractionEligibilityParityTest,
    "SHAR.Interaction.Selection.EligibilityParity",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharInteractionSelectionOrderingTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharInteractionSubsystem* Subsystem =
        MakeConfiguredInteractionSubsystem();
    const FSharInteractionSourceState ZetaSource = MakeInteractionSource({
        .SourceId = FName(TEXT("source_zeta")),
        .InteractionId = MakeInteractionId(TEXT("zeta_interaction")),
        .bExclusive = false,
    });
    const FSharInteractionSourceState AlphaSource = MakeInteractionSource({
        .SourceId = FName(TEXT("source_alpha")),
        .InteractionId = MakeInteractionId(TEXT("alpha_interaction")),
        .bExclusive = false,
    });
    Subsystem->RegisterSource(ZetaSource);
    Subsystem->RegisterSource(AlphaSource);

    FSharInteractionQuery Query = MakeInteractionQuery({
        .QueryId = FName(TEXT("overlap_query")),
        .InteractorId = FName(TEXT("player_01")),
    });
    Query.Candidates.Add(MakeInteractionCandidate({
        .SourceId = FName(TEXT("source_zeta")),
        .InteractionId = MakeInteractionId(TEXT("zeta_interaction")),
        .Priority = EqualPriority,
        .DistanceSquared = EqualDistanceSquared,
    }));
    Query.Candidates.Add(MakeInteractionCandidate({
        .SourceId = FName(TEXT("source_alpha")),
        .InteractionId = MakeInteractionId(TEXT("alpha_interaction")),
        .Priority = EqualPriority,
        .DistanceSquared = EqualDistanceSquared,
    }));

    FSharInteractionCandidate Winner;
    TestTrue(
        TEXT("Equal candidates produce one accepted winner"),
        Subsystem->SelectCandidate(Query, Winner)
            == ESharInteractionResultCode::Accepted
    );
    TestTrue(
        TEXT("Canonical interaction identity resolves the tie"),
        Winner.InteractionId == MakeInteractionId(TEXT("alpha_interaction"))
    );
    return true;
}

bool FSharInteractionEligibilityParityTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharInteractionSubsystem* Subsystem =
        MakeConfiguredInteractionSubsystem();
    const FSharInteractionSourceState Source = MakeInteractionSource({
        .SourceId = FName(TEXT("vehicle_door_source")),
        .InteractionId = MakeInteractionId(TEXT("enter_vehicle")),
        .bExclusive = true,
    });
    Subsystem->RegisterSource(Source);

    FSharInteractionQuery Query = MakeInteractionQuery({
        .QueryId = FName(TEXT("vehicle_entry_query")),
        .InteractorId = FName(TEXT("player_01")),
    });
    Query.Candidates.Add(MakeInteractionCandidate({
        .SourceId = FName(TEXT("vehicle_door_source")),
        .InteractionId = MakeInteractionId(TEXT("enter_vehicle")),
        .Priority = EqualPriority,
        .DistanceSquared = EqualDistanceSquared,
    }));
    FSharInteractionCandidate Winner;
    TestTrue(
        TEXT("Prompt query selects the eligible candidate"),
        Subsystem->SelectCandidate(Query, Winner)
            == ESharInteractionResultCode::Accepted
    );

    Query.Candidates.Last().bEligible = false;
    Query.Candidates.Last().EligibilityReasonId =
        FName(TEXT("vehicle_locked"));
    TestTrue(
        TEXT("Execution rejects the now-ineligible prompt candidate"),
        Subsystem->BeginTransaction(
            Query,
            Winner,
            FName(TEXT("vehicle_entry_transaction"))
        ) == ESharInteractionResultCode::NotEligible
    );
    return true;
}

#endif
