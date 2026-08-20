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
//   - Shar save schema catalog subsystem composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar save schema catalog subsystem composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar save schema catalog subsystem composition module.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "Save/SharSaveSchemaDefinition.h"
#include "SharSaveSchemaCatalogSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharSaveSchemaCatalogResult : uint8
{
    Accepted,
    InvalidRevision,
    InvalidDefinition,
    DuplicateSchema,
    AlreadyActive,
    EmptyCatalog,
};

UCLASS()
class SHARSAVE_API USharSaveSchemaCatalogSubsystem final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Save")
    bool ConfigureRevision(const FString& InCatalogRevision);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Save")
    ESharSaveSchemaCatalogResult RegisterSchema(
        USharSaveSchemaDefinition* Definition
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Save")
    ESharSaveSchemaCatalogResult Activate();

    UFUNCTION(BlueprintPure, Category = "SHAR|Save")
    [[nodiscard]] bool IsActive() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Save")
    [[nodiscard]] int32 GetSchemaCount() const;

    [[nodiscard]] const USharSaveSchemaDefinition* FindSchema(
        const FName& SchemaId
    ) const;

    [[nodiscard]] const FString& GetCatalogRevision() const;

private:
    UPROPERTY(Transient)
    FString CatalogRevision;

    UPROPERTY(Transient)
    TArray<TObjectPtr<USharSaveSchemaDefinition>> Definitions;

    UPROPERTY(Transient)
    bool bActive = false;
};
