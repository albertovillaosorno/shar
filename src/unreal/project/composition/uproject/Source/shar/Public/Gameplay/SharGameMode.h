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
//   - Defines the minimal SHAR Game Mode bootstrap.
// - Description:
//   - Delegates native player spawning and possession to Unreal framework code.
// - Usage:
//   - Selected as the project's global default Game Mode.
// - Defaults:
//   - Uses neutral project-owned framework classes without gameplay tuning.
//

//! Minimal project-owned Unreal Game Mode bootstrap.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/GameModeBase.h"

#include "SharGameMode.generated.h"

UCLASS()
class SHAR_API ASharGameMode final : public AGameModeBase
{
    GENERATED_BODY()

public:
    ASharGameMode();
};
