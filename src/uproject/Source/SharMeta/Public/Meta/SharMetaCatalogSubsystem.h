// File: SharMetaCatalogSubsystem.h
// Path: src/uproject/Source/SharMeta/Public/Meta/SharMetaCatalogSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: shared meta-catalog registration, validation, immutable activation, and lookup only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "Meta/SharMetaCatalogDefinition.h"
#include "SharMetaCatalogSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharMetaCatalogResult : uint8
{
    Accepted,
    InvalidRevision,
    InvalidDefinition,
    DuplicateCatalog,
    AlreadyActive,
    EmptyCatalog,
};

UCLASS()
class SHARMETA_API USharMetaCatalogSubsystem final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Meta")
    bool ConfigureRevision(const FString& InCatalogRevision);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Meta")
    ESharMetaCatalogResult RegisterCatalog(
        USharMetaCatalogDefinition* Definition
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Meta")
    ESharMetaCatalogResult Activate();

    UFUNCTION(BlueprintPure, Category = "SHAR|Meta")
    [[nodiscard]] bool IsActive() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Meta")
    [[nodiscard]] int32 GetCatalogCount() const;

    [[nodiscard]] const USharMetaCatalogDefinition* FindCatalog(
        const FName& CatalogId
    ) const;

    [[nodiscard]] const FSharCheatDefinition* FindCheatBySequence(
        const FName& CatalogId,
        const TArray<ESharCheatInputToken>& InputTokens
    ) const;

    [[nodiscard]] const FString& GetCatalogRevision() const;

private:
    UPROPERTY(Transient)
    FString CatalogRevision;

    UPROPERTY(Transient)
    TArray<TObjectPtr<USharMetaCatalogDefinition>> Definitions;

    UPROPERTY(Transient)
    bool bActive = false;
};
