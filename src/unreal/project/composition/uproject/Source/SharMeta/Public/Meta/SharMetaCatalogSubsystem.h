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
//   - Shar meta catalog subsystem composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar meta catalog subsystem composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar meta catalog subsystem composition module.

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
