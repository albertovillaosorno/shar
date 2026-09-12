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
//   - Canonical factory creation of review vehicle Animation Blueprints.
// - Must-Not:
//   - Save, overwrite, wire graphs, or infer a Skeleton from other assets.
// - Allows:
//   - Use Unreal's AnimBlueprintFactory and AssetTools create-only boundary.
// - Split-When:
//   - Graph construction or persistence needs an independent transaction.
// - Merge-When:
//   - Another adapter owns the same generated AnimBlueprint publication.
// - Summary:
//   - Implements vehicle drive-test Animation Blueprint creation.
// - Description:
//   - Validates identity, loads the exact Skeleton, and invokes the factory.
// - Usage:
//   - Invoked through ToolsetRegistry before Blueprint graph editing.
// - Defaults:
//   - Invalid or existing generated object paths fail closed.
//

//! Review-only vehicle Animation Blueprint publication adapter.

#include "Import/SharVehicleDriveTestAnimToolset.h"

#include "Animation/AnimBlueprint.h"
#include "Animation/Skeleton.h"
#include "AssetToolsModule.h"
#include "Factories/AnimBlueprintFactory.h"
#include "IAssetTools.h"
#include "Kismet/KismetSystemLibrary.h"
#include "Misc/PackageName.h"
#include "UObject/Package.h"
#include "UObject/UObjectGlobals.h"
#include "VehicleAnimationInstance.h"

namespace UE::SharImportEditor::Private
{
namespace
{
constexpr TCHAR GeneratedRoot[] = TEXT("/Game/Generated/SHAR/");

void RaiseDriveTestAnimError(const FString& Message)
{
    UKismetSystemLibrary::RaiseScriptError(
        FString::Printf(
            TEXT("SharVehicleDriveTestAnimToolset: %s"),
            *Message
        )
    );
}

bool BuildDestination(
    const FString& FolderPath,
    const FString& AssetName,
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
    const FString PackagePath = FolderPath + TEXT("/") + AssetName;
    if (!FPackageName::IsValidLongPackageName(PackagePath))
    {
        OutError = TEXT("destination package path is invalid");
        return false;
    }
    OutObjectPath = FString::Printf(
        TEXT("%s.%s"),
        *PackagePath,
        *AssetName
    );
    if (FindObject<UObject>(nullptr, *OutObjectPath) != nullptr
        || FindPackage(nullptr, *PackagePath) != nullptr
        || FPackageName::DoesPackageExist(PackagePath))
    {
        OutError = TEXT("vehicle AnimBlueprint output already exists");
        return false;
    }
    return true;
}

bool IsCanonicalGeneratedSkeletonPath(const FString& ObjectPath)
{
    const FString PackagePath =
        FPackageName::ObjectPathToPackageName(ObjectPath);
    const FString ObjectName =
        FPackageName::ObjectPathToObjectName(ObjectPath);
    return PackagePath.StartsWith(
            GeneratedRoot,
            ESearchCase::CaseSensitive
        )
        && !ObjectName.IsEmpty()
        && FPackageName::IsValidLongPackageName(PackagePath)
        && ObjectPath.Equals(
            FString::Printf(TEXT("%s.%s"), *PackagePath, *ObjectName),
            ESearchCase::CaseSensitive
        );
}
} // namespace
} // namespace UE::SharImportEditor::Private

FString USharVehicleDriveTestAnimToolset::CreateVehicleDriveTestAnimBlueprint(
    const FString& FolderPath,
    const FString& AssetName,
    const FString& SkeletonPath
)
{
    using namespace UE::SharImportEditor::Private;
    FString ObjectPath;
    FString Error;
    if (!BuildDestination(FolderPath, AssetName, ObjectPath, Error))
    {
        RaiseDriveTestAnimError(Error);
        return {};
    }
    if (!IsCanonicalGeneratedSkeletonPath(SkeletonPath))
    {
        RaiseDriveTestAnimError(
            TEXT("skeleton_path is not generated and canonical")
        );
        return {};
    }
    USkeleton* Skeleton = LoadObject<USkeleton>(nullptr, *SkeletonPath);
    if (Skeleton == nullptr)
    {
        RaiseDriveTestAnimError(TEXT("skeleton_path did not load"));
        return {};
    }

    UAnimBlueprintFactory* Factory = NewObject<UAnimBlueprintFactory>();
    Factory->TargetSkeleton = Skeleton;
    Factory->ParentClass = UVehicleAnimationInstance::StaticClass();
    UObject* Created = FAssetToolsModule::GetModule().Get().CreateAsset(
        AssetName,
        FolderPath,
        UAnimBlueprint::StaticClass(),
        Factory
    );
    UAnimBlueprint* AnimBlueprint = Cast<UAnimBlueprint>(Created);
    if (AnimBlueprint == nullptr
        || !AnimBlueprint->GetPathName().Equals(
            ObjectPath,
            ESearchCase::CaseSensitive
        )
        || AnimBlueprint->TargetSkeleton != Skeleton
        || AnimBlueprint->ParentClass
            != UVehicleAnimationInstance::StaticClass())
    {
        RaiseDriveTestAnimError(
            TEXT("AnimBlueprint factory postconditions failed")
        );
        return {};
    }
    return AnimBlueprint->GetPathName();
}
