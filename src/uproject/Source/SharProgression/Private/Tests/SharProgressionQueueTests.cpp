// File: SharProgressionQueueTests.cpp
// Path: src/uproject/Source/SharProgression/Private/Tests/SharProgressionQueueTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic progression mutation priority and lexical ordering tests only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharProgressionTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "Progression/SharProgressionContracts.h"
#include "Progression/SharProgressionSubsystem.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharProgressionQueueOrderingTest,
    "SHAR.Progression.Queue.DeterministicOrdering",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

static FSharProgressionMutationRequest MakeQueueMutation(
    const FName& MutationId,
    const ESharProgressionMutationPriority Priority,
    const FName& TransactionId,
    const FString& MutationRevision
)
{
    return MakeProgressionMutationRequest(
        MutationId,
        Priority,
        MutationRevision,
        {
            MakeProgressionOperationRequest(
                TransactionId,
                FName(TEXT("grant_currency")),
                FName(TEXT("coins")),
                GrantedCoinQuantity
            ),
        }
    );
}

bool FSharProgressionQueueOrderingTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharProgressionRuntimeFixture Runtime = MakeProgressionRuntime();
    const FSharProgressionMutationRequest Background = MakeQueueMutation(
        FName(TEXT("background_mutation")),
        ESharProgressionMutationPriority::Background,
        FName(TEXT("background_currency")),
        TEXT("sha256:background_mutation_v1")
    );
    const FSharProgressionMutationRequest UserB = MakeQueueMutation(
        FName(TEXT("user_mutation_b")),
        ESharProgressionMutationPriority::User,
        FName(TEXT("user_currency_b")),
        TEXT("sha256:user_mutation_b_v1")
    );
    const FSharProgressionMutationRequest UserA = MakeQueueMutation(
        FName(TEXT("user_mutation_a")),
        ESharProgressionMutationPriority::User,
        FName(TEXT("user_currency_a")),
        TEXT("sha256:user_mutation_a_v1")
    );
    Runtime.ProgressionSubsystem->Submit(Background);
    Runtime.ProgressionSubsystem->Submit(UserB);
    Runtime.ProgressionSubsystem->Submit(UserA);

    TestTrue(
        TEXT("Equal priority uses lexical mutation identity"),
        Runtime.ProgressionSubsystem->GetQueuePosition(UserA.MutationId)
            == FirstQueuePosition
    );
    TestTrue(
        TEXT("Second user mutation follows lexical identity"),
        Runtime.ProgressionSubsystem->GetQueuePosition(UserB.MutationId)
            == SecondQueuePosition
    );
    TestTrue(
        TEXT("Background mutation follows user mutations"),
        Runtime.ProgressionSubsystem->GetQueuePosition(Background.MutationId)
            == ThirdQueuePosition
    );
    TestTrue(
        TEXT("Non-head mutation cannot begin"),
        Runtime.ProgressionSubsystem->Begin(Background.MutationId)
            == ESharProgressionMutationResult::NotHead
    );
    return true;
}

#endif
