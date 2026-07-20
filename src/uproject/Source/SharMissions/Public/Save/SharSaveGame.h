// File: SharSaveGame.h
// Path: src/uproject/Source/SharMissions/Public/Save/SharSaveGame.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: canonical save identities, progression state, and mod-state compatibility; no raw UObject paths or server-state merging.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#pragma once

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"
#include "GameFramework/SaveGame.h"
#include "Progression/SharProgressionState.h"

#include "SharSaveGame.generated.h"

USTRUCT(BlueprintType)
struct SHARMISSIONS_API FSharNamespacedModSaveState
{
    GENERATED_BODY()

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Mod")
    FName NamespaceId;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Mod")
    int32 SchemaVersion = 1;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Mod")
    FString StateRevision;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Mod")
    bool bRequired = false;
};

UCLASS(BlueprintType)
class SHARMISSIONS_API USharSaveGame final : public USaveGame
{
    GENERATED_BODY()

public:
    static constexpr int32 CurrentSaveSchemaVersion = 1;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Save")
    int32 SaveSchemaVersion = CurrentSaveSchemaVersion;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Save")
    FString TransactionRevision;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Save")
    FPrimaryAssetId GameModeId;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Mission")
    FPrimaryAssetId ActiveMissionId;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Mission")
    FName ActiveMissionStageId;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Selection")
    FPrimaryAssetId CurrentCharacterId;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Selection")
    FPrimaryAssetId CurrentVehicleId;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Progression")
    TArray<FSharProgressionValue> ProgressionValues;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Progression")
    TArray<FName> AppliedPermanentTransactions;

    UPROPERTY(SaveGame, BlueprintReadWrite, Category = "Mod")
    TArray<FSharNamespacedModSaveState> ModStates;

    UFUNCTION(BlueprintCallable, Category = "SHAR|Save")
    void GatherValidationErrors(TArray<FText>& OutErrors) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Save")
    [[nodiscard]] static bool CanMigrateFrom(int32 SourceSchemaVersion);
};
