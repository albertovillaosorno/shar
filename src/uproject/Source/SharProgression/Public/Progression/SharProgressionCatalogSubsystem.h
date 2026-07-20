// File: SharProgressionCatalogSubsystem.h
// Path: src/uproject/Source/SharProgression/Public/Progression/SharProgressionCatalogSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: progression catalog registration, validation, immutable activation, and lookup only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "Progression/SharProgressionCatalogDefinition.h"
#include "SharProgressionCatalogSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharProgressionCatalogResult : uint8
{
    Accepted,
    InvalidRevision,
    InvalidDefinition,
    DuplicateCatalog,
    AlreadyActive,
    EmptyCatalog,
};

UCLASS()
class SHARPROGRESSION_API USharProgressionCatalogSubsystem final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Progression")
    bool ConfigureRevision(const FString& InCatalogRevision);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Progression")
    ESharProgressionCatalogResult RegisterCatalog(
        USharProgressionCatalogDefinition* Definition
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Progression")
    ESharProgressionCatalogResult Activate();

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] bool IsActive() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] int32 GetCatalogCount() const;

    [[nodiscard]] const USharProgressionCatalogDefinition* FindCatalog(
        const FName& CatalogId
    ) const;

    [[nodiscard]] const FString& GetCatalogRevision() const;

private:
    UPROPERTY(Transient)
    FString CatalogRevision;

    UPROPERTY(Transient)
    TArray<TObjectPtr<USharProgressionCatalogDefinition>> Definitions;

    UPROPERTY(Transient)
    bool bActive = false;
};
