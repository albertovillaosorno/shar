// File: SharSaveGameTests.cpp
// Path: src/uproject/Source/SharMissions/Private/Tests/SharSaveGameTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient save-envelope and schema-compatibility tests only.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#if WITH_DEV_AUTOMATION_TESTS

#include "Save/SharSaveGame.h"

#include "Engine/DataAsset.h"
#include "Misc/AutomationTest.h"

static constexpr int32 UnsupportedFutureSchema = 2;

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharSaveCompatibilityTest,
    "SHAR.Missions.Save.Compatibility",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharSaveCompatibilityTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Save = NewObject<USharSaveGame>();
    Save->TransactionRevision = TEXT("sha256:save_transaction_v1");
    Save->GameModeId = {
        FPrimaryAssetType(TEXT("SharGameMode")),
        FName(TEXT("open_world_campaign")),
    };
    Save->ActiveMissionStageId = FName(TEXT("start"));

    FSharNamespacedModSaveState ModState;
    ModState.NamespaceId = FName(TEXT("example_mod"));
    ModState.StateRevision = TEXT("sha256:example_mod_state_v1");
    Save->ModStates.Add(ModState);

    TArray<FText> Errors;
    Save->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid save envelope passes"), Errors.IsEmpty());
    TestTrue(
        TEXT("Current schema is migratable"),
        USharSaveGame::CanMigrateFrom(
            USharSaveGame::CurrentSaveSchemaVersion
        )
    );
    TestFalse(
        TEXT("Future schema is not silently accepted"),
        USharSaveGame::CanMigrateFrom(UnsupportedFutureSchema)
    );
    return true;
}

#endif
