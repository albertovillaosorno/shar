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
//   - Shar presentation definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar presentation definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar presentation definition composition module.

#include "Presentation/SharPresentationDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddPresentationDefinitionError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool IsCanonicalPresentationId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsCanonicalOrNone(const FName& Candidate)
{
    return Candidate.IsNone() || IsCanonicalPresentationId(Candidate);
}

static void AppendRequiredPolicyErrors(
    const USharPresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalid =
        !IsCanonicalPresentationId(Definition.AssetSetId)
        || !IsCanonicalPresentationId(Definition.OwnerPolicyId)
        || !IsCanonicalPresentationId(Definition.PlaybackPolicyId)
        || !IsCanonicalPresentationId(Definition.FallbackPolicyId)
        || !IsCanonicalPresentationId(Definition.ResultPolicyId)
        || !IsCanonicalPresentationId(Definition.TeardownPolicyId);
    if (bInvalid)
    {
        AddPresentationDefinitionError(
            OutErrors,
            // jig-ignore-next-line: exact syntax is indivisible
            TEXT("Presentation asset, owner, playback, fallback, result, and teardown identities must be canonical.")
        );
    }
}

static void AppendOptionalPolicyErrors(
    const USharPresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalid =
        !IsCanonicalOrNone(Definition.ExclusivityPolicyId)
        || !IsCanonicalOrNone(Definition.CameraPolicyId)
        || !IsCanonicalOrNone(Definition.CharacterLayerPolicyId);
    if (bInvalid)
    {
        AddPresentationDefinitionError(
            OutErrors,
            // jig-ignore-next-line: exact syntax is indivisible
            TEXT("Optional presentation policy identities must be canonical when present.")
        );
    }
}

static void AppendExclusivityAndTeardownErrors(
    const USharPresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (Definition.bRequiresScopedLeases
        && Definition.ExclusivityPolicyId.IsNone())
    {
        AddPresentationDefinitionError(
            OutErrors,
            TEXT("Exclusive presentation requires a scoped exclusivity policy.")
        );
    }
    if (!Definition.bHasCompleteReleasePath)
    {
        AddPresentationDefinitionError(
            OutErrors,
            TEXT("Presentation teardown requires a complete release path.")
        );
    }
}

void USharPresentationDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    AppendRequiredPolicyErrors(*this, OutErrors);
    AppendOptionalPolicyErrors(*this, OutErrors);
    AppendExclusivityAndTeardownErrors(*this, OutErrors);
}

FPrimaryAssetType USharPresentationDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharPresentation")};
}
