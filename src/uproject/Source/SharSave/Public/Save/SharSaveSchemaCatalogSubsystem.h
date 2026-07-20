// File: SharSaveSchemaCatalogSubsystem.h
// Path: src/uproject/Source/SharSave/Public/Save/SharSaveSchemaCatalogSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: portable save schema registration, lookup, validation, and revision activation only.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md

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
