// File: SharMessagingTestFixtures.h
// Path: src/uproject/Source/SharMessaging/Private/Tests/SharMessagingTestFixtures.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: explicit transient schema, envelope, subscription, and router fixtures only.
// Specification: docs/technical/unreal/typed-event-and-observation-routing-runtime.md
// LARGE-FILE owner=SharMessaging; reason=cohesive typed messaging test fixtures;
// split=extract schema fixtures if additional routing scopes appear;
// validation=validate.sh SharMessaging plus Unreal automation; review=2027-01.

#pragma once

#include "Messaging/SharMessageSchemaCatalog.h"
#include "Messaging/SharWorldMessageRouterSubsystem.h"

struct FSharMessageSchemaFixture
{
    FName SchemaId;
    FName ChannelId;
    ESharMessageDeliveryPhase DeliveryPhase =
        ESharMessageDeliveryPhase::EndOfDomainTransaction;
    ESharMessageReplayPolicy ReplayPolicy = ESharMessageReplayPolicy::None;
    int32 MaximumRecursionDepth =
        FSharMessageSchemaDefinition::DefaultMaximumRecursionDepth;
};

struct FSharMessageSubscriptionFixture
{
    FName HandleId;
    FName SubscriberId;
    FName ChannelId;
    FName SubjectFilterId;
};

constexpr int32 DefaultFixtureCanonicalIdentityCount = 8;

struct FSharMessageEnvelopeFixture
{
    FName MessageId;
    FName SchemaId;
    FName ChannelId;
    FName SubjectId;
    int32 CausationDepth = 0;
};

inline FSharMessageSchemaDefinition MakeMessageSchema(
    const FSharMessageSchemaFixture& Fixture
)
{
    FSharMessageSchemaDefinition Definition;
    Definition.SchemaId = Fixture.SchemaId;
    Definition.ChannelId = Fixture.ChannelId;
    Definition.SchemaRevision = TEXT("sha256:schema_v1");
    Definition.OwningModuleId = FName(TEXT("shar_world"));
    Definition.PublisherFamilyId = FName(TEXT("world_lifecycle"));
    Definition.MessageKind = ESharMessageKind::LifecycleObservation;
    Definition.Scope = ESharMessageScope::World;
    Definition.DeliveryPhase = Fixture.DeliveryPhase;
    Definition.ReplayPolicy = Fixture.ReplayPolicy;
    Definition.MaximumRecursionDepth = Fixture.MaximumRecursionDepth;
    return Definition;
}

inline FSharMessageEnvelope MakeMessageEnvelope(
    const FSharMessageEnvelopeFixture& Fixture
)
{
    FSharMessageEnvelope Envelope;
    Envelope.MessageId = Fixture.MessageId;
    Envelope.SchemaId = Fixture.SchemaId;
    Envelope.ChannelId = Fixture.ChannelId;
    Envelope.SchemaRevision = TEXT("sha256:schema_v1");
    Envelope.Scope = ESharMessageScope::World;
    Envelope.ScopeId = FName(TEXT("springfield_world"));
    Envelope.PublisherId = FName(TEXT("world_runtime"));
    Envelope.FrameObservationId = FName(TEXT("frame_001"));
    Envelope.WorldId = FName(TEXT("springfield_world"));
    Envelope.SessionId = FName(TEXT("session_01"));
    Envelope.SubjectId = Fixture.SubjectId;
    Envelope.CorrelationId = FName(TEXT("correlation_01"));
    Envelope.CausationId = FName(TEXT("causation_01"));
    Envelope.TransactionId = FName(TEXT("transaction_01"));
    Envelope.WorldRevision = TEXT("sha256:world_v1");
    Envelope.CanonicalIdentityCount = DefaultFixtureCanonicalIdentityCount;
    Envelope.CausationDepth = Fixture.CausationDepth;
    return Envelope;
}

inline FSharSubscriptionHandle MakeMessageSubscription(
    const FSharMessageSubscriptionFixture& Fixture
)
{
    FSharSubscriptionHandle Handle;
    Handle.HandleId = Fixture.HandleId;
    Handle.ChannelId = Fixture.ChannelId;
    Handle.SubscriberId = Fixture.SubscriberId;
    Handle.ScopeId = FName(TEXT("springfield_world"));
    Handle.SubjectFilterId = Fixture.SubjectFilterId;
    Handle.WorldRevision = TEXT("sha256:world_v1");
    return Handle;
}

inline USharWorldMessageRouterSubsystem* MakeMessageRouter(
    USharMessageSchemaCatalog& Catalog
)
{
    auto* Router = NewObject<USharWorldMessageRouterSubsystem>();
    Router->ConfigureWorld(
        FName(TEXT("springfield_world")),
        TEXT("sha256:world_v1"),
        &Catalog
    );
    return Router;
}
