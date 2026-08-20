// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar frontend catalog subsystem composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar frontend catalog subsystem composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar frontend catalog subsystem composition module.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"

// jig-ignore-next-line: exact syntax is indivisible
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
