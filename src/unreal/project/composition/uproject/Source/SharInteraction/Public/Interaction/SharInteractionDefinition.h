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
//   - Shar interaction definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar interaction definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar interaction definition composition module.

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
