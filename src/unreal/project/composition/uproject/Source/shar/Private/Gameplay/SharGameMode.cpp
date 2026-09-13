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
//   - Native Unreal gameplay-framework class selection for SHAR worlds.
// - Must-Not:
//   - Commit application modes, grant input leases, or own gameplay-domain
//   - state.
// - Allows:
//   - Selecting project Game State, Player Controller, and default Pawn
//   - classes.
// - Split-When:
//   - A gameplay mode requires independently testable spawn policy.
// - Merge-When:
//   - Another project Game Mode owns the identical framework selection.
// - Summary:
//   - Implements the minimal SHAR Game Mode bootstrap.
// - Description:
//   - Configures only project-owned native framework classes.
// - Usage:
//   - Constructed by Unreal for worlds selecting ASharGameMode.
// - Defaults:
//   - Leaves native spawning and possession behavior intact.
//

//! Minimal project-owned Unreal Game Mode bootstrap.

#include "Gameplay/SharGameMode.h"

#include "Gameplay/SharGameState.h"
#include "Gameplay/SharPlayerController.h"
#include "Gameplay/SharPlayerPawn.h"

ASharGameMode::ASharGameMode()
{
    GameStateClass = ASharGameState::StaticClass();
    PlayerControllerClass = ASharPlayerController::StaticClass();
    ReplaySpectatorPlayerControllerClass = ASharPlayerController::StaticClass();
    DefaultPawnClass = ASharPlayerPawn::StaticClass();
}
