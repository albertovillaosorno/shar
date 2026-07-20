// File: SharWorldMessageDeliveryTests.cpp
// Path: src/uproject/Source/SharMessaging/Private/Tests/SharWorldMessageDeliveryTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immediate, phased, ordered, consumed, and released delivery tests only.
// Specification: docs/technical/unreal/typed-event-and-observation-routing-runtime.md
// LARGE-FILE owner=SharMessaging; reason=two cohesive delivery-phase scenarios;
// split=separate consumption tests if typed payload adapters appear;
// validation=validate.sh SharMessaging plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharMessagingTestFixtures.h"

#include "Messaging/SharMessageSchemaCatalog.h"
#include "Messaging/SharWorldMessageRouterSubsystem.h"
#include "Misc/AutomationTest.h"

namespace
{
constexpr int32 ExpectedImmediateDeliveryCount = 1;
constexpr int32 ExpectedQueuedMessageCount = 2;
constexpr int64 FirstPublicationSequence = 1;
constexpr int64 SecondPublicationSequence = 2;

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldMessageImmediateDeliveryTest,
    "SHAR.Messaging.WorldRouter.ImmediateDelivery",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldMessagePhasedDeliveryTest,
    "SHAR.Messaging.WorldRouter.PhasedDelivery",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharWorldMessageImmediateDeliveryTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Catalog = NewObject<USharMessageSchemaCatalog>();
    const FName ChannelId(TEXT("shar.lifecycle.world.ready"));
    Catalog->RegisterSchema(MakeMessageSchema({
        .SchemaId = FName(TEXT("world_ready_v1")),
        .ChannelId = ChannelId,
        .DeliveryPhase = ESharMessageDeliveryPhase::ImmediateReadOnly,
    }));
    USharWorldMessageRouterSubsystem* Router = MakeMessageRouter(*Catalog);
    const FSharSubscriptionHandle Handle = MakeMessageSubscription({
        .HandleId = FName(TEXT("world_ready_subscription")),
        .SubscriberId = FName(TEXT("mission_runtime")),
        .ChannelId = ChannelId,
        .SubjectFilterId = FName(TEXT("springfield_world")),
    });
    Router->Subscribe(Handle);
    const FSharMessageEnvelope First = MakeMessageEnvelope({
        .MessageId = FName(TEXT("world_ready_message")),
        .SchemaId = FName(TEXT("world_ready_v1")),
        .ChannelId = ChannelId,
        .SubjectId = FName(TEXT("springfield_world")),
    });

    TestTrue(
        TEXT("Accepted immediate fact routes synchronously"),
        Router->Publish(First) == ESharMessageRouteResult::Accepted
    );
    TestTrue(
        TEXT("Matching subscriber receives one message"),
        Router->GetDeliveryCount(Handle.HandleId)
            == ExpectedImmediateDeliveryCount
    );
    FSharMessageEnvelope Delivered;
    TestTrue(
        TEXT("Subscriber consumes the routed envelope"),
        Router->ConsumeNext(Handle.HandleId, Delivered)
            == ESharMessageRouteResult::Accepted
    );
    TestTrue(
        TEXT("First accepted fact receives first sequence"),
        Delivered.PublicationSequence == FirstPublicationSequence
    );
    TestTrue(
        TEXT("Consumed delivery leaves no pending message for subscriber"),
        Router->ConsumeNext(Handle.HandleId, Delivered)
            == ESharMessageRouteResult::NothingPending
    );
    TestTrue(
        TEXT("Subscription releases explicitly"),
        Router->ReleaseSubscription(Handle.HandleId)
            == ESharMessageRouteResult::Accepted
    );
    TestTrue(
        TEXT("Subscription release is idempotent"),
        Router->ReleaseSubscription(Handle.HandleId)
            == ESharMessageRouteResult::AlreadyReleased
    );
    return true;
}

bool FSharWorldMessagePhasedDeliveryTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Catalog = NewObject<USharMessageSchemaCatalog>();
    const FName ChannelId(TEXT("shar.mission.step.completed"));
    Catalog->RegisterSchema(MakeMessageSchema({
        .SchemaId = FName(TEXT("mission_step_completed_v1")),
        .ChannelId = ChannelId,
        .DeliveryPhase = ESharMessageDeliveryPhase::EndOfDomainTransaction,
    }));
    USharWorldMessageRouterSubsystem* Router = MakeMessageRouter(*Catalog);
    const FSharSubscriptionHandle Handle = MakeMessageSubscription({
        .HandleId = FName(TEXT("mission_step_subscription")),
        .SubscriberId = FName(TEXT("progression_runtime")),
        .ChannelId = ChannelId,
        .SubjectFilterId = FName(),
    });
    Router->Subscribe(Handle);
    const FSharMessageEnvelope First = MakeMessageEnvelope({
        .MessageId = FName(TEXT("mission_step_message_01")),
        .SchemaId = FName(TEXT("mission_step_completed_v1")),
        .ChannelId = ChannelId,
        .SubjectId = FName(TEXT("mission_l1_m1")),
    });
    const FSharMessageEnvelope Second = MakeMessageEnvelope({
        .MessageId = FName(TEXT("mission_step_message_02")),
        .SchemaId = FName(TEXT("mission_step_completed_v1")),
        .ChannelId = ChannelId,
        .SubjectId = FName(TEXT("mission_l1_m1")),
    });
    Router->Publish(First);
    Router->Publish(Second);

    TestTrue(
        TEXT("Queued phase holds messages before explicit flush"),
        Router->GetPendingMessageCount() == ExpectedQueuedMessageCount
    );
    TestTrue(
        TEXT("Queued phase produces no early delivery"),
        Router->GetDeliveryCount(Handle.HandleId) == 0
    );
    TestTrue(
        TEXT("Phase flush dispatches both accepted facts"),
        Router->FlushPhase(
            ESharMessageDeliveryPhase::EndOfDomainTransaction
        ) == ExpectedQueuedMessageCount
    );
    FSharMessageEnvelope Delivered;
    Router->ConsumeNext(Handle.HandleId, Delivered);
    TestTrue(
        TEXT("Delivery preserves first publication sequence"),
        Delivered.PublicationSequence == FirstPublicationSequence
    );
    Router->ConsumeNext(Handle.HandleId, Delivered);
    TestTrue(
        TEXT("Delivery preserves second publication sequence"),
        Delivered.PublicationSequence == SecondPublicationSequence
    );
    TestTrue(
        TEXT("Already flushed phase does not redeliver facts"),
        Router->FlushPhase(
            ESharMessageDeliveryPhase::EndOfDomainTransaction
        ) == 0
    );
    return true;
}

#endif
