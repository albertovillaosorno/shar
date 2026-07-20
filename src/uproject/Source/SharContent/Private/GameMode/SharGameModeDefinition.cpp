// File: SharGameModeDefinition.cpp
// Path: src/uproject/Source/SharContent/Private/GameMode/SharGameModeDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free game mode identity and dependency validation; no game mode activation.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharContent; reason=cohesive game mode-definition validation;
// split=extract dependency validation if more game mode families appear;
// validation=validate.sh SharContent plus Unreal automation; review=2027-01.

#include "GameMode/SharGameModeDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddGameModeError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static void AppendPrimaryAssetListErrors(
    const TArray<FPrimaryAssetId>& AssetIds,
    TArray<FText>& OutErrors
)
{
    TSet<FPrimaryAssetId> Seen;
    for (const FPrimaryAssetId& AssetId : AssetIds)
    {
        if (!AssetId.IsValid())
        {
            AddGameModeError(
                OutErrors,
                TEXT("Game mode dependency identities must be valid.")
            );
        }
        if (Seen.Contains(AssetId))
        {
            AddGameModeError(
                OutErrors,
                TEXT("Game mode dependency identities must be unique.")
            );
        }
        Seen.Add(AssetId);
    }
}

static void AppendFeatureNamespaceErrors(
    const TArray<FName>& Namespaces,
    TArray<FText>& OutErrors
)
{
    TSet<FName> Seen;
    for (const FName& NamespaceId : Namespaces)
    {
        if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(NamespaceId))
        {
            AddGameModeError(
                OutErrors,
                TEXT("Feature namespaces must use canonical identities.")
            );
        }
        if (Seen.Contains(NamespaceId))
        {
            AddGameModeError(
                OutErrors,
                TEXT("Feature namespaces must be unique.")
            );
        }
        Seen.Add(NamespaceId);
    }
}

void USharGameModeDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    if (!WorldDefinitionId.IsValid())
    {
        AddGameModeError(
            OutErrors,
            TEXT("Game mode requires a valid world definition identity.")
        );
    }
    if (!DefaultCharacterId.IsValid())
    {
        AddGameModeError(
            OutErrors,
            TEXT("Game mode requires a default character identity.")
        );
    }
    if (!DefaultPlatformProfileId.IsValid())
    {
        AddGameModeError(
            OutErrors,
            TEXT("Game mode requires a platform profile identity.")
        );
    }
    if (SaveSchemaVersion <= 0)
    {
        AddGameModeError(
            OutErrors,
            TEXT("Game mode save schema version must be positive.")
        );
    }
    AppendPrimaryAssetListErrors(StartupMissionIds, OutErrors);
    AppendFeatureNamespaceErrors(RequiredFeatureNamespaces, OutErrors);
}

FPrimaryAssetType USharGameModeDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharGameMode")};
}
