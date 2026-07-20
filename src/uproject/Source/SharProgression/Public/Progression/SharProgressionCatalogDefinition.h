// File: SharProgressionCatalogDefinition.h
// Path: src/uproject/Source/SharProgression/Public/Progression/SharProgressionCatalogDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable progression operation, quantity, batch, and snapshot-schema policy only; no profile state, save I/O, UI, or gameplay effects.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharProgression; reason=cohesive reflected progression catalog contract;
// split=extract domain-specific catalog rows when currency and collectibles require independent assets;
// validation=validate.sh SharProgression plus Unreal automation; review=2027-01.

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
