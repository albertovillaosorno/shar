// File: SharProgressionMutationGuardTests.cpp
// Path: src/uproject/Source/SharProgression/Private/Tests/SharProgressionMutationGuardTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: stale save-commit evidence, terminal uniqueness, release, and pre-commit failure preservation tests only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharProgression; reason=two cohesive progression mutation failure scenarios;
// split=separate commit-correlation tests when a concrete save adapter is integrated;
// validation=validate.sh SharProgression plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharProgressionTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "Progression/SharProgressionContracts.h"
#include "Progression/SharProgressionSubsystem.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharProgressionStaleEvidenceGuardTest,
    "SHAR.Progression.Mutation.StaleEvidenceAndTerminalGuards",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharProgressionPreCommitFailureTest,
    "SHAR.Progression.Mutation.PreCommitFailurePreservesSnapshot",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

static FSharProgressionMutationRequest MakeGuardMutation(
    const FName& MutationId,
    const FName& TransactionId,
    const FString& MutationRevision
)
{
    return MakeProgressionMutationRequest(
        MutationId,
        ESharProgressionMutationPriority::Gameplay,
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

bool FSharProgressionStaleEvidenceGuardTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharProgressionRuntimeFixture Runtime = MakeProgressionRuntime();
    const FSharProgressionMutationRequest Request = MakeGuardMutation(
        FName(TEXT("stale_commit_guard")),
        FName(TEXT("stale_guard_currency")),
        TEXT("sha256:progression_v2")
    );
    Runtime.ProgressionSubsystem->Submit(Request);
    BeginAndPrepareProgressionMutation(*Runtime.ProgressionSubsystem, Request);
    FSharProgressionCommitEvidence Evidence =
        MakeProgressionCommitEvidence(Request, TEXT("sha256:save_v2"));
    Evidence.ExpectedSaveRevision = TEXT("sha256:save_old");

    TestTrue(
        TEXT("Stale save evidence is rejected"),
        Runtime.ProgressionSubsystem->AcceptCommitEvidence(Evidence)
            == ESharProgressionMutationResult::StaleRevision
    );
    TestTrue(
        TEXT("Stale evidence preserves accepted currency"),
        Runtime.ProgressionSubsystem->GetQuantity(
            FName(TEXT("grant_currency")),
            FName(TEXT("coins"))
        ) == InitialCoinQuantity
    );
    TestTrue(
        TEXT("Cancellation publishes one terminal result"),
        Runtime.ProgressionSubsystem->Resolve(MakeProgressionResolution(
            Request,
            ESharProgressionResolutionCommand::Cancel
        )) == ESharProgressionMutationResult::Accepted
    );
    TestTrue(
        TEXT("Duplicate terminal resolution is rejected"),
        Runtime.ProgressionSubsystem->Resolve(MakeProgressionResolution(
            Request,
            ESharProgressionResolutionCommand::Fail
        )) == ESharProgressionMutationResult::AlreadyTerminal
    );
    TestTrue(
        TEXT("Terminal mutation releases explicitly"),
        Runtime.ProgressionSubsystem->Release(Request.MutationId)
            == ESharProgressionMutationResult::Accepted
    );
    TestTrue(
        TEXT("Released mutation leaves no active operation"),
        Runtime.ProgressionSubsystem->GetObservation().UnreleasedMutationCount
            == 0
    );
    return true;
}

bool FSharProgressionPreCommitFailureTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharProgressionRuntimeFixture Runtime = MakeProgressionRuntime();
    const FSharProgressionMutationRequest Request = MakeGuardMutation(
        FName(TEXT("pre_commit_failure")),
        FName(TEXT("uncommitted_currency")),
        TEXT("sha256:progression_v2")
    );
    Runtime.ProgressionSubsystem->Submit(Request);
    BeginAndPrepareProgressionMutation(*Runtime.ProgressionSubsystem, Request);

    TestTrue(
        TEXT("Prepared mutation may fail before save commit"),
        Runtime.ProgressionSubsystem->Resolve(MakeProgressionResolution(
            Request,
            ESharProgressionResolutionCommand::Fail
        )) == ESharProgressionMutationResult::Accepted
    );
    const FSharProgressionObservation Observation =
        Runtime.ProgressionSubsystem->GetObservation();
    TestTrue(
        TEXT("Pre-commit failure preserves save revision"),
        Observation.ActiveSnapshot.SaveRevision == TEXT("sha256:save_v1")
    );
    TestTrue(
        TEXT("Pre-commit failure preserves snapshot revision"),
        Observation.ActiveSnapshot.SnapshotRevision
            == TEXT("sha256:progression_v1")
    );
    TestFalse(
        TEXT("Uncommitted transaction is not accepted"),
        Runtime.ProgressionSubsystem->HasAppliedTransaction(
            FName(TEXT("uncommitted_currency"))
        )
    );
    TestTrue(
        TEXT("Failed mutation publishes failed terminal result"),
        Runtime.ProgressionSubsystem->GetTerminalResult(Request.MutationId)
            == ESharProgressionTerminalResult::Failed
    );
    return true;
}

#endif
