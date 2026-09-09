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
//   - Transient Physics Asset construction from verified vehicle shape recipes.
// - Must-Not:
//   - Read source catalogs, save packages, or approximate unsupported shapes.
// - Allows:
//   - Create analytic sphere and box bodies against an imported skeletal mesh.
// - Split-When:
//   - Persisted Physics Asset publication gains an independent lifecycle.
// - Merge-When:
//   - Another editor boundary owns identical vehicle collision construction.
// - Summary:
//   - Vehicle Physics Asset construction kernel implementation.
// - Description:
//   - Preserves source magnitudes under the verified scene-unit skeletal scale
//   - consumed by Unreal when it instantiates per-bone physics bodies.
// - Usage:
//   - Called only after the revision-bound vehicle-physics plan is verified.
// - Defaults:
//   - Invalid or unsupported construction requests fail without mutation.
//

//! Vehicle Physics Asset construction kernel implementation.


#include "Import/SharVehiclePhysicsAssetBuilder.h"

#include "Engine/SkeletalMesh.h"
#include "PhysicsEngine/PhysicsAsset.h"
#include "PhysicsEngine/SkeletalBodySetup.h"

namespace UE::SharImportEditor::Private
{
namespace
{
constexpr float ExpectedSceneUnitScale = 100.0F;
constexpr float ScaleTolerance = 0.02F;
constexpr float BasisTolerance = 1.0e-4F;

bool IsFiniteVector(const FVector& Value)
{
    return FMath::IsFinite(Value.X)
        && FMath::IsFinite(Value.Y)
        && FMath::IsFinite(Value.Z);
}

bool IsUnitVector(const FVector& Value)
{
    return IsFiniteVector(Value)
        && FMath::IsNearlyEqual(
            static_cast<float>(Value.SizeSquared()),
            1.0F,
            BasisTolerance
        );
}

bool IsProperBasis(const FSharVehiclePhysicsShapeRecipe& Shape)
{
    if (!IsUnitVector(Shape.AxisX)
        || !IsUnitVector(Shape.AxisY)
        || !IsUnitVector(Shape.AxisZ))
    {
        return false;
    }
    if (!FMath::IsNearlyZero(
            static_cast<float>(FVector::DotProduct(Shape.AxisX, Shape.AxisY)),
            BasisTolerance
        )
        || !FMath::IsNearlyZero(
            static_cast<float>(FVector::DotProduct(Shape.AxisX, Shape.AxisZ)),
            BasisTolerance
        )
        || !FMath::IsNearlyZero(
            static_cast<float>(FVector::DotProduct(Shape.AxisY, Shape.AxisZ)),
            BasisTolerance
        ))
    {
        return false;
    }
    return FVector::DotProduct(
        FVector::CrossProduct(Shape.AxisX, Shape.AxisY),
        Shape.AxisZ
    ) > 0.0;
}

USkeletalBodySetup* FindOrAddBodySetup(
    UPhysicsAsset& PhysicsAsset,
    const FName BoneName,
    TMap<FName, USkeletalBodySetup*>& BodyByBone
)
{
    if (USkeletalBodySetup** Existing = BodyByBone.Find(BoneName))
    {
        return *Existing;
    }
    USkeletalBodySetup* Body = NewObject<USkeletalBodySetup>(&PhysicsAsset);
    Body->BoneName = BoneName;
    Body->bConsiderForBounds = true;
    PhysicsAsset.SkeletalBodySetups.Add(Body);
    BodyByBone.Add(BoneName, Body);
    return Body;
}
} // namespace

bool BuildTransientVehiclePhysicsAsset(
    USkeletalMesh& SkeletalMesh,
    const FName RigIdentity,
    const int32 JointCount,
    const TArray<FSharVehiclePhysicsShapeRecipe>& Shapes,
    UPhysicsAsset*& OutPhysicsAsset,
    FString& OutError
)
{
    OutPhysicsAsset = nullptr;
    OutError.Reset();
    const FReferenceSkeleton& Ref = SkeletalMesh.GetRefSkeleton();
    const int32 RigIndex = Ref.FindBoneIndex(RigIdentity);
    if (RigIdentity.IsNone()
        || JointCount <= 0
        || Ref.GetNum() == 0
        || Ref.GetNum() > JointCount
        || RigIndex != 0)
    {
        OutError = FString::Printf(
            TEXT(
                "vehicle physics rig identity or retained joint count "
                "disagrees (imported=%d source=%d root=%s rig_index=%d)"
            ),
            Ref.GetNum(),
            JointCount,
            *Ref.GetBoneName(0).ToString(),
            RigIndex
        );
        return false;
    }
    const FVector RootScale = Ref.GetRefBonePose()[0].GetScale3D();
    if (!RootScale.Equals(
            FVector(ExpectedSceneUnitScale),
            ScaleTolerance
        ))
    {
        OutError = TEXT("vehicle skeletal import has no scene-unit root scale");
        return false;
    }
    if (Shapes.IsEmpty())
    {
        OutError = TEXT("vehicle physics request has no shapes");
        return false;
    }

    UPhysicsAsset* Candidate = NewObject<UPhysicsAsset>(GetTransientPackage());
    TMap<FName, USkeletalBodySetup*> BodyByBone;
    for (const FSharVehiclePhysicsShapeRecipe& Shape : Shapes)
    {
        if (Shape.BoneName.IsNone()
            || Ref.FindBoneIndex(Shape.BoneName) == INDEX_NONE
            || !IsFiniteVector(Shape.Center))
        {
            OutError = TEXT("vehicle physics shape bone or center is invalid");
            return false;
        }
        USkeletalBodySetup* Body = FindOrAddBodySetup(
            *Candidate,
            Shape.BoneName,
            BodyByBone
        );
        if (Shape.Kind == ESharVehiclePhysicsShapeKind::Sphere)
        {
            if (!FMath::IsFinite(Shape.Radius) || Shape.Radius <= 0.0F)
            {
                OutError = TEXT("vehicle physics sphere radius is invalid");
                return false;
            }
            FKSphereElem Sphere;
            Sphere.Center = Shape.Center;
            Sphere.Radius = Shape.Radius;
            Body->AggGeom.SphereElems.Add(Sphere);
            continue;
        }
        if (!IsFiniteVector(Shape.BoxExtents)
            || Shape.BoxExtents.X <= 0.0
            || Shape.BoxExtents.Y <= 0.0
            || Shape.BoxExtents.Z <= 0.0
            || !IsProperBasis(Shape))
        {
            OutError = TEXT("vehicle physics box geometry is invalid");
            return false;
        }
        FMatrix RotationMatrix = FMatrix::Identity;
        RotationMatrix.SetAxes(&Shape.AxisX, &Shape.AxisY, &Shape.AxisZ);
        FKBoxElem Box;
        Box.Center = Shape.Center;
        Box.Rotation = RotationMatrix.Rotator();
        Box.X = static_cast<float>(Shape.BoxExtents.X);
        Box.Y = static_cast<float>(Shape.BoxExtents.Y);
        Box.Z = static_cast<float>(Shape.BoxExtents.Z);
        Body->AggGeom.BoxElems.Add(Box);
    }
    Candidate->SetPreviewMesh(&SkeletalMesh, false);
    Candidate->UpdateBodySetupIndexMap();
    Candidate->UpdateBoundsBodiesArray();
    OutPhysicsAsset = Candidate;
    return true;
}
} // namespace UE::SharImportEditor::Private
