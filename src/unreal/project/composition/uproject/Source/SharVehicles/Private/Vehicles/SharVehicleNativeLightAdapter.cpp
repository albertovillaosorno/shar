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
//   - Native headlight emitter construction, attachment, refresh, and teardown.
// - Must-Not:
//   - Infer source tuning, mutate material slots, or create glow billboards.
// - Allows:
//   - Convert reviewed hardpoints into transient spot light components.
// - Split-When:
//   - Another light role requires different component or photometric semantics.
// - Merge-When:
//   - Vehicle construction owns the identical transient presentation lifecycle.
// - Summary:
//   - Implements the semantic-to-native vehicle headlight adapter.
// - Description:
//   - Creates one spot light per unique headlight bone, never per part.
// - Usage:
//   - Called after native vehicle construction has installed the presentation.
// - Defaults:
//   - Invalid, incomplete, or mismatched configuration fails without mutation.
//

//! Semantic vehicle headlight state to native Unreal spot light output.

#include "Vehicles/SharVehicleNativeLightAdapter.h"

#include "Vehicles/SharVehiclePawn.h"
#include "Vehicles/SharVehiclePresentationDefinition.h"
#include "Vehicles/SharVehiclePresentationState.h"

#include "Components/SkeletalMeshComponent.h"
#include "Components/SpotLightComponent.h"
#include "Engine/EngineTypes.h"

namespace
{
bool IsFiniteColor(const FLinearColor& Color)
{
    return FMath::IsFinite(Color.R)
        && FMath::IsFinite(Color.G)
        && FMath::IsFinite(Color.B)
        && FMath::IsFinite(Color.A);
}

bool IsValidHeadlightConfiguration(
    const FSharVehicleNativeHeadlightConfiguration& Configuration
)
{
    const FVector& Direction = Configuration.BoneLocalDirection;
    const bool bDirectionValid =
        FMath::IsFinite(Direction.X)
        && FMath::IsFinite(Direction.Y)
        && FMath::IsFinite(Direction.Z)
        && !Direction.IsNearlyZero();
    const bool bColorValid =
        IsFiniteColor(Configuration.LightColor)
        && Configuration.LightColor.R >= 0.0F
        && Configuration.LightColor.G >= 0.0F
        && Configuration.LightColor.B >= 0.0F
        && Configuration.LightColor.GetMax() > 0.0F;
    return FMath::IsFinite(Configuration.IntensityLumens)
        && Configuration.IntensityLumens > 0.0F
        && FMath::IsFinite(Configuration.AttenuationRadiusCentimeters)
        && Configuration.AttenuationRadiusCentimeters > 0.0F
        && FMath::IsFinite(Configuration.InnerConeAngleDegrees)
        && Configuration.InnerConeAngleDegrees >= 0.0F
        && FMath::IsFinite(Configuration.OuterConeAngleDegrees)
        && Configuration.OuterConeAngleDegrees
            > Configuration.InnerConeAngleDegrees
        && Configuration.OuterConeAngleDegrees < 90.0F
        && bDirectionValid
        && bColorValid;
}
} // namespace

bool USharVehicleNativeLightAdapter::ConfigureHeadlights(
    ASharVehiclePawn* InTargetPawn,
    USharVehiclePresentationDefinition* InPresentation,
    USharVehiclePresentationState* InState,
    const FSharVehicleNativeHeadlightConfiguration& InConfiguration
)
{
    if (
        InTargetPawn == nullptr
        || InPresentation == nullptr
        || InState == nullptr
        || !IsValidHeadlightConfiguration(InConfiguration)
    )
    {
        return false;
    }
    TArray<FText> Errors;
    InPresentation->GatherValidationErrors(Errors);
    USkeletalMeshComponent* Mesh = InTargetPawn->GetMesh();
    if (
        !Errors.IsEmpty()
        || Mesh == nullptr
        || Mesh->GetSkeletalMeshAsset() != InPresentation->SkeletalMesh.Get()
    )
    {
        return false;
    }

    TArray<FName> Bones;
    TArray<TArray<FName>> BindingIdsByBone;
    for (
        const FSharVehicleLightPresentationBinding& Binding
        : InPresentation->LightBindings
    )
    {
        if (Binding.Role != ESharVehicleLightPresentationRole::Headlight)
        {
            continue;
        }
        if (Mesh->GetBoneIndex(Binding.BoneName) == INDEX_NONE)
        {
            return false;
        }
        int32 BoneIndex = Bones.IndexOfByKey(Binding.BoneName);
        if (BoneIndex == INDEX_NONE)
        {
            BoneIndex = Bones.Add(Binding.BoneName);
            BindingIdsByBone.AddDefaulted();
        }
        BindingIdsByBone[BoneIndex].Add(Binding.BindingId);
    }

    ResetHeadlights();
    if (!InState->Configure(InPresentation))
    {
        return false;
    }
    TargetPawn = InTargetPawn;
    State = InState;
    HeadlightEmitters.Reserve(Bones.Num());
    for (int32 Index = 0; Index < Bones.Num(); ++Index)
    {
        FSharVehicleNativeHeadlightEmitter& Emitter =
            HeadlightEmitters.AddDefaulted_GetRef();
        Emitter.BoneName = Bones[Index];
        Emitter.BindingIds = BindingIdsByBone[Index];
        const FName ComponentName = MakeUniqueObjectName(
            InTargetPawn,
            USpotLightComponent::StaticClass(),
            FName(TEXT("SharNativeHeadlight"))
        );
        Emitter.Component = NewObject<USpotLightComponent>(
            InTargetPawn,
            ComponentName,
            RF_Transient
        );
        if (Emitter.Component == nullptr)
        {
            ResetHeadlights();
            return false;
        }
        InTargetPawn->AddInstanceComponent(Emitter.Component);
        Emitter.Component->SetupAttachment(Mesh, Emitter.BoneName);
        Emitter.Component->SetRelativeLocation(FVector::ZeroVector);
        Emitter.Component->SetRelativeRotation(
            InConfiguration.BoneLocalDirection.GetSafeNormal().Rotation()
        );
        Emitter.Component->SetMobility(EComponentMobility::Movable);
        Emitter.Component->SetIntensityUnits(ELightUnits::Lumens);
        Emitter.Component->SetIntensity(InConfiguration.IntensityLumens);
        Emitter.Component->SetAttenuationRadius(
            InConfiguration.AttenuationRadiusCentimeters
        );
        Emitter.Component->SetInnerConeAngle(
            InConfiguration.InnerConeAngleDegrees
        );
        Emitter.Component->SetOuterConeAngle(
            InConfiguration.OuterConeAngleDegrees
        );
        Emitter.Component->SetLightColor(InConfiguration.LightColor);
        Emitter.Component->SetVisibility(false);
        if (InTargetPawn->GetWorld() != nullptr)
        {
            Emitter.Component->RegisterComponent();
        }
    }
    return RefreshHeadlights();
}

bool USharVehicleNativeLightAdapter::RefreshHeadlights()
{
    if (TargetPawn == nullptr || State == nullptr || !State->IsConfigured())
    {
        return false;
    }
    for (FSharVehicleNativeHeadlightEmitter& Emitter : HeadlightEmitters)
    {
        if (Emitter.Component == nullptr)
        {
            return false;
        }
        bool bVisible = false;
        for (const FName BindingId : Emitter.BindingIds)
        {
            bool bBindingVisible = false;
            if (!State->GetBindingVisibility(BindingId, bBindingVisible))
            {
                return false;
            }
            bVisible = bVisible || bBindingVisible;
        }
        Emitter.Component->SetVisibility(bVisible);
    }
    return true;
}

void USharVehicleNativeLightAdapter::ResetHeadlights()
{
    for (FSharVehicleNativeHeadlightEmitter& Emitter : HeadlightEmitters)
    {
        if (Emitter.Component == nullptr)
        {
            continue;
        }
        if (TargetPawn != nullptr)
        {
            TargetPawn->RemoveInstanceComponent(Emitter.Component);
        }
        Emitter.Component->DestroyComponent();
    }
    HeadlightEmitters.Reset();
    TargetPawn = nullptr;
    State = nullptr;
}

int32 USharVehicleNativeLightAdapter::GetHeadlightEmitterCount() const
{
    return HeadlightEmitters.Num();
}

FName USharVehicleNativeLightAdapter::GetHeadlightEmitterBone(
    const int32 Index
) const
{
    return HeadlightEmitters.IsValidIndex(Index)
        ? HeadlightEmitters[Index].BoneName
        : NAME_None;
}

USpotLightComponent* USharVehicleNativeLightAdapter::GetHeadlightEmitter(
    const int32 Index
) const
{
    return HeadlightEmitters.IsValidIndex(Index)
        ? HeadlightEmitters[Index].Component.Get()
        : nullptr;
}
