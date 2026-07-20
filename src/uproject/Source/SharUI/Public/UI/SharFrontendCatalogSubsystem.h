// File: SharFrontendCatalogSubsystem.h
// Path: src/uproject/Source/SharUI/Public/UI/SharFrontendCatalogSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: frontend catalog registration, cross-catalog validation, immutable activation, and lookup only.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"

// NOLINTNEXTLINE(llvm-include-order) -- Unreal requires the generated header last.
#include "UI/SharFrontendCatalogDefinition.h"
#include "SharFrontendCatalogSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharFrontendCatalogResult : uint8
{
    Accepted,
    InvalidRevision,
    InvalidRootCatalog,
    InvalidDefinition,
    DuplicateCatalog,
    DuplicateScreen,
    MissingRootCatalog,
    AlreadyActive,
    EmptyCatalog,
};

UCLASS()
class SHARUI_API USharFrontendCatalogSubsystem final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Frontend")
    bool Configure(
        const FString& InCatalogRevision,
        const FName& InRootCatalogId
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Frontend")
    ESharFrontendCatalogResult RegisterCatalog(
        USharFrontendCatalogDefinition* Definition
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Frontend")
    ESharFrontendCatalogResult Activate();

    UFUNCTION(BlueprintPure, Category = "SHAR|Frontend")
    [[nodiscard]] bool IsActive() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Frontend")
    [[nodiscard]] int32 GetCatalogCount() const;

    [[nodiscard]] const USharFrontendCatalogDefinition* FindCatalog(
        const FName& CatalogId
    ) const;

    [[nodiscard]] const FSharFrontendScreenDefinition* FindScreen(
        const FName& ScreenId
    ) const;

    [[nodiscard]] const FSharFrontendScreenDefinition* GetInitialScreen() const;

    [[nodiscard]] const FString& GetCatalogRevision() const;

    [[nodiscard]] const FName& GetRootCatalogId() const;

private:
    UPROPERTY(Transient)
    FString CatalogRevision;

    UPROPERTY(Transient)
    FName RootCatalogId;

    UPROPERTY(Transient)
    TArray<TObjectPtr<USharFrontendCatalogDefinition>> Definitions;

    UPROPERTY(Transient)
    bool bActive = false;
};
