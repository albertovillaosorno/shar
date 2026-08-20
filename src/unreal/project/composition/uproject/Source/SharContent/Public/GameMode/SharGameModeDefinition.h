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
//   - Shar game mode definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar game mode definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar game mode definition composition module.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"
#include "Engine/DataAsset.h"

#include "SharGameModeDefinition.generated.h"

UCLASS(BlueprintType)
class SHARCONTENT_API USharGameModeDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GameMode")
    FPrimaryAssetId WorldDefinitionId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GameMode")
    FPrimaryAssetId DefaultCharacterId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GameMode")
    FPrimaryAssetId DefaultPlatformProfileId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GameMode")
    TArray<FPrimaryAssetId> StartupMissionIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GameMode")
    TArray<FName> RequiredFeatureNamespaces;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Save")
    int32 SaveSchemaVersion = 1;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
