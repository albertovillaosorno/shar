// File: SharCameraProfileDefinition.cpp
// Path: src/uproject/Source/SharCamera/Private/Camera/SharCameraProfileDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free camera profile validation only; no view calculation.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

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
