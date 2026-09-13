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
//   - Root gameplay catalog contract tests.
// - Must-Not:
//   - Depend on imported game assets or editor state.
// - Allows:
//   - Synthetic Primary Asset identities and transient catalog objects.
// - Split-When:
//   - Runtime loading tests require a separate adapter fixture.
// - Merge-When:
//   - Another suite proves the identical catalog invariants.
// - Summary:
//   - Verifies root catalog validation, activation, and lookup.
// - Description:
//   - Uses synthetic definitions so final art cannot block runtime contracts.
// - Usage:
//   - Runs through Unreal Automation in editor, client, and commandlet
//   - contexts.
// - Defaults:
//   - No external assets are required.
//

//! Root gameplay catalog contract tests.

#if WITH_DEV_AUTOMATION_TESTS

#include "Catalog/SharGameplayCatalog.h"
#include "Catalog/SharGameplayCatalogSubsystem.h"

#include "Engine/GameInstance.h"
#include "Misc/AutomationTest.h"

namespace
{
USharGameplayCatalog* MakeCatalog()
{
    auto* Catalog = NewObject<USharGameplayCatalog>();
    Catalog->CanonicalId = FName(TEXT("gameplay"));
    Catalog->DisplayName = FText::FromString(TEXT("SHAR gameplay catalog"));
    Catalog->SourcePackageIds = {FName(TEXT("gameplay_catalog_contract"))};
    Catalog->RevisionToken = TEXT("sha256:gameplay_catalog_v1");
    Catalog->ValidationProfile = FName(TEXT("gameplay_catalog_v1"));
    Catalog->OwningFeature = FName(TEXT("base"));

    FSharGameplayCatalogFamily Modes;
    Modes.FamilyId = FName(TEXT("application_modes"));
    Modes.PrimaryAssetTypeName = FName(TEXT("SharApplicationMode"));
    Modes.DefinitionIds = {
        FPrimaryAssetId(
            FPrimaryAssetType(TEXT("SharApplicationMode")),
            FName(TEXT("entry"))
        ),
        FPrimaryAssetId(
            FPrimaryAssetType(TEXT("SharApplicationMode")),
            FName(TEXT("gameplay"))
        ),
    };
    Catalog->Families.Add(Modes);
    return Catalog;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharGameplayCatalogValidationTest,
    "SHAR.Content.GameplayCatalog.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharGameplayCatalogActivationTest,
    "SHAR.Content.GameplayCatalog.Activation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharGameplayCatalogValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharGameplayCatalog* Catalog = MakeCatalog();
    TArray<FText> Errors;
    Catalog->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid root catalog passes"), Errors.IsEmpty());
    TestTrue(
        TEXT("Root catalog publishes SharCatalog identity"),
        Catalog->GetPrimaryAssetId()
            == FPrimaryAssetId(
                FPrimaryAssetType(TEXT("SharCatalog")),
                FName(TEXT("gameplay"))
            )
    );

    const FPrimaryAssetId DuplicateId =
        Catalog->Families[0].DefinitionIds[0];
    Catalog->Families[0].DefinitionIds.Add(DuplicateId);
    Errors.Reset();
    Catalog->GatherValidationErrors(Errors);
    TestFalse(TEXT("Duplicate definition is rejected"), Errors.IsEmpty());
    return true;
}

bool FSharGameplayCatalogActivationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharGameplayCatalog* Catalog = MakeCatalog();
    auto* GameInstance = NewObject<UGameInstance>();
    auto* Subsystem =
        NewObject<USharGameplayCatalogSubsystem>(GameInstance);
    const FPrimaryAssetId GameplayMode(
        FPrimaryAssetType(TEXT("SharApplicationMode")),
        FName(TEXT("gameplay"))
    );

    TestTrue(
        TEXT("Valid root catalog activates"),
        Subsystem->Activate(Catalog)
            == ESharGameplayCatalogActivationResult::Accepted
    );
    TestTrue(TEXT("Catalog becomes active"), Subsystem->IsActive());
    TestTrue(
        TEXT("Revision is exposed exactly"),
        Subsystem->GetCatalogRevision() == Catalog->RevisionToken
    );
    TestTrue(
        TEXT("Catalog resolves declared definition"),
        Subsystem->ContainsDefinition(GameplayMode)
    );
    TestTrue(
        TEXT("Active catalog cannot be replaced in place"),
        Subsystem->Activate(MakeCatalog())
            == ESharGameplayCatalogActivationResult::AlreadyActive
    );
    return true;
}

#endif
