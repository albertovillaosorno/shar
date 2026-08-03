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
//   - Shar action sequence definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar action sequence definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar action sequence definition composition module.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"
#include "Engine/DataAsset.h"

#include "SharActionSequenceDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharActionSequenceFailurePolicy : uint8
{
    Abort,
    Compensate,
    Fallback,
    Retry,
    Continue,
};

USTRUCT(BlueprintType)
struct SHARACTION_API FSharActionSequenceStep
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Identity")
    FName StepId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Action")
    FPrimaryAssetId ActionId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Action")
    FString ExpectedActionRevision;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Action")
    FName ParameterBindingId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Ordering")
    int32 Ordinal = 0;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Failure")
    ESharActionSequenceFailurePolicy FailurePolicy =
        ESharActionSequenceFailurePolicy::Abort;
};

UCLASS(BlueprintType)
class SHARACTION_API USharActionSequenceDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    static constexpr float DefaultSequenceTimeoutSeconds = 30.0F;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "StateTree")
    FName StateTreeTemplateId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Sequence")
    TArray<FSharActionSequenceStep> Steps;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Timeout")
    float SequenceTimeoutSeconds = DefaultSequenceTimeoutSeconds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Timeout")
    bool bAllowsNoTimeout = false;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Context")
    TArray<FName> RequiredContextIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Verification")
    FName VerificationPolicyId;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
