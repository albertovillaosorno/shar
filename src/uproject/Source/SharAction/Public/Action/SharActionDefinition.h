// File: SharActionDefinition.h
// Path: src/uproject/Source/SharAction/Public/Action/SharActionDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable reusable action contract only; execution remains in registered native StateTree tasks.
// ADR: docs/adr/unreal/runtime/typed-state-tree-action-sequences.md
// LARGE-FILE owner=SharAction; reason=cohesive reflected action definition and resource claim schema;
// split=extract presentation policy if independently versioned assets appear;
// validation=validate.sh SharAction plus Unreal automation; review=2027-01.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"
#include "GameplayTagContainer.h"

#include "SharActionDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharActionExecutionKind : uint8
{
    Wait,
    PublishEvent,
    MoveCharacter,
    OrientCharacter,
    PositionCharacter,
    SnapToGround,
    PlayAnimation,
    HoldAnimation,
    SetLocomotion,
    OpenVehicleDoor,
    Jump,
    Dodge,
    React,
    Recover,
    CommitInteractionResult,
    RequestPresentation,
};

UENUM(BlueprintType)
enum class ESharActionResourceAccess : uint8
{
    Shared,
    Exclusive,
};

USTRUCT(BlueprintType)
struct SHARACTION_API FSharActionResourceClaim
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Resource")
    FName ResourceId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Resource")
    ESharActionResourceAccess Access = ESharActionResourceAccess::Exclusive;
};

UCLASS(BlueprintType)
class SHARACTION_API USharActionDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    static constexpr float DefaultTimeoutSeconds = 10.0F;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Action")
    ESharActionExecutionKind ExecutionKind = ESharActionExecutionKind::Wait;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Classification")
    FGameplayTagContainer GameplayTags;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Parameters")
    FName ParameterSchemaId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Resource")
    TArray<FSharActionResourceClaim> RequiredResources;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Precondition")
    FName PreconditionsPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Timeout")
    float TimeoutSeconds = DefaultTimeoutSeconds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Timeout")
    bool bAllowsNoTimeout = false;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Cancellation")
    FName CancellationPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Verification")
    FName VerificationPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Presentation")
    FName PresentationPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Execution")
    FName ExecutorId;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
