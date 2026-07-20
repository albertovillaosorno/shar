// File: SharMessageSchemaCatalog.h
// Path: src/uproject/Source/SharMessaging/Public/Messaging/SharMessageSchemaCatalog.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: generated schema and delivery-policy metadata only; owning modules retain typed payload definitions.
// Specification: docs/technical/unreal/typed-event-and-observation-routing-runtime.md
// LARGE-FILE owner=SharMessaging; reason=cohesive reflected schema and policy vocabulary;
// split=extract alias metadata if historical aliases become runtime-visible;
// validation=validate.sh SharMessaging plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"

#include "SharMessageSchemaCatalog.generated.h"

UENUM(BlueprintType)
enum class ESharMessageKind : uint8
{
    DomainEvent,
    LifecycleObservation,
    PresentationCue,
    DiagnosticObservation,
    ExternalAdapterObservation,
};

UENUM(BlueprintType)
enum class ESharMessageScope : uint8
{
    Process,
    GameInstance,
    World,
    Session,
    LocalPlayer,
    Entity,
};

UENUM(BlueprintType)
enum class ESharMessageDeliveryPhase : uint8
{
    ImmediateReadOnly,
    EndOfDomainTransaction,
    EndOfTickGroup,
    NextWorldFrame,
    AfterApplicationModeCommit,
    AsyncAdapterCompletion,
};

UENUM(BlueprintType)
enum class ESharMessageReplayPolicy : uint8
{
    None,
    LastAccepted,
};

USTRUCT(BlueprintType)
struct SHARMESSAGING_API FSharMessageSchemaDefinition
{
    GENERATED_BODY()

    static constexpr int32 DefaultMaximumCanonicalIdentities = 16;
    static constexpr int32 DefaultMaximumRecursionDepth = 4;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName SchemaId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ChannelId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString SchemaRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Ownership")
    FName OwningModuleId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Ownership")
    FName PublisherFamilyId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Message")
    ESharMessageKind MessageKind = ESharMessageKind::DomainEvent;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Routing")
    ESharMessageScope Scope = ESharMessageScope::World;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Routing")
    ESharMessageDeliveryPhase DeliveryPhase =
        ESharMessageDeliveryPhase::EndOfDomainTransaction;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Routing")
    ESharMessageReplayPolicy ReplayPolicy = ESharMessageReplayPolicy::None;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Bounds")
    int32 MaximumCanonicalIdentities = DefaultMaximumCanonicalIdentities;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Bounds")
    int32 MaximumRecursionDepth = DefaultMaximumRecursionDepth;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Lifetime")
    bool bDurable = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Lifetime")
    bool bAllowsTransientObjectReferences = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Availability")
    bool bDevelopmentOnly = false;
};

UCLASS(BlueprintType)
class SHARMESSAGING_API USharMessageSchemaCatalog final : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Messaging")
    bool RegisterSchema(const FSharMessageSchemaDefinition& Definition);

    UFUNCTION(BlueprintPure, Category = "SHAR|Messaging")
    [[nodiscard]] int32 GetSchemaCount() const;

    [[nodiscard]] const FSharMessageSchemaDefinition* FindSchema(
        const FName& SchemaId
    ) const;

    [[nodiscard]] const FSharMessageSchemaDefinition* FindSchemaByChannel(
        const FName& ChannelId
    ) const;

    [[nodiscard]] static bool IsSemanticChannel(const FName& ChannelId);

private:
    UPROPERTY(Transient)
    TArray<FSharMessageSchemaDefinition> Schemas;
};
