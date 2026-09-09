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
//   - Vehicle dynamic-light presentation state implementation.
// - Must-Not:
//   - Write material parameters, gameplay damage, or source-era renderer state.
// - Allows:
//   - Resolve semantic light visibility from validated presentation bindings.
// - Split-When:
//   - Material application or light intensity requires independent lifecycle.
// - Merge-When:
//   - Another runtime object owns identical semantic light state.
// - Summary:
//   - Implements vehicle dynamic-light presentation state.
// - Description:
//   - Models visibility requests without becoming rendering authority.
// - Usage:
//   - Used after vehicle presentation construction and before rendering output.
// - Defaults:
//   - Invalid configuration or requests fail closed without changing state.
//

//! Vehicle dynamic-light presentation state implementation.

#include "Vehicles/SharVehiclePresentationState.h"

#include "Vehicles/SharVehiclePresentationDefinition.h"

bool USharVehiclePresentationState::Configure(
    USharVehiclePresentationDefinition* InPresentation
)
{
    if (InPresentation == nullptr)
    {
        return false;
    }
    TArray<FText> Errors;
    InPresentation->GatherValidationErrors(Errors);
    if (!Errors.IsEmpty())
    {
        return false;
    }
    Presentation = InPresentation;
    bHeadlightsEnabled = false;
    bBrakeLightsEnabled = false;
    bReverseLightsEnabled = false;
    bLightsSuppressedByDamage = false;
    FadeOpacity = 1.0F;
    return true;
}

bool USharVehiclePresentationState::SetHeadlightsEnabled(const bool bEnabled)
{
    if (Presentation == nullptr)
    {
        return false;
    }
    bHeadlightsEnabled = bEnabled;
    return true;
}

bool USharVehiclePresentationState::SetBrakeState(
    const bool bBrakeActive,
    const bool bInReverse
)
{
    if (Presentation == nullptr)
    {
        return false;
    }
    bBrakeLightsEnabled = bBrakeActive && !bInReverse;
    bReverseLightsEnabled = bBrakeActive && bInReverse;
    return true;
}

bool USharVehiclePresentationState::SetLightsSuppressedByDamage(
    const bool bSuppressed
)
{
    if (Presentation == nullptr)
    {
        return false;
    }
    bLightsSuppressedByDamage = bSuppressed;
    return true;
}

bool USharVehiclePresentationState::SetFadeOpacity(const float InOpacity)
{
    if (
        Presentation == nullptr
        || !FMath::IsFinite(InOpacity)
        || InOpacity < 0.0F
        || InOpacity > 1.0F
    )
    {
        return false;
    }
    FadeOpacity = InOpacity;
    return true;
}

bool USharVehiclePresentationState::GetBindingVisibility(
    const FName BindingId,
    bool& bOutVisible
) const
{
    bOutVisible = false;
    if (Presentation == nullptr || BindingId.IsNone())
    {
        return false;
    }
    const FSharVehicleLightPresentationBinding* Binding =
        Presentation->LightBindings.FindByPredicate(
            [BindingId](const FSharVehicleLightPresentationBinding& Candidate)
            {
                return Candidate.BindingId == BindingId;
            }
        );
    if (Binding == nullptr)
    {
        return false;
    }
    bool bRequested = false;
    switch (Binding->Role)
    {
        case ESharVehicleLightPresentationRole::Headlight:
            bRequested = bHeadlightsEnabled;
            break;
        case ESharVehicleLightPresentationRole::Brake:
            bRequested = bBrakeLightsEnabled;
            break;
        case ESharVehicleLightPresentationRole::Reverse:
            bRequested = bReverseLightsEnabled;
            break;
        default:
            return false;
    }
    bOutVisible =
        bRequested && !bLightsSuppressedByDamage && FadeOpacity > 0.0F;
    return true;
}

bool USharVehiclePresentationState::IsConfigured() const
{
    return Presentation != nullptr;
}
