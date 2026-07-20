// File: SharActionDefinitionTests.cpp
// Path: src/uproject/Source/SharAction/Private/Tests/SharActionDefinitionTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient action and sequence definition validation tests only.
// ADR: docs/adr/unreal/runtime/typed-state-tree-action-sequences.md
// LARGE-FILE owner=SharAction; reason=two cohesive definition-validation scenarios;
// split=separate sequence tests if parameter binding schemas expand;
// validation=validate.sh SharAction plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "Action/SharActionDefinition.h"
#include "Action/SharActionSequenceDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"
#include "Misc/AutomationTest.h"

static void FillPrimaryActionBase(
    USharPrimaryContentDefinition& Definition,
    const FName& CanonicalId,
    const TCHAR* Revision
)
{
    Definition.CanonicalId = CanonicalId;
    Definition.DisplayName = FText::FromString(CanonicalId.ToString());
    Definition.SourcePackageIds = {FName(TEXT("action_contract"))};
    Definition.RevisionToken = Revision;
    Definition.ValidationProfile = FName(TEXT("action_definition_v1"));
    Definition.OwningFeature = FName(TEXT("base"));
}

static void FillActionBase(USharActionDefinition& Definition)
{
    FillPrimaryActionBase(
        Definition,
        FName(TEXT("enter_vehicle_position")),
        TEXT("sha256:action_v1")
    );
    Definition.ExecutionKind = ESharActionExecutionKind::PositionCharacter;
    Definition.ParameterSchemaId = FName(TEXT("position_character_v1"));
    Definition.PreconditionsPolicyId = FName(TEXT("actor_ready_v1"));
    Definition.CancellationPolicyId = FName(TEXT("compensate_before_commit"));
    Definition.VerificationPolicyId = FName(TEXT("actor_at_slot_v1"));
    Definition.PresentationPolicyId = FName(TEXT("vehicle_entry_presentation_v1"));
    Definition.ExecutorId = FName(TEXT("position_character_executor_v1"));

    FSharActionResourceClaim MovementClaim;
    MovementClaim.ResourceId = FName(TEXT("character_movement"));
    MovementClaim.Access = ESharActionResourceAccess::Exclusive;
    Definition.RequiredResources.Add(MovementClaim);
}

namespace
{
constexpr int32 InvalidSparseOrdinal = 3;

struct FSharActionSequenceStepFixture
{
    FName StepId;
    FPrimaryAssetId ActionId;
    int32 Ordinal = 0;
};
} // namespace

static FSharActionSequenceStep MakeSequenceStep(
    const FSharActionSequenceStepFixture& Fixture
)
{
    FSharActionSequenceStep Step;
    Step.StepId = Fixture.StepId;
    Step.ActionId = Fixture.ActionId;
    Step.ExpectedActionRevision = TEXT("sha256:action_v1");
    Step.ParameterBindingId = FName(TEXT("vehicle_entry_binding_v1"));
    Step.Ordinal = Fixture.Ordinal;
    return Step;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharActionDefinitionValidationTest,
    "SHAR.Action.Definition.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharActionSequenceValidationTest,
    "SHAR.Action.Sequence.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharActionDefinitionValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Definition = NewObject<USharActionDefinition>();
    FillActionBase(*Definition);

    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid action definition passes"), Errors.IsEmpty());

    const FSharActionResourceClaim DuplicateClaim =
        Definition->RequiredResources.Last();
    Definition->RequiredResources.Add(DuplicateClaim);
    Errors.Reset();
    Definition->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Duplicate resource claims are rejected"),
        Errors.IsEmpty()
    );
    return true;
}

bool FSharActionSequenceValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Definition = NewObject<USharActionSequenceDefinition>();
    FillPrimaryActionBase(
        *Definition,
        FName(TEXT("enter_vehicle_sequence")),
        TEXT("sha256:sequence_v1")
    );
    Definition->StateTreeTemplateId = FName(TEXT("ordered_action_sequence_v1"));
    Definition->VerificationPolicyId = FName(TEXT("vehicle_entry_complete_v1"));
    Definition->RequiredContextIds = {
        FName(TEXT("actor")),
        FName(TEXT("vehicle")),
        FName(TEXT("interaction")),
    };
    Definition->Steps.Add(MakeSequenceStep({
        .StepId = FName(TEXT("position_actor")),
        .ActionId = {
            FPrimaryAssetType(TEXT("SharAction")),
            FName(TEXT("enter_vehicle_position")),
        },
        .Ordinal = 0,
    }));
    Definition->Steps.Add(MakeSequenceStep({
        .StepId = FName(TEXT("commit_entry")),
        .ActionId = {
            FPrimaryAssetType(TEXT("SharAction")),
            FName(TEXT("enter_vehicle_commit")),
        },
        .Ordinal = 1,
    }));

    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    TestTrue(TEXT("Dense action sequence passes"), Errors.IsEmpty());

    Definition->Steps.Last().Ordinal = InvalidSparseOrdinal;
    Errors.Reset();
    Definition->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Non-dense action sequence is rejected"),
        Errors.IsEmpty()
    );
    return true;
}

#endif
