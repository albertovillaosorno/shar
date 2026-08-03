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
//   - Shar primary content definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar primary content definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar primary content definition composition module.

#include "Content/SharPrimaryContentDefinition.h"

#include "Content/SharPrimaryContentValidation.h"
#include "Engine/DataAsset.h"

#if WITH_EDITOR
#include "Misc/DataValidation.h"
#endif

static bool HasInvalidIdentifierBoundary(const FString& Value)
{
    return Value.IsEmpty()
        || Value.StartsWith(TEXT("_"))
        || Value.EndsWith(TEXT("_"));
}

static bool IsCanonicalIdentifierCharacter(const TCHAR Character)
{
    const bool bIsLowercaseAscii = Character >= 'a' && Character <= 'z';
    const bool bIsDigit = Character >= '0' && Character <= '9';
    return bIsLowercaseAscii || bIsDigit || Character == '_';
}

FPrimaryAssetId USharPrimaryContentDefinition::GetPrimaryAssetId() const
{
    const FPrimaryAssetType AssetType = GetDefinitionAssetType();
    if (!AssetType.IsValid() || CanonicalId.IsNone())
    {
        return {};
    }
    return {AssetType, CanonicalId};
}

void USharPrimaryContentDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    const FPrimaryAssetType AssetType = GetDefinitionAssetType();
    const FPrimaryAssetId SelfId = GetPrimaryAssetId();
    FSharPrimaryContentValidation::AppendIdentityErrors(
        *this,
        AssetType,
        OutErrors
    );
    FSharPrimaryContentValidation::AppendProvenanceErrors(*this, OutErrors);
    FSharPrimaryContentValidation::AppendDependencyErrors(
        *this,
        SelfId,
        OutErrors
    );
}

bool USharPrimaryContentDefinition::IsCanonicalIdentifier(
    const FName& Candidate
)
{
    if (Candidate.IsNone())
    {
        return false;
    }

    const FString Value = Candidate.ToString();
    if (HasInvalidIdentifierBoundary(Value))
    {
        return false;
    }

    bool bPreviousWasUnderscore = false;
    for (const TCHAR Character : Value)
    {
        if (!IsCanonicalIdentifierCharacter(Character))
        {
            return false;
        }
        const bool bIsUnderscore = Character == '_';
        if (bIsUnderscore && bPreviousWasUnderscore)
        {
            return false;
        }
        bPreviousWasUnderscore = bIsUnderscore;
    }
    return true;
}

#if WITH_EDITOR
EDataValidationResult USharPrimaryContentDefinition::IsDataValid(
    FDataValidationContext& Context
) const
{
    TArray<FText> Errors;
    GatherValidationErrors(Errors);
    for (const FText& Error : Errors)
    {
        Context.AddError(Error);
    }
    return Errors.IsEmpty()
        ? EDataValidationResult::Valid
        : EDataValidationResult::Invalid;
}
#endif

FPrimaryAssetType USharPrimaryContentDefinition::GetDefinitionAssetType() const
{
    return {};
}
