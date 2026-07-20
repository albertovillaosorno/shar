// File: SharSaveSchemaTests.cpp
// Path: src/uproject/Source/SharSave/Private/Tests/SharSaveSchemaTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient portable save schema and migration-plan validation tests only.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md

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
