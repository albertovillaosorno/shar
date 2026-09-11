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
//   - Safe generated SkeletalMesh in-place reimport and rollback orchestration.
// - Must-Not:
//   - Save packages, create identities, or retain failed in-memory revisions.
// - Allows:
//   - Invoke Unreal reimport and reload the previous saved packages on failure.
// - Split-When:
//   - Another asset family requires independently reviewed reimport policy.
// - Merge-When:
//   - Another adapter owns the same SkeletalMesh revision transaction.
// - Summary:
//   - Implements generated SkeletalMesh revision reimport with rollback.
// - Description:
//   - Preserves mesh, Skeleton, PhysicsAsset identity and import policy.
// - Usage:
//   - Invoked through ToolsetRegistry after repository-side preflight.
// - Defaults:
//   - Missing saved rollback state or unexpected mutation fails closed.
//

//! Generated SkeletalMesh revision reimport with saved-package rollback.

#include "Import/SharSkeletalMeshReimportToolset.h"

#include "Import/SharFbxImportPolicy.h"
#include "Import/SharImportValidation.h"

#include "Animation/Skeleton.h"
#include "EditorReimportHandler.h"
#include "Engine/SkeletalMesh.h"
#include "Factories/FbxImportUI.h"
#include "Factories/FbxSkeletalMeshImportData.h"
#include "Factories/ReimportFbxSkeletalMeshFactory.h"
#include "HAL/FileManager.h"
#include "Kismet/KismetSystemLibrary.h"
#include "Misc/PackageName.h"
#include "PackageTools.h"
#include "PhysicsEngine/PhysicsAsset.h"
#include "UObject/Package.h"
#include "UObject/StrongObjectPtr.h"
#include "UObject/UObjectGlobals.h"

#include UE_INLINE_GENERATED_CPP_BY_NAME(SharSkeletalMeshReimportToolset)

namespace UE::SharImportEditor::Private
{
namespace
{
void RaiseReimportError(const FString& Message)
{
    UKismetSystemLibrary::RaiseScriptError(
        FString::Printf(TEXT("SharSkeletalMeshReimportToolset: %s"), *Message)
    );
}

bool RequireCleanSavedPackage(
    UPackage* Package,
    const TCHAR* Label,
    FString& OutError
)
{
    if (Package == nullptr)
    {
        OutError = FString::Printf(TEXT("%s package is unavailable"), Label);
        return false;
    }
    if (Package->IsDirty())
    {
        OutError = FString::Printf(TEXT("%s package is dirty"), Label);
        return false;
    }
    if (!FPackageName::DoesPackageExist(Package->GetName()))
    {
        OutError = FString::Printf(
            TEXT("%s package has no saved rollback revision"),
            Label
        );
        return false;
    }
    return true;
}

void AddRollbackPackage(UPackage* Package, TArray<UPackage*>& Packages)
{
    if (Package != nullptr)
    {
        Packages.AddUnique(Package);
    }
}

bool ReloadSavedRevision(
    const TArray<UPackage*>& Packages,
    FString& OutError
)
{
    FText ReloadError;
    if (!UPackageTools::ReloadPackages(
            Packages,
            ReloadError,
            EReloadPackagesInteractionMode::AssumePositive
        ))
    {
        OutError = FString::Printf(
            TEXT("saved revision reload failed: %s"),
            *ReloadError.ToString()
        );
        return false;
    }
    return true;
}

void ConfigureReimportFactory(
    UReimportFbxSkeletalMeshFactory& Factory,
    USkeleton* Skeleton,
    UPhysicsAsset* PhysicsAsset
)
{
    UFbxImportUI* ImportUI = Factory.ImportUI;
    check(ImportUI != nullptr);
    check(ImportUI->SkeletalMeshImportData != nullptr);

    ImportUI->bAutomatedImportShouldDetectType = false;
    ImportUI->bOverrideFullName = true;
    ImportUI->bImportAsSkeletal = true;
    ImportUI->bImportMesh = true;
    ImportUI->MeshTypeToImport = FBXIT_SkeletalMesh;
    ImportUI->OriginalImportType = FBXIT_SkeletalMesh;
    ImportUI->bImportMaterials = false;
    ImportUI->bImportTextures = false;
    ImportUI->bImportAnimations = false;
    ImportUI->Skeleton = Skeleton;
    ImportUI->bCreatePhysicsAsset = false;
    ImportUI->PhysicsAsset = PhysicsAsset;

    UFbxSkeletalMeshImportData* MeshImport = ImportUI->SkeletalMeshImportData;
    ApplyFbxSceneUnitPolicy(*MeshImport);
    MeshImport->ImportContentType = FBXICT_All;
    MeshImport->NormalImportMethod = FBXNIM_ImportNormals;
    MeshImport->bComputeWeightedNormals = false;
    MeshImport->bImportMeshLODs = false;
    MeshImport->bUpdateSkeletonReferencePose = false;
    MeshImport->bUseT0AsRefPose = false;
    MeshImport->bPreserveSmoothingGroups = false;
    MeshImport->bKeepSectionsSeparate = false;
    MeshImport->bImportMeshesInBoneHierarchy = false;
    MeshImport->bImportMorphTargets = false;
    MeshImport->bImportVertexAttributes = false;
    MeshImport->VertexColorImportOption = EVertexColorImportOption::Replace;
    Factory.SetDetectImportTypeOnImport(false);
}

bool ValidateExistingRevision(
    USkeletalMesh* Mesh,
    const FSkeletalMeshImportPaths& Paths,
    USkeleton*& OutSkeleton,
    UPhysicsAsset*& OutPhysicsAsset,
    TArray<UPackage*>& OutRollbackPackages,
    FString& OutError
)
{
    if (Mesh == nullptr)
    {
        OutError = TEXT("skeletal mesh revision does not exist");
        return false;
    }
    if (!Mesh->GetPathName().Equals(
            Paths.MeshObjectPath,
            ESearchCase::CaseSensitive
        ))
    {
        OutError = TEXT("skeletal mesh revision identity is unexpected");
        return false;
    }
    OutSkeleton = Mesh->GetSkeleton();
    if (
        OutSkeleton == nullptr
        || !OutSkeleton->GetPathName().Equals(
            Paths.SkeletonObjectPath,
            ESearchCase::CaseSensitive
        )
    )
    {
        OutError = TEXT("skeletal mesh Skeleton companion identity drifted");
        return false;
    }
    OutPhysicsAsset = Mesh->GetPhysicsAsset();
    if (
        OutPhysicsAsset != nullptr
        && !OutPhysicsAsset->GetPackage()->GetName().StartsWith(
            TEXT("/Game/Generated/SHAR/"),
            ESearchCase::CaseSensitive
        )
    )
    {
        OutError = TEXT("PhysicsAsset companion is outside generated root");
        return false;
    }

    AddRollbackPackage(Mesh->GetPackage(), OutRollbackPackages);
    AddRollbackPackage(OutSkeleton->GetPackage(), OutRollbackPackages);
    if (OutPhysicsAsset != nullptr)
    {
        AddRollbackPackage(OutPhysicsAsset->GetPackage(), OutRollbackPackages);
    }
    if (!RequireCleanSavedPackage(Mesh->GetPackage(), TEXT("mesh"), OutError))
    {
        return false;
    }
    if (!RequireCleanSavedPackage(
            OutSkeleton->GetPackage(),
            TEXT("Skeleton"),
            OutError
        ))
    {
        return false;
    }
    if (
        OutPhysicsAsset != nullptr
        && !RequireCleanSavedPackage(
            OutPhysicsAsset->GetPackage(),
            TEXT("PhysicsAsset"),
            OutError
        )
    )
    {
        return false;
    }
    return true;
}

bool ValidateReimportPostconditions(
    USkeletalMesh* Mesh,
    USkeleton* Skeleton,
    UPhysicsAsset* PhysicsAsset,
    const FSkeletalMeshImportPaths& Paths,
    FString& OutError
)
{
    if (
        FindObject<USkeletalMesh>(nullptr, *Paths.MeshObjectPath) != Mesh
        || Mesh->GetSkeleton() != Skeleton
        || Mesh->GetPhysicsAsset() != PhysicsAsset
    )
    {
        OutError = TEXT("reimport changed native companion identity");
        return false;
    }
    if (!Mesh->GetPackage()->IsDirty())
    {
        OutError = TEXT("reimport did not produce a dirty mesh revision");
        return false;
    }
    if (Skeleton->GetPackage()->IsDirty())
    {
        OutError = TEXT("reimport unexpectedly dirtied the Skeleton package");
        return false;
    }
    if (PhysicsAsset != nullptr && PhysicsAsset->GetPackage()->IsDirty())
    {
        OutError = TEXT(
            "reimport unexpectedly dirtied the PhysicsAsset package"
        );
        return false;
    }
    return true;
}

TArray<FString> FailAndReload(
    const TArray<UPackage*>& RollbackPackages,
    const FString& Failure
)
{
    FString ReloadError;
    if (!ReloadSavedRevision(RollbackPackages, ReloadError))
    {
        RaiseReimportError(Failure + TEXT("; ") + ReloadError);
        return {};
    }
    RaiseReimportError(Failure);
    return {};
}
} // namespace
} // namespace UE::SharImportEditor::Private

TArray<FString> USharSkeletalMeshReimportToolset::ReimportSkeletalMeshRevision(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName
)
{
    using namespace UE::SharImportEditor::Private;
    FSkeletalMeshImportPaths Paths;
    FString Error;
    if (!ValidateSkeletalMeshRequest(
            SourceFile,
            FolderPath,
            AssetName,
            Paths,
            Error
        ))
    {
        RaiseReimportError(Error);
        return {};
    }
    if (!IFileManager::Get().FileExists(*SourceFile))
    {
        RaiseReimportError(TEXT("source_file does not exist"));
        return {};
    }

    USkeletalMesh* Mesh = LoadObject<USkeletalMesh>(
        nullptr,
        *Paths.MeshObjectPath
    );
    USkeleton* Skeleton = nullptr;
    UPhysicsAsset* PhysicsAsset = nullptr;
    TArray<UPackage*> RollbackPackages;
    if (!ValidateExistingRevision(
            Mesh,
            Paths,
            Skeleton,
            PhysicsAsset,
            RollbackPackages,
            Error
        ))
    {
        RaiseReimportError(Error);
        return {};
    }

    TStrongObjectPtr<UReimportFbxSkeletalMeshFactory> Factory(
        NewObject<UReimportFbxSkeletalMeshFactory>()
    );
    if (
        Factory->ImportUI == nullptr
        || Factory->ImportUI->SkeletalMeshImportData == nullptr
    )
    {
        RaiseReimportError(
            TEXT("FBX skeletal reimport settings are unavailable")
        );
        return {};
    }
    ConfigureReimportFactory(*Factory, Skeleton, PhysicsAsset);

    const bool Reimported = FReimportManager::Instance()->Reimport(
        Mesh,
        false,
        false,
        SourceFile,
        Factory.Get(),
        INDEX_NONE,
        true,
        true,
        false
    );
    if (!Reimported)
    {
        return FailAndReload(
            RollbackPackages,
            TEXT("FBX skeletal reimport failed")
        );
    }
    if (!ValidateReimportPostconditions(
            Mesh,
            Skeleton,
            PhysicsAsset,
            Paths,
            Error
        ))
    {
        return FailAndReload(RollbackPackages, Error);
    }
    return {Paths.MeshObjectPath, Paths.SkeletonObjectPath};
}
