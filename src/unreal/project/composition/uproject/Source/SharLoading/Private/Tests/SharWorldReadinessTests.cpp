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
//   - Shar world readiness tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar world readiness tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar world readiness tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharLoadingTestFixtures.h"

#include "Loading/SharWorldReadinessSubsystem.h"
#include "Misc/AutomationTest.h"

namespace
{
constexpr int32 ExpectedCheckpointCount = 3;
constexpr int32 ExpectedBarrierCount = 1;

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldReadinessBarrierTest,
    "SHAR.Loading.WorldReadiness.BarrierLifecycle",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharWorldReadinessBarrierTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Readiness = NewObject<USharWorldReadinessSubsystem>();
    Readiness->ConfigureWorld(
        FName(TEXT("springfield_world")),
        TEXT("sha256:world_v1")
    );
    const FSharWorldReadinessBarrier Barrier = MakeWorldReadinessBarrier();

    TestTrue(
        TEXT("Valid barrier registers"),
        Readiness->RegisterBarrier(Barrier)
            == ESharWorldReadinessResult::Accepted
    );
    TestTrue(
        TEXT("Required checkpoint count is stable"),
        Readiness->GetRequiredCheckpointCount(Barrier.BarrierId)
            == ExpectedCheckpointCount
    );
    TestTrue(
        TEXT("Stale world callback is rejected"),
        Readiness->CompleteCheckpoint({
                .BarrierId = Barrier.BarrierId,
                .CheckpointId = FName(TEXT("actors_ready")),
                .WorldRevision = TEXT("sha256:world_old"),
            }) == ESharWorldReadinessResult::StaleWorld
    );
    TestTrue(
        TEXT("Unknown checkpoint is rejected"),
        Readiness->CompleteCheckpoint({
                .BarrierId = Barrier.BarrierId,
                .CheckpointId = FName(TEXT("lighting_ready")),
                .WorldRevision = TEXT("sha256:world_v1"),
            }) == ESharWorldReadinessResult::CheckpointMissing
    );
    TestTrue(
        TEXT("First checkpoint is accepted"),
        Readiness->CompleteCheckpoint({
                .BarrierId = Barrier.BarrierId,
                .CheckpointId = FName(TEXT("actors_ready")),
                .WorldRevision = TEXT("sha256:world_v1"),
            }) == ESharWorldReadinessResult::Accepted
    );
    TestTrue(
        TEXT("Duplicate checkpoint is rejected"),
        Readiness->CompleteCheckpoint({
                .BarrierId = Barrier.BarrierId,
                .CheckpointId = FName(TEXT("actors_ready")),
                .WorldRevision = TEXT("sha256:world_v1"),
            }) == ESharWorldReadinessResult::DuplicateCheckpoint
    );
    Readiness->CompleteCheckpoint({
            .BarrierId = Barrier.BarrierId,
            .CheckpointId = FName(TEXT("collision_ready")),
            .WorldRevision = TEXT("sha256:world_v1"),
        });
    Readiness->CompleteCheckpoint({
            .BarrierId = Barrier.BarrierId,
            .CheckpointId = FName(TEXT("navigation_ready")),
            .WorldRevision = TEXT("sha256:world_v1"),
        });
    TestTrue(
        TEXT("Barrier becomes ready exactly once"),
        Readiness->IsReady(Barrier.BarrierId)
    );
    TestTrue(
        TEXT("Ready barrier rejects later checkpoint writes"),
        Readiness->CompleteCheckpoint({
                .BarrierId = Barrier.BarrierId,
                .CheckpointId = FName(TEXT("navigation_ready")),
                .WorldRevision = TEXT("sha256:world_v1"),
            }) == ESharWorldReadinessResult::AlreadyReady
    );
    TestTrue(
        TEXT("World teardown removes registered barrier"),
        Readiness->TeardownWorld() == ExpectedBarrierCount
    );
    return true;
}

#endif
