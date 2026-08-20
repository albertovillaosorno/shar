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
//   - Shar game instance tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar game instance tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar game instance tests composition module.

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
