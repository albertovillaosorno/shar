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
//   - Native gameplay-framework bootstrap contract tests.
// - Must-Not:
//   - Depend on final character assets, movement tuning, or map mutations.
// - Allows:
//   - Class-default and project-settings validation for native framework types.
// - Split-When:
//   - World spawning or possession requires a separate integration fixture.
// - Merge-When:
//   - Another suite proves the identical native framework bootstrap contract.
// - Summary:
//   - Verifies project-owned Game Mode, controller, state, and Pawn selection.
// - Description:
//   - Proves neutral class defaults without asserting future gameplay tuning.
// - Usage:
//   - Runs through Unreal Automation without imported content.
// - Defaults:
//   - Reads class defaults and configured global Game Mode only.
//

//! Native gameplay-framework bootstrap contract tests.

#if WITH_DEV_AUTOMATION_TESTS

#include "Gameplay/SharGameMode.h"

#include "GameMapsSettings.h"
#include "Gameplay/SharGameState.h"
#include "Gameplay/SharPlayerController.h"
#include "Gameplay/SharPlayerPawn.h"
#include "Misc/AutomationTest.h"

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharGameplayFrameworkBootstrapTest,
    "SHAR.Runtime.GameplayFramework.Bootstrap",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharGameplayFrameworkBootstrapTest::RunTest(const FString& Parameters)
{
    (void)Parameters;

    const ASharGameMode* GameMode = GetDefault<ASharGameMode>();
    const ASharPlayerPawn* PlayerPawn = GetDefault<ASharPlayerPawn>();

    TestTrue(
        TEXT("Project global Game Mode selects SHAR framework"),
        UGameMapsSettings::GetGlobalDefaultGameMode()
            == TEXT("/Script/shar.SharGameMode")
    );
    TestTrue(
        TEXT("Game Mode selects SHAR Game State"),
        GameMode->GameStateClass == ASharGameState::StaticClass()
    );
    TestTrue(
        TEXT("Game Mode selects SHAR Player Controller"),
        GameMode->PlayerControllerClass == ASharPlayerController::StaticClass()
    );
    TestTrue(
        TEXT("Game Mode selects SHAR replay controller boundary"),
        GameMode->ReplaySpectatorPlayerControllerClass
            == ASharPlayerController::StaticClass()
    );
    TestTrue(
        TEXT("Game Mode selects representative SHAR player Pawn"),
        GameMode->DefaultPawnClass == ASharPlayerPawn::StaticClass()
    );
    TestNotNull(
        TEXT("Representative player Pawn owns a neutral transform root"),
        PlayerPawn->GetRootComponent()
    );
    TestTrue(
        TEXT("Representative Pawn cannot auto-possess a local player"),
        PlayerPawn->AutoPossessPlayer == EAutoReceiveInput::Disabled
    );
    TestTrue(
        TEXT("Representative Pawn cannot auto-possess AI"),
        PlayerPawn->AutoPossessAI == EAutoPossessAI::Disabled
    );
    TestFalse(
        TEXT("Representative Pawn has no independent tick policy"),
        PlayerPawn->PrimaryActorTick.bCanEverTick
    );
    return true;
}

#endif
