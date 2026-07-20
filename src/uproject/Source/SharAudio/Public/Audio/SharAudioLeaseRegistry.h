// File: SharAudioLeaseRegistry.h
// Path: src/uproject/Source/SharAudio/Public/Audio/SharAudioLeaseRegistry.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: audio request, lease, completion, and owner-teardown state only; no audio engine ownership.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharAudio; reason=cohesive reflected audio lease lifecycle contract;
// split=extract playback observations if callback correlation expands;
// validation=validate.sh SharAudio plus Unreal automation; review=2027-01.

#pragma once

#include "Audio/SharAudioProfileDefinition.h"
#include "CoreMinimal.h"
#include "Engine/DataAsset.h"

#include "SharAudioLeaseRegistry.generated.h"

UENUM(BlueprintType)
enum class ESharAudioPlaybackResult : uint8
{
    Accepted,
    InvalidRequest,
    LeaseRequired,
    DuplicateRequest,
    NotFound,
    Completed,
    Cancelled,
};

USTRUCT(BlueprintType)
struct SHARAUDIO_API FSharAudioPlaybackRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Audio")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Audio")
    FName OwnerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Audio")
    FPrimaryAssetId ProfileId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Audio")
    FName LeaseId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Audio")
    ESharAudioPlaybackPolicy PlaybackPolicy =
        ESharAudioPlaybackPolicy::OneShot;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Audio")
    bool bLooping = false;
};

USTRUCT(BlueprintType)
struct SHARAUDIO_API FSharAudioPlaybackState
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Audio")
    FSharAudioPlaybackRequest Request;

    UPROPERTY(BlueprintReadOnly, Category = "Audio")
    ESharAudioPlaybackResult Result = ESharAudioPlaybackResult::Accepted;
};

UCLASS(BlueprintType)
class SHARAUDIO_API USharAudioLeaseRegistry final : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Audio")
    ESharAudioPlaybackResult BeginPlayback(
        const FSharAudioPlaybackRequest& Request
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Audio")
    bool CompletePlayback(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Audio")
    bool CancelPlayback(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Audio")
    int32 ReleaseOwner(const FName& OwnerId);

    UFUNCTION(BlueprintPure, Category = "SHAR|Audio")
    [[nodiscard]] ESharAudioPlaybackResult GetResult(
        const FName& RequestId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Audio")
    [[nodiscard]] int32 GetActiveCount() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Audio")
    [[nodiscard]] const TArray<FSharAudioPlaybackState>& GetStates() const;

private:
    UPROPERTY(Transient)
    TArray<FSharAudioPlaybackState> States;

    [[nodiscard]] FSharAudioPlaybackState* FindState(
        const FName& RequestId
    );
    [[nodiscard]] const FSharAudioPlaybackState* FindState(
        const FName& RequestId
    ) const;
    [[nodiscard]] static bool IsValidRequestIdentity(
        const FSharAudioPlaybackRequest& Request
    );
    [[nodiscard]] static bool RequestRequiresLease(
        const FSharAudioPlaybackRequest& Request
    );
};
