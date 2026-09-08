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
//   - Construction of the project-owned Chaos vehicle Pawn subobjects.
// - Must-Not:
//   - Apply presentation assets or gameplay tuning directly.
// - Allows:
//   - Inheriting the engine wheeled-vehicle defaults unchanged.
// - Split-When:
//   - Project-owned default subobjects need independent construction policy.
// - Merge-When:
//   - The Pawn no longer needs a distinct native runtime identity.
// - Summary:
//   - Constructs the concrete SHAR vehicle Pawn.
// - Description:
//   - Delegates component creation to the Chaos wheeled-vehicle base class.
// - Usage:
//   - Spawned or instantiated before a construction transaction is prepared.
// - Defaults:
//   - Uses the engine default Chaos wheeled movement component.
//

//! Project-owned concrete Chaos wheeled vehicle Pawn construction.

#include "Vehicles/SharVehiclePawn.h"

ASharVehiclePawn::ASharVehiclePawn(const FObjectInitializer& ObjectInitializer)
    : Super(ObjectInitializer)
{
}
