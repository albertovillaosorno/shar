// File: SharPresentationDefinition.cpp
// Path: src/uproject/Source/SharPresentation/Private/Presentation/SharPresentationDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free presentation-policy validation only; adapters and playback state remain external.
// Specification: docs/technical/unreal/presentation-playback-runtime.md
// LARGE-FILE owner=SharPresentation; reason=cohesive presentation-policy validation;
// split=extract adapter-policy validation if platform-specific contracts expand;
// validation=validate.sh SharPresentation plus Unreal automation; review=2027-01.

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
