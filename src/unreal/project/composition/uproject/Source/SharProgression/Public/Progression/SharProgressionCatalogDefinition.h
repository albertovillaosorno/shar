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
//   - Shar progression catalog definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar progression catalog definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar progression catalog definition composition module.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharProgressionCatalogDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharProgressionValuePolicy : uint8
{
    Additive,
    SetOnce,
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharProgressionOperationDefinition
{
    GENERATED_BODY()

    static constexpr int32 DefaultMaximumQuantity = 2147483647;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Operation")
    FName OperationId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Operation")
    ESharProgressionValuePolicy ValuePolicy =
        ESharProgressionValuePolicy::Additive;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Operation")
    int32 MaximumQuantity = DefaultMaximumQuantity;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Operation")
    bool bPermanentAllowed = true;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Operation")
    bool bTransientAllowed = false;
};

UCLASS(BlueprintType)
class SHARPROGRESSION_API USharProgressionCatalogDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    static constexpr int32 DefaultMaximumMutationOperations = 64;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Schema")
    int32 SnapshotSchemaVersion = 1;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mutation")
    int32 MaximumMutationOperations = DefaultMaximumMutationOperations;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Operation")
    TArray<FSharProgressionOperationDefinition> Operations;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

    [[nodiscard]] const FSharProgressionOperationDefinition* FindOperation(
        const FName& OperationId
    ) const;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
