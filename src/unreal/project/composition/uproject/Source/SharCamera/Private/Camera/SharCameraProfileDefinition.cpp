// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar camera profile definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar camera profile definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar camera profile definition composition module.

#include "Camera/SharCameraProfileDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddCameraProfileError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool IsCanonicalPolicyId(const FName& PolicyId)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(PolicyId);
}

void USharCameraProfileDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);

    const bool bInvalidPolicyIdentity =
        !IsCanonicalPolicyId(PresetId)
        || !IsCanonicalPolicyId(TransitionPolicyId)
        || !IsCanonicalPolicyId(CollisionPolicyId)
        || !IsCanonicalPolicyId(InputPolicyId)
        || !IsCanonicalPolicyId(VerificationPolicyId);
    if (bInvalidPolicyIdentity)
    {
        AddCameraProfileError(
            OutErrors,
            TEXT("Camera policy identities must be canonical.")
        );
    }

    const bool bInvalidFov =
        !FMath::IsFinite(MinimumFovDegrees)
        || !FMath::IsFinite(MaximumFovDegrees)
        || MinimumFovDegrees < MinimumAllowedFovDegrees
        || MaximumFovDegrees > MaximumAllowedFovDegrees
        || MinimumFovDegrees > MaximumFovDegrees;
    if (bInvalidFov)
    {
        AddCameraProfileError(
            OutErrors,
            // jig-ignore-next-line: exact syntax is indivisible
            TEXT("Camera FOV bounds must be finite, ordered, and physically valid.")
        );
    }

    if (ModeKind == ESharCameraModeKind::Debug
        && PriorityClass != ESharCameraPriorityClass::Debug)
    {
        AddCameraProfileError(
            OutErrors,
            TEXT("Debug camera modes must use the debug priority class.")
        );
    }
    if (ModeKind == ESharCameraModeKind::Animated && !bAllowsSkipInput)
    {
        AddCameraProfileError(
            OutErrors,
            TEXT("Animated camera profiles require explicit skip-input policy.")
        );
    }
}

FPrimaryAssetType USharCameraProfileDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharCameraProfile")};
}
