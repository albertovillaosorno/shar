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
//   - The project-owned concrete Chaos wheeled vehicle Pawn type.
// - Must-Not:
//   - Resolve content identities or choose vehicle presentation policy.
// - Allows:
//   - Native Chaos wheeled-vehicle components inherited from the engine Pawn.
// - Split-When:
//   - Vehicle simulation requires independently lifecycle-managed components.
// - Merge-When:
//   - Another project Pawn owns the identical wheeled-vehicle boundary.
// - Summary:
//   - Provides the concrete Pawn configured by vehicle construction.
// - Description:
//   - Keeps engine-owned Chaos defaults behind a project-owned runtime type.
// - Usage:
//   - Constructed through the vehicle construction transaction.
// - Defaults:
//   - Starts with the engine Chaos vehicle subobjects and no presentation.
//

//! Project-owned concrete Chaos wheeled vehicle Pawn.

#pragma once

#include "CoreMinimal.h"
#include "WheeledVehiclePawn.h"

#include "SharVehiclePawn.generated.h"

UCLASS(BlueprintType, Blueprintable)
class SHARVEHICLES_API ASharVehiclePawn : public AWheeledVehiclePawn
{
    GENERATED_BODY()

public:
    explicit ASharVehiclePawn(
        const FObjectInitializer& ObjectInitializer = FObjectInitializer::Get()
    );
};
