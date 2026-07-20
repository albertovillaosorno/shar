// File: SharWorldMessageSubscriptions.cpp
// Path: src/uproject/Source/SharMessaging/Private/Messaging/SharWorldMessageSubscriptions.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: world configuration, subscription ownership, replay, and teardown only; no domain fact creation.
// Specification: docs/technical/unreal/typed-event-and-observation-routing-runtime.md
// LARGE-FILE owner=SharMessaging; reason=cohesive subscription and world-lifetime implementation;
// split=extract replay storage if additional replay policies appear;
// validation=validate.sh SharMessaging plus Unreal automation; review=2027-01.

#include "Messaging/SharWorldMessageRouterSubsystem.h"

#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Messaging/SharMessageSchemaCatalog.h"

static bool IsWorldRevision(const FString& Revision)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

bool USharWorldMessageRouterSubsystem::IsCanonicalOrNone(
    const FName& Candidate
)
{
    return Candidate.IsNone()
        || USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

FSharSubscriptionHandle* USharWorldMessageRouterSubsystem::FindSubscription(
    const FName& HandleId
)
{
    return Algo::FindByPredicate(
        Subscriptions,
        [&HandleId](const FSharSubscriptionHandle& Handle)
        {
            return Handle.HandleId == HandleId;
        }
    );
}

const FSharSubscriptionHandle*
USharWorldMessageRouterSubsystem::FindSubscription(const FName& HandleId) const
{
    return Algo::FindByPredicate(
        Subscriptions,
        [&HandleId](const FSharSubscriptionHandle& Handle)
        {
            return Handle.HandleId == HandleId;
        }
    );
}

bool USharWorldMessageRouterSubsystem::ConfigureWorld(
    const FName& InWorldId,
    const FString& InWorldRevision,
    USharMessageSchemaCatalog* InCatalog
)
{
    const bool bInvalid =
        !USharPrimaryContentDefinition::IsCanonicalIdentifier(InWorldId)
        || !IsWorldRevision(InWorldRevision)
        || InCatalog == nullptr;
    if (bInvalid)
    {
        return false;
    }
    WorldId = InWorldId;
    WorldRevision = InWorldRevision;
    Catalog = InCatalog;
    Subscriptions.Reset();
    Deliveries.Reset();
    PendingMessages.Reset();
    DispatchedMessages.Reset();
    SeenMessageIds.Reset();
    NextPublicationSequence = 1;
    return true;
}

ESharMessageRouteResult USharWorldMessageRouterSubsystem::Subscribe(
    const FSharSubscriptionHandle& Handle
)
{
    if (Catalog == nullptr)
    {
        return ESharMessageRouteResult::CatalogMissing;
    }
    const bool bInvalidIdentity =
        !USharPrimaryContentDefinition::IsCanonicalIdentifier(Handle.HandleId)
        || !USharMessageSchemaCatalog::IsSemanticChannel(Handle.ChannelId)
        || !USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Handle.SubscriberId
        )
        || !IsCanonicalOrNone(Handle.SubjectFilterId);
    if (bInvalidIdentity || Handle.ScopeId != WorldId
        || Handle.WorldRevision != WorldRevision)
    {
        return ESharMessageRouteResult::InvalidRequest;
    }
    if (FindSubscription(Handle.HandleId) != nullptr)
    {
        return ESharMessageRouteResult::InvalidRequest;
    }
    const FSharMessageSchemaDefinition* Schema =
        Catalog->FindSchemaByChannel(Handle.ChannelId);
    if (Schema == nullptr)
    {
        return ESharMessageRouteResult::SchemaMissing;
    }
    if (Schema->Scope != ESharMessageScope::World)
    {
        return ESharMessageRouteResult::ScopeMismatch;
    }

    FSharSubscriptionHandle StoredHandle = Handle;
    StoredHandle.bActive = true;
    Subscriptions.Add(StoredHandle);
    ReplayLastAccepted(StoredHandle, *Schema);
    return ESharMessageRouteResult::Accepted;
}

ESharMessageRouteResult
USharWorldMessageRouterSubsystem::ReleaseSubscription(const FName& HandleId)
{
    FSharSubscriptionHandle* Handle = FindSubscription(HandleId);
    if (Handle == nullptr)
    {
        return ESharMessageRouteResult::SubscriptionMissing;
    }
    if (!Handle->bActive)
    {
        return ESharMessageRouteResult::AlreadyReleased;
    }
    Handle->bActive = false;
    return ESharMessageRouteResult::Accepted;
}

int32 USharWorldMessageRouterSubsystem::ReleaseSubscriber(
    const FName& SubscriberId
)
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(SubscriberId))
    {
        return 0;
    }
    int32 ReleasedCount = 0;
    for (FSharSubscriptionHandle& Handle : Subscriptions)
    {
        if (Handle.SubscriberId == SubscriberId && Handle.bActive)
        {
            Handle.bActive = false;
            ++ReleasedCount;
        }
    }
    return ReleasedCount;
}

bool USharWorldMessageRouterSubsystem::MatchesSubscription(
    const FSharSubscriptionHandle& Handle,
    const FSharMessageEnvelope& Envelope
)
{
    const bool bFilterMatches =
        Handle.SubjectFilterId.IsNone()
        || Handle.SubjectFilterId == Envelope.SubjectId;
    return Handle.bActive
        && Handle.ChannelId == Envelope.ChannelId
        && Handle.ScopeId == Envelope.ScopeId
        && Handle.WorldRevision == Envelope.WorldRevision
        && bFilterMatches;
}

void USharWorldMessageRouterSubsystem::ReplayLastAccepted(
    const FSharSubscriptionHandle& Handle,
    const FSharMessageSchemaDefinition& Schema
)
{
    if (Schema.ReplayPolicy != ESharMessageReplayPolicy::LastAccepted)
    {
        return;
    }
    const FSharMessageEnvelope* Latest = nullptr;
    for (const FSharMessageEnvelope& Envelope : DispatchedMessages)
    {
        const bool bNewerMatch =
            Envelope.ChannelId == Handle.ChannelId
            && MatchesSubscription(Handle, Envelope)
            && (Latest == nullptr
                || Envelope.PublicationSequence > Latest->PublicationSequence);
        if (bNewerMatch)
        {
            Latest = &Envelope;
        }
    }
    if (Latest == nullptr)
    {
        return;
    }
    FSharMessageDelivery Delivery;
    Delivery.HandleId = Handle.HandleId;
    Delivery.SubscriberId = Handle.SubscriberId;
    Delivery.Envelope = *Latest;
    Deliveries.Add(Delivery);
}

int32 USharWorldMessageRouterSubsystem::TeardownWorld()
{
    int32 ReleasedCount = 0;
    for (FSharSubscriptionHandle& Handle : Subscriptions)
    {
        if (Handle.bActive)
        {
            Handle.bActive = false;
            ++ReleasedCount;
        }
    }
    for (FSharPendingMessage& Pending : PendingMessages)
    {
        Pending.bDispatched = true;
    }
    Catalog = nullptr;
    WorldId = FName();
    WorldRevision = FString();
    return ReleasedCount;
}

int32 USharWorldMessageRouterSubsystem::GetDeliveryCount(
    const FName& HandleId
) const
{
    int32 DeliveryCount = 0;
    for (const FSharMessageDelivery& Delivery : Deliveries)
    {
        DeliveryCount += Delivery.HandleId == HandleId && !Delivery.bConsumed
            ? 1
            : 0;
    }
    return DeliveryCount;
}

int64 USharWorldMessageRouterSubsystem::GetLastDeliverySequence(
    const FName& HandleId
) const
{
    int64 LastSequence = 0;
    for (const FSharMessageDelivery& Delivery : Deliveries)
    {
        if (Delivery.HandleId == HandleId
            && Delivery.Envelope.PublicationSequence > LastSequence)
        {
            LastSequence = Delivery.Envelope.PublicationSequence;
        }
    }
    return LastSequence;
}

int32 USharWorldMessageRouterSubsystem::GetActiveSubscriptionCount() const
{
    int32 ActiveCount = 0;
    for (const FSharSubscriptionHandle& Handle : Subscriptions)
    {
        ActiveCount += Handle.bActive ? 1 : 0;
    }
    return ActiveCount;
}

int32 USharWorldMessageRouterSubsystem::GetPendingMessageCount() const
{
    int32 PendingCount = 0;
    for (const FSharPendingMessage& Pending : PendingMessages)
    {
        PendingCount += Pending.bDispatched ? 0 : 1;
    }
    return PendingCount;
}
