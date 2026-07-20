// File: SharProgressionState.cpp
// Path: src/uproject/Source/SharMissions/Private/Progression/SharProgressionState.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic idempotent progression mutation; no save serialization or gameplay side effects.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharMissions; reason=cohesive idempotent progression behavior;
// split=extract operation registry if reward families become independently extensible;
// validation=validate.sh SharMissions plus Unreal automation; review=2027-01.

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

bool USharProgressionState::IsSetOperation(const FName& OperationId)
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
    const bool bSetOperation = IsSetOperation(Request.OperationId);
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
