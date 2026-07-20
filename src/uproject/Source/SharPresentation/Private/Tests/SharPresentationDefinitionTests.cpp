// File: SharPresentationDefinitionTests.cpp
// Path: src/uproject/Source/SharPresentation/Private/Tests/SharPresentationDefinitionTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient presentation-definition validation tests only.
// Specification: docs/technical/unreal/presentation-playback-runtime.md

#if WITH_DEV_AUTOMATION_TESTS

#include "Presentation/SharPresentationDefinition.h"

#include "Misc/AutomationTest.h"

static void FillPresentationDefinitionBase(
    USharPresentationDefinition& Definition
)
{
    Definition.CanonicalId = FName(TEXT("kwik_e_mart_intro"));
    Definition.DisplayName = FText::FromString(TEXT("Kwik-E-Mart intro"));
    Definition.SourcePackageIds = {FName(TEXT("presentation_contract"))};
    Definition.RevisionToken = TEXT("sha256:presentation_v1");
    Definition.ValidationProfile = FName(TEXT("presentation_definition_v1"));
    Definition.OwningFeature = FName(TEXT("base"));
    Definition.PresentationKind = ESharPresentationKind::Sequence;
    Definition.AssetSetId = FName(TEXT("kwik_e_mart_intro_assets"));
    Definition.OwnerPolicyId = FName(TEXT("interaction_owner_v1"));
    Definition.PlaybackPolicyId = FName(TEXT("play_once_v1"));
    Definition.ExclusivityPolicyId = FName(TEXT("cinematic_focus_v1"));
    Definition.SkipPolicy = ESharPresentationSkipPolicy::Immediate;
    Definition.TimePolicy = ESharPresentationTimePolicy::Sequence;
    Definition.CameraPolicyId = FName(TEXT("sequence_camera_v1"));
    Definition.CharacterLayerPolicyId = FName(TEXT("dialogue_layer_v1"));
    Definition.FallbackPolicyId = FName(TEXT("static_fade_fallback_v1"));
    Definition.ResultPolicyId = FName(TEXT("owner_terminal_result_v1"));
    Definition.TeardownPolicyId = FName(TEXT("restore_and_release_v1"));
    Definition.bRequiresScopedLeases = true;
    Definition.bHasCompleteReleasePath = true;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharPresentationDefinitionValidationTest,
    "SHAR.Presentation.Definition.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharPresentationDefinitionValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Definition = NewObject<USharPresentationDefinition>();
    FillPresentationDefinitionBase(*Definition);

    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid presentation definition passes"), Errors.IsEmpty());

    Definition->bHasCompleteReleasePath = false;
    Errors.Reset();
    Definition->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Presentation without complete release path is rejected"),
        Errors.IsEmpty()
    );
    return true;
}

#endif
