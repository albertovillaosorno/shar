// File: SharApplicationModeCatalogSubsystem.h
// Path: src/uproject/Source/SharApplication/Public/Application/SharApplicationModeCatalogSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: application-mode definition registration, graph validation, lookup, and revision activation only.
// Specification: docs/technical/unreal/application-lifecycle-and-mode-runtime.md
// LARGE-FILE owner=SharApplication; reason=cohesive reflected mode-catalog contract;
// split=extract diagnostics if graph findings become persistent artifacts;
// validation=validate.sh SharApplication plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "Application/SharApplicationModeDefinition.h"
#include "SharApplicationModeCatalogSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharApplicationCatalogResult : uint8
{
    Accepted,
    InvalidRevision,
    InvalidDefinition,
    DuplicateMode,
    AlreadyActive,
    EntryMissing,
    ExitMissing,
    EdgeMissing,
    EdgeNotReciprocal,
    UnreachableMode,
    LoadingTargetMissing,
};

UCLASS()
class SHARAPPLICATION_API USharApplicationModeCatalogSubsystem final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Application")
    bool ConfigureRevision(const FString& InCatalogRevision);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Application")
    ESharApplicationCatalogResult RegisterMode(
        USharApplicationModeDefinition* Definition
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Application")
    ESharApplicationCatalogResult Activate();

    UFUNCTION(BlueprintPure, Category = "SHAR|Application")
    [[nodiscard]] int32 GetModeCount() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Application")
    [[nodiscard]] bool IsActive() const;

    [[nodiscard]] const USharApplicationModeDefinition* FindMode(
        const FName& ModeId
    ) const;

    [[nodiscard]] bool IsTransitionAllowed(
        const FName& SourceModeId,
        const FName& TargetModeId
    ) const;

    [[nodiscard]] ESharApplicationCatalogResult ValidateGraph() const;

    [[nodiscard]] const FString& GetCatalogRevision() const;

private:
    UPROPERTY(Transient)
    FString CatalogRevision;

    UPROPERTY(Transient)
    TArray<TObjectPtr<USharApplicationModeDefinition>> Definitions;

    UPROPERTY(Transient)
    bool bActive = false;

    [[nodiscard]] const USharApplicationModeDefinition* FindModeByKind(
        ESharApplicationModeKind ModeKind
    ) const;
    [[nodiscard]] bool AreEdgesResolvable() const;
    [[nodiscard]] bool AreEdgesReciprocal() const;
    [[nodiscard]] bool AreLoadingTargetsResolvable() const;
    [[nodiscard]] bool IsEveryModeReachableFrom(
        const FName& EntryModeId
    ) const;
};
