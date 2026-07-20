// File: SharProgressionCatalogDefinition.cpp
// Path: src/uproject/Source/SharProgression/Private/Progression/SharProgressionCatalogDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free progression operation catalog validation and lookup only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharProgression; reason=cohesive operation catalog validation and lookup;
// split=extract value-policy validation if operation families gain separate schemas;
// validation=validate.sh SharProgression plus Unreal automation; review=2027-01.

#include "Progression/SharProgressionCatalogDefinition.h"

#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"
#include "Progression/SharProgressionState.h"

static void AddProgressionCatalogError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool HasDuplicateOperationIds(
    const TArray<FSharProgressionOperationDefinition>& Operations
)
{
    return Algo::AnyOf(
        Operations,
        [&Operations](const FSharProgressionOperationDefinition& Candidate)
        {
            int32 MatchCount = 0;
            for (const FSharProgressionOperationDefinition& Operation : Operations)
            {
                MatchCount += Operation.OperationId == Candidate.OperationId
                    ? 1
                    : 0;
            }
            return MatchCount > 1;
        }
    );
}

static bool IsValidOperationDefinition(
    const FSharProgressionOperationDefinition& Operation
)
{
    const bool bValidIdentity =
        USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Operation.OperationId
        )
        && USharProgressionState::IsSupportedOperation(Operation.OperationId);
    const ESharProgressionValuePolicy ExpectedPolicy =
        USharProgressionState::UsesSetSemantics(Operation.OperationId)
        ? ESharProgressionValuePolicy::SetOnce
        : ESharProgressionValuePolicy::Additive;
    const bool bValidPolicy = Operation.MaximumQuantity > 0
        && (Operation.bPermanentAllowed || Operation.bTransientAllowed)
        && Operation.ValuePolicy == ExpectedPolicy;
    return bValidIdentity && bValidPolicy;
}

void USharProgressionCatalogDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    if (SnapshotSchemaVersion <= 0 || MaximumMutationOperations <= 0)
    {
        AddProgressionCatalogError(
            OutErrors,
            TEXT("Progression schema and mutation-operation bound must be positive.")
        );
    }
    if (Operations.IsEmpty() || HasDuplicateOperationIds(Operations))
    {
        AddProgressionCatalogError(
            OutErrors,
            TEXT("Progression operations must be present and uniquely identified.")
        );
    }
    for (const FSharProgressionOperationDefinition& Operation : Operations)
    {
        if (!IsValidOperationDefinition(Operation))
        {
            AddProgressionCatalogError(
                OutErrors,
                TEXT("Progression operation policy is invalid or unsupported.")
            );
        }
    }
}

const FSharProgressionOperationDefinition*
USharProgressionCatalogDefinition::FindOperation(
    const FName& OperationId
) const
{
    for (const FSharProgressionOperationDefinition& Operation : Operations)
    {
        if (Operation.OperationId == OperationId)
        {
            return &Operation;
        }
    }
    return nullptr;
}

FPrimaryAssetType
USharProgressionCatalogDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharProgressionCatalog")};
}
