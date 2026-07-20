// File: SharGameModeDefinition.h
// Path: src/uproject/Source/SharContent/Public/GameMode/SharGameModeDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: game mode composition identities and validation; no synchronous asset loading or concrete map classes.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"
#include "Engine/DataAsset.h"

#include "SharGameModeDefinition.generated.h"

UCLASS(BlueprintType)
class SHARCONTENT_API USharGameModeDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GameMode")
    FPrimaryAssetId WorldDefinitionId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GameMode")
    FPrimaryAssetId DefaultCharacterId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GameMode")
    FPrimaryAssetId DefaultPlatformProfileId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GameMode")
    TArray<FPrimaryAssetId> StartupMissionIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GameMode")
    TArray<FName> RequiredFeatureNamespaces;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Save")
    int32 SaveSchemaVersion = 1;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
