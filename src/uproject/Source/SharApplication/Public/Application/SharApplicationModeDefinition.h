// File: SharApplicationModeDefinition.h
// Path: src/uproject/Source/SharApplication/Public/Application/SharApplicationModeDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable application-mode identity, graph, readiness, recovery, and ownership policy only.
// Specification: docs/technical/unreal/application-lifecycle-and-mode-runtime.md
// LARGE-FILE owner=SharApplication; reason=cohesive reflected mode-definition schema;
// split=extract lease policy if ownership declarations become independently versioned assets;
// validation=validate.sh SharApplication plus Unreal automation; review=2027-01.

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
