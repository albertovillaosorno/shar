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
//   - Project identity for native Unreal world gameplay state projection.
// - Must-Not:
//   - Duplicate application-mode, mission, progression, or save authority.
// - Allows:
//   - Native Game State lifecycle and later read-only world projections.
// - Split-When:
//   - A world projection gains an independent replicated lifecycle.
// - Merge-When:
//   - Another project Game State owns the same native framework boundary.
// - Summary:
//   - Defines the minimal SHAR Game State bootstrap.
// - Description:
//   - Adds no domain state before a validated projection requires it.
// - Usage:
//   - Selected by ASharGameMode for gameplay worlds.
// - Defaults:
//   - Inherits native AGameStateBase behavior only.
//

//! Project-owned native Game State boundary.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/GameStateBase.h"

#include "SharGameState.generated.h"

UCLASS()
class SHAR_API ASharGameState final : public AGameStateBase
{
    GENERATED_BODY()
};
