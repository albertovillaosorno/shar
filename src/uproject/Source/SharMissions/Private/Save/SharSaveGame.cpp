// File: SharSaveGame.cpp
// Path: src/uproject/Source/SharMissions/Private/Save/SharSaveGame.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free save validation and schema-compatibility checks; no disk I/O or automatic content substitution.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharMissions; reason=cohesive save-envelope validation;
// split=extract mod-state validation if namespaced persistence expands;
// validation=validate.sh SharMissions plus Unreal automation; review=2027-01.

#include "Save/SharSaveGame.h"

#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"
#include "Progression/SharProgressionState.h"

static void AddSaveError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool ProgressionKeysMatch(
    const FSharProgressionValue& Left,
    const FSharProgressionValue& Right
)
{
    return Left.OperationId == Right.OperationId
        && Left.TargetId == Right.TargetId;
}

static void AppendProgressionErrors(
    const TArray<FSharProgressionValue>& Values,
    TArray<FText>& OutErrors
)
{
    TArray<FSharProgressionValue> SeenValues;
    for (const FSharProgressionValue& Value : Values)
    {
        const bool bInvalid =
            !USharProgressionState::IsSupportedOperation(Value.OperationId)
            || !USharPrimaryContentDefinition::IsCanonicalIdentifier(
                Value.TargetId
            )
            || Value.Quantity < 0;
        if (bInvalid)
        {
            AddSaveError(
                OutErrors,
                TEXT("Saved progression contains an invalid value.")
            );
        }
        const bool bDuplicate = Algo::AnyOf(
            SeenValues,
            [&Value](const FSharProgressionValue& SeenValue)
            {
                return ProgressionKeysMatch(Value, SeenValue);
            }
        );
        if (bDuplicate)
        {
            AddSaveError(
                OutErrors,
                TEXT("Saved progression keys must be unique.")
            );
        }
        SeenValues.Add(Value);
    }
}

static void AppendTransactionErrors(
    const TArray<FName>& TransactionIds,
    TArray<FText>& OutErrors
)
{
    TSet<FName> Seen;
    for (const FName& TransactionId : TransactionIds)
    {
        if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(TransactionId))
        {
            AddSaveError(
                OutErrors,
                TEXT("Saved reward transaction identities must be canonical.")
            );
        }
        if (Seen.Contains(TransactionId))
        {
            AddSaveError(
                OutErrors,
                TEXT("Saved reward transaction identities must be unique.")
            );
        }
        Seen.Add(TransactionId);
    }
}

static void AppendModStateErrors(
    const TArray<FSharNamespacedModSaveState>& ModStates,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenNamespaces;
    for (const FSharNamespacedModSaveState& ModState : ModStates)
    {
        const bool bInvalid =
            !USharPrimaryContentDefinition::IsCanonicalIdentifier(
                ModState.NamespaceId
            )
            || ModState.SchemaVersion <= 0
            || !ModState.StateRevision.StartsWith(TEXT("sha256:"));
        if (bInvalid)
        {
            AddSaveError(
                OutErrors,
                TEXT("Namespaced mod save state is invalid.")
            );
        }
        if (SeenNamespaces.Contains(ModState.NamespaceId))
        {
            AddSaveError(
                OutErrors,
                TEXT("Namespaced mod save identities must be unique.")
            );
        }
        SeenNamespaces.Add(ModState.NamespaceId);
    }
}

bool USharSaveGame::CanMigrateFrom(const int32 SourceSchemaVersion)
{
    return SourceSchemaVersion > 0
        && SourceSchemaVersion <= CurrentSaveSchemaVersion;
}

void USharSaveGame::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    if (!CanMigrateFrom(SaveSchemaVersion))
    {
        AddSaveError(
            OutErrors,
            TEXT("Save schema version is unsupported.")
        );
    }
    if (!TransactionRevision.StartsWith(TEXT("sha256:")))
    {
        AddSaveError(
            OutErrors,
            TEXT("Save transaction revision requires SHA-256 identity.")
        );
    }
    if (!GameModeId.IsValid())
    {
        AddSaveError(
            OutErrors,
            TEXT("Save requires a valid game mode identity.")
        );
    }
    if (!ActiveMissionStageId.IsNone()
        && !USharPrimaryContentDefinition::IsCanonicalIdentifier(
            ActiveMissionStageId
        ))
    {
        AddSaveError(
            OutErrors,
            TEXT("Active mission stage identity must be canonical.")
        );
    }
    AppendProgressionErrors(ProgressionValues, OutErrors);
    AppendTransactionErrors(AppliedPermanentTransactions, OutErrors);
    AppendModStateErrors(ModStates, OutErrors);
}
