// File:
//   - SharPrimaryContentDefinition.cpp
// Path:
//   - src/uproject/Source/SharContent/Private/Content/SharPrimaryContentDefinition.cpp
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Deterministic validation and Asset Manager identity for shared content.
// - Must-Not:
//   - Load referenced assets, scan directories, or apply gameplay policy.
// - Allows:
//   - Pure validation of authored identity, dependencies, and provenance.
// - Split-When:
//   - A validation rule belongs only to one concrete content family.
// - Merge-When:
//   - Another implementation owns the same cross-family invariants.
// - Summary:
//   - Implements shared native content identity and validation.
// - Description:
//   - Rejects ambiguous definitions before import publication or activation.
// - Usage:
//   - Called by Asset Manager consumers, tests, and Unreal Data Validation.
// - Defaults:
//   - Validation performs no synchronous loads or mutable editor queries.
//
// ADRs:
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// - docs/adr/unreal/runtime/data-driven-gameplay-content-catalog.md
//
// Large file:
//   - false
//

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
