// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar application mode catalog subsystem composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar application mode catalog subsystem composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar application mode catalog subsystem composition module.

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
