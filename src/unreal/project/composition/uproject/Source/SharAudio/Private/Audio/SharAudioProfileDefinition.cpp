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
//   - Shar audio profile definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar audio profile definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar audio profile definition composition module.

#include "Audio/SharAudioProfileDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddAudioProfileError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool IsCanonicalAudioId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

bool USharAudioProfileDefinition::RequiresLease(
    const ESharAudioPlaybackPolicy PlaybackPolicy
)
{
    switch (PlaybackPolicy)
    {
    case ESharAudioPlaybackPolicy::LeasedContinuous:
    case ESharAudioPlaybackPolicy::OwnerScopedPersistent:
        return true;
    case ESharAudioPlaybackPolicy::OneShot:
    case ESharAudioPlaybackPolicy::FiniteLoop:
    case ESharAudioPlaybackPolicy::Queued:
    case ESharAudioPlaybackPolicy::Attached:
    default:
        return false;
    }
}

void USharAudioProfileDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);

    const bool bInvalidIdentity =
        !IsCanonicalAudioId(SourceAssetId)
        || !IsCanonicalAudioId(ParameterSchemaId)
        || !IsCanonicalAudioId(AttenuationPolicyId)
        || !IsCanonicalAudioId(ConcurrencyPolicyId)
        || !IsCanonicalAudioId(RoutingPolicyId)
        || !IsCanonicalAudioId(ResidencyPolicyId);
    if (bInvalidIdentity)
    {
        AddAudioProfileError(
            OutErrors,
            TEXT("Audio source and policy identities must be canonical.")
        );
    }
    if (MaximumConcurrentInstances <= 0)
    {
        AddAudioProfileError(
            OutErrors,
            TEXT("Audio concurrency limit must be positive.")
        );
    }
    if (RequiresLease(PlaybackPolicy)
        && CompletionPolicy == ESharAudioCompletionPolicy::Ignored)
    {
        AddAudioProfileError(
            OutErrors,
            TEXT("Leased audio requires observable completion or cancellation.")
        );
    }
    if (bPositional && Role == ESharAudioRole::UserInterface)
    {
        AddAudioProfileError(
            OutErrors,
            TEXT("User-interface audio cannot be positional.")
        );
    }
}

FPrimaryAssetType USharAudioProfileDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharAudioProfile")};
}
