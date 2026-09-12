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
//   - Direct Snake review-wheel defaults supported by source evidence.
// - Must-Not:
//   - Approximate legacy suspension, tire forces, braking torque, or gearing.
// - Allows:
//   - Convert source meters to Chaos centimeters and degrees directly.
// - Split-When:
//   - Another vehicle needs the same reviewed conversion policy.
// - Merge-When:
//   - Production wheel construction owns equivalent source-backed defaults.
// - Summary:
//   - Implements Snake drive-test wheel defaults.
// - Description:
//   - Preserves only radius, axle role, braking role, and steering angle.
// - Usage:
//   - Constructed by the Snake review presentation, never final gameplay.
// - Defaults:
//   - Rear radius 41.1083281 cm; front radius 34.9751204 cm.
//

//! Source-backed Snake review-wheel defaults.

#include "Vehicles/SharSnakeDriveTestWheel.h"

namespace
{
constexpr float SnakeRearWheelRadiusCentimeters = 41.1083281F;
constexpr float SnakeFrontWheelRadiusCentimeters = 34.9751204F;
constexpr float SnakeMaximumSteerDegrees = 30.0F;
}

USharSnakeDriveTestRearWheel::USharSnakeDriveTestRearWheel()
{
    WheelRadius = SnakeRearWheelRadiusCentimeters;
    bAffectedByEngine = true;
    bAffectedByBrake = true;
    bAffectedByHandbrake = true;
    bAffectedBySteering = false;
}

USharSnakeDriveTestFrontWheel::USharSnakeDriveTestFrontWheel()
{
    WheelRadius = SnakeFrontWheelRadiusCentimeters;
    bAffectedByEngine = false;
    bAffectedByBrake = false;
    bAffectedByHandbrake = false;
    bAffectedBySteering = true;
    MaxSteerAngle = SnakeMaximumSteerDegrees;
}
