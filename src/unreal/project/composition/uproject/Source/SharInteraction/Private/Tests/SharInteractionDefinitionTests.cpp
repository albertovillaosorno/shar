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
//   - Shar interaction definition tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar interaction definition tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar interaction definition tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "Interaction/SharInteractionDefinition.h"

#include "Misc/AutomationTest.h"

static void FillInteractionDefinitionBase(
    USharInteractionDefinition& Definition
)
{
    Definition.CanonicalId = FName(TEXT("enter_family_sedan"));
    Definition.DisplayName = FText::FromString(TEXT("Enter family sedan"));
    Definition.SourcePackageIds = {FName(TEXT("interaction_contract"))};
    Definition.RevisionToken = TEXT("sha256:interaction_v1");
    Definition.ValidationProfile = FName(TEXT("interaction_definition_v1"));
    Definition.OwningFeature = FName(TEXT("base"));
    Definition.ExecutionKind = ESharInteractionExecutionKind::EnterVehicle;
    Definition.InputPolicy = ESharInteractionInputPolicy::ManualPress;
    Definition.PromptTextId = FName(TEXT("prompt_enter_vehicle"));
    Definition.PromptIconId = FName(TEXT("icon_interact"));
    Definition.AccessibilityDescriptionId =
        FName(TEXT("accessibility_enter_vehicle"));
    // jig-ignore-next-line: exact syntax is indivisible
    Definition.EligibilityPolicyId = FName(TEXT("vehicle_entry_eligibility_v1"));
    Definition.SlotPolicyId = FName(TEXT("vehicle_driver_slot_v1"));
    // jig-ignore-next-line: exact syntax is indivisible
    Definition.PresentationPolicyId = FName(TEXT("vehicle_entry_presentation_v1"));
    Definition.EffectPolicyId = FName(TEXT("vehicle_entry_effect_v1"));
    Definition.CooldownPolicyId = FName(TEXT("no_cooldown"));
    Definition.VerificationPolicyId = FName(TEXT("vehicle_seat_occupied_v1"));
    Definition.ExecutorId = FName(TEXT("enter_vehicle_executor_v1"));
    Definition.bRequiresExclusiveSlot = true;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharInteractionDefinitionValidationTest,
    "SHAR.Interaction.Definition.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharInteractionDefinitionValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Definition = NewObject<USharInteractionDefinition>();
    FillInteractionDefinitionBase(*Definition);

    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid interaction definition passes"), Errors.IsEmpty());

    Definition->InputPolicy = ESharInteractionInputPolicy::PassivePickup;
    Errors.Reset();
    Definition->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Passive pickup with exclusive slot is rejected"),
        Errors.IsEmpty()
    );
    return true;
}

#endif
