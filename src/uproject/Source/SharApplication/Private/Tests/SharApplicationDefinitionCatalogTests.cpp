// File: SharApplicationDefinitionCatalogTests.cpp
// Path: src/uproject/Source/SharApplication/Private/Tests/SharApplicationDefinitionCatalogTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient application-mode policy and catalog graph validation tests only.
// Specification: docs/technical/unreal/application-lifecycle-and-mode-runtime.md
// LARGE-FILE owner=SharApplication; reason=two cohesive definition and catalog validation scenarios;
// split=separate graph diagnostics if catalog policies expand;
// validation=validate.sh SharApplication plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharApplicationTestFixtures.h"

#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Application/SharApplicationModeDefinition.h"
#include "Engine/GameInstance.h"
#include "Misc/AutomationTest.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharApplicationDefinitionValidationTest,
    "SHAR.Application.Definition.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharApplicationCatalogGraphValidationTest,
    "SHAR.Application.Catalog.GraphValidation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharApplicationDefinitionValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharApplicationModeDefinition* Definition = MakeApplicationMode({
        .ModeId = FName(TEXT("loading_demo")),
        .ModeKind = ESharApplicationModeKind::Loading,
        .PredecessorIds = {FName(TEXT("front_end"))},
        .SuccessorIds = {FName(TEXT("demo"))},
        .RequiredServiceIds = {FName(TEXT("world_service"))},
        .SuccessModeId = FName(TEXT("demo")),
        .RecoveryModeId = FName(TEXT("front_end")),
        .ReturnModeId = FName(),
        .WorldPolicy = ESharApplicationWorldPolicy::Prepare,
        .ProgressionPolicy = ESharApplicationProgressionPolicy::ReadOnly,
        .bDemonstrationMode = true,
    });
    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid loading mode definition passes"), Errors.IsEmpty());

    Definition->ProgressionPolicy =
        ESharApplicationProgressionPolicy::Durable;
    Errors.Reset();
    Definition->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Demonstration mode with durable progression is rejected"),
        Errors.IsEmpty()
    );
    return true;
}

bool FSharApplicationCatalogGraphValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* ValidGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* ValidCatalog =
        MakeApplicationCatalog(
            *ValidGameInstance,
            ESharApplicationCatalogShape::Valid,
            false
        );
    TestTrue(
        TEXT("Complete reciprocal mode graph activates"),
        ValidCatalog->Activate() == ESharApplicationCatalogResult::Accepted
    );
    TestTrue(TEXT("Activated catalog is immutable"), ValidCatalog->IsActive());

    auto* BrokenGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* BrokenCatalog =
        MakeApplicationCatalog(
            *BrokenGameInstance,
            ESharApplicationCatalogShape::BrokenReciprocalEdge,
            false
        );
    TestTrue(
        TEXT("Non-reciprocal graph is rejected"),
        BrokenCatalog->Activate()
            == ESharApplicationCatalogResult::EdgeNotReciprocal
    );
    return true;
}

#endif
