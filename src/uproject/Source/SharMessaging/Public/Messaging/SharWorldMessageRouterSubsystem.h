// File: SharWorldMessageRouterSubsystem.h
// Path: src/uproject/Source/SharMessaging/Public/Messaging/SharWorldMessageRouterSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: world-scoped envelope routing and subscription lifetime only; payload schemas and domain facts remain owner-defined.
// Specification: docs/technical/unreal/typed-event-and-observation-routing-runtime.md
// LARGE-FILE owner=SharMessaging; reason=cohesive reflected routing, subscription, delivery, and correlation contract;
// split=extract diagnostics if delivery evidence becomes persistent;
// validation=validate.sh SharMessaging plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/WorldSubsystem.h"

#include "Messaging/SharMessageSchemaCatalog.h"
#include "SharWorldMessageRouterSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharMessageRouteResult : uint8
{
    Accepted,
    InvalidRequest,
    CatalogMissing,
    SchemaMissing,
    ChannelMismatch,
    ScopeMismatch,
    StaleWorld,
    DuplicateMessage,
    RecursionLimit,
    SubscriptionMissing,
    AlreadyReleased,
    NothingPending,
};

USTRUCT(BlueprintType)
struct SHARMESSAGING_API FSharMessageEnvelope
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName MessageId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ChannelId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName SchemaId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString SchemaRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Scope")
    ESharMessageScope Scope = ESharMessageScope::World;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Scope")
    FName ScopeId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Ownership")
    FName PublisherId;

    UPROPERTY(BlueprintReadOnly, Category = "Ordering")
    int64 PublicationSequence = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Observation")
    FName FrameObservationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Correlation")
    FName WorldId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Correlation")
    FName SessionId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Correlation")
    FName LocalPlayerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Correlation")
    FName SubjectId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Correlation")
    FName CorrelationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Correlation")
    FName CausationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Correlation")
    FName TransactionId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString WorldRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Bounds")
    int32 CanonicalIdentityCount = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Bounds")
    int32 CausationDepth = 0;
};

USTRUCT(BlueprintType)
struct SHARMESSAGING_API FSharSubscriptionHandle
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName HandleId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ChannelId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName SubscriberId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Scope")
    FName ScopeId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Scope")
    FName SubjectFilterId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString WorldRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Lifetime")
    bool bActive = true;
};

USTRUCT(BlueprintType)
struct SHARMESSAGING_API FSharMessageDelivery
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Subscription")
    FName HandleId;

    UPROPERTY(BlueprintReadOnly, Category = "Subscription")
    FName SubscriberId;

    UPROPERTY(BlueprintReadOnly, Category = "Message")
    FSharMessageEnvelope Envelope;

    UPROPERTY(BlueprintReadOnly, Category = "Delivery")
    bool bConsumed = false;
};

USTRUCT()
struct FSharPendingMessage
{
    GENERATED_BODY()

    UPROPERTY()
    FSharMessageEnvelope Envelope;

    UPROPERTY()
    ESharMessageDeliveryPhase DeliveryPhase =
        ESharMessageDeliveryPhase::EndOfDomainTransaction;

    UPROPERTY()
    bool bDispatched = false;
};

UCLASS()
class SHARMESSAGING_API USharWorldMessageRouterSubsystem final
    : public UWorldSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Messaging")
    bool ConfigureWorld(
        const FName& InWorldId,
        const FString& InWorldRevision,
        USharMessageSchemaCatalog* InCatalog
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Messaging")
    ESharMessageRouteResult Subscribe(const FSharSubscriptionHandle& Handle);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Messaging")
    ESharMessageRouteResult ReleaseSubscription(const FName& HandleId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Messaging")
    int32 ReleaseSubscriber(const FName& SubscriberId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Messaging")
    ESharMessageRouteResult Publish(const FSharMessageEnvelope& Envelope);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Messaging")
    int32 FlushPhase(ESharMessageDeliveryPhase DeliveryPhase);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Messaging")
    ESharMessageRouteResult ConsumeNext(
        const FName& HandleId,
        FSharMessageEnvelope& OutEnvelope
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Messaging")
    int32 TeardownWorld();

    UFUNCTION(BlueprintPure, Category = "SHAR|Messaging")
    [[nodiscard]] int32 GetDeliveryCount(const FName& HandleId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Messaging")
    [[nodiscard]] int64 GetLastDeliverySequence(const FName& HandleId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Messaging")
    [[nodiscard]] int32 GetActiveSubscriptionCount() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Messaging")
    [[nodiscard]] int32 GetPendingMessageCount() const;

private:
    UPROPERTY(Transient)
    FName WorldId;

    UPROPERTY(Transient)
    FString WorldRevision;

    UPROPERTY(Transient)
    USharMessageSchemaCatalog* Catalog = nullptr;

    UPROPERTY(Transient)
    TArray<FSharSubscriptionHandle> Subscriptions;

    UPROPERTY(Transient)
    TArray<FSharMessageDelivery> Deliveries;

    UPROPERTY(Transient)
    TArray<FSharPendingMessage> PendingMessages;

    UPROPERTY(Transient)
    TArray<FSharMessageEnvelope> DispatchedMessages;

    UPROPERTY(Transient)
    TArray<FName> SeenMessageIds;

    UPROPERTY(Transient)
    int64 NextPublicationSequence = 1;

    [[nodiscard]] FSharSubscriptionHandle* FindSubscription(
        const FName& HandleId
    );
    [[nodiscard]] const FSharSubscriptionHandle* FindSubscription(
        const FName& HandleId
    ) const;
    [[nodiscard]] ESharMessageRouteResult ValidateEnvelope(
        const FSharMessageEnvelope& Envelope,
        const FSharMessageSchemaDefinition*& OutSchema
    ) const;
    void Dispatch(const FSharMessageEnvelope& Envelope);
    void ReplayLastAccepted(
        const FSharSubscriptionHandle& Handle,
        const FSharMessageSchemaDefinition& Schema
    );
    [[nodiscard]] static bool IsCanonicalOrNone(const FName& Candidate);
    [[nodiscard]] static bool MatchesSubscription(
        const FSharSubscriptionHandle& Handle,
        const FSharMessageEnvelope& Envelope
    );
};
