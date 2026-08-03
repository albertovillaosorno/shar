// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar mission tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar mission tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar mission tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "Missions/SharMissionDefinition.h"
#include "Missions/SharMissionRuntime.h"

#include "Misc/AutomationTest.h"

static constexpr int32 FailureStageOrder = 2;
static constexpr int32 FixtureRewardQuantity = 100;
static constexpr int32 InvalidStageOrder = 7;

static void FillMissionBase(USharMissionDefinition& Mission)
{
    Mission.CanonicalId = FName(TEXT("mission_fixture"));
    Mission.DisplayName = FText::FromString(TEXT("Mission fixture"));
    Mission.SourcePackageIds = {FName(TEXT("mission_fixture_package"))};
    Mission.RevisionToken = TEXT("sha256:mission_fixture_v1");
    Mission.ValidationProfile = FName(TEXT("mission_definition_v1"));
    Mission.OwningFeature = FName(TEXT("base"));
    Mission.ChapterId = FName(TEXT("chapter_01"));
    Mission.MissionClassId = FName(TEXT("story"));
    Mission.InitialStageId = FName(TEXT("start"));

    FSharMissionStageDefinition Start;
    Start.StageId = FName(TEXT("start"));
    Start.Order = 0;
    Start.ObjectiveKind = FName(TEXT("talk"));
    Start.SuccessStageId = FName(TEXT("complete"));
    Start.FailureStageId = FName(TEXT("failed"));
    Start.bCheckpoint = true;
    Mission.Stages.Add(Start);

    FSharMissionStageDefinition Complete;
    Complete.StageId = FName(TEXT("complete"));
    Complete.Order = 1;
    Complete.ObjectiveKind = FName(TEXT("action_sequence"));
    Complete.TerminalOutcome = ESharMissionTerminalOutcome::Success;
    Mission.Stages.Add(Complete);

    FSharMissionStageDefinition Failed;
    Failed.StageId = FName(TEXT("failed"));
    Failed.Order = FailureStageOrder;
    Failed.ObjectiveKind = FName(TEXT("action_sequence"));
    Failed.TerminalOutcome = ESharMissionTerminalOutcome::Failure;
    Mission.Stages.Add(Failed);

    FSharMissionRewardOperation Reward;
    Reward.OperationId = FName(TEXT("grant_story_currency"));
    Reward.OperationKind = FName(TEXT("grant_currency"));
    Reward.TargetId = FName(TEXT("coins"));
    Reward.Quantity = FixtureRewardQuantity;
    Mission.RewardOperations.Add(Reward);
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharMissionDefinitionValidationTest,
    "SHAR.Missions.Definition.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharMissionRuntimeTransitionTest,
    "SHAR.Missions.Runtime.Transitions",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharMissionDefinitionValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Mission = NewObject<USharMissionDefinition>();
    FillMissionBase(*Mission);
    TArray<FText> Errors;
    Mission->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid mission definition passes"), Errors.IsEmpty());

    Mission->Stages.Last().Order = InvalidStageOrder;
    Mission->Stages.Last().ObjectiveKind =
        FName(TEXT("execute_arbitrary_script"));
    Errors.Reset();
    Mission->GatherValidationErrors(Errors);
    // jig-ignore-next-line: exact syntax is indivisible
    TestFalse(TEXT("Malformed mission definition is rejected"), Errors.IsEmpty());
    return true;
}

bool FSharMissionRuntimeTransitionTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Mission = NewObject<USharMissionDefinition>();
    FillMissionBase(*Mission);
    auto* Runtime = NewObject<USharMissionRuntime>();

    TestTrue(TEXT("Mission starts"), Runtime->StartMission(Mission));
    TestTrue(
        TEXT("Mission enters active state"),
        Runtime->GetState() == ESharMissionRuntimeState::Active
    );
    TestTrue(TEXT("Objective succeeds"), Runtime->ResolveObjective(true));
    TestTrue(
        TEXT("Success terminal is applied"),
        Runtime->GetState() == ESharMissionRuntimeState::Succeeded
    );

    auto* FailureRuntime = NewObject<USharMissionRuntime>();
    // jig-ignore-next-line: exact syntax is indivisible
    TestTrue(TEXT("Second mission starts"), FailureRuntime->StartMission(Mission));
    // jig-ignore-next-line: exact syntax is indivisible
    TestTrue(TEXT("Objective failure resolves"), FailureRuntime->ResolveObjective(false));
    TestTrue(
        TEXT("Failure terminal is applied"),
        FailureRuntime->GetState() == ESharMissionRuntimeState::Failed
    );
    return true;
}

#endif
