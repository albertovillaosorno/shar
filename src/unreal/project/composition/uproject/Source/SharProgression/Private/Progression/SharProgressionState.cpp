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
//   - Shar progression state composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar progression state composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar progression state composition module.

#include "Progression/SharProgressionState.h"

#include "Algo/AnyOf.h"
#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"

static bool NamesMatch(
    const FSharProgressionValue& Value,
    const FName& OperationId,
    const FName& TargetId
)
{
    return Value.OperationId == OperationId && Value.TargetId == TargetId;
}

static bool IsValidSnapshotValue(const FSharProgressionValue& Value)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Value.OperationId
    )
        && USharPrimaryContentDefinition::IsCanonicalIdentifier(Value.TargetId)
        && USharProgressionState::IsSupportedOperation(Value.OperationId)
        && Value.Quantity > 0;
}

static bool HasDuplicateSnapshotValues(
    const TArray<FSharProgressionValue>& Values
)
{
    return Algo::AnyOf(
        Values,
        [&Values](const FSharProgressionValue& Candidate)
        {
            int32 MatchCount = 0;
            for (const FSharProgressionValue& Value : Values)
            {
                MatchCount += NamesMatch(
                    Value,
                    Candidate.OperationId,
                    Candidate.TargetId
                )
                    ? 1
                    : 0;
            }
            return MatchCount > 1;
        }
    );
}

static bool HasInvalidOrDuplicateTransactionIds(
    const TArray<FName>& TransactionIds
)
{
    return Algo::AnyOf(
        TransactionIds,
        [&TransactionIds](const FName& Candidate)
        {
            // jig-ignore-next-line: exact syntax is indivisible
            if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate))
            {
                return true;
            }
            int32 MatchCount = 0;
            for (const FName& TransactionId : TransactionIds)
            {
                MatchCount += TransactionId == Candidate ? 1 : 0;
            }
            return MatchCount > 1;
        }
    );
}

bool USharProgressionState::InitializeSnapshot(
    const TArray<FSharProgressionValue>& InValues,
    const TArray<FName>& InAppliedPermanentTransactions
)
{
    const bool bInvalidValues = HasDuplicateSnapshotValues(InValues)
        || Algo::AnyOf(
            InValues,
            [](const FSharProgressionValue& Value)
            {
                return !IsValidSnapshotValue(Value);
            }
        );
    if (bInvalidValues
        || HasInvalidOrDuplicateTransactionIds(InAppliedPermanentTransactions))
    {
        return false;
    }
    Values = InValues;
    AppliedPermanentTransactions = InAppliedPermanentTransactions;
    return true;
}

bool USharProgressionState::IsSupportedOperation(
    const FName& OperationId
)
{
    const TArray<FName> SupportedOperations = {
        FName(TEXT("grant_currency")),
        FName(TEXT("unlock_character")),
        FName(TEXT("unlock_vehicle")),
        FName(TEXT("unlock_costume")),
        FName(TEXT("unlock_ability")),
        FName(TEXT("unlock_world_region")),
        FName(TEXT("unlock_activity")),
        FName(TEXT("grant_collectible")),
        FName(TEXT("set_progression_flag")),
        FName(TEXT("grant_achievement_progress")),
    };
    return Algo::AnyOf(
        SupportedOperations,
        [&OperationId](const FName& SupportedOperation)
        {
            return SupportedOperation == OperationId;
        }
    );
}

bool USharProgressionState::UsesSetSemantics(const FName& OperationId)
{
    return OperationId == FName(TEXT("unlock_character"))
        || OperationId == FName(TEXT("unlock_vehicle"))
        || OperationId == FName(TEXT("unlock_costume"))
        || OperationId == FName(TEXT("unlock_ability"))
        || OperationId == FName(TEXT("unlock_world_region"))
        || OperationId == FName(TEXT("unlock_activity"))
        || OperationId == FName(TEXT("set_progression_flag"));
}

FSharProgressionValue* USharProgressionState::FindValue(
    const FName& OperationId,
    const FName& TargetId
)
{
    return Algo::FindByPredicate(
        Values,
        [&OperationId, &TargetId](const FSharProgressionValue& Value)
        {
            return NamesMatch(Value, OperationId, TargetId);
        }
    );
}

const FSharProgressionValue* USharProgressionState::FindValue(
    const FName& OperationId,
    const FName& TargetId
) const
{
    return Algo::FindByPredicate(
        Values,
        [&OperationId, &TargetId](const FSharProgressionValue& Value)
        {
            return NamesMatch(Value, OperationId, TargetId);
        }
    );
}

bool USharProgressionState::HasAppliedTransaction(
    const FName& TransactionId
) const
{
    return Algo::AnyOf(
        AppliedPermanentTransactions,
        [&TransactionId](const FName& AppliedId)
        {
            return AppliedId == TransactionId;
        }
    );
}

ESharRewardApplyResult USharProgressionState::ValidateRewardRequest(
    const FSharRewardRequest& Request
) const
{
    const bool bInvalidIdentity =
        !USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Request.TransactionId
        )
        || !USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Request.OperationId
        )
        || !USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Request.TargetId
        );
    if (bInvalidIdentity || Request.Quantity <= 0)
    {
        return ESharRewardApplyResult::InvalidRequest;
    }
    if (!IsSupportedOperation(Request.OperationId))
    {
        return ESharRewardApplyResult::UnsupportedOperation;
    }
    if (Request.bPermanent && HasAppliedTransaction(Request.TransactionId))
    {
        return ESharRewardApplyResult::AlreadyApplied;
    }
    return ESharRewardApplyResult::Applied;
}

void USharProgressionState::ApplyRewardValue(
    const FSharRewardRequest& Request
)
{
    FSharProgressionValue* Value = FindValue(
        Request.OperationId,
        Request.TargetId
    );
    const bool bSetOperation = UsesSetSemantics(Request.OperationId);
    const int32 AppliedQuantity = bSetOperation ? 1 : Request.Quantity;
    if (Value == nullptr)
    {
        FSharProgressionValue NewValue;
        NewValue.OperationId = Request.OperationId;
        NewValue.TargetId = Request.TargetId;
        NewValue.Quantity = AppliedQuantity;
        Values.Add(NewValue);
        return;
    }
    Value->Quantity = bSetOperation
        ? 1
        : Value->Quantity + AppliedQuantity;
}

ESharRewardApplyResult USharProgressionState::ApplyReward(
    const FSharRewardRequest& Request
)
{
    const ESharRewardApplyResult ValidationResult =
        ValidateRewardRequest(Request);
    if (ValidationResult != ESharRewardApplyResult::Applied)
    {
        return ValidationResult;
    }
    ApplyRewardValue(Request);
    if (Request.bPermanent)
    {
        AppliedPermanentTransactions.Add(Request.TransactionId);
    }
    return ESharRewardApplyResult::Applied;
}

int32 USharProgressionState::GetQuantity(
    const FName& OperationId,
    const FName& TargetId
) const
{
    const FSharProgressionValue* Value = FindValue(OperationId, TargetId);
    return Value == nullptr ? 0 : Value->Quantity;
}

const TArray<FSharProgressionValue>& USharProgressionState::GetValues() const
{
    return Values;
}

const TArray<FName>& USharProgressionState::GetAppliedTransactions() const
{
    return AppliedPermanentTransactions;
}
