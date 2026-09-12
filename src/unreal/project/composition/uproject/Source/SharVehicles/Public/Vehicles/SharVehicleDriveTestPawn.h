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
//   - Local keyboard input and chase camera for vehicle inspection.
// - Must-Not:
//   - Define production handling, content identity, or vehicle tuning policy.
// - Allows:
//   - Drive a constructed SHAR vehicle through public Chaos input APIs.
// - Split-When:
//   - Review input or camera behavior gains independent test lifecycles.
// - Merge-When:
//   - A project-wide vehicle inspection harness owns the same boundary.
// - Summary:
//   - Provides an explicitly non-authoritative vehicle drive-test Pawn.
// - Description:
//   - Adds review-only keyboard controls and a chase camera to the base Pawn.
//   - Reclaims local Player0 when the level GameMode spawns a default Pawn.
// - Usage:
//   - Place in an inspection level after verified vehicle construction.
// - Defaults:
//   - W accelerates, S brakes, A/D steer, and Space uses the handbrake.
//

//! Review-only keyboard and chase-camera vehicle harness.

#pragma once

#include "CoreMinimal.h"
#include "Vehicles/SharVehiclePawn.h"

#include "SharVehicleDriveTestPawn.generated.h"

class UCameraComponent;
class USpringArmComponent;

UCLASS(BlueprintType, Blueprintable)
class SHARVEHICLES_API ASharVehicleDriveTestPawn final
    : public ASharVehiclePawn
{
    GENERATED_BODY()

public:
    explicit ASharVehicleDriveTestPawn(
        const FObjectInitializer& ObjectInitializer = FObjectInitializer::Get()
    );

    void Tick(float DeltaSeconds) override;

protected:
    void BeginPlay() override;

public:
    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle|Drive Test")
    [[nodiscard]] USpringArmComponent* GetDriveTestCameraBoom() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle|Drive Test")
    [[nodiscard]] UCameraComponent* GetDriveTestCamera() const;

private:
    void AlignCameraToWheelbase();

    void EnsureLocalPlayerPossession();

    void ApplyKeyboardControls();

    UPROPERTY(
        VisibleAnywhere,
        BlueprintReadOnly,
        Category = "Drive Test",
        meta = (AllowPrivateAccess = "true")
    )
    TObjectPtr<USpringArmComponent> CameraBoom;

    UPROPERTY(
        VisibleAnywhere,
        BlueprintReadOnly,
        Category = "Drive Test",
        meta = (AllowPrivateAccess = "true")
    )
    TObjectPtr<UCameraComponent> ChaseCamera;
};
