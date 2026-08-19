// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar import toolset implementation.
// - Must-Not:
//   - Own runtime gameplay policy or mutate content outside generated roots.
// - Allows:
//   - Editor-only inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one imported asset family gains an independent lifecycle.
// - Merge-When:
//   - Merge when another editor module owns the identical responsibility.
// - Summary:
//   - Shar import toolset implementation.
// - Description:
//   - Implements bounded editor-only import and lifecycle tools.
// - Usage:
//   - Used through the SharImportEditor module and its native toolset boundary.
// - Defaults:
//   - Invalid, ambiguous, or replacement requests fail explicitly.
//
#include "Import/SharImportToolset.h"

#include "Import/SharImportValidation.h"

#include "AssetImportTask.h"
#include "Animation/Skeleton.h"
#include "AssetRegistry/AssetRegistryModule.h"
#include "AssetToolsModule.h"
#include "Factories/FbxFactory.h"
#include "Factories/FbxImportUI.h"
#include "Factories/FbxStaticMeshImportData.h"
#include "Factories/FbxSkeletalMeshImportData.h"
#include "Factories/SoundFactory.h"
#include "FileMediaSource.h"
#include "Engine/SkeletalMesh.h"
#include "Engine/StaticMesh.h"
#include "HAL/FileManager.h"
#include "Kismet/KismetSystemLibrary.h"
#include "Misc/Guid.h"
#include "Misc/PackageName.h"
#include "Misc/Paths.h"
#include "ObjectTools.h"
#include "PhysicsEngine/PhysicsAsset.h"
#include "UObject/Package.h"
#include "UObject/StrongObjectPtr.h"
#include "UObject/UObjectGlobals.h"

#include UE_INLINE_GENERATED_CPP_BY_NAME(SharImportToolset)

namespace UE::SharImportEditor::Private
{
namespace
{
constexpr TCHAR GeneratedRoot[] = TEXT("/Game/Generated/SHAR/");
constexpr TCHAR MovieRoot[] = TEXT("Movies/Generated/SHAR");

void RaiseError(const FString& Message)
{
    UKismetSystemLibrary::RaiseScriptError(
        FString::Printf(TEXT("SharImportToolset: %s"), *Message)
    );
}

bool ValidateGeneratedDestination(
    const FString& FolderPath,
    const FString& AssetName,
    FString& OutPackagePath,
    FString& OutError
)
{
    if (!FolderPath.StartsWith(GeneratedRoot, ESearchCase::CaseSensitive))
    {
        OutError = TEXT("folder_path must be beneath /Game/Generated/SHAR");
        return false;
    }
    if (
        AssetName.IsEmpty()
        || AssetName.Contains(TEXT("/"))
        || AssetName.Contains(TEXT("."))
    )
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
    return true;
}

bool BuildFileMediaSourcePaths(
    const FString& FolderPath,
    const FString& AssetName,
    FFileMediaSourcePaths& OutPaths,
    FString& OutError
)
{
    OutPaths = {};
    if (!ValidateGeneratedDestination(
            FolderPath,
            AssetName,
            OutPaths.PackagePath,
            OutError
        ))
    {
        return false;
    }
    OutPaths.ObjectPath = FString::Printf(
        TEXT("%s.%s"),
        *OutPaths.PackagePath,
        *AssetName
    );
    const FString RelativePackage = OutPaths.PackagePath.Mid(
        UE_ARRAY_COUNT(GeneratedRoot) - 1
    );
    if (RelativePackage.IsEmpty())
    {
        OutError = TEXT("media package has no generated relative identity");
        return false;
    }
    const FString RelativeMovie = RelativePackage + TEXT(".mov");
    OutPaths.RelativePayloadPath = FString::Printf(
        TEXT("./%s/%s"),
        MovieRoot,
        *RelativeMovie
    );
    OutPaths.FullPayloadPath = FPaths::ConvertRelativePathToFull(
        FPaths::Combine(
            FPaths::ProjectContentDir(),
            MovieRoot,
            RelativeMovie
        )
    );
    FPaths::NormalizeFilename(OutPaths.FullPayloadPath);
    return true;
}

bool AssetIdentityExists(
    const FString& PackagePath,
    const FString& ObjectPath
)
{
    return FindObject<UObject>(nullptr, *ObjectPath) != nullptr
        || FindPackage(nullptr, *PackagePath) != nullptr
        || FPackageName::DoesPackageExist(PackagePath);
}

void AddImportedSkeletalObject(
    UObject* Object,
    TArray<UObject*>& OutObjects
)
{
    if (Object == nullptr)
    {
        return;
    }
    OutObjects.AddUnique(Object);
    USkeletalMesh* SkeletalMesh = Cast<USkeletalMesh>(Object);
    if (SkeletalMesh == nullptr)
    {
        return;
    }
    OutObjects.AddUnique(SkeletalMesh->GetSkeleton());
    OutObjects.AddUnique(SkeletalMesh->GetPhysicsAsset());
}

void DiscardSkeletalImport(
    const UAssetImportTask* Task,
    USkeletalMesh* PrimaryMesh
)
{
    TArray<UObject*> Objects;
    AddImportedSkeletalObject(PrimaryMesh, Objects);
    if (Task != nullptr)
    {
        for (const FString& ImportedPath : Task->ImportedObjectPaths)
        {
            AddImportedSkeletalObject(
                FindObject<UObject>(nullptr, *ImportedPath),
                Objects
            );
        }
    }
    Objects.Remove(nullptr);
    if (!Objects.IsEmpty())
    {
        (void)ObjectTools::ForceDeleteObjects(Objects, false);
    }
}

void DiscardCreatedMediaSource(
    UFileMediaSource* MediaSource,
    UPackage* Package
)
{
    if (MediaSource != nullptr)
    {
        MediaSource->ClearFlags(RF_Public | RF_Standalone);
        MediaSource->MarkAsGarbage();
    }
    if (Package != nullptr)
    {
        Package->ClearDirtyFlag();
        Package->MarkAsGarbage();
    }
}
} // namespace

bool ValidateStaticMeshRequest(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName,
    FString& OutError
)
{
    OutError.Reset();
    if (SourceFile.IsEmpty() || FPaths::IsRelative(SourceFile))
    {
        OutError = TEXT("source_file must be an absolute path");
        return false;
    }
    if (
        !FPaths::GetExtension(SourceFile).Equals(
            TEXT("fbx"),
            ESearchCase::IgnoreCase
        )
    )
    {
        OutError = TEXT("source_file must have an FBX extension");
        return false;
    }
    FString PackagePath;
    return ValidateGeneratedDestination(
        FolderPath,
        AssetName,
        PackagePath,
        OutError
    );
}

bool ValidateSkeletalMeshRequest(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName,
    FSkeletalMeshImportPaths& OutPaths,
    FString& OutError
)
{
    OutPaths = {};
    OutError.Reset();
    if (SourceFile.IsEmpty() || FPaths::IsRelative(SourceFile))
    {
        OutError = TEXT("source_file must be an absolute path");
        return false;
    }
    if (
        !FPaths::GetExtension(SourceFile).Equals(
            TEXT("fbx"),
            ESearchCase::IgnoreCase
        )
    )
    {
        OutError = TEXT("source_file must have an FBX extension");
        return false;
    }
    if (!ValidateGeneratedDestination(
            FolderPath,
            AssetName,
            OutPaths.MeshPackagePath,
            OutError
        ))
    {
        return false;
    }
    OutPaths.MeshObjectPath = FString::Printf(
        TEXT("%s.%s"),
        *OutPaths.MeshPackagePath,
        *AssetName
    );
    const FString SkeletonName = AssetName + TEXT("_Skeleton");
    if (!ValidateGeneratedDestination(
            FolderPath,
            SkeletonName,
            OutPaths.SkeletonPackagePath,
            OutError
        ))
    {
        return false;
    }
    OutPaths.SkeletonObjectPath = FString::Printf(
        TEXT("%s.%s"),
        *OutPaths.SkeletonPackagePath,
        *SkeletonName
    );
    return true;
}

bool ValidateSoundWaveRequest(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName,
    FString& OutError
)
{
    OutError.Reset();
    if (SourceFile.IsEmpty() || FPaths::IsRelative(SourceFile))
    {
        OutError = TEXT("source_file must be an absolute path");
        return false;
    }
    if (
        !FPaths::GetExtension(SourceFile).Equals(
            TEXT("wav"),
            ESearchCase::IgnoreCase
        )
    )
    {
        OutError = TEXT("source_file must have a WAV extension");
        return false;
    }
    FString PackagePath;
    return ValidateGeneratedDestination(
        FolderPath,
        AssetName,
        PackagePath,
        OutError
    );
}

bool ValidateFileMediaSourceRequest(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName,
    FFileMediaSourcePaths& OutPaths,
    FString& OutError
)
{
    OutError.Reset();
    if (SourceFile.IsEmpty() || FPaths::IsRelative(SourceFile))
    {
        OutError = TEXT("source_file must be an absolute path");
        return false;
    }
    if (
        !FPaths::GetExtension(SourceFile).Equals(
            TEXT("mov"),
            ESearchCase::IgnoreCase
        )
    )
    {
        OutError = TEXT("source_file must have a MOV extension");
        return false;
    }
    return BuildFileMediaSourcePaths(
        FolderPath,
        AssetName,
        OutPaths,
        OutError
    );
}

bool BuildFileMediaSourcePathsFromObjectPath(
    const FString& AssetPath,
    FFileMediaSourcePaths& OutPaths,
    FString& OutError
)
{
    OutError.Reset();
    const FString PackagePath = FPackageName::ObjectPathToPackageName(AssetPath);
    const FString AssetName = FPackageName::ObjectPathToObjectName(AssetPath);
    if (
        PackagePath.IsEmpty()
        || AssetName.IsEmpty()
        || FPackageName::GetLongPackageAssetName(PackagePath) != AssetName
    )
    {
        OutError = TEXT("asset_path is not a canonical top-level object path");
        return false;
    }
    if (!BuildFileMediaSourcePaths(
            FPackageName::GetLongPackagePath(PackagePath),
            AssetName,
            OutPaths,
            OutError
        ))
    {
        return false;
    }
    if (!OutPaths.ObjectPath.Equals(AssetPath, ESearchCase::CaseSensitive))
    {
        OutError = TEXT("asset_path is not canonical");
        return false;
    }
    return true;
}
} // namespace UE::SharImportEditor::Private

TArray<FString> USharImportToolset::ImportStaticMesh(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    if (!ValidateStaticMeshRequest(
            SourceFile,
            FolderPath,
            AssetName,
            Error
        ))
    {
        RaiseError(Error);
        return {};
    }
    IFileManager& Files = IFileManager::Get();
    if (!Files.FileExists(*SourceFile))
    {
        RaiseError(TEXT("source_file does not exist"));
        return {};
    }

    FString PackagePath;
    if (!ValidateGeneratedDestination(
            FolderPath,
            AssetName,
            PackagePath,
            Error
        ))
    {
        RaiseError(Error);
        return {};
    }
    const FString ObjectPath = FString::Printf(
        TEXT("%s.%s"),
        *PackagePath,
        *AssetName
    );
    if (
        FindObject<UObject>(nullptr, *ObjectPath) != nullptr
        || FindPackage(nullptr, *PackagePath) != nullptr
        || FPackageName::DoesPackageExist(PackagePath)
    )
    {
        RaiseError(TEXT("destination asset already exists"));
        return {};
    }

    TStrongObjectPtr<UAssetImportTask> Task(NewObject<UAssetImportTask>());
    TStrongObjectPtr<UFbxFactory> Factory(NewObject<UFbxFactory>());
    UFbxImportUI* ImportUI = Factory->ImportUI;
    if (ImportUI == nullptr || ImportUI->StaticMeshImportData == nullptr)
    {
        RaiseError(TEXT("FBX static import settings are unavailable"));
        return {};
    }

    ImportUI->bAutomatedImportShouldDetectType = false;
    ImportUI->bImportAsSkeletal = false;
    ImportUI->bImportMesh = true;
    ImportUI->MeshTypeToImport = FBXIT_StaticMesh;
    ImportUI->OriginalImportType = FBXIT_StaticMesh;
    ImportUI->bImportMaterials = false;
    ImportUI->bImportTextures = false;
    ImportUI->bImportAnimations = false;
    ImportUI->bCreatePhysicsAsset = false;

    UFbxStaticMeshImportData* MeshImport = ImportUI->StaticMeshImportData;
    MeshImport->NormalImportMethod = FBXNIM_ImportNormals;
    MeshImport->bComputeWeightedNormals = false;
    MeshImport->bCombineMeshes = true;
    MeshImport->bImportMeshLODs = false;
    MeshImport->bRemoveDegenerates = false;
    MeshImport->bBuildReversedIndexBuffer = false;
    MeshImport->bBuildNanite = false;
    MeshImport->bGenerateLightmapUVs = false;
    MeshImport->bAutoGenerateCollision = false;
    MeshImport->bTransformVertexToAbsolute = true;
    MeshImport->bBakePivotInVertex = false;
    MeshImport->VertexColorImportOption = EVertexColorImportOption::Replace;

    Factory->SetDetectImportTypeOnImport(false);
    Task->Filename = SourceFile;
    Task->DestinationPath = FolderPath;
    Task->DestinationName = AssetName;
    Task->bAutomated = true;
    Task->bAsync = false;
    Task->bReplaceExisting = false;
    Task->bReplaceExistingSettings = false;
    Task->bSave = false;
    Task->Factory = Factory.Get();
    Task->Options = ImportUI;
    Factory->SetAssetImportTask(Task.Get());

    FAssetToolsModule::GetModule().Get().ImportAssetTasks({Task.Get()});
    if (
        Task->ImportedObjectPaths.Num() != 1
        || !Task->ImportedObjectPaths[0].Equals(
            ObjectPath,
            ESearchCase::CaseSensitive
        )
    )
    {
        RaiseError(TEXT("FBX static import produced unexpected assets"));
        return {};
    }
    UStaticMesh* StaticMesh = FindObject<UStaticMesh>(nullptr, *ObjectPath);
    if (StaticMesh == nullptr)
    {
        RaiseError(TEXT("FBX static import did not create a StaticMesh"));
        return {};
    }
    return Task->ImportedObjectPaths;
}

TArray<FString> USharImportToolset::ImportSkeletalMesh(
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
        RaiseError(Error);
        return {};
    }
    if (!IFileManager::Get().FileExists(*SourceFile))
    {
        RaiseError(TEXT("source_file does not exist"));
        return {};
    }
    if (
        AssetIdentityExists(Paths.MeshPackagePath, Paths.MeshObjectPath)
        || AssetIdentityExists(
            Paths.SkeletonPackagePath,
            Paths.SkeletonObjectPath
        )
    )
    {
        RaiseError(TEXT("skeletal import output already exists"));
        return {};
    }

    TStrongObjectPtr<UAssetImportTask> Task(NewObject<UAssetImportTask>());
    TStrongObjectPtr<UFbxFactory> Factory(NewObject<UFbxFactory>());
    UFbxImportUI* ImportUI = Factory->ImportUI;
    if (ImportUI == nullptr || ImportUI->SkeletalMeshImportData == nullptr)
    {
        RaiseError(TEXT("FBX skeletal import settings are unavailable"));
        return {};
    }

    ImportUI->bAutomatedImportShouldDetectType = false;
    ImportUI->bOverrideFullName = true;
    ImportUI->bImportAsSkeletal = true;
    ImportUI->bImportMesh = true;
    ImportUI->MeshTypeToImport = FBXIT_SkeletalMesh;
    ImportUI->OriginalImportType = FBXIT_SkeletalMesh;
    ImportUI->bImportMaterials = false;
    ImportUI->bImportTextures = false;
    ImportUI->bImportAnimations = false;
    ImportUI->Skeleton = nullptr;
    ImportUI->bCreatePhysicsAsset = false;
    ImportUI->PhysicsAsset = nullptr;

    UFbxSkeletalMeshImportData* MeshImport = ImportUI->SkeletalMeshImportData;
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

    Factory->SetDetectImportTypeOnImport(false);
    Task->Filename = SourceFile;
    Task->DestinationPath = FolderPath;
    Task->DestinationName = AssetName;
    Task->bAutomated = true;
    Task->bAsync = false;
    Task->bReplaceExisting = false;
    Task->bReplaceExistingSettings = false;
    Task->bSave = false;
    Task->Factory = Factory.Get();
    Task->Options = ImportUI;
    Factory->SetAssetImportTask(Task.Get());

    FAssetToolsModule::GetModule().Get().ImportAssetTasks({Task.Get()});
    USkeletalMesh* SkeletalMesh = FindObject<USkeletalMesh>(
        nullptr,
        *Paths.MeshObjectPath
    );
    const bool PrimaryResultMatches =
        Task->ImportedObjectPaths.Num() == 1
        && Task->ImportedObjectPaths[0].Equals(
            Paths.MeshObjectPath,
            ESearchCase::CaseSensitive
        );
    if (!PrimaryResultMatches || SkeletalMesh == nullptr)
    {
        DiscardSkeletalImport(Task.Get(), SkeletalMesh);
        RaiseError(TEXT("FBX skeletal import produced unexpected primary assets"));
        return {};
    }

    USkeleton* Skeleton = SkeletalMesh->GetSkeleton();
    if (
        Skeleton == nullptr
        || !Skeleton->GetPathName().Equals(
            Paths.SkeletonObjectPath,
            ESearchCase::CaseSensitive
        )
        || FindObject<USkeleton>(nullptr, *Paths.SkeletonObjectPath) != Skeleton
        || SkeletalMesh->GetPhysicsAsset() != nullptr
        || !SkeletalMesh->GetPackage()->IsDirty()
        || !Skeleton->GetPackage()->IsDirty()
    )
    {
        DiscardSkeletalImport(Task.Get(), SkeletalMesh);
        RaiseError(TEXT("FBX skeletal import produced unexpected companions"));
        return {};
    }
    return {Paths.MeshObjectPath, Paths.SkeletonObjectPath};
}

TArray<FString> USharImportToolset::ImportSoundWave(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName
)
{
    FString Error;
    if (!UE::SharImportEditor::Private::ValidateSoundWaveRequest(
            SourceFile,
            FolderPath,
            AssetName,
            Error
        ))
    {
        UE::SharImportEditor::Private::RaiseError(Error);
        return {};
    }
    if (!IFileManager::Get().FileExists(*SourceFile))
    {
        UE::SharImportEditor::Private::RaiseError(
            TEXT("source_file does not exist")
        );
        return {};
    }
    TStrongObjectPtr<UAssetImportTask> Task(NewObject<UAssetImportTask>());
    TStrongObjectPtr<USoundFactory> Factory(NewObject<USoundFactory>());
    Factory->SuppressImportDialogs();
    Factory->bAutoCreateCue = false;

    Task->Filename = SourceFile;
    Task->DestinationPath = FolderPath;
    Task->DestinationName = AssetName;
    Task->bAutomated = true;
    Task->bAsync = false;
    Task->bReplaceExisting = false;
    Task->bReplaceExistingSettings = false;
    Task->bSave = false;
    Task->Factory = Factory.Get();
    Factory->SetAssetImportTask(Task.Get());

    FAssetToolsModule::GetModule().Get().ImportAssetTasks({Task.Get()});
    if (Task->ImportedObjectPaths.IsEmpty())
    {
        UE::SharImportEditor::Private::RaiseError(
            TEXT("WAV import produced no assets")
        );
        return {};
    }
    return Task->ImportedObjectPaths;
}

TArray<FString> USharImportToolset::ImportFileMediaSource(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName
)
{
    using namespace UE::SharImportEditor::Private;
    FFileMediaSourcePaths Paths;
    FString Error;
    if (!ValidateFileMediaSourceRequest(
            SourceFile,
            FolderPath,
            AssetName,
            Paths,
            Error
        ))
    {
        RaiseError(Error);
        return {};
    }
    IFileManager& Files = IFileManager::Get();
    if (!Files.FileExists(*SourceFile))
    {
        RaiseError(TEXT("source_file does not exist"));
        return {};
    }
    if (
        FindObject<UObject>(nullptr, *Paths.ObjectPath) != nullptr
        || FindPackage(nullptr, *Paths.PackagePath) != nullptr
        || FPackageName::DoesPackageExist(Paths.PackagePath)
    )
    {
        RaiseError(TEXT("destination asset already exists"));
        return {};
    }
    if (Files.FileExists(*Paths.FullPayloadPath))
    {
        RaiseError(TEXT("destination movie payload already exists"));
        return {};
    }

    const FString PayloadDirectory = FPaths::GetPath(Paths.FullPayloadPath);
    if (!Files.MakeDirectory(*PayloadDirectory, true))
    {
        RaiseError(TEXT("destination movie directory could not be created"));
        return {};
    }
    const FString TemporaryPayload = FString::Printf(
        TEXT("%s.tmp-%s"),
        *Paths.FullPayloadPath,
        *FGuid::NewGuid().ToString(EGuidFormats::Digits)
    );
    if (
        Files.Copy(
            *TemporaryPayload,
            *SourceFile,
            false,
            false,
            false
        ) != COPY_OK
    )
    {
        RaiseError(TEXT("movie payload copy failed"));
        return {};
    }
    if (!Files.Move(*Paths.FullPayloadPath, *TemporaryPayload, false))
    {
        Files.Delete(*TemporaryPayload, false, true, true);
        RaiseError(TEXT("movie payload publication failed"));
        return {};
    }

    UPackage* Package = CreatePackage(*Paths.PackagePath);
    UFileMediaSource* MediaSource = NewObject<UFileMediaSource>(
        Package,
        FName(*AssetName),
        RF_Public | RF_Standalone | RF_Transactional
    );
    if (Package == nullptr || MediaSource == nullptr)
    {
        Files.Delete(*Paths.FullPayloadPath, false, true, true);
        DiscardCreatedMediaSource(MediaSource, Package);
        RaiseError(TEXT("FileMediaSource creation failed"));
        return {};
    }
    MediaSource->SetFilePath(Paths.FullPayloadPath);
    if (
        !MediaSource->GetFilePath().Equals(
            Paths.RelativePayloadPath,
            ESearchCase::CaseSensitive
        )
        || !MediaSource->Validate()
    )
    {
        Files.Delete(*Paths.FullPayloadPath, false, true, true);
        DiscardCreatedMediaSource(MediaSource, Package);
        RaiseError(TEXT("FileMediaSource payload validation failed"));
        return {};
    }

    FAssetRegistryModule::AssetCreated(MediaSource);
    Package->MarkPackageDirty();
    return {Paths.ObjectPath};
}

FString USharImportToolset::GetFileMediaSourcePath(const FString& AssetPath)
{
    using namespace UE::SharImportEditor::Private;
    FFileMediaSourcePaths Paths;
    FString Error;
    if (!BuildFileMediaSourcePathsFromObjectPath(AssetPath, Paths, Error))
    {
        RaiseError(Error);
        return {};
    }
    UFileMediaSource* MediaSource = LoadObject<UFileMediaSource>(
        nullptr,
        *Paths.ObjectPath
    );
    if (MediaSource == nullptr)
    {
        RaiseError(TEXT("FileMediaSource asset does not exist"));
        return {};
    }
    return MediaSource->GetFilePath();
}

bool USharImportToolset::FileMediaSourcePayloadExists(
    const FString& AssetPath
)
{
    using namespace UE::SharImportEditor::Private;
    FFileMediaSourcePaths Paths;
    FString Error;
    if (!BuildFileMediaSourcePathsFromObjectPath(AssetPath, Paths, Error))
    {
        RaiseError(Error);
        return false;
    }
    return IFileManager::Get().FileExists(*Paths.FullPayloadPath);
}

bool USharImportToolset::DeleteFileMediaSourcePayload(
    const FString& AssetPath
)
{
    using namespace UE::SharImportEditor::Private;
    FFileMediaSourcePaths Paths;
    FString Error;
    if (!BuildFileMediaSourcePathsFromObjectPath(AssetPath, Paths, Error))
    {
        RaiseError(Error);
        return false;
    }
    IFileManager& Files = IFileManager::Get();
    if (!Files.FileExists(*Paths.FullPayloadPath))
    {
        return false;
    }
    return Files.Delete(*Paths.FullPayloadPath, true, true, true);
}
