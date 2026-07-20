// File: SharWorldMessagePublication.cpp
// Path: src/uproject/Source/SharMessaging/Private/Messaging/SharWorldMessagePublication.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: accepted envelope validation, phase dispatch, replay evidence, and delivery consumption only.
// Specification: docs/technical/unreal/typed-event-and-observation-routing-runtime.md
// LARGE-FILE owner=SharMessaging; reason=cohesive publication and phase-delivery implementation;
// split=extract delivery storage if typed callback adapters are introduced;
// validation=validate.sh SharMessaging plus Unreal automation; review=2027-01.

#include "Messaging/SharWorldMessageRouterSubsystem.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Messaging/SharMessageSchemaCatalog.h"

static bool IsCanonicalEnvelopeIdentity(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsCanonicalEnvelopeOrNone(const FName& Candidate)
{
    return Candidate.IsNone() || IsCanonicalEnvelopeIdentity(Candidate);
}

static bool HasInvalidEnvelopeIdentity(const FSharMessageEnvelope& Envelope)
{
    return !IsCanonicalEnvelopeIdentity(Envelope.MessageId)
        || !USharMessageSchemaCatalog::IsSemanticChannel(Envelope.ChannelId)
        || !IsCanonicalEnvelopeIdentity(Envelope.SchemaId)
        || !IsCanonicalEnvelopeIdentity(Envelope.ScopeId)
        || !IsCanonicalEnvelopeIdentity(Envelope.PublisherId)
        || !IsCanonicalEnvelopeOrNone(Envelope.FrameObservationId)
        || !IsCanonicalEnvelopeOrNone(Envelope.SessionId)
        || !IsCanonicalEnvelopeOrNone(Envelope.LocalPlayerId)
        || !IsCanonicalEnvelopeOrNone(Envelope.SubjectId)
        || !IsCanonicalEnvelopeOrNone(Envelope.CorrelationId)
        || !IsCanonicalEnvelopeOrNone(Envelope.CausationId)
        || !IsCanonicalEnvelopeOrNone(Envelope.TransactionId);
}

static bool HasInvalidEnvelopeBounds(
    const FSharMessageEnvelope& Envelope,
    const FSharMessageSchemaDefinition& Schema
)
{
    return Envelope.PublicationSequence != 0
        || Envelope.CanonicalIdentityCount < 0
        || Envelope.CanonicalIdentityCount > Schema.MaximumCanonicalIdentities
        || Envelope.CausationDepth < 0;
}

static ESharMessageRouteResult ClassifyEnvelopeBinding(
    const FSharMessageEnvelope& Envelope,
    const FSharMessageSchemaDefinition& Schema,
    const FName& WorldId,
    const FString& WorldRevision
)
{
    if (Envelope.ChannelId != Schema.ChannelId
        || Envelope.SchemaRevision != Schema.SchemaRevision)
    {
        return ESharMessageRouteResult::ChannelMismatch;
    }
    if (Schema.Scope != ESharMessageScope::World
        || Envelope.Scope != ESharMessageScope::World)
    {
        return ESharMessageRouteResult::ScopeMismatch;
    }
    if (Envelope.ScopeId != WorldId || Envelope.WorldId != WorldId
        || Envelope.WorldRevision != WorldRevision)
    {
        return ESharMessageRouteResult::StaleWorld;
    }
    return ESharMessageRouteResult::Accepted;
}

ESharMessageRouteResult USharWorldMessageRouterSubsystem::ValidateEnvelope(
    const FSharMessageEnvelope& Envelope,
    const FSharMessageSchemaDefinition*& OutSchema
) const
{
    OutSchema = nullptr;
    if (Catalog == nullptr)
    {
        return ESharMessageRouteResult::CatalogMissing;
    }
    const FSharMessageSchemaDefinition* Schema =
        Catalog->FindSchema(Envelope.SchemaId);
    if (Schema == nullptr)
    {
        return ESharMessageRouteResult::SchemaMissing;
    }
    OutSchema = Schema;
    const ESharMessageRouteResult BindingResult = ClassifyEnvelopeBinding(
        Envelope,
        *Schema,
        WorldId,
        WorldRevision
    );
    if (BindingResult != ESharMessageRouteResult::Accepted)
    {
        return BindingResult;
    }
    if (HasInvalidEnvelopeIdentity(Envelope)
        || HasInvalidEnvelopeBounds(Envelope, *Schema))
    {
        return ESharMessageRouteResult::InvalidRequest;
    }
    if (Envelope.CausationDepth > Schema->MaximumRecursionDepth)
    {
        return ESharMessageRouteResult::RecursionLimit;
    }
    const bool bDuplicate = SeenMessageIds.ContainsByPredicate(
        [&Envelope](const FName& MessageId)
        {
            return MessageId == Envelope.MessageId;
        }
    );
    return bDuplicate
        ? ESharMessageRouteResult::DuplicateMessage
        : ESharMessageRouteResult::Accepted;
}

void USharWorldMessageRouterSubsystem::Dispatch(
    const FSharMessageEnvelope& Envelope
)
{
    const TArray<FSharSubscriptionHandle> SubscriptionSnapshot = Subscriptions;
    for (const FSharSubscriptionHandle& Handle : SubscriptionSnapshot)
    {
        if (!MatchesSubscription(Handle, Envelope))
        {
            continue;
        }
        FSharMessageDelivery Delivery;
        Delivery.HandleId = Handle.HandleId;
        Delivery.SubscriberId = Handle.SubscriberId;
        Delivery.Envelope = Envelope;
        Deliveries.Add(Delivery);
    }
    DispatchedMessages.Add(Envelope);
}

ESharMessageRouteResult USharWorldMessageRouterSubsystem::Publish(
    const FSharMessageEnvelope& Envelope
)
{
    const FSharMessageSchemaDefinition* Schema = nullptr;
    const ESharMessageRouteResult ValidationResult =
        ValidateEnvelope(Envelope, Schema);
    if (ValidationResult != ESharMessageRouteResult::Accepted)
    {
        return ValidationResult;
    }
    if (Schema == nullptr)
    {
        return ESharMessageRouteResult::SchemaMissing;
    }

    FSharMessageEnvelope AcceptedEnvelope = Envelope;
    AcceptedEnvelope.PublicationSequence = NextPublicationSequence;
    ++NextPublicationSequence;
    SeenMessageIds.Add(AcceptedEnvelope.MessageId);
    if (Schema->DeliveryPhase
        == ESharMessageDeliveryPhase::ImmediateReadOnly)
    {
        Dispatch(AcceptedEnvelope);
        return ESharMessageRouteResult::Accepted;
    }

    FSharPendingMessage Pending;
    Pending.Envelope = AcceptedEnvelope;
    Pending.DeliveryPhase = Schema->DeliveryPhase;
    PendingMessages.Add(Pending);
    return ESharMessageRouteResult::Accepted;
}

int32 USharWorldMessageRouterSubsystem::FlushPhase(
    const ESharMessageDeliveryPhase DeliveryPhase
)
{
    if (Catalog == nullptr)
    {
        return 0;
    }
    int32 DispatchCount = 0;
    for (FSharPendingMessage& Pending : PendingMessages)
    {
        if (Pending.bDispatched || Pending.DeliveryPhase != DeliveryPhase)
        {
            continue;
        }
        const FSharMessageSchemaDefinition* Schema =
            Catalog->FindSchema(Pending.Envelope.SchemaId);
        if (Schema == nullptr)
        {
            Pending.bDispatched = true;
            continue;
        }
        Dispatch(Pending.Envelope);
        Pending.bDispatched = true;
        ++DispatchCount;
    }
    return DispatchCount;
}

ESharMessageRouteResult USharWorldMessageRouterSubsystem::ConsumeNext(
    const FName& HandleId,
    FSharMessageEnvelope& OutEnvelope
)
{
    const FSharSubscriptionHandle* Handle = FindSubscription(HandleId);
    if (Handle == nullptr)
    {
        return ESharMessageRouteResult::SubscriptionMissing;
    }
    if (!Handle->bActive)
    {
        return ESharMessageRouteResult::AlreadyReleased;
    }
    FSharMessageDelivery* NextDelivery = nullptr;
    for (FSharMessageDelivery& Delivery : Deliveries)
    {
        const bool bEarlierMatch =
            Delivery.HandleId == HandleId
            && !Delivery.bConsumed
            && (NextDelivery == nullptr
                || Delivery.Envelope.PublicationSequence
                    < NextDelivery->Envelope.PublicationSequence);
        if (bEarlierMatch)
        {
            NextDelivery = &Delivery;
        }
    }
    if (NextDelivery == nullptr)
    {
        return ESharMessageRouteResult::NothingPending;
    }
    NextDelivery->bConsumed = true;
    OutEnvelope = NextDelivery->Envelope;
    return ESharMessageRouteResult::Accepted;
}
