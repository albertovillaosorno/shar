// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar progression mutation tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar progression mutation tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar progression mutation tests composition module.

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
