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
//   - Shar save game tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar save game tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar save game tests composition module.

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
