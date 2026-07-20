// File: SharAudioProfileDefinition.h
// Path: src/uproject/Source/SharAudio/Public/Audio/SharAudioProfileDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable gameplay audio source, routing, residency, and lifecycle policy metadata only.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharAudio; reason=cohesive reflected audio profile contract;
// split=extract parameter schemas if independently versioned assets are introduced;
// validation=validate.sh SharAudio plus Unreal automation; review=2027-01.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharAudioProfileDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharAudioRole : uint8
{
    GameplayEffect,
    UserInterface,
    Ambience,
    Vehicle,
    Dialogue,
    Cinematic,
    Music,
};

UENUM(BlueprintType)
enum class ESharAudioPlaybackPolicy : uint8
{
    OneShot,
    FiniteLoop,
    LeasedContinuous,
    Queued,
    Attached,
    OwnerScopedPersistent,
};

UENUM(BlueprintType)
enum class ESharAudioPausePolicy : uint8
{
    Pause,
    Continue,
    Duck,
    Virtualize,
    Stop,
    Restart,
};

UENUM(BlueprintType)
enum class ESharAudioCompletionPolicy : uint8
{
    Observable,
    Ignored,
    ChainedPresentation,
    Barrier,
};

UENUM(BlueprintType)
enum class ESharAudioNetworkPolicy : uint8
{
    LocalOnly,
    Authority,
    Replicated,
    Predicted,
    OwnerOnly,
};

UENUM(BlueprintType)
enum class ESharAudioFallbackPolicy : uint8
{
    MissingOptionalLayer,
    AlternateSource,
    SilentResult,
    ActivationFailure,
};

UCLASS(BlueprintType)
class SHARAUDIO_API USharAudioProfileDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    static constexpr int32 DefaultMaximumConcurrentInstances = 8;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Source")
    FName SourceAssetId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Source")
    ESharAudioRole Role = ESharAudioRole::GameplayEffect;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Playback")
    ESharAudioPlaybackPolicy PlaybackPolicy =
        ESharAudioPlaybackPolicy::OneShot;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Parameters")
    FName ParameterSchemaId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    FName AttenuationPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    FName ConcurrencyPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    FName RoutingPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    FName ResidencyPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Playback")
    ESharAudioPausePolicy PausePolicy = ESharAudioPausePolicy::Pause;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Playback")
    ESharAudioCompletionPolicy CompletionPolicy =
        ESharAudioCompletionPolicy::Observable;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Network")
    ESharAudioNetworkPolicy NetworkPolicy =
        ESharAudioNetworkPolicy::LocalOnly;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Fallback")
    ESharAudioFallbackPolicy FallbackPolicy =
        ESharAudioFallbackPolicy::SilentResult;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Concurrency")
    int32 MaximumConcurrentInstances = DefaultMaximumConcurrentInstances;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Spatial")
    bool bPositional = false;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

    [[nodiscard]] static bool RequiresLease(
        ESharAudioPlaybackPolicy PlaybackPolicy
    );

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
