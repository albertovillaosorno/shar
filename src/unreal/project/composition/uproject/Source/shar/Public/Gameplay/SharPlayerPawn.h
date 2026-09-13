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
//   - Defines the representative SHAR player Pawn bootstrap.
// - Description:
//   - Provides only a scene root and disables autonomous possession and
//   - ticking.
// - Usage:
//   - Spawned and possessed through ASharGameMode until character composition.
// - Defaults:
//   - No automatic player or AI possession and no Actor tick.
//

//! Neutral project-owned Pawn used as the initial possession boundary.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Pawn.h"

#include "SharPlayerPawn.generated.h"

UCLASS()
class SHAR_API ASharPlayerPawn final : public APawn
{
    GENERATED_BODY()

public:
    ASharPlayerPawn();
};
