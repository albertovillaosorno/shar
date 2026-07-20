// File: SharGameInstanceTests.cpp
// Path: src/uproject/Source/SharUI/Private/Tests/SharGameInstanceTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient startup-flow tests only; no widgets, maps, online services, or asset loading.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#if WITH_DEV_AUTOMATION_TESTS

#include "UI/SharGameInstance.h"

#include "Engine/DataAsset.h"
#include "Misc/AutomationTest.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharStartupFlowTest,
    "SHAR.UI.StartupFlow",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharStartupFlowTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* GameInstance = NewObject<USharGameInstance>();
    const FPrimaryAssetId ExperienceId(
        FPrimaryAssetType(TEXT("SharGameMode")),
        FName(TEXT("open_world_campaign"))
    );

    TestFalse(TEXT("Cannot skip boot"), GameInstance->CompleteBoot());
    TestTrue(TEXT("Boot starts"), GameInstance->StartBootFlow());
    TestTrue(TEXT("Boot completes"), GameInstance->CompleteBoot());
    TestTrue(TEXT("Main menu opens"), GameInstance->OpenMainMenu());
    TestTrue(
        TEXT("Experience load begins"),
        GameInstance->BeginExperienceLoad(ExperienceId)
    );
    TestTrue(TEXT("Gameplay begins"), GameInstance->EnterGameplay());
    TestTrue(TEXT("Gameplay returns to menu"), GameInstance->ReturnToMenu());
    TestTrue(
        TEXT("Flow ends in main menu"),
        GameInstance->GetStartupState() == ESharStartupState::MainMenu
    );
    return true;
}

#endif
