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
//   - Review-only keyboard sampling and chase-camera construction.
// - Must-Not:
//   - Approximate production vehicle dynamics or source-authored tuning.
// - Allows:
//   - Forward normalized input to the public Chaos vehicle interface.
// - Split-When:
//   - Input devices or review cameras require independent policies.
// - Merge-When:
//   - Another inspection Pawn owns the identical review controls.
// - Summary:
//   - Implements the vehicle drive-test Pawn.
// - Description:
//   - Samples local keys each frame and forwards normalized test controls.
// - Usage:
//   - Used only for interactive vehicle inspection levels.
// - Defaults:
//   - Releases every control when no local PlayerController is present.
//

//! Review-only vehicle keyboard controls and chase camera.

#include "Vehicles/SharVehicleDriveTestPawn.h"

#include "Camera/CameraComponent.h"
#include "ChaosVehicleMovementComponent.h"
#include "ChaosWheeledVehicleMovementComponent.h"
#include "Components/SkeletalMeshComponent.h"
#include "Engine/World.h"
#include "GameFramework/PlayerController.h"
#include "GameFramework/SpringArmComponent.h"
#include "InputCoreTypes.h"

namespace
{
constexpr float DriveTestCameraArmCentimeters = 650.0F;
constexpr float DriveTestCameraHeightCentimeters = 120.0F;
constexpr float DriveTestCameraPitchDegrees = -10.0F;
}

ASharVehicleDriveTestPawn::ASharVehicleDriveTestPawn(
    const FObjectInitializer& ObjectInitializer
)
    : Super(ObjectInitializer)
{
    PrimaryActorTick.bCanEverTick = true;
    AutoPossessPlayer = EAutoReceiveInput::Player0;

    USkeletalMeshComponent* VehicleMesh = GetMesh();
    if (VehicleMesh != nullptr)
    {
        VehicleMesh->SetSimulatePhysics(true);
        VehicleMesh->SetEnableGravity(true);
    }

    auto* WheeledMovement = Cast<UChaosWheeledVehicleMovementComponent>(
        GetVehicleMovementComponent()
    );
    if (WheeledMovement != nullptr)
    {
        FRichCurve* TorqueCurve =
            WheeledMovement->EngineSetup.TorqueCurve.GetRichCurve();
        TorqueCurve->AddKey(0.0F, 1.0F);
        TorqueCurve->AddKey(WheeledMovement->EngineSetup.MaxRPM, 1.0F);
    }

    CameraBoom = CreateDefaultSubobject<USpringArmComponent>(
        TEXT("DriveTestCameraBoom")
    );
    CameraBoom->SetupAttachment(RootComponent);
    CameraBoom->TargetArmLength = DriveTestCameraArmCentimeters;
    CameraBoom->SetRelativeLocation(FVector(
        0.0F,
        0.0F,
        DriveTestCameraHeightCentimeters
    ));
    CameraBoom->SetRelativeRotation(FRotator(
        DriveTestCameraPitchDegrees,
        0.0F,
        0.0F
    ));
    CameraBoom->bUsePawnControlRotation = false;

    ChaseCamera = CreateDefaultSubobject<UCameraComponent>(
        TEXT("DriveTestChaseCamera")
    );
    ChaseCamera->SetupAttachment(
        CameraBoom,
        USpringArmComponent::SocketName
    );
    ChaseCamera->bAutoActivate = true;
}

void ASharVehicleDriveTestPawn::BeginPlay()
{
    Super::BeginPlay();
    AlignCameraToWheelbase();
    if (USkeletalMeshComponent* VehicleMesh = GetMesh())
    {
        VehicleMesh->SetEnableGravity(true);
        VehicleMesh->WakeAllRigidBodies();
    }
}

void ASharVehicleDriveTestPawn::AlignCameraToWheelbase()
{
    const USkeletalMeshComponent* VehicleMesh = GetMesh();
    if (VehicleMesh == nullptr || CameraBoom == nullptr)
    {
        return;
    }

    const FVector RearCenter = 0.5 * (
        VehicleMesh->GetBoneLocation(TEXT("w0"), EBoneSpaces::ComponentSpace)
        + VehicleMesh->GetBoneLocation(TEXT("w1"), EBoneSpaces::ComponentSpace)
    );
    const FVector FrontCenter = 0.5 * (
        VehicleMesh->GetBoneLocation(TEXT("w2"), EBoneSpaces::ComponentSpace)
        + VehicleMesh->GetBoneLocation(TEXT("w3"), EBoneSpaces::ComponentSpace)
    );
    FVector Forward = FrontCenter - RearCenter;
    Forward.Z = 0.0;
    if (!Forward.Normalize())
    {
        return;
    }

    CameraBoom->SetRelativeRotation(FRotator(
        DriveTestCameraPitchDegrees,
        Forward.Rotation().Yaw,
        0.0
    ));
}

void ASharVehicleDriveTestPawn::Tick(const float DeltaSeconds)
{
    Super::Tick(DeltaSeconds);
    EnsureLocalPlayerPossession();
    ApplyKeyboardControls();
}

USpringArmComponent* ASharVehicleDriveTestPawn::GetDriveTestCameraBoom() const
{
    return CameraBoom;
}

UCameraComponent* ASharVehicleDriveTestPawn::GetDriveTestCamera() const
{
    return ChaseCamera;
}

void ASharVehicleDriveTestPawn::EnsureLocalPlayerPossession()
{
    UWorld* World = GetWorld();
    if (World == nullptr)
    {
        return;
    }

    APlayerController* PlayerController = World->GetFirstPlayerController();
    if (
        PlayerController != nullptr
        && PlayerController->IsLocalController()
        && PlayerController->GetPawn() != this
    )
    {
        PlayerController->Possess(this);
    }
}

void ASharVehicleDriveTestPawn::ApplyKeyboardControls()
{
    UChaosVehicleMovementComponent* Movement = GetVehicleMovementComponent();
    const APlayerController* PlayerController = Cast<APlayerController>(
        GetController()
    );
    if (Movement == nullptr)
    {
        return;
    }

    float Throttle = 0.0F;
    float Brake = 0.0F;
    float Steering = 0.0F;
    bool bHandbrake = false;
    if (PlayerController != nullptr)
    {
        Throttle = PlayerController->IsInputKeyDown(EKeys::W) ? 1.0F : 0.0F;
        Brake = PlayerController->IsInputKeyDown(EKeys::S) ? 1.0F : 0.0F;
        Steering =
            (PlayerController->IsInputKeyDown(EKeys::D) ? 1.0F : 0.0F)
            - (PlayerController->IsInputKeyDown(EKeys::A) ? 1.0F : 0.0F);
        bHandbrake = PlayerController->IsInputKeyDown(EKeys::SpaceBar);
    }

    Movement->SetThrottleInput(Throttle);
    Movement->SetBrakeInput(Brake);
    Movement->SetSteeringInput(Steering);
    Movement->SetHandbrakeInput(bHandbrake);
}
