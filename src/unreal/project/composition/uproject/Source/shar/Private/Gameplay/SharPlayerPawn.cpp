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
//   - Neutral native Pawn boundary for SHAR player possession and transform.
// - Must-Not:
//   - Invent movement, collision, camera, mesh, animation, or input policy.
// - Allows:
//   - Native possession and a root transform for later avatar construction.
// - Split-When:
//   - Validated on-foot character construction owns its native Actor class.
// - Merge-When:
//   - Another project Pawn provides the same neutral possession boundary.
// - Summary:
//   - Implements the representative SHAR player Pawn bootstrap.
// - Description:
//   - Creates only a scene root and disables autonomous ownership and ticking.
// - Usage:
//   - Spawned by the project Game Mode as a temporary native possession target.
// - Defaults:
//   - No automatic player or AI possession and no Actor tick.
//

//! Neutral project-owned Pawn used as the initial possession boundary.

#include "Gameplay/SharPlayerPawn.h"

#include "Components/SceneComponent.h"

ASharPlayerPawn::ASharPlayerPawn()
{
    RootComponent = CreateDefaultSubobject<USceneComponent>(TEXT("Root"));
    PrimaryActorTick.bCanEverTick = false;
    AutoPossessPlayer = EAutoReceiveInput::Disabled;
    AutoPossessAI = EAutoPossessAI::Disabled;
}
