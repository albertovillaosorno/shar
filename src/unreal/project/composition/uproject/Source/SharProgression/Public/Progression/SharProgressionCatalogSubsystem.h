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
//   - Shar progression catalog subsystem composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar progression catalog subsystem composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar progression catalog subsystem composition module.

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
