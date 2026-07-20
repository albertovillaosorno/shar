// File: SharInteractionDefinition.cpp
// Path: src/uproject/Source/SharInteraction/Private/Interaction/SharInteractionDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free interaction definition validation only; no candidate query or effects.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md

#include "Interaction/SharInteractionDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddInteractionDefinitionError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool IsCanonicalInteractionId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsCanonicalOrNone(const FName& Candidate)
{
    return Candidate.IsNone() || IsCanonicalInteractionId(Candidate);
}

static bool RequiresManualPrompt(
    const ESharInteractionInputPolicy InputPolicy
)
{
    return InputPolicy == ESharInteractionInputPolicy::ManualPress;
}

static bool IsAutomaticInput(
    const ESharInteractionInputPolicy InputPolicy
)
{
    return InputPolicy == ESharInteractionInputPolicy::AutomaticEnter
        || InputPolicy == ESharInteractionInputPolicy::AutomaticExit
        || InputPolicy == ESharInteractionInputPolicy::PassivePickup;
}

static void AppendRequiredIdentityErrors(
    const USharInteractionDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalid =
        !IsCanonicalInteractionId(Definition.EligibilityPolicyId)
        || !IsCanonicalInteractionId(Definition.PresentationPolicyId)
        || !IsCanonicalInteractionId(Definition.EffectPolicyId)
        || !IsCanonicalInteractionId(Definition.CooldownPolicyId)
        || !IsCanonicalInteractionId(Definition.VerificationPolicyId)
        || !IsCanonicalInteractionId(Definition.ExecutorId);
    if (bInvalid)
    {
        AddInteractionDefinitionError(
            OutErrors,
            TEXT("Interaction execution and policy identities must be canonical.")
        );
    }
}

static void AppendOptionalIdentityErrors(
    const USharInteractionDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalid =
        !IsCanonicalOrNone(Definition.PromptTextId)
        || !IsCanonicalOrNone(Definition.PromptIconId)
        || !IsCanonicalOrNone(Definition.AccessibilityDescriptionId)
        || !IsCanonicalOrNone(Definition.SlotPolicyId);
    if (bInvalid)
    {
        AddInteractionDefinitionError(
            OutErrors,
            TEXT("Optional interaction prompt and slot identities must be canonical when present.")
        );
    }
}

static void AppendPromptAndSlotErrors(
    const USharInteractionDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (RequiresManualPrompt(Definition.InputPolicy)
        && Definition.PromptTextId.IsNone())
    {
        AddInteractionDefinitionError(
            OutErrors,
            TEXT("Manual interactions require a prompt text identity.")
        );
    }
    if (Definition.bRequiresExclusiveSlot && Definition.SlotPolicyId.IsNone())
    {
        AddInteractionDefinitionError(
            OutErrors,
            TEXT("Exclusive interactions require a slot policy.")
        );
    }
    if (IsAutomaticInput(Definition.InputPolicy)
        && Definition.bRequiresExclusiveSlot)
    {
        AddInteractionDefinitionError(
            OutErrors,
            TEXT("Automatic and passive interactions cannot require an exclusive authored use slot.")
        );
    }
}

static void AppendPersistenceErrors(
    const USharInteractionDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalidGenericPersistence =
        Definition.ExecutionKind == ESharInteractionExecutionKind::GenericEvent
        && Definition.PersistencePolicy
            == ESharInteractionPersistencePolicy::PermanentCollection;
    if (bInvalidGenericPersistence)
    {
        AddInteractionDefinitionError(
            OutErrors,
            TEXT("Generic events cannot own permanent collection persistence.")
        );
    }
}

void USharInteractionDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    AppendRequiredIdentityErrors(*this, OutErrors);
    AppendOptionalIdentityErrors(*this, OutErrors);
    AppendPromptAndSlotErrors(*this, OutErrors);
    AppendPersistenceErrors(*this, OutErrors);
}

FPrimaryAssetType USharInteractionDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharInteraction")};
}
