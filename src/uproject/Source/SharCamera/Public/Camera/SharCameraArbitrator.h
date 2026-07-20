// File: SharCameraArbitrator.h
// Path: src/uproject/Source/SharCamera/Public/Camera/SharCameraArbitrator.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic camera-request arbitration state only; no camera manager or view mutation.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharCamera; reason=cohesive reflected request and arbitration contract;
// split=extract request-state snapshots if external observation expands;
// validation=validate.sh SharCamera plus Unreal automation; review=2027-01.

#pragma once

#include "Camera/SharCameraProfileDefinition.h"
#include "CoreMinimal.h"
#include "Engine/DataAsset.h"

#include "SharCameraArbitrator.generated.h"

UENUM(BlueprintType)
enum class ESharCameraRequestStatus : uint8
{
    Rejected,
    Queued,
    Active,
    Superseded,
    Cancelled,
};

USTRUCT(BlueprintType)
struct SHARCAMERA_API FSharCameraRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Camera")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Camera")
    FName RequesterId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Camera")
    FPrimaryAssetId CameraProfileId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Camera")
    FName TargetId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Camera")
    ESharCameraPriorityClass PriorityClass =
        ESharCameraPriorityClass::Default;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Camera")
    int32 PriorityOffset = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Camera")
    bool bCanDefer = true;
};

USTRUCT(BlueprintType)
struct SHARCAMERA_API FSharCameraRequestState
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Camera")
    FSharCameraRequest Request;

    UPROPERTY(BlueprintReadOnly, Category = "Camera")
    ESharCameraRequestStatus Status = ESharCameraRequestStatus::Queued;
};

UCLASS(BlueprintType)
class SHARCAMERA_API USharCameraArbitrator final : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Camera")
    ESharCameraRequestStatus SubmitRequest(
        const FSharCameraRequest& Request
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Camera")
    bool CancelRequest(const FName& RequestId);

    UFUNCTION(BlueprintPure, Category = "SHAR|Camera")
    [[nodiscard]] FName GetActiveRequestId() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Camera")
    [[nodiscard]] FPrimaryAssetId GetActiveProfileId() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Camera")
    [[nodiscard]] ESharCameraRequestStatus GetRequestStatus(
        const FName& RequestId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Camera")
    [[nodiscard]] const TArray<FSharCameraRequestState>& GetRequests() const;

private:
    UPROPERTY(Transient)
    TArray<FSharCameraRequestState> Requests;

    UPROPERTY(Transient)
    FName ActiveRequestId;

    UPROPERTY(Transient)
    FPrimaryAssetId ActiveProfileId;

    [[nodiscard]] FSharCameraRequestState* FindRequestState(
        const FName& RequestId
    );
    [[nodiscard]] const FSharCameraRequestState* FindRequestState(
        const FName& RequestId
    ) const;
    [[nodiscard]] static bool IsValidRequest(
        const FSharCameraRequest& Request
    );
    [[nodiscard]] static bool Outranks(
        const FSharCameraRequest& Candidate,
        const FSharCameraRequest& Current
    );
    [[nodiscard]] static int32 GetPriorityRank(
        ESharCameraPriorityClass PriorityClass
    );
    void ResetActiveRequestState();
    [[nodiscard]] FSharCameraRequestState* FindBestQueuedRequest();
    void SupersedeNonDeferrableRequests(
        const FSharCameraRequestState* BestState
    );
    void ActivateRequest(FSharCameraRequestState* BestState);
    void RecalculateActiveRequest();
};
