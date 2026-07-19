// File: SharMissionRuntime.h
// Path: src/uproject/Source/SharMissions/Public/Missions/SharMissionRuntime.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient mission execution and snapshot projection; no world, reward, or presentation authority.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#pragma once

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"
#include "Missions/SharMissionDefinition.h"
#include "UObject/ObjectPtr.h"

#include "SharMissionRuntime.generated.h"

UENUM(BlueprintType)
enum class ESharMissionRuntimeState : uint8
{
    Idle,
    Active,
    Succeeded,
    Failed,
    Aborted,
};

USTRUCT(BlueprintType)
struct SHARMISSIONS_API FSharMissionRuntimeSnapshot
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Mission")
    FPrimaryAssetId MissionId;

    UPROPERTY(BlueprintReadOnly, Category = "Mission")
    FName StageId;

    UPROPERTY(BlueprintReadOnly, Category = "Mission")
    ESharMissionRuntimeState State = ESharMissionRuntimeState::Idle;

    UPROPERTY(BlueprintReadOnly, Category = "Mission")
    int32 SaveSchemaVersion = 1;
};

UCLASS(BlueprintType)
class SHARMISSIONS_API USharMissionRuntime final : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Mission")
    bool StartMission(USharMissionDefinition* Definition);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Mission")
    bool ResolveObjective(bool bSucceeded);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Mission")
    bool AbortMission();

    UFUNCTION(BlueprintPure, Category = "SHAR|Mission")
    [[nodiscard]] ESharMissionRuntimeState GetState() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Mission")
    [[nodiscard]] FName GetActiveStageId() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Mission")
    [[nodiscard]] FText GetLastError() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Mission")
    [[nodiscard]] FSharMissionRuntimeSnapshot CreateSnapshot() const;

private:
    UPROPERTY(Transient)
    TObjectPtr<USharMissionDefinition> ActiveDefinition;

    UPROPERTY(Transient)
    FName ActiveStageId;

    UPROPERTY(Transient)
    ESharMissionRuntimeState State = ESharMissionRuntimeState::Idle;

    UPROPERTY(Transient)
    FText LastError;

    [[nodiscard]] const FSharMissionStageDefinition* FindStage(
        const FName& StageId
    ) const;
    bool MoveToStage(const FName& StageId);
    void SetFailure(const TCHAR* Message);
};
