// File: SharGameModeDefinitionTests.cpp
// Path: src/uproject/Source/SharContent/Private/Tests/SharGameModeDefinitionTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient game mode-definition tests only; no asset loading.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#if WITH_DEV_AUTOMATION_TESTS

#include "GameMode/SharGameModeDefinition.h"

#include "Engine/DataAsset.h"
#include "Misc/AutomationTest.h"

static void FillGameModeBase(USharGameModeDefinition& GameMode)
{
    GameMode.CanonicalId = FName(TEXT("open_world_campaign"));
    GameMode.DisplayName = FText::FromString(TEXT("Open world campaign"));
    GameMode.SourcePackageIds = {FName(TEXT("game_mode_contract"))};
    GameMode.RevisionToken = TEXT("sha256:game_mode_v1");
    GameMode.ValidationProfile = FName(TEXT("game_mode_definition_v1"));
    GameMode.OwningFeature = FName(TEXT("base"));
    GameMode.WorldDefinitionId = {
        FPrimaryAssetType(TEXT("SharWorld")),
        FName(TEXT("open_world")),
    };
    GameMode.DefaultCharacterId = {
        FPrimaryAssetType(TEXT("SharCharacter")),
        FName(TEXT("homer")),
    };
    GameMode.DefaultPlatformProfileId = {
        FPrimaryAssetType(TEXT("SharPlatformProfile")),
        FName(TEXT("windows_x8664")),
    };
    GameMode.StartupMissionIds.Add({
        FPrimaryAssetType(TEXT("SharMission")),
        FName(TEXT("chapter_01_intro")),
    });
    GameMode.RequiredFeatureNamespaces.Add(FName(TEXT("base")));
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharGameModeDefinitionValidationTest,
    "SHAR.Content.GameMode.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharGameModeDefinitionValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* GameMode = NewObject<USharGameModeDefinition>();
    FillGameModeBase(*GameMode);

    TArray<FText> Errors;
    GameMode->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid game mode passes"), Errors.IsEmpty());

    GameMode->RequiredFeatureNamespaces.Add(FName(TEXT("base")));
    Errors.Reset();
    GameMode->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Duplicate feature namespace is rejected"),
        Errors.IsEmpty()
    );
    return true;
}

#endif
