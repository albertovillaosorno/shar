// File: SharInteractionDefinition.h
// Path: src/uproject/Source/SharInteraction/Public/Interaction/SharInteractionDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable interaction identity, prompt, eligibility, execution, persistence, and verification policy only.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharInteraction; reason=cohesive reflected interaction definition;
// split=extract prompt or slot policies if independently versioned assets appear;
// validation=validate.sh SharInteraction plus Unreal automation; review=2027-01.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"
#include "GameplayTagContainer.h"

#include "SharInteractionDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharInteractionExecutionKind : uint8
{
    MissionDialogue,
    EnterInterior,
    EnterVehicle,
    SummonVehicle,
    PropAttach,
    PropToggle,
    PropReverse,
    PropPlayOnce,
    PropPlayLoop,
    PropAutoPlay,
    PropAutoInOut,
    DestroyProp,
    VendingMachine,
    PrankPhone,
    Doorbell,
    OpenDoor,
    TalkFood,
    TalkCollectible,
    Collectible,
    RepairPickup,
    NitroPickup,
    Teleport,
    PurchaseVehicle,
    PurchaseCostume,
    GenericEvent,
};

UENUM(BlueprintType)
enum class ESharInteractionInputPolicy : uint8
{
    ManualPress,
    AutomaticEnter,
    AutomaticExit,
    PassivePickup,
};

UENUM(BlueprintType)
enum class ESharInteractionPersistencePolicy : uint8
{
    None,
    Session,
    Level,
    Profile,
    PermanentCollection,
};

UENUM(BlueprintType)
enum class ESharInteractionCancellationPolicy : uint8
{
    BeforeCommit,
    CompensatedAfterPrepare,
    UninterruptibleAfterCommit,
};

UCLASS(BlueprintType)
class SHARINTERACTION_API USharInteractionDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Interaction")
    ESharInteractionExecutionKind ExecutionKind =
        ESharInteractionExecutionKind::GenericEvent;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Interaction")
    ESharInteractionInputPolicy InputPolicy =
        ESharInteractionInputPolicy::ManualPress;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Interaction")
    int32 Priority = 0;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Classification")
    FGameplayTagContainer InteractionTags;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Prompt")
    FName PromptTextId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Prompt")
    FName PromptIconId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Prompt")
    FName AccessibilityDescriptionId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Eligibility")
    FName EligibilityPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Slot")
    FName SlotPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Presentation")
    FName PresentationPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Effect")
    FName EffectPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Persistence")
    ESharInteractionPersistencePolicy PersistencePolicy =
        ESharInteractionPersistencePolicy::None;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Cooldown")
    FName CooldownPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Cancellation")
    ESharInteractionCancellationPolicy CancellationPolicy =
        ESharInteractionCancellationPolicy::BeforeCommit;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Verification")
    FName VerificationPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Execution")
    FName ExecutorId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Slot")
    bool bRequiresExclusiveSlot = false;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
