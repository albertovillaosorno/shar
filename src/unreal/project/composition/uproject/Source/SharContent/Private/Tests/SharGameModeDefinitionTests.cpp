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
//   - Shar game mode definition tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar game mode definition tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar game mode definition tests composition module.

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
