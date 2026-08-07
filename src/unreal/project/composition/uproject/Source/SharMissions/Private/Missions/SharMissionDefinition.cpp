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
//   - Shar mission definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar mission definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar mission definition composition module.

#include "Missions/SharMissionDefinition.h"

#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"
#include "Progression/SharProgressionState.h"

static void AddMissionError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool ContainsName(
    const TArray<FName>& Values,
    const FName& Candidate
)
{
    return Algo::AnyOf(
        Values,
        [&Candidate](const FName& Value)
        {
            return Value == Candidate;
        }
    );
}

static bool ContainsStage(
    const TArray<FSharMissionStageDefinition>& Stages,
    const FName& StageId
)
{
    return Algo::AnyOf(
        Stages,
        [&StageId](const FSharMissionStageDefinition& Stage)
        {
            return Stage.StageId == StageId;
        }
    );
}

static const FSharObjectivePolicyRow* FindObjectivePolicy(
    const TArray<FSharObjectivePolicyRow>& Policies,
    const FName& PolicyId
)
{
    return Policies.FindByPredicate(
        [&PolicyId](const FSharObjectivePolicyRow& Policy)
        {
            return Policy.PolicyId == PolicyId;
        }
    );
}

static bool IsSupportedObjectiveStartTrigger(const FName& Trigger)
{
    const TArray<FName> SupportedTriggers = {
        FName(TEXT("immediate")),
        FName(TEXT("proximity")),
        FName(TEXT("interaction")),
        FName(TEXT("dialogue_completion")),
        FName(TEXT("explicit_signal")),
    };
    return ContainsName(SupportedTriggers, Trigger);
}

static bool IsSupportedObjectiveRecoveryRule(const FName& RecoveryRule)
{
    if (RecoveryRule.IsNone())
    {
        return true;
    }
    const TArray<FName> SupportedRules = {
        FName(TEXT("restart_stage")),
        FName(TEXT("restart_mission")),
        FName(TEXT("restore_checkpoint")),
        FName(TEXT("return_to_free_roam")),
    };
    return ContainsName(SupportedRules, RecoveryRule);
}

static void AppendOptionalCanonicalIdentityError(
    const FName& Value,
    const TCHAR* Message,
    TArray<FText>& OutErrors
)
{
    if (
        !Value.IsNone()
        && !USharPrimaryContentDefinition::IsCanonicalIdentifier(Value)
    )
    {
        AddMissionError(OutErrors, Message);
    }
}

static void AppendObjectivePolicyErrors(
    const FSharObjectivePolicyRow& Policy,
    TSet<FName>& SeenPolicyIds,
    TArray<FText>& OutErrors
)
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(Policy.PolicyId))
    {
        AddMissionError(
            OutErrors,
            TEXT("Every objective policy requires a canonical identity.")
        );
    }
    if (SeenPolicyIds.Contains(Policy.PolicyId))
    {
        AddMissionError(
            OutErrors,
            TEXT("Objective policy identities must be unique.")
        );
    }
    SeenPolicyIds.Add(Policy.PolicyId);
    if (!USharMissionDefinition::IsSupportedObjectiveKind(Policy.ObjectiveKind))
    {
        AddMissionError(
            OutErrors,
            TEXT("Objective policy uses an unsupported objective kind.")
        );
    }
    if (!IsSupportedObjectiveStartTrigger(Policy.StartTrigger))
    {
        AddMissionError(
            OutErrors,
            TEXT("Objective policy uses an unsupported start trigger.")
        );
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Policy.CompletionRule
    ))
    {
        AddMissionError(
            OutErrors,
            TEXT("Objective policy requires a canonical completion rule.")
        );
    }
    AppendOptionalCanonicalIdentityError(
        Policy.FailureRule,
        TEXT("Objective policy failure rule must be canonical."),
        OutErrors
    );
    if (!IsSupportedObjectiveRecoveryRule(Policy.RecoveryRule))
    {
        AddMissionError(
            OutErrors,
            TEXT("Objective policy uses an unsupported recovery rule.")
        );
    }
    AppendOptionalCanonicalIdentityError(
        Policy.RouteId,
        TEXT("Objective policy route identity must be canonical."),
        OutErrors
    );
    AppendOptionalCanonicalIdentityError(
        Policy.NotorietyPolicyId,
        TEXT("Objective notoriety policy identity must be canonical."),
        OutErrors
    );
    AppendOptionalCanonicalIdentityError(
        Policy.CatchUpProfileId,
        TEXT("Objective catch-up profile identity must be canonical."),
        OutErrors
    );
    AppendOptionalCanonicalIdentityError(
        Policy.DropSequenceId,
        TEXT("Objective drop sequence identity must be canonical."),
        OutErrors
    );
    AppendOptionalCanonicalIdentityError(
        Policy.PresentationProfileId,
        TEXT("Objective presentation profile identity must be canonical."),
        OutErrors
    );

    TSet<FName> SeenTargetIds;
    for (const FName& TargetId : Policy.TargetIds)
    {
        if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(TargetId))
        {
            AddMissionError(
                OutErrors,
                TEXT("Objective target identities must be canonical.")
            );
        }
        if (SeenTargetIds.Contains(TargetId))
        {
            AddMissionError(
                OutErrors,
                TEXT("Objective target identities must be unique.")
            );
        }
        SeenTargetIds.Add(TargetId);
    }
}

static void AppendObjectivePoliciesErrors(
    const TArray<FSharObjectivePolicyRow>& Policies,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenPolicyIds;
    for (const FSharObjectivePolicyRow& Policy : Policies)
    {
        AppendObjectivePolicyErrors(Policy, SeenPolicyIds, OutErrors);
    }
}

static void AppendStageIdentityErrors(
    const FSharMissionStageDefinition& Stage,
    const int32 ExpectedOrder,
    const TArray<FSharObjectivePolicyRow>& ObjectivePolicies,
    TSet<FName>& SeenStageIds,
    TArray<FText>& OutErrors
)
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(Stage.StageId))
    {
        AddMissionError(
            OutErrors,
            TEXT("Every stage requires a canonical stage identity.")
        );
    }
    if (SeenStageIds.Contains(Stage.StageId))
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission stage identities must be unique.")
        );
    }
    SeenStageIds.Add(Stage.StageId);
    if (Stage.Order != ExpectedOrder)
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission stages must use dense zero-based order.")
        );
    }
    if (!USharMissionDefinition::IsSupportedObjectiveKind(
        Stage.ObjectiveKind
    ))
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission stage uses an unsupported objective kind.")
        );
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Stage.ObjectivePolicyId
    ))
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission stage requires a canonical objective policy id.")
        );
        return;
    }
    const FSharObjectivePolicyRow* Policy = FindObjectivePolicy(
        ObjectivePolicies,
        Stage.ObjectivePolicyId
    );
    if (Policy == nullptr)
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission stage references an unknown objective policy.")
        );
        return;
    }
    if (Policy->ObjectiveKind != Stage.ObjectiveKind)
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission stage objective kind disagrees with its policy.")
        );
    }
}

static void AppendTransitionTargetError(
    const TArray<FSharMissionStageDefinition>& Stages,
    const FName& TargetStageId,
    const TCHAR* Message,
    TArray<FText>& OutErrors
)
{
    if (!TargetStageId.IsNone() && !ContainsStage(Stages, TargetStageId))
    {
        AddMissionError(OutErrors, Message);
    }
}

static void AppendStageTransitionErrors(
    const FSharMissionStageDefinition& Stage,
    const TArray<FSharMissionStageDefinition>& Stages,
    TArray<FText>& OutErrors
)
{
    const bool bTerminal =
        Stage.TerminalOutcome != ESharMissionTerminalOutcome::None;
    const bool bHasOutgoingTransition =
        !Stage.SuccessStageId.IsNone() || !Stage.FailureStageId.IsNone();
    if (bTerminal && bHasOutgoingTransition)
    {
        AddMissionError(
            OutErrors,
            TEXT("Terminal stages cannot declare outgoing transitions.")
        );
    }
    if (!bTerminal && Stage.SuccessStageId.IsNone())
    {
        AddMissionError(
            OutErrors,
            TEXT("Non-terminal stages require an explicit success transition.")
        );
    }
    AppendTransitionTargetError(
        Stages,
        Stage.SuccessStageId,
        TEXT("A success transition references an unknown stage."),
        OutErrors
    );
    AppendTransitionTargetError(
        Stages,
        Stage.FailureStageId,
        TEXT("A failure transition references an unknown stage."),
        OutErrors
    );
}

static bool AddReachableStage(
    const FName& StageId,
    TSet<FName>& Reachable
)
{
    if (StageId.IsNone() || Reachable.Contains(StageId))
    {
        return false;
    }
    Reachable.Add(StageId);
    return true;
}

static bool ExpandReachableStages(
    const TArray<FSharMissionStageDefinition>& Stages,
    TSet<FName>& Reachable
)
{
    bool bChanged = false;
    for (const FSharMissionStageDefinition& Stage : Stages)
    {
        if (!Reachable.Contains(Stage.StageId))
        {
            continue;
        }
        bChanged = AddReachableStage(Stage.SuccessStageId, Reachable)
            || bChanged;
        bChanged = AddReachableStage(Stage.FailureStageId, Reachable)
            || bChanged;
    }
    return bChanged;
}

static void AppendReachabilityErrors(
    const USharMissionDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    TSet<FName> Reachable;
    Reachable.Add(Definition.InitialStageId);
    while (ExpandReachableStages(Definition.Stages, Reachable))
    {
    }
    for (const FSharMissionStageDefinition& Stage : Definition.Stages)
    {
        if (!Reachable.Contains(Stage.StageId))
        {
            AddMissionError(
                OutErrors,
                TEXT("Mission contains an unreachable stage.")
            );
        }
    }
}

static void AppendRewardIdentityErrors(
    const FSharMissionRewardOperation& Reward,
    TSet<FName>& SeenOperationIds,
    TArray<FText>& OutErrors
)
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Reward.OperationId
    ))
    {
        AddMissionError(
            OutErrors,
            TEXT("Every reward requires a canonical idempotency identity.")
        );
    }
    if (SeenOperationIds.Contains(Reward.OperationId))
    {
        AddMissionError(
            OutErrors,
            TEXT("Reward operation identities must be unique.")
        );
    }
    SeenOperationIds.Add(Reward.OperationId);
}

static void AppendRewardValueErrors(
    const FSharMissionRewardOperation& Reward,
    TArray<FText>& OutErrors
)
{
    if (!USharMissionDefinition::IsSupportedRewardOperation(
        Reward.OperationKind
    ))
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission reward uses an unsupported operation kind.")
        );
    }
    if (Reward.TargetId.IsNone())
    {
        AddMissionError(
            OutErrors,
            TEXT("Reward operations require an explicit target identity.")
        );
    }
    if (Reward.Quantity <= 0)
    {
        AddMissionError(
            OutErrors,
            TEXT("Reward quantities must be positive.")
        );
    }
}

static void AppendRewardErrors(
    const TArray<FSharMissionRewardOperation>& Rewards,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenOperationIds;
    for (const FSharMissionRewardOperation& Reward : Rewards)
    {
        AppendRewardIdentityErrors(Reward, SeenOperationIds, OutErrors);
        AppendRewardValueErrors(Reward, OutErrors);
    }
}

static bool HasSuccessOutcome(
    const TArray<FSharMissionStageDefinition>& Stages
)
{
    return Algo::AnyOf(
        Stages,
        [](const FSharMissionStageDefinition& Stage)
        {
            return Stage.TerminalOutcome
                == ESharMissionTerminalOutcome::Success;
        }
    );
}

bool USharMissionDefinition::IsSupportedObjectiveKind(
    const FName& ObjectiveKind
)
{
    const TArray<FName> SupportedKinds = {
        FName(TEXT("talk")),
        FName(TEXT("dialogue")),
        FName(TEXT("cinematic")),
        FName(TEXT("enter_vehicle")),
        FName(TEXT("exit_vehicle")),
        FName(TEXT("enter_interior")),
        FName(TEXT("exit_interior")),
        FName(TEXT("travel")),
        FName(TEXT("collect")),
        FName(TEXT("item_pickup")),
        FName(TEXT("dumped_collectible")),
        FName(TEXT("deliver")),
        FName(TEXT("destroy")),
        FName(TEXT("hit_and_collect")),
        FName(TEXT("follow")),
        FName(TEXT("follow_and_collect")),
        FName(TEXT("race")),
        FName(TEXT("time_trial")),
        FName(TEXT("wager_entry")),
        FName(TEXT("timer")),
        FName(TEXT("avoid")),
        FName(TEXT("protect")),
        FName(TEXT("interact")),
        FName(TEXT("boss_phase")),
        FName(TEXT("buy_vehicle")),
        FName(TEXT("buy_costume")),
        FName(TEXT("action_sequence")),
    };
    return ContainsName(SupportedKinds, ObjectiveKind);
}

bool USharMissionDefinition::IsSupportedRewardOperation(
    const FName& OperationKind
)
{
    return USharProgressionState::IsSupportedOperation(OperationKind);
}

void USharMissionDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    if (!IsCanonicalIdentifier(ChapterId))
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission chapter identity must be canonical.")
        );
    }
    if (!IsCanonicalIdentifier(MissionClassId))
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission class identity must be canonical.")
        );
    }
    if (SequenceOrdinal < 0 || SaveSchemaVersion <= 0)
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission sequence and save-schema values are invalid.")
        );
    }
    if (Stages.IsEmpty())
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission definitions require at least one stage.")
        );
        return;
    }
    if (!ContainsStage(Stages, InitialStageId))
    {
        AddMissionError(
            OutErrors,
            TEXT("Initial mission stage must resolve inside the definition.")
        );
    }

    AppendObjectivePoliciesErrors(ObjectivePolicies, OutErrors);

    TSet<FName> SeenStageIds;
    int32 ExpectedOrder = 0;
    for (const FSharMissionStageDefinition& Stage : Stages)
    {
        AppendStageIdentityErrors(
            Stage,
            ExpectedOrder,
            ObjectivePolicies,
            SeenStageIds,
            OutErrors
        );
        AppendStageTransitionErrors(Stage, Stages, OutErrors);
        ++ExpectedOrder;
    }
    if (!HasSuccessOutcome(Stages))
    {
        AddMissionError(
            OutErrors,
            TEXT("Mission graph requires a success terminal outcome.")
        );
    }
    AppendReachabilityErrors(*this, OutErrors);
    AppendRewardErrors(RewardOperations, OutErrors);
}

FPrimaryAssetType
USharMissionDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharMission")};
}
