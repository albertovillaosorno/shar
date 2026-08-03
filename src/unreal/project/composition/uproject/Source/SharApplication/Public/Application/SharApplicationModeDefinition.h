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
//   - Shar application mode definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar application mode definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar application mode definition composition module.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharApplicationModeDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharApplicationModeKind : uint8
{
    Entry,
    Boot,
    FrontEnd,
    Loading,
    Active,
    Overlay,
    Suspension,
    Exit,
};

UENUM(BlueprintType)
enum class ESharApplicationWorldPolicy : uint8
{
    None,
    Prepare,
    Retain,
    Own,
    TearDown,
};

UENUM(BlueprintType)
enum class ESharApplicationProgressionPolicy : uint8
{
    None,
    ReadOnly,
    Durable,
};

UCLASS(BlueprintType)
class SHARAPPLICATION_API USharApplicationModeDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mode")
    ESharApplicationModeKind ModeKind = ESharApplicationModeKind::Active;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Graph")
    TArray<FName> AllowedPredecessorIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Graph")
    TArray<FName> AllowedSuccessorIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Plan")
    FName EntryPlanId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Plan")
    FName ExitPlanId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Readiness")
    TArray<FName> RequiredServiceIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Readiness")
    FName ReadinessBarrierId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Recovery")
    FName SuccessModeId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Recovery")
    FName RecoveryModeId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Overlay")
    FName ReturnModeId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "World")
    ESharApplicationWorldPolicy WorldPolicy =
        ESharApplicationWorldPolicy::None;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Progression")
    ESharApplicationProgressionPolicy ProgressionPolicy =
        ESharApplicationProgressionPolicy::None;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    bool bSupportsCancellation = true;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    bool bHasBoundedTimeout = true;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    bool bAllowsDegradedEntry = false;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    bool bDemonstrationMode = false;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Development")
    bool bDevelopmentOnly = false;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
