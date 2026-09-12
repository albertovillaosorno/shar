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
//   - Source-backed wheel role/radius defaults for Snake drive inspection.
// - Must-Not:
//   - Claim final Chaos handling, invent suspension, tire, or torque tuning.
// - Allows:
//   - Preserve authored radius, driven/steered axle roles, and steer limit.
// - Split-When:
//   - Production wheel conversion gains a reviewed general policy.
// - Merge-When:
//   - Generic vehicle wheel compilation supersedes this review harness.
// - Summary:
//   - Snake-specific non-authoritative Chaos wheel review classes.
// - Description:
//   - Maps directly supported legacy wheel semantics into Chaos defaults.
// - Usage:
//   - Used only by the Snake vehicle drive-test presentation.
// - Defaults:
//   - Source wheel order is w0 rear-right, w1 rear-left, w2 front-left,
//   - w3 front-right; rear wheels drive/brake and front wheels steer to 30
//   - source degrees.
//

//! Snake review-only Chaos wheel classes.

#pragma once

#include "ChaosVehicleWheel.h"
#include "CoreMinimal.h"

#include "SharSnakeDriveTestWheel.generated.h"

UCLASS(BlueprintType)
class SHARVEHICLES_API USharSnakeDriveTestRearWheel final
    : public UChaosVehicleWheel
{
    GENERATED_BODY()

public:
    USharSnakeDriveTestRearWheel();
};

UCLASS(BlueprintType)
class SHARVEHICLES_API USharSnakeDriveTestFrontWheel final
    : public UChaosVehicleWheel
{
    GENERATED_BODY()

public:
    USharSnakeDriveTestFrontWheel();
};
