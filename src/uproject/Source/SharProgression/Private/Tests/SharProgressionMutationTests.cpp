// File: SharProgressionMutationTests.cpp
// Path: src/uproject/Source/SharProgression/Private/Tests/SharProgressionMutationTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: successful profile progression mutation, save-commit correlation, and immutable count projection tests only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharProgression; reason=one cohesive committed mutation and projection scenario;
// split=separate projection tests when more query families are implemented;
// validation=validate.sh SharProgression plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharProgressionTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "Progression/SharProgressionContracts.h"
#include "Progression/SharProgressionSubsystem.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharProgressionMutationSuccessTest,
    "SHAR.Progression.Mutation.SuccessLifecycle",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

static FSharProgressionMutationRequest MakeSuccessfulMutation()
{
    return MakeProgressionMutationRequest(
        FName(TEXT("complete_progression_batch")),
        ESharProgressionMutationPriority::Gameplay,
        TEXT("sha256:progression_v2"),
        {
            MakeProgressionOperationRequest(
                FName(TEXT("mission_currency_reward")),
                FName(TEXT("grant_currency")),
                FName(TEXT("coins")),
                GrantedCoinQuantity
            ),
            MakeProgressionOperationRequest(
                FName(TEXT("card_reward_01")),
                FName(TEXT("grant_collectible")),
                FName(TEXT("collector_card_level_01_01")),
                CollectibleCatalogMaximum
            ),
            MakeProgressionOperationRequest(
                FName(TEXT("card_reward_02")),
                FName(TEXT("grant_collectible")),
                FName(TEXT("collector_card_level_01_02")),
                CollectibleCatalogMaximum
            ),
        }
    );
}

static bool HasCommittedCollectibleProjection(
    const USharProgressionSubsystem& Subsystem
)
{
    FSharProgressionCountProjection Projection;
    const FSharProgressionCountQuery Query{
        .OperationId = FName(TEXT("grant_collectible")),
        .RequiredTargetIds = {
            FName(TEXT("collector_card_level_01_01")),
            FName(TEXT("collector_card_level_01_02")),
        },
        .ExcludedTargetIds = {},
    };
    return Subsystem.ProjectCount(Query, Projection)
        && Projection.Numerator == ExpectedCollectibleCount
        && Projection.bComplete
        && Projection.SaveRevision == TEXT("sha256:save_v2");
}

bool FSharProgressionMutationSuccessTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharProgressionRuntimeFixture Runtime = MakeProgressionRuntime();
    const FSharProgressionMutationRequest Request = MakeSuccessfulMutation();
    TestTrue(
        TEXT("Mutation request is accepted"),
        Runtime.ProgressionSubsystem->Submit(Request)
            == ESharProgressionMutationResult::Accepted
    );
    TestTrue(
        TEXT("Head mutation begins"),
        Runtime.ProgressionSubsystem->Begin(Request.MutationId)
            == ESharProgressionMutationResult::Accepted
    );
    TestTrue(
        TEXT("Candidate snapshot prepares in isolation"),
        Runtime.ProgressionSubsystem->Prepare(Request.MutationId)
            == ESharProgressionMutationResult::Accepted
    );
    TestTrue(
        TEXT("Prepared candidate does not mutate accepted currency"),
        Runtime.ProgressionSubsystem->GetQuantity(
            FName(TEXT("grant_currency")),
            FName(TEXT("coins"))
        ) == InitialCoinQuantity
    );
    TestTrue(
        TEXT("Correlated save commit activates candidate"),
        Runtime.ProgressionSubsystem->AcceptCommitEvidence(
            MakeProgressionCommitEvidence(Request, TEXT("sha256:save_v2"))
        ) == ESharProgressionMutationResult::Accepted
    );
    TestTrue(
        TEXT("Committed currency is visible"),
        Runtime.ProgressionSubsystem->GetQuantity(
            FName(TEXT("grant_currency")),
            FName(TEXT("coins"))
        ) == ExpectedCommittedCoinQuantity
    );
    TestTrue(
        TEXT("Permanent transaction is queryable"),
        Runtime.ProgressionSubsystem->HasAppliedTransaction(
            FName(TEXT("mission_currency_reward"))
        )
    );
    TestTrue(
        TEXT("Committed collectibles project exactly"),
        HasCommittedCollectibleProjection(*Runtime.ProgressionSubsystem)
    );
    TestTrue(
        TEXT("Mutation has one successful terminal result"),
        Runtime.ProgressionSubsystem->GetTerminalResult(Request.MutationId)
            == ESharProgressionTerminalResult::Success
    );
    return true;
}

#endif
