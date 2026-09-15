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
//   - Shar application definition catalog tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar application definition catalog tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar application definition catalog tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharApplicationTestFixtures.h"

#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Application/SharApplicationModeDefinition.h"
#include "Engine/GameInstance.h"
#include "Misc/AutomationTest.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharApplicationDefinitionValidationTest,
    "SHAR.Application.Definition.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharApplicationCatalogGraphValidationTest,
    "SHAR.Application.Catalog.GraphValidation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharApplicationDefinitionValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharApplicationModeDefinition* Definition = MakeApplicationMode({
        .ModeId = FName(TEXT("loading_demo")),
        .ModeKind = ESharApplicationModeKind::Loading,
        .PredecessorIds = {FName(TEXT("front_end"))},
        .SuccessorIds = {FName(TEXT("demo"))},
        .RequiredServiceIds = {FName(TEXT("world_service"))},
        .SuccessModeId = FName(TEXT("demo")),
        .RecoveryModeId = FName(TEXT("front_end")),
        .ReturnModeId = FName(),
        .WorldPolicy = ESharApplicationWorldPolicy::Prepare,
        .ProgressionPolicy = ESharApplicationProgressionPolicy::ReadOnly,
        .bDemonstrationMode = true,
    });
    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid loading mode definition passes"), Errors.IsEmpty());

    Definition->ProgressionPolicy =
        ESharApplicationProgressionPolicy::Durable;
    Errors.Reset();
    Definition->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Demonstration mode with durable progression is rejected"),
        Errors.IsEmpty()
    );

    Definition->ProgressionPolicy =
        ESharApplicationProgressionPolicy::ReadOnly;
    Definition->SessionPolicy = ESharApplicationSessionPolicy::Retain;
    Definition->RecoveryModeId = FName();
    Errors.Reset();
    Definition->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Session-retaining mode requires explicit recovery"),
        Errors.IsEmpty()
    );

    Definition->SessionPolicy = ESharApplicationSessionPolicy::None;
    Definition->ProfilePolicy = ESharApplicationProfilePolicy::Retain;
    Errors.Reset();
    Definition->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Profile-retaining mode requires explicit recovery"),
        Errors.IsEmpty()
    );
    return true;
}

bool FSharApplicationCatalogGraphValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* ValidGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* ValidCatalog =
        MakeApplicationCatalog(
            *ValidGameInstance,
            ESharApplicationCatalogShape::Valid,
            false
        );
    USharApplicationModeDefinition* Uncatalogued = MakeApplicationMode({
        .ModeId = FName(TEXT("uncatalogued_mode")),
        .ModeKind = ESharApplicationModeKind::Active,
    });
    TestTrue(
        TEXT("Root catalog rejects undeclared application mode"),
        ValidCatalog->RegisterMode(Uncatalogued)
            == ESharApplicationCatalogResult::DefinitionNotCatalogued
    );
    TestTrue(
        TEXT("Complete reciprocal mode graph activates"),
        ValidCatalog->Activate() == ESharApplicationCatalogResult::Accepted
    );
    TestTrue(TEXT("Activated catalog is immutable"), ValidCatalog->IsActive());

    auto* BrokenGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* BrokenCatalog =
        MakeApplicationCatalog(
            *BrokenGameInstance,
            ESharApplicationCatalogShape::BrokenReciprocalEdge,
            false
        );
    TestTrue(
        TEXT("Non-reciprocal graph is rejected"),
        BrokenCatalog->Activate()
            == ESharApplicationCatalogResult::EdgeNotReciprocal
    );

    auto* MissingReturnGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* MissingReturnCatalog =
        MakeApplicationCatalog(
            *MissingReturnGameInstance,
            ESharApplicationCatalogShape::MissingOverlayReturn,
            false
        );
    TestTrue(
        TEXT("Overlay without a resolvable return owner is rejected"),
        MissingReturnCatalog->Activate()
            == ESharApplicationCatalogResult::OverlayReturnMissing
    );

    auto* MissingRecoveryGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* MissingRecoveryCatalog =
        MakeApplicationCatalog(
            *MissingRecoveryGameInstance,
            ESharApplicationCatalogShape::MissingRecoveryTarget,
            false
        );
    TestTrue(
        TEXT("Non-loading recovery target must resolve in catalog"),
        MissingRecoveryCatalog->Activate()
            == ESharApplicationCatalogResult::RecoveryTargetMissing
    );

    auto* MissingWorldAuthorityGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* MissingWorldAuthorityCatalog =
        MakeApplicationCatalog(
            *MissingWorldAuthorityGameInstance,
            ESharApplicationCatalogShape::RetainWorldFromAbsentAuthority,
            false
        );
    TestTrue(
        TEXT("Retained world requires authority on every predecessor"),
        MissingWorldAuthorityCatalog->Activate()
            == ESharApplicationCatalogResult::AuthorityPolicyMismatch
    );

    auto* MissingSessionAuthorityGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* MissingSessionAuthorityCatalog =
        MakeApplicationCatalog(
            *MissingSessionAuthorityGameInstance,
            ESharApplicationCatalogShape::RetainSessionFromAbsentAuthority,
            false
        );
    TestTrue(
        TEXT("Retained session requires authority on every predecessor"),
        MissingSessionAuthorityCatalog->Activate()
            == ESharApplicationCatalogResult::AuthorityPolicyMismatch
    );

    auto* MissingOwnedWorldGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* MissingOwnedWorldCatalog =
        MakeApplicationCatalog(
            *MissingOwnedWorldGameInstance,
            ESharApplicationCatalogShape::OwnWorldFromAbsentAuthority,
            false
        );
    TestTrue(
        TEXT("Owned world requires authority on every predecessor"),
        MissingOwnedWorldCatalog->Activate()
            == ESharApplicationCatalogResult::AuthorityPolicyMismatch
    );

    auto* MissingOwnedSessionGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* MissingOwnedSessionCatalog =
        MakeApplicationCatalog(
            *MissingOwnedSessionGameInstance,
            ESharApplicationCatalogShape::OwnSessionFromAbsentAuthority,
            false
        );
    TestTrue(
        TEXT("Owned session requires authority on every predecessor"),
        MissingOwnedSessionCatalog->Activate()
            == ESharApplicationCatalogResult::AuthorityPolicyMismatch
    );

    auto* MissingOwnedProfileGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* MissingOwnedProfileCatalog =
        MakeApplicationCatalog(
            *MissingOwnedProfileGameInstance,
            ESharApplicationCatalogShape::OwnProfileFromAbsentAuthority,
            false
        );
    TestTrue(
        TEXT("Owned profile requires authority on every predecessor"),
        MissingOwnedProfileCatalog->Activate()
            == ESharApplicationCatalogResult::AuthorityPolicyMismatch
    );

    auto* MissingRetainedProfileGameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* MissingRetainedProfileCatalog =
        MakeApplicationCatalog(
            *MissingRetainedProfileGameInstance,
            ESharApplicationCatalogShape::RetainProfileFromAbsentAuthority,
            false
        );
    TestTrue(
        TEXT("Retained profile requires authority on every predecessor"),
        MissingRetainedProfileCatalog->Activate()
            == ESharApplicationCatalogResult::AuthorityPolicyMismatch
    );
    return true;
}

#endif
