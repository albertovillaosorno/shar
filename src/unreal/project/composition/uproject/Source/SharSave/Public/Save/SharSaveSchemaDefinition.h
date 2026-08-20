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
//   - Shar save schema definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar save schema definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar save schema definition composition module.

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
