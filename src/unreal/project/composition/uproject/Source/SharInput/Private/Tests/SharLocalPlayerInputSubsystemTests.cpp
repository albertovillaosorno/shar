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
//   - Local-player input lease lifecycle automation.
// - Must-Not:
//   - Depend on authored mapping assets or final gameplay input bindings.
// - Allows:
//   - Synthetic mapping contexts and transient native player objects.
// - Split-When:
//   - Device hotplug or rebinding requires independent fixtures.
// - Merge-When:
//   - Another suite proves the identical lease lifecycle.
// - Summary:
//   - Verifies stage, commit, revision fencing, and release behavior.
// - Description:
//   - Exercises real Enhanced Input context registration with synthetic data.
// - Usage:
//   - Runs through Unreal Automation without imported content.
// - Defaults:
//   - Uses one transient local player and player controller.
//

//! Local-player input lease lifecycle automation.

#if WITH_DEV_AUTOMATION_TESTS

#include "Input/SharLocalPlayerInputSubsystem.h"

#include "EnhancedInputSubsystems.h"
#include "EnhancedPlayerInput.h"
#include "Engine/Engine.h"
#include "Engine/LocalPlayer.h"
#include "Engine/World.h"
#include "GameFramework/PlayerController.h"
#include "InputMappingContext.h"
#include "Misc/AutomationTest.h"

namespace
{
constexpr int32 GameplayPriority = 100;

FSharInputContextLeaseRequest MakeGameplayLease(UInputMappingContext* Context)
{
    FSharInputContextLeaseRequest Request;
    Request.LeaseId = FName(TEXT("gameplay_input_lease"));
    Request.ContextId = FName(TEXT("gameplay_on_foot"));
    Request.OwnerModeId = FName(TEXT("gameplay"));
    Request.OwnerModeRevision = TEXT("sha256:gameplay_v1");
    Request.TransitionRevision = TEXT("sha256:commit_gameplay_v1");
    Request.LeaseRevision = TEXT("sha256:gameplay_input_lease_v1");
    Request.MappingContext = Context;
    Request.Priority = GameplayPriority;
    return Request;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharLocalPlayerInputLeaseLifecycleTest,
    "SHAR.Input.LocalPlayer.LeaseLifecycle",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharLocalPlayerInputLeaseLifecycleTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    UWorld* World = UWorld::CreateWorld(EWorldType::Game, false);
    auto* PlayerController = World->SpawnActor<APlayerController>();
    auto* LocalPlayer = NewObject<ULocalPlayer>(GEngine);
    LocalPlayer->PlayerAdded(nullptr, 0);
    LocalPlayer->SwitchController(PlayerController);

    USharLocalPlayerInputSubsystem* Input =
        LocalPlayer->GetSubsystem<USharLocalPlayerInputSubsystem>();
    UEnhancedInputLocalPlayerSubsystem* EnhancedInput =
        LocalPlayer->GetSubsystem<UEnhancedInputLocalPlayerSubsystem>();
    auto* MappingContext = NewObject<UInputMappingContext>();
    const FSharInputContextLeaseRequest Request =
        MakeGameplayLease(MappingContext);

    TestNotNull(TEXT("SHAR input subsystem is created"), Input);
    TestNotNull(TEXT("Enhanced Input subsystem is created"), EnhancedInput);
    TestTrue(
        TEXT("Valid context stages without activation"),
        Input->StageLease(Request) == ESharInputContextLeaseResult::Accepted
    );
    TestFalse(
        TEXT("Staging does not install native mapping"),
        EnhancedInput->HasMappingContext(MappingContext)
    );
    TestTrue(
        TEXT("Duplicate lease identity is rejected"),
        Input->StageLease(Request)
            == ESharInputContextLeaseResult::DuplicateLease
    );
    TestTrue(
        TEXT("Stale commit revision is rejected"),
        Input->CommitLease(
            Request.LeaseId,
            TEXT("sha256:stale"),
            Request.TransitionRevision
        )
            == ESharInputContextLeaseResult::StaleRevision
    );
    TestTrue(
        TEXT("Readiness fails before Enhanced Player Input exists"),
        Input->CheckCommitReadiness(
            Request.LeaseId,
            Request.LeaseRevision,
            Request.TransitionRevision
        ) == ESharInputContextLeaseResult::EnhancedInputUnavailable
    );
    TestTrue(
        TEXT("Commit fails closed before Enhanced Player Input exists"),
        Input->CommitLease(
            Request.LeaseId,
            Request.LeaseRevision,
            Request.TransitionRevision
        ) == ESharInputContextLeaseResult::EnhancedInputUnavailable
    );
    TestTrue(
        TEXT("Unavailable input leaves lease staged"),
        Input->GetStagedLeaseCount() == 1
    );
    PlayerController->PlayerInput = NewObject<UEnhancedPlayerInput>(
        PlayerController
    );
    TestTrue(
        TEXT("Stale transition cannot become commit-ready"),
        Input->CheckCommitReadiness(
            Request.LeaseId,
            Request.LeaseRevision,
            TEXT("sha256:stale_transition")
        ) == ESharInputContextLeaseResult::StaleRevision
    );
    TestTrue(
        TEXT("Validated staged lease becomes commit-ready"),
        Input->CheckCommitReadiness(
            Request.LeaseId,
            Request.LeaseRevision,
            Request.TransitionRevision
        ) == ESharInputContextLeaseResult::Accepted
    );
    TestFalse(
        TEXT("Readiness preflight does not install native mapping"),
        EnhancedInput->HasMappingContext(MappingContext)
    );
    TestTrue(
        TEXT("Validated commit activates native mapping"),
        Input->CommitLease(
            Request.LeaseId,
            Request.LeaseRevision,
            Request.TransitionRevision
        )
            == ESharInputContextLeaseResult::Accepted
    );
    int32 AppliedPriority = 0;
    TestTrue(
        TEXT("Committed mapping is installed"),
        EnhancedInput->HasMappingContext(MappingContext, AppliedPriority)
    );
    TestTrue(
        TEXT("Committed mapping preserves priority"),
        AppliedPriority == GameplayPriority
    );
    TestTrue(
        TEXT("Committed lease becomes active"),
        Input->GetActiveLeaseCount() == 1
    );

    FSharInputContextLeaseRequest ConflictingRequest = Request;
    ConflictingRequest.LeaseId = FName(TEXT("second_gameplay_lease"));
    ConflictingRequest.LeaseRevision = TEXT("sha256:second_lease_v1");
    TestTrue(
        TEXT("One mapping cannot have two SHAR owners"),
        Input->StageLease(ConflictingRequest)
            == ESharInputContextLeaseResult::MappingContextInUse
    );
    TestTrue(
        TEXT("Stale release revision is rejected"),
        Input->ReleaseLease(Request.LeaseId, TEXT("sha256:stale"))
            == ESharInputContextLeaseResult::StaleRevision
    );
    TestTrue(
        TEXT("Release removes committed lease"),
        Input->ReleaseLease(Request.LeaseId, Request.LeaseRevision)
            == ESharInputContextLeaseResult::Accepted
    );
    TestFalse(
        TEXT("Released lease removes native mapping"),
        EnhancedInput->HasMappingContext(MappingContext)
    );
    TestTrue(
        TEXT("Released lease cannot release twice"),
        Input->ReleaseLease(Request.LeaseId, Request.LeaseRevision)
            == ESharInputContextLeaseResult::AlreadyReleased
    );

    auto* PauseContext = NewObject<UInputMappingContext>();
    FSharInputContextLeaseRequest PauseLease;
    PauseLease.LeaseId = FName(TEXT("pause_input_lease"));
    PauseLease.ContextId = FName(TEXT("pause"));
    PauseLease.OwnerModeId = FName(TEXT("pause"));
    PauseLease.OwnerModeRevision = TEXT("sha256:pause_v1");
    PauseLease.TransitionRevision = TEXT("sha256:enter_pause_v1");
    PauseLease.LeaseRevision = TEXT("sha256:pause_input_lease_v1");
    PauseLease.MappingContext = PauseContext;
    PauseLease.Priority = GameplayPriority + 1;
    TestTrue(
        TEXT("Second context stages after first lease release"),
        Input->StageLease(PauseLease) == ESharInputContextLeaseResult::Accepted
    );
    TestTrue(
        TEXT("Second context commits before local-player teardown"),
        Input->CommitLease(
            PauseLease.LeaseId,
            PauseLease.LeaseRevision,
            PauseLease.TransitionRevision
        )
            == ESharInputContextLeaseResult::Accepted
    );
    TestTrue(
        TEXT("Second native mapping is active before teardown"),
        EnhancedInput->HasMappingContext(PauseContext)
    );

    LocalPlayer->PlayerRemoved();
    TestFalse(
        TEXT("Local-player teardown removes active native mapping"),
        EnhancedInput->HasMappingContext(PauseContext)
    );
    World->DestroyWorld(false);
    return true;
}

#endif
