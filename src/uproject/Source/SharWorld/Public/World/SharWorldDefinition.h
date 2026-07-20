// File: SharWorldDefinition.h
// Path: src/uproject/Source/SharWorld/Public/World/SharWorldDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: connected-world region, Runtime Data Layer, HLOD, grid, and day-cycle contracts; no actor labels or package paths as identity.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharWorldDefinition.generated.h"

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharWorldRegionDefinition
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World")
    FName RegionId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Streaming")
    FName RuntimeGridId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Streaming")
    FName HlodProfileId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Streaming")
    bool bAlwaysLoaded = false;
};

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharDataLayerDefinition
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Data Layer")
    FName LayerId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Data Layer")
    TArray<FName> RequiredLayerIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Data Layer")
    bool bRuntime = true;
};

UCLASS(BlueprintType)
class SHARWORLD_API USharWorldDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    static constexpr float DefaultDayLengthSeconds = 1440.0F;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World")
    TArray<FSharWorldRegionDefinition> Regions;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Data Layer")
    TArray<FSharDataLayerDefinition> DataLayers;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Time")
    float DayLengthSeconds = DefaultDayLengthSeconds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Streaming")
    bool bUsesWorldPartition = true;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Streaming")
    bool bSupportsPredictiveStreaming = true;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
