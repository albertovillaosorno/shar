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
//   - Generated-root publication around the vehicle physics construction
//   - kernel.
// - Must-Not:
//   - Parse source files, save packages, replace assets, or repair recipes.
// - Allows:
//   - Deep publication of a transient candidate after exact native read-back.
// - Split-When:
//   - Save/rollback policy becomes independently transactional.
// - Merge-When:
//   - Another vehicle editor adapter owns identical publication semantics.
// - Summary:
//   - Vehicle Physics Asset publication adapter.
// - Description:
//   - Validates destination and mesh identity, builds, duplicates, and reads
//   - back.
// - Usage:
//   - Exposed through USharVehiclePhysicsToolset.
// - Defaults:
//   - Only generated-root Skeletal Meshes and create-only outputs are accepted.
//

//! Vehicle Physics Asset publication adapter.

#include "Import/SharVehiclePhysicsToolset.h"

#include "Import/SharVehiclePhysicsAssetBuilder.h"

#include "AssetRegistry/AssetRegistryModule.h"
#include "Engine/SkeletalMesh.h"
#include "Kismet/KismetSystemLibrary.h"
#include "Misc/PackageName.h"
#include "PhysicsEngine/PhysicsAsset.h"
#include "PhysicsEngine/SkeletalBodySetup.h"
#include "UObject/Package.h"
#include "UObject/UObjectGlobals.h"

namespace UE::SharImportEditor::Private
{
namespace
{
constexpr TCHAR GeneratedRoot[] = TEXT("/Game/Generated/SHAR/");

void RaiseVehiclePhysicsError(const FString& Message)
{
    UKismetSystemLibrary::RaiseScriptError(
        FString::Printf(TEXT("SharVehiclePhysicsToolset: %s"), *Message)
    );
}

bool IsCanonicalGeneratedObjectPath(const FString& ObjectPath)
{
    const FString PackagePath =
        FPackageName::ObjectPathToPackageName(ObjectPath);
    const FString ObjectName =
        FPackageName::ObjectPathToObjectName(ObjectPath);
    return !PackagePath.IsEmpty()
        && !ObjectName.IsEmpty()
        && PackagePath.StartsWith(GeneratedRoot, ESearchCase::CaseSensitive)
        && FPackageName::IsValidLongPackageName(PackagePath)
        && FPackageName::GetLongPackageAssetName(PackagePath) == ObjectName
        && ObjectPath.Equals(
            FString::Printf(TEXT("%s.%s"), *PackagePath, *ObjectName),
            ESearchCase::CaseSensitive
        );
}

bool BuildDestination(
    const FString& FolderPath,
    const FString& AssetName,
    FString& OutPackagePath,
    FString& OutObjectPath,
    FString& OutError
)
{
    if (!FolderPath.StartsWith(GeneratedRoot, ESearchCase::CaseSensitive))
    {
        OutError = TEXT("folder_path must be beneath /Game/Generated/SHAR");
        return false;
    }
    if (AssetName.IsEmpty()
        || AssetName.Contains(TEXT("/"))
        || AssetName.Contains(TEXT(".")))
    {
        OutError = TEXT("asset_name is not canonical");
        return false;
    }
    OutPackagePath = FolderPath + TEXT("/") + AssetName;
    if (!FPackageName::IsValidLongPackageName(OutPackagePath))
    {
        OutError = TEXT("destination package path is invalid");
        return false;
    }
    OutObjectPath = FString::Printf(
        TEXT("%s.%s"),
        *OutPackagePath,
        *AssetName
    );
    if (FindObject<UObject>(nullptr, *OutObjectPath) != nullptr
        || FindPackage(nullptr, *OutPackagePath) != nullptr
        || FPackageName::DoesPackageExist(OutPackagePath))
    {
        OutError = TEXT("vehicle Physics Asset output already exists");
        return false;
    }
    return true;
}

bool ConvertShapes(
    const TArray<FSharVehiclePhysicsShapeInput>& Inputs,
    TArray<FSharVehiclePhysicsShapeRecipe>& OutShapes,
    FString& OutError
)
{
    OutShapes.Reset();
    OutShapes.Reserve(Inputs.Num());
    for (const FSharVehiclePhysicsShapeInput& Input : Inputs)
    {
        FSharVehiclePhysicsShapeRecipe Shape;
        switch (Input.Kind)
        {
        case ESharVehiclePhysicsShapeInputKind::Sphere:
            Shape.Kind = ESharVehiclePhysicsShapeKind::Sphere;
            break;
        case ESharVehiclePhysicsShapeInputKind::Box:
            Shape.Kind = ESharVehiclePhysicsShapeKind::Box;
            break;
        default:
            OutError = TEXT("vehicle Physics Asset shape kind is unsupported");
            return false;
        }
        Shape.BoneName = Input.BoneName;
        Shape.Center = Input.Center;
        Shape.Radius = Input.Radius;
        Shape.AxisX = Input.AxisX;
        Shape.AxisY = Input.AxisY;
        Shape.AxisZ = Input.AxisZ;
        Shape.BoxExtents = Input.BoxExtents;
        OutShapes.Add(Shape);
    }
    return true;
}

bool HasEquivalentBodies(
    const UPhysicsAsset& Candidate,
    const UPhysicsAsset& Published
)
{
    if (
        Candidate.SkeletalBodySetups.Num()
        != Published.SkeletalBodySetups.Num()
    )
    {
        return false;
    }
    for (int32 Index = 0; Index < Candidate.SkeletalBodySetups.Num(); ++Index)
    {
        const USkeletalBodySetup* Expected =
            Candidate.SkeletalBodySetups[Index];
        const USkeletalBodySetup* Actual = Published.SkeletalBodySetups[Index];
        if (Expected == nullptr
            || Actual == nullptr
            || Expected->BoneName != Actual->BoneName
            || Expected->AggGeom.SphereElems != Actual->AggGeom.SphereElems
            || Expected->AggGeom.BoxElems != Actual->AggGeom.BoxElems)
        {
            return false;
        }
    }
    return true;
}

void DiscardPublishedAsset(UPhysicsAsset* Asset)
{
    if (Asset == nullptr)
    {
        return;
    }
    UPackage* Package = Asset->GetPackage();
    Asset->ClearFlags(RF_Public | RF_Standalone);
    Asset->MarkAsGarbage();
    if (Package != nullptr)
    {
        Package->ClearDirtyFlag();
        Package->MarkAsGarbage();
    }
}
} // namespace
} // namespace UE::SharImportEditor::Private

FString USharVehiclePhysicsToolset::CreateVehiclePhysicsAsset(
    const FString& FolderPath,
    const FString& AssetName,
    const FString& SkeletalMeshPath,
    FName RigIdentity,
    int32 SourceJointCount,
    const TArray<FSharVehiclePhysicsShapeInput>& Shapes
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    FString PackagePath;
    FString ObjectPath;
    if (!BuildDestination(
            FolderPath,
            AssetName,
            PackagePath,
            ObjectPath,
            Error
        ))
    {
        RaiseVehiclePhysicsError(Error);
        return {};
    }
    if (!IsCanonicalGeneratedObjectPath(SkeletalMeshPath))
    {
        RaiseVehiclePhysicsError(
            TEXT("skeletal_mesh_path is not generated and canonical")
        );
        return {};
    }
    USkeletalMesh* Mesh = LoadObject<USkeletalMesh>(nullptr, *SkeletalMeshPath);
    if (Mesh == nullptr)
    {
        RaiseVehiclePhysicsError(TEXT("skeletal_mesh_path did not load"));
        return {};
    }
    TArray<FSharVehiclePhysicsShapeRecipe> NativeShapes;
    if (!ConvertShapes(Shapes, NativeShapes, Error))
    {
        RaiseVehiclePhysicsError(Error);
        return {};
    }
    UPhysicsAsset* Candidate = nullptr;
    if (!BuildTransientVehiclePhysicsAsset(
            *Mesh,
            RigIdentity,
            SourceJointCount,
            NativeShapes,
            Candidate,
            Error
        ))
    {
        RaiseVehiclePhysicsError(Error);
        return {};
    }

    UPackage* Package = CreatePackage(*PackagePath);
    UPhysicsAsset* Published = DuplicateObject<UPhysicsAsset>(
        Candidate,
        Package,
        FName(*AssetName)
    );
    if (Published == nullptr)
    {
        RaiseVehiclePhysicsError(
            TEXT("vehicle Physics Asset duplication failed")
        );
        return {};
    }
    Published->ClearFlags(RF_Transient);
    Published->SetFlags(RF_Public | RF_Standalone | RF_Transactional);
    for (USkeletalBodySetup* Body : Published->SkeletalBodySetups)
    {
        if (Body != nullptr)
        {
            Body->ClearFlags(RF_Transient);
            Body->SetFlags(RF_Transactional);
        }
    }
    Published->UpdateBodySetupIndexMap();
    Published->UpdateBoundsBodiesArray();
    if (Published->GetPreviewMesh() != Mesh
        || !HasEquivalentBodies(*Candidate, *Published))
    {
        DiscardPublishedAsset(Published);
        RaiseVehiclePhysicsError(
            TEXT("vehicle Physics Asset read-back drifted")
        );
        return {};
    }
    FAssetRegistryModule::AssetCreated(Published);
    Package->MarkPackageDirty();
    return ObjectPath;
}
