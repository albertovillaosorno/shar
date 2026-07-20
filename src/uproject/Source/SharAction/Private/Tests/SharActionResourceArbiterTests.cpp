// File: SharActionResourceArbiterTests.cpp
// Path: src/uproject/Source/SharAction/Private/Tests/SharActionResourceArbiterTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient shared, exclusive, duplicate, and owner-teardown lease tests only.
// ADR: docs/adr/unreal/runtime/typed-state-tree-action-sequences.md
// LARGE-FILE owner=SharAction; reason=two cohesive resource-lease scenarios;
// split=separate duplicate-request tests if diagnostics expand;
// validation=validate.sh SharAction plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "Action/SharActionResourceArbiter.h"

#include "Action/SharActionDefinition.h"
#include "Misc/AutomationTest.h"

namespace
{
constexpr int32 ExpectedOwnerLeaseCount = 2;

struct FSharActionLeaseFixture
{
    FName LeaseId;
    FName OwnerId;
    FName ResourceId;
    ESharActionResourceAccess Access = ESharActionResourceAccess::Exclusive;
};
} // namespace

static FSharActionResourceRequest MakeLeaseRequest(
    const FSharActionLeaseFixture& Fixture
)
{
    FSharActionResourceRequest Request;
    Request.LeaseId = Fixture.LeaseId;
    Request.OwnerId = Fixture.OwnerId;
    Request.ResourceId = Fixture.ResourceId;
    Request.OwnerRevision = TEXT("sha256:owner_v1");
    Request.Access = Fixture.Access;
    return Request;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharActionSharedExclusiveLeaseTest,
    "SHAR.Action.Resources.SharedAndExclusive",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharActionOwnerTeardownTest,
    "SHAR.Action.Resources.OwnerTeardown",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharActionSharedExclusiveLeaseTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Arbiter = NewObject<USharActionResourceArbiter>();
    const FSharActionResourceRequest SharedA = MakeLeaseRequest({
        .LeaseId = FName(TEXT("camera_shared_a")),
        .OwnerId = FName(TEXT("sequence_a")),
        .ResourceId = FName(TEXT("camera_observation")),
        .Access = ESharActionResourceAccess::Shared,
    });
    const FSharActionResourceRequest SharedB = MakeLeaseRequest({
        .LeaseId = FName(TEXT("camera_shared_b")),
        .OwnerId = FName(TEXT("sequence_b")),
        .ResourceId = FName(TEXT("camera_observation")),
        .Access = ESharActionResourceAccess::Shared,
    });
    const FSharActionResourceRequest Exclusive = MakeLeaseRequest({
        .LeaseId = FName(TEXT("camera_exclusive")),
        .OwnerId = FName(TEXT("sequence_c")),
        .ResourceId = FName(TEXT("camera_observation")),
        .Access = ESharActionResourceAccess::Exclusive,
    });

    TestTrue(
        TEXT("First shared lease is granted"),
        Arbiter->Acquire(SharedA) == ESharActionLeaseResult::Granted
    );
    TestTrue(
        TEXT("Second shared lease is granted"),
        Arbiter->Acquire(SharedB) == ESharActionLeaseResult::Granted
    );
    TestTrue(
        TEXT("Exclusive lease conflicts with active shared leases"),
        Arbiter->Acquire(Exclusive)
            == ESharActionLeaseResult::ResourceConflict
    );
    TestTrue(
        TEXT("Duplicate lease identity is rejected"),
        Arbiter->Acquire(SharedA) == ESharActionLeaseResult::DuplicateLease
    );
    return true;
}

bool FSharActionOwnerTeardownTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Arbiter = NewObject<USharActionResourceArbiter>();
    const FSharActionResourceRequest Movement = MakeLeaseRequest({
        .LeaseId = FName(TEXT("movement_lease")),
        .OwnerId = FName(TEXT("sequence_owner")),
        .ResourceId = FName(TEXT("character_movement")),
        .Access = ESharActionResourceAccess::Exclusive,
    });
    const FSharActionResourceRequest Animation = MakeLeaseRequest({
        .LeaseId = FName(TEXT("animation_lease")),
        .OwnerId = FName(TEXT("sequence_owner")),
        .ResourceId = FName(TEXT("animation_slot_upper_body")),
        .Access = ESharActionResourceAccess::Exclusive,
    });
    Arbiter->Acquire(Movement);
    Arbiter->Acquire(Animation);

    TestTrue(
        TEXT("Owner holds two active leases"),
        Arbiter->GetActiveLeaseCount() == ExpectedOwnerLeaseCount
    );
    TestTrue(
        TEXT("Owner teardown releases every lease"),
        Arbiter->ReleaseOwner(FName(TEXT("sequence_owner")))
            == ExpectedOwnerLeaseCount
    );
    TestTrue(
        TEXT("Owner teardown leaves no active leases"),
        Arbiter->GetActiveLeaseCount() == 0
    );
    TestTrue(
        TEXT("Released resource accepts a new exclusive lease"),
        Arbiter->Acquire(MakeLeaseRequest({
            .LeaseId = FName(TEXT("movement_replacement")),
            .OwnerId = FName(TEXT("replacement_sequence")),
            .ResourceId = FName(TEXT("character_movement")),
            .Access = ESharActionResourceAccess::Exclusive,
        })) == ESharActionLeaseResult::Granted
    );
    return true;
}

#endif
