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
//   - Shar save game composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar save game composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar save game composition module.

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
        // jig-ignore-next-line: exact syntax is indivisible
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
