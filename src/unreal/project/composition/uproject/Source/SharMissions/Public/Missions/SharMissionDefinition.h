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

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharMissionDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharMissionTerminalOutcome : uint8
{
    None,
    Success,
    Failure,
    Abort,
};

USTRUCT(BlueprintType)
struct SHARMISSIONS_API FSharObjectivePolicyRow
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    FName PolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    FName ObjectiveKind;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    FName RouteId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    TArray<FName> TargetIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    FName StartTrigger;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    FName CompletionRule;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    FName FailureRule;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    FName RecoveryRule;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    FName NotorietyPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    FName CatchUpProfileId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    FName DropSequenceId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    FName PresentationProfileId;
};

USTRUCT(BlueprintType)
struct SHARMISSIONS_API FSharMissionStageDefinition
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    FName StageId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    int32 Order = 0;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    FName ObjectiveKind;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    FName ObjectivePolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    FName SuccessStageId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    FName FailureStageId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    // jig-ignore-next-line: exact syntax is indivisible
    ESharMissionTerminalOutcome TerminalOutcome = ESharMissionTerminalOutcome::None;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    bool bCheckpoint = false;
};

USTRUCT(BlueprintType)
struct SHARMISSIONS_API FSharMissionRewardOperation
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Reward")
    FName OperationId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Reward")
    FName OperationKind;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Reward")
    FName TargetId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Reward")
    int32 Quantity = 1;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Reward")
    bool bPermanent = true;
};

UCLASS(BlueprintType)
class SHARMISSIONS_API USharMissionDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    FName ChapterId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    FName MissionClassId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    int32 SequenceOrdinal = 0;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    FName InitialStageId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    TArray<FSharMissionStageDefinition> Stages;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Objective")
    TArray<FSharObjectivePolicyRow> ObjectivePolicies;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Reward")
    TArray<FSharMissionRewardOperation> RewardOperations;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Save")
    int32 SaveSchemaVersion = 1;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

    // jig-ignore-next-line: exact syntax is indivisible
    [[nodiscard]] static bool IsSupportedObjectiveKind(const FName& ObjectiveKind);
    // jig-ignore-next-line: exact syntax is indivisible
    [[nodiscard]] static bool IsSupportedRewardOperation(const FName& OperationKind);

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
