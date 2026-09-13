// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar save schema tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar save schema tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar save schema tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharSaveTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "Save/SharSaveSchemaDefinition.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharSaveSchemaValidationTest,
    "SHAR.Save.Schema.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharSaveSchemaMigrationPlanTest,
    "SHAR.Save.Schema.MigrationPlan",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharSaveSchemaValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharSaveSchemaDefinition* Schema = MakeSaveSchema();
    TArray<FText> Errors;
    Schema->GatherValidationErrors(Errors);
    TestTrue(TEXT("Complete save schema is valid"), Errors.IsEmpty());

    for (FSharSaveMigrationStep& Step : Schema->MigrationSteps)
    {
        if (Step.SourceVersion == IntermediateSaveSchemaVersion)
        {
            Step.TargetVersion = FutureSaveSchemaVersion;
        }
    }
    Errors.Reset();
    Schema->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Skipped migration version is rejected"),
        Errors.IsEmpty()
    );
    return true;
}

bool FSharSaveSchemaMigrationPlanTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const USharSaveSchemaDefinition* Schema = MakeSaveSchema();
    TArray<FName> MigrationIds;
    TestTrue(
        TEXT("Version one builds a complete migration plan"),
        Schema->BuildMigrationPlan(
            InitialSaveSchemaVersion,
            MigrationIds
        )
    );
    const TArray<FName> ExpectedMigrationIds = {
        FName(TEXT("save_v1_to_v2")),
        FName(TEXT("save_v2_to_v3")),
    };
    TestTrue(
        TEXT("Migration plan contains both ordered steps"),
        MigrationIds.Num() == ExpectedMigrationIds.Num()
    );
    auto ExpectedIterator = ExpectedMigrationIds.begin();
    for (const FName& MigrationId : MigrationIds)
    {
        TestTrue(
            TEXT("Migration step order matches the schema chain"),
            MigrationId == *ExpectedIterator
        );
        ++ExpectedIterator;
    }
    TestTrue(
        TEXT("Current version requires no migration steps"),
        Schema->BuildMigrationPlan(CurrentSaveSchemaVersion, MigrationIds)
            && MigrationIds.IsEmpty()
    );
    TestFalse(
        TEXT("Unknown future version cannot migrate"),
        Schema->BuildMigrationPlan(FutureSaveSchemaVersion, MigrationIds)
    );
    return true;
}

#endif
