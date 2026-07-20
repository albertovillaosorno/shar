// File: SharSaveSchemaDefinition.h
// Path: src/uproject/Source/SharSave/Public/Save/SharSaveSchemaDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable portable save schema, section, content-requirement, and migration metadata only.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md
// LARGE-FILE owner=SharSave; reason=cohesive reflected portable-save schema contract;
// split=extract section schemas if field-level validation becomes independently authored;
// validation=validate.sh SharSave plus Unreal automation; review=2027-01.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharSaveSchemaDefinition.generated.h"

USTRUCT(BlueprintType)
struct SHARSAVE_API FSharSaveMigrationStep
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Migration")
    int32 SourceVersion = 0;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Migration")
    int32 TargetVersion = 0;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Migration")
    FName MigrationId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Migration")
    FString MigrationRevision;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Migration")
    bool bIdempotent = true;
};

UCLASS(BlueprintType)
class SHARSAVE_API USharSaveSchemaDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    static constexpr int32 DefaultMaximumContentRequirements = 256;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Schema")
    int32 CurrentSchemaVersion = 1;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Schema")
    TArray<FName> RequiredSectionIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Schema")
    int32 MaximumContentRequirements = DefaultMaximumContentRequirements;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Migration")
    TArray<FSharSaveMigrationStep> MigrationSteps;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Compatibility")
    bool bPreserveUnknownOptionalContent = true;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

    [[nodiscard]] bool CanMigrateFrom(int32 SourceVersion) const;

    [[nodiscard]] bool BuildMigrationPlan(
        int32 SourceVersion,
        TArray<FName>& OutMigrationIds
    ) const;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
