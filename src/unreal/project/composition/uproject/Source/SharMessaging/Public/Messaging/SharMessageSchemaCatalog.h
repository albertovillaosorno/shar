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
//   - Shar message schema catalog composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar message schema catalog composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar message schema catalog composition module.

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
