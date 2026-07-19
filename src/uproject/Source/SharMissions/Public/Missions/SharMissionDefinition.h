// File: SharMissionDefinition.h
// Path: src/uproject/Source/SharMissions/Public/Missions/SharMissionDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: mission identity, stages, rewards, and load-free validation; no arbitrary executable scripts.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

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
    FName SuccessStageId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
    FName FailureStageId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mission")
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

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Reward")
    TArray<FSharMissionRewardOperation> RewardOperations;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Save")
    int32 SaveSchemaVersion = 1;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

    [[nodiscard]] static bool IsSupportedObjectiveKind(const FName& ObjectiveKind);
    [[nodiscard]] static bool IsSupportedRewardOperation(const FName& OperationKind);

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
