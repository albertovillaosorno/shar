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
//   - Shar progression state composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar progression state composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar progression state composition module.

#pragma once

#include "CoreMinimal.h"

#include "SharProgressionState.generated.h"

UENUM(BlueprintType)
enum class ESharRewardApplyResult : uint8
{
    Applied,
    AlreadyApplied,
    InvalidRequest,
    UnsupportedOperation,
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharRewardRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Reward")
    FName TransactionId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Reward")
    FName OperationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Reward")
    FName TargetId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Reward")
    int32 Quantity = 1;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Reward")
    bool bPermanent = true;
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharProgressionValue
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Progression")
    FName OperationId;

    UPROPERTY(BlueprintReadOnly, Category = "Progression")
    FName TargetId;

    UPROPERTY(BlueprintReadOnly, Category = "Progression")
    int32 Quantity = 0;
};

UCLASS(BlueprintType)
class SHARPROGRESSION_API USharProgressionState final : public UObject
{
    GENERATED_BODY()

public:
    bool InitializeSnapshot(
        const TArray<FSharProgressionValue>& InValues,
        const TArray<FName>& InAppliedPermanentTransactions
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Progression")
    ESharRewardApplyResult ApplyReward(const FSharRewardRequest& Request);

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] int32 GetQuantity(
        const FName& OperationId,
        const FName& TargetId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] bool HasAppliedTransaction(
        const FName& TransactionId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] const TArray<FSharProgressionValue>& GetValues() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] const TArray<FName>& GetAppliedTransactions() const;

    [[nodiscard]] static bool IsSupportedOperation(
        const FName& OperationId
    );

    [[nodiscard]] static bool UsesSetSemantics(const FName& OperationId);

private:
    UPROPERTY(Transient)
    TArray<FSharProgressionValue> Values;

    UPROPERTY(Transient)
    TArray<FName> AppliedPermanentTransactions;

    [[nodiscard]] FSharProgressionValue* FindValue(
        const FName& OperationId,
        const FName& TargetId
    );
    [[nodiscard]] const FSharProgressionValue* FindValue(
        const FName& OperationId,
        const FName& TargetId
    ) const;
    [[nodiscard]] ESharRewardApplyResult ValidateRewardRequest(
        const FSharRewardRequest& Request
    ) const;
    void ApplyRewardValue(const FSharRewardRequest& Request);
};
