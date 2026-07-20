// File: SharWorldMessageGuardTests.cpp
// Path: src/uproject/Source/SharMessaging/Private/Tests/SharWorldMessageGuardTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: stale-world, duplicate, causation-depth, subject-filter, replay, and teardown tests only.
// Specification: docs/technical/unreal/typed-event-and-observation-routing-runtime.md
// LARGE-FILE owner=SharMessaging; reason=three cohesive routing-guard and lifecycle scenarios;
// split=separate replay tests if additional replay policies appear;
// validation=validate.sh SharMessaging plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharMessagingTestFixtures.h"

#include "Messaging/SharMessageSchemaCatalog.h"
#include "Messaging/SharWorldMessageRouterSubsystem.h"
#include "Misc/AutomationTest.h"

namespace
{
constexpr int32 MaximumGuardDepth = 2;
constexpr int32 ExcessiveGuardDepth = 3;
constexpr int32 ExpectedTeardownSubscriptionCount = 2;

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldMessagePublicationGuardsTest,
    "SHAR.Messaging.WorldRouter.PublicationGuards",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldMessageReplayAndFilterTest,
    "SHAR.Messaging.WorldRouter.ReplayAndFilter",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldMessageTeardownTest,
    "SHAR.Messaging.WorldRouter.Teardown",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharWorldMessagePublicationGuardsTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Catalog = NewObject<USharMessageSchemaCatalog>();
    const FName ChannelId(TEXT("shar.vehicle.destroyed"));
    Catalog->RegisterSchema(MakeMessageSchema({
        .SchemaId = FName(TEXT("vehicle_destroyed_v1")),
        .ChannelId = ChannelId,
        .DeliveryPhase = ESharMessageDeliveryPhase::ImmediateReadOnly,
        .MaximumRecursionDepth = MaximumGuardDepth,
    }));
    USharWorldMessageRouterSubsystem* Router = MakeMessageRouter(*Catalog);
    const FSharMessageEnvelope Accepted = MakeMessageEnvelope({
        .MessageId = FName(TEXT("vehicle_destroyed_message")),
        .SchemaId = FName(TEXT("vehicle_destroyed_v1")),
        .ChannelId = ChannelId,
        .SubjectId = FName(TEXT("family_sedan_01")),
    });

    TestTrue(
        TEXT("Accepted fact publishes once"),
        Router->Publish(Accepted) == ESharMessageRouteResult::Accepted
    );
    TestTrue(
        TEXT("Duplicate message identity is rejected"),
        Router->Publish(Accepted) == ESharMessageRouteResult::DuplicateMessage
    );

    FSharMessageEnvelope Stale = MakeMessageEnvelope({
        .MessageId = FName(TEXT("vehicle_destroyed_stale")),
        .SchemaId = FName(TEXT("vehicle_destroyed_v1")),
        .ChannelId = ChannelId,
        .SubjectId = FName(TEXT("family_sedan_02")),
    });
    Stale.WorldRevision = TEXT("sha256:world_old");
    TestTrue(
        TEXT("Stale world revision is rejected"),
        Router->Publish(Stale) == ESharMessageRouteResult::StaleWorld
    );

    const FSharMessageEnvelope Recursive = MakeMessageEnvelope({
        .MessageId = FName(TEXT("vehicle_destroyed_recursive")),
        .SchemaId = FName(TEXT("vehicle_destroyed_v1")),
        .ChannelId = ChannelId,
        .SubjectId = FName(TEXT("family_sedan_03")),
        .CausationDepth = ExcessiveGuardDepth,
    });
    TestTrue(
        TEXT("Excessive causation depth is rejected"),
        Router->Publish(Recursive) == ESharMessageRouteResult::RecursionLimit
    );
    return true;
}

bool FSharWorldMessageReplayAndFilterTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Catalog = NewObject<USharMessageSchemaCatalog>();
    const FName ChannelId(TEXT("shar.lifecycle.world.ready"));
    Catalog->RegisterSchema(MakeMessageSchema({
        .SchemaId = FName(TEXT("world_ready_replay_v1")),
        .ChannelId = ChannelId,
        .DeliveryPhase = ESharMessageDeliveryPhase::ImmediateReadOnly,
        .ReplayPolicy = ESharMessageReplayPolicy::LastAccepted,
    }));
    USharWorldMessageRouterSubsystem* Router = MakeMessageRouter(*Catalog);
    const FSharMessageEnvelope Published = MakeMessageEnvelope({
        .MessageId = FName(TEXT("world_ready_replay_message")),
        .SchemaId = FName(TEXT("world_ready_replay_v1")),
        .ChannelId = ChannelId,
        .SubjectId = FName(TEXT("springfield_world")),
    });
    Router->Publish(Published);

    const FSharSubscriptionHandle Matching = MakeMessageSubscription({
        .HandleId = FName(TEXT("world_ready_replay_matching")),
        .SubscriberId = FName(TEXT("mission_runtime")),
        .ChannelId = ChannelId,
        .SubjectFilterId = FName(TEXT("springfield_world")),
    });
    const FSharSubscriptionHandle Filtered = MakeMessageSubscription({
        .HandleId = FName(TEXT("world_ready_replay_filtered")),
        .SubscriberId = FName(TEXT("frontend_runtime")),
        .ChannelId = ChannelId,
        .SubjectFilterId = FName(TEXT("shelbyville_world")),
    });
    Router->Subscribe(Matching);
    Router->Subscribe(Filtered);

    TestTrue(
        TEXT("Matching late subscriber receives last accepted fact"),
        Router->GetDeliveryCount(Matching.HandleId) == 1
    );
    TestTrue(
        TEXT("Subject filter rejects unrelated replay"),
        Router->GetDeliveryCount(Filtered.HandleId) == 0
    );
    return true;
}

bool FSharWorldMessageTeardownTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Catalog = NewObject<USharMessageSchemaCatalog>();
    const FName ChannelId(TEXT("shar.mission.step.completed"));
    Catalog->RegisterSchema(MakeMessageSchema({
        .SchemaId = FName(TEXT("teardown_message_v1")),
        .ChannelId = ChannelId,
        .DeliveryPhase = ESharMessageDeliveryPhase::NextWorldFrame,
    }));
    USharWorldMessageRouterSubsystem* Router = MakeMessageRouter(*Catalog);
    Router->Subscribe(MakeMessageSubscription({
        .HandleId = FName(TEXT("teardown_subscription_01")),
        .SubscriberId = FName(TEXT("mission_runtime")),
        .ChannelId = ChannelId,
        .SubjectFilterId = FName(),
    }));
    Router->Subscribe(MakeMessageSubscription({
        .HandleId = FName(TEXT("teardown_subscription_02")),
        .SubscriberId = FName(TEXT("audio_runtime")),
        .ChannelId = ChannelId,
        .SubjectFilterId = FName(),
    }));
    Router->Publish(MakeMessageEnvelope({
        .MessageId = FName(TEXT("teardown_pending_message")),
        .SchemaId = FName(TEXT("teardown_message_v1")),
        .ChannelId = ChannelId,
        .SubjectId = FName(TEXT("mission_l1_m1")),
    }));

    TestTrue(
        TEXT("World teardown releases every active subscription"),
        Router->TeardownWorld() == ExpectedTeardownSubscriptionCount
    );
    TestTrue(
        TEXT("World teardown leaves no active subscriptions"),
        Router->GetActiveSubscriptionCount() == 0
    );
    TestTrue(
        TEXT("World teardown marks pending messages unavailable"),
        Router->GetPendingMessageCount() == 0
    );
    TestTrue(
        TEXT("World teardown prevents later phase dispatch"),
        Router->FlushPhase(ESharMessageDeliveryPhase::NextWorldFrame) == 0
    );
    return true;
}

#endif
