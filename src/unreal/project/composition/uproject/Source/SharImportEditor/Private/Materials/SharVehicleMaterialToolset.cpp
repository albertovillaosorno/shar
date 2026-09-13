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
//   - Native construction of reviewed vehicle simple materials.
// - Must-Not:
//   - Parse source catalogs, save packages, overwrite assets, or promote
//   - special vehicle presentation semantics.
// - Allows:
//   - Exact verified plan fields in reviewed simple material subsets.
// - Split-When:
//   - Another shader family or runtime material mutation gains construction.
// - Merge-When:
//   - Another editor adapter owns identical vehicle master construction.
// - Summary:
//   - Vehicle simple material construction adapter.
// - Description:
//   - Applies vehicle-specific policy to the shared reviewed graph kernel and
//   - reads back the result before returning an unsaved generated asset.
// - Usage:
//   - Exposed through the Shar vehicle-material ToolsetRegistry toolset.
// - Defaults:
//   - Only graph candidates are accepted; presentation remains separately
//   - gated.
//

//! Vehicle simple material construction adapter.

// CSpell:ignore MATUSAGE

#include "Materials/SharVehicleMaterialToolset.h"

#include "Materials/SharVehicleMaterialPolicy.h"

#include "AssetToolsModule.h"
#include "Factories/MaterialFactoryNew.h"
#include "Factories/MaterialInstanceConstantFactoryNew.h"
#include "Engine/SkeletalMesh.h"
#include "Engine/Texture2D.h"
#include "Kismet/KismetSystemLibrary.h"
#include "MaterialEditingLibrary.h"
#include "Materials/Material.h"
#include "Materials/MaterialInstanceConstant.h"
#include "Misc/PackageName.h"
#include "ObjectTools.h"

namespace UE::SharImportEditor::Private
{
namespace
{
constexpr TCHAR GeneratedVehicleMaterialRoot[] =
    TEXT("/Game/Generated/SHAR/Materials/Vehicles");
constexpr TCHAR GeneratedVehicleMaterialRootPrefix[] =
    TEXT("/Game/Generated/SHAR/Materials/Vehicles/");
constexpr TCHAR GeneratedVehicleTextureRootPrefix[] =
    TEXT("/Game/Generated/SHAR/Textures/Vehicles/");
constexpr TCHAR GeneratedVehicleMasterRootPrefix[] =
    TEXT("/Game/Generated/SHAR/Materials/Vehicles/Masters/");
constexpr TCHAR GeneratedVehicleInstanceRootPrefix[] =
    TEXT("/Game/Generated/SHAR/Materials/Vehicles/Instances/");
constexpr TCHAR GeneratedVehicleSkeletalMeshRootPrefix[] =
    TEXT("/Game/Generated/SHAR/cars/");
constexpr TCHAR BaseColorTextureParameter[] = TEXT("BaseColorTexture");
constexpr TCHAR BaseColorTintParameter[] = TEXT("BaseColorTint");
constexpr TCHAR AlphaReferenceParameter[] = TEXT("AlphaReference");

void RaiseVehicleMaterialError(const FString& Message)
{
    UKismetSystemLibrary::RaiseScriptError(
        FString::Printf(TEXT("SharVehicleMaterialToolset: %s"), *Message)
    );
}

bool ValidateVehicleMaterialDestination(
    const FString& FolderPath,
    const FString& AssetName,
    FString& OutPackagePath,
    FString& OutObjectPath,
    FString& OutError
)
{
    OutError.Reset();
    if (
        !FolderPath.Equals(
            GeneratedVehicleMaterialRoot,
            ESearchCase::CaseSensitive
        )
        && !FolderPath.StartsWith(
            GeneratedVehicleMaterialRootPrefix,
            ESearchCase::CaseSensitive
        )
    )
    {
        OutError = TEXT(
            "folder_path must be beneath "
            "/Game/Generated/SHAR/Materials/Vehicles"
        );
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
        OutError = TEXT("vehicle material destination package path is invalid");
        return false;
    }
    OutObjectPath = FString::Printf(
        TEXT("%s.%s"),
        *OutPackagePath,
        *AssetName
    );
    if (
        FindObject<UObject>(nullptr, *OutObjectPath) != nullptr
        || FindPackage(nullptr, *OutPackagePath) != nullptr
        || FPackageName::DoesPackageExist(OutPackagePath)
    )
    {
        OutError = TEXT("vehicle master material output already exists");
        return false;
    }
    return true;
}

void DiscardCreatedVehicleMaterial(UObject* Material)
{
    if (Material == nullptr)
    {
        return;
    }
    TArray<UObject*> Objects{Material};
    (void)ObjectTools::ForceDeleteObjects(Objects, false);
}

bool IsCanonicalGeneratedObjectPath(
    const FString& ObjectPath,
    const FString& RequiredPackagePrefix
)
{
    const FString PackagePath =
        FPackageName::ObjectPathToPackageName(ObjectPath);
    const FString AssetName =
        FPackageName::ObjectPathToObjectName(ObjectPath);
    return !PackagePath.IsEmpty()
        && !AssetName.IsEmpty()
        && FPackageName::IsValidLongPackageName(PackagePath)
        && FPackageName::GetLongPackageAssetName(PackagePath) == AssetName
        && PackagePath.StartsWith(
            RequiredPackagePrefix,
            ESearchCase::CaseSensitive
        )
        && ObjectPath.Equals(
            FString::Printf(TEXT("%s.%s"), *PackagePath, *AssetName),
            ESearchCase::CaseSensitive
        );
}

template <typename TObject>
TObject* FindOrLoadGeneratedObject(const FString& ObjectPath)
{
    TObject* Object = FindObject<TObject>(nullptr, *ObjectPath);
    return Object != nullptr
        ? Object
        : LoadObject<TObject>(nullptr, *ObjectPath);
}

bool HasParameter(
    const UMaterialInterface& Material,
    const FName ParameterName,
    EMaterialParameterType Type
)
{
    TArray<FMaterialParameterInfo> Infos;
    TArray<FGuid> Ids;
    Material.GetAllParameterInfoOfType(Type, Infos, Ids);
    return Infos.ContainsByPredicate(
        [ParameterName](const FMaterialParameterInfo& Info)
        {
            return Info.Name == ParameterName;
        }
    );
}

bool IsNormalizedColor(const FLinearColor& Color)
{
    const auto IsNormalized = [](const float Value)
    {
        return FMath::IsFinite(Value) && Value >= 0.0F && Value <= 1.0F;
    };
    return IsNormalized(Color.R)
        && IsNormalized(Color.G)
        && IsNormalized(Color.B)
        && IsNormalized(Color.A);
}

bool ResolveVehicleMaterialInstanceInputs(
    const FString& ParentMaterialPath,
    const FString& BaseColorTexturePath,
    const FLinearColor& BaseColorTint,
    bool bSetAlphaReference,
    float AlphaReference,
    UMaterial*& OutParent,
    UTexture2D*& OutTexture,
    FString& OutError
)
{
    OutParent = nullptr;
    OutTexture = nullptr;
    OutError.Reset();
    if (!IsCanonicalGeneratedObjectPath(
            ParentMaterialPath,
            GeneratedVehicleMaterialRootPrefix
        ))
    {
        OutError = TEXT("parent is not a canonical generated vehicle material");
        return false;
    }
    OutParent = FindOrLoadGeneratedObject<UMaterial>(ParentMaterialPath);
    if (OutParent == nullptr)
    {
        OutError = TEXT("parent does not resolve to a vehicle UMaterial");
        return false;
    }
    if (
        !HasParameter(
            *OutParent,
            BaseColorTintParameter,
            EMaterialParameterType::Vector
        )
        || !HasParameter(
            *OutParent,
            BaseColorTextureParameter,
            EMaterialParameterType::Texture
        )
    )
    {
        OutError = TEXT("vehicle parent material parameters drifted");
        return false;
    }
    if (!IsNormalizedColor(BaseColorTint))
    {
        OutError = TEXT("base color tint must be finite normalized RGBA");
        return false;
    }
    const bool bParentHasAlphaReference = HasParameter(
        *OutParent,
        AlphaReferenceParameter,
        EMaterialParameterType::Scalar
    );
    if (bSetAlphaReference != bParentHasAlphaReference)
    {
        OutError = TEXT("alpha-reference request does not match parent family");
        return false;
    }
    if (
        bSetAlphaReference
        && (
            !FMath::IsFinite(AlphaReference)
            || AlphaReference < 0.0F
            || AlphaReference > 1.0F
        )
    )
    {
        OutError = TEXT("alpha reference must be finite and normalized");
        return false;
    }
    if (BaseColorTexturePath.IsEmpty())
    {
        return true;
    }
    if (!IsCanonicalGeneratedObjectPath(
            BaseColorTexturePath,
            GeneratedVehicleTextureRootPrefix
        ))
    {
        OutError = TEXT("texture is not a canonical generated vehicle texture");
        return false;
    }
    OutTexture = FindOrLoadGeneratedObject<UTexture2D>(BaseColorTexturePath);
    if (OutTexture == nullptr)
    {
        OutError = TEXT("vehicle texture does not resolve to a UTexture2D");
        return false;
    }
    return true;
}
} // namespace

bool ResolveSimpleUnlitVehicleMasterRecipe(
    const FString& ShaderFamily,
    bool bLit,
    int32 BlendMode,
    bool bAlphaTest,
    int32 AlphaCompare,
    bool bTwoSided,
    FSharSimpleUnlitVehicleMasterRecipe& OutRecipe,
    FString& OutError
)
{
    OutRecipe = {};
    OutError.Reset();
    if (!ShaderFamily.Equals(TEXT("simple"), ESearchCase::CaseSensitive))
    {
        OutError = TEXT("shader_family is not the reviewed simple family");
        return false;
    }
    if (bLit)
    {
        OutError = TEXT("lit vehicle master policy is not reviewed");
        return false;
    }
    switch (BlendMode)
    {
    case 0:
        OutRecipe.Blend = ESharSimpleUnlitBlend::Opaque;
        break;
    case 1:
        OutRecipe.Blend = ESharSimpleUnlitBlend::SourceAlpha;
        break;
    case 2:
        OutRecipe.Blend = ESharSimpleUnlitBlend::Additive;
        break;
    default:
        OutError = TEXT("blend_mode is not a reviewed vehicle blend mode");
        return false;
    }
    if (AlphaCompare != 4)
    {
        OutError = TEXT("alpha_compare is not reviewed PDDI Greater");
        return false;
    }
    OutRecipe.bAlphaTest = bAlphaTest;
    OutRecipe.bTwoSided = bTwoSided;
    return true;
}

bool ResolveSimpleLitVehicleMasterRecipe(
    const FString& ShaderFamily,
    bool bLit,
    int32 BlendMode,
    bool bAlphaTest,
    int32 AlphaCompare,
    bool bTwoSided,
    FLinearColor SourceAmbient,
    FLinearColor SourceSpecular,
    FLinearColor SourceEmissive,
    float SourceShininess,
    FSharSimpleLitVehicleMasterRecipe& OutRecipe,
    FString& OutError
)
{
    OutRecipe = {};
    OutError.Reset();
    if (!ShaderFamily.Equals(TEXT("simple"), ESearchCase::CaseSensitive))
    {
        OutError = TEXT("shader_family is not the reviewed simple family");
        return false;
    }
    if (!bLit)
    {
        OutError = TEXT("simple-lit vehicle master requires lit source state");
        return false;
    }
    if (BlendMode != 0 || bAlphaTest || AlphaCompare != 4)
    {
        OutError = TEXT("simple-lit raster policy is not reviewed");
        return false;
    }
    const auto IsBlack = [](const FLinearColor& Color)
    {
        return IsNormalizedColor(Color)
            && FMath::IsNearlyZero(Color.R)
            && FMath::IsNearlyZero(Color.G)
            && FMath::IsNearlyZero(Color.B)
            && FMath::IsNearlyEqual(Color.A, 1.0F);
    };
    if (
        !IsBlack(SourceAmbient)
        || !IsBlack(SourceSpecular)
        || !IsBlack(SourceEmissive)
    )
    {
        OutError = TEXT(
            "simple-lit nonblack ambient/specular/emissive is not reviewed"
        );
        return false;
    }
    if (
        !FMath::IsFinite(SourceShininess)
        || SourceShininess < 0.0F
        || SourceShininess > 128.0F
    )
    {
        OutError = TEXT("source shininess is outside the reviewed GL range");
        return false;
    }
    OutRecipe.SourceShininess = SourceShininess;
    OutRecipe.bTwoSided = bTwoSided;
    return true;
}

bool HasVehicleSkeletalUsage(
    const UMaterial& Material,
    FString& OutError
)
{
    if (!Material.GetUsageByFlag(MATUSAGE_SkeletalMesh))
    {
        OutError = TEXT("vehicle master lacks SkeletalMesh material usage");
        return false;
    }
    return true;
}

bool UpgradeVehicleMasterSkeletalUsage(
    UMaterial& Material,
    const TFunctionRef<bool(FString&)> ReadGraph,
    FString& OutError
)
{
    if (Material.GetPackage()->IsDirty())
    {
        OutError = TEXT("vehicle master must be clean before usage upgrade");
        return false;
    }
    if (!ReadGraph(OutError))
    {
        return false;
    }
    if (Material.GetUsageByFlag(MATUSAGE_SkeletalMesh))
    {
        return true;
    }

    Material.Modify();
    Material.SetUsageByFlag(MATUSAGE_SkeletalMesh, true);
    Material.PostEditChange();
    if (
        !Material.GetUsageByFlag(MATUSAGE_SkeletalMesh)
        || !ReadGraph(OutError)
    )
    {
        Material.SetUsageByFlag(MATUSAGE_SkeletalMesh, false);
        Material.PostEditChange();
        Material.GetPackage()->SetDirtyFlag(false);
        if (OutError.IsEmpty())
        {
            OutError = TEXT("vehicle master usage upgrade read-back drifted");
        }
        return false;
    }
    Material.MarkPackageDirty();
    return true;
}

bool BuildSimpleLitVehicleMaster(
    UMaterial& Material,
    const FSharSimpleLitVehicleMasterRecipe& Recipe,
    FString& OutError
)
{
    Material.SetUsageByFlag(MATUSAGE_SkeletalMesh, true);
    const FSharSimpleLitMaterialGraphRecipe GraphRecipe{
        Recipe.SourceShininess,
        Recipe.bTwoSided,
    };
    return BuildSimpleLitMaterialGraph(Material, GraphRecipe, OutError);
}

bool ReadBackSimpleLitVehicleMaster(
    const UMaterial& Material,
    const FSharSimpleLitVehicleMasterRecipe& Recipe,
    FString& OutError
)
{
    const FSharSimpleLitMaterialGraphRecipe GraphRecipe{
        Recipe.SourceShininess,
        Recipe.bTwoSided,
    };
    return HasVehicleSkeletalUsage(Material, OutError)
        && ReadBackSimpleLitMaterialGraph(Material, GraphRecipe, OutError);
}

bool BuildSimpleUnlitVehicleMaster(
    UMaterial& Material,
    const FSharSimpleUnlitVehicleMasterRecipe& Recipe,
    FString& OutError
)
{
    Material.SetUsageByFlag(MATUSAGE_SkeletalMesh, true);
    const FSharSimpleUnlitMaterialGraphRecipe GraphRecipe{
        Recipe.Blend,
        Recipe.bAlphaTest,
        Recipe.bTwoSided,
    };
    return BuildSimpleUnlitMaterialGraph(Material, GraphRecipe, OutError);
}

bool ReadBackSimpleUnlitVehicleMaster(
    const UMaterial& Material,
    const FSharSimpleUnlitVehicleMasterRecipe& Recipe,
    FString& OutError
)
{
    const FSharSimpleUnlitMaterialGraphRecipe GraphRecipe{
        Recipe.Blend,
        Recipe.bAlphaTest,
        Recipe.bTwoSided,
    };
    return HasVehicleSkeletalUsage(Material, OutError)
        && ReadBackSimpleUnlitMaterialGraph(Material, GraphRecipe, OutError);
}

FString MaterialObjectPath(const UMaterialInterface* Material)
{
    return Material == nullptr ? FString{} : Material->GetPathName();
}

bool ResolveVehicleSkeletalMesh(
    const FString& SkeletalMeshPath,
    USkeletalMesh*& OutMesh,
    FString& OutError
)
{
    OutMesh = nullptr;
    OutError.Reset();
    if (!IsCanonicalGeneratedObjectPath(
            SkeletalMeshPath,
            GeneratedVehicleSkeletalMeshRootPrefix
        ))
    {
        OutError = TEXT("skeletal_mesh_path is not a generated vehicle mesh");
        return false;
    }
    OutMesh = FindOrLoadGeneratedObject<USkeletalMesh>(SkeletalMeshPath);
    if (OutMesh == nullptr)
    {
        OutError = TEXT("generated vehicle Skeletal Mesh does not exist");
        return false;
    }
    return true;
}

bool ValidateVehicleSlotSelection(
    const USkeletalMesh& Mesh,
    const TArray<int32>& SlotIndices,
    const TArray<FString>& ExpectedSlotNames,
    FString& OutError
)
{
    OutError.Reset();
    if (SlotIndices.IsEmpty() || SlotIndices.Num() != ExpectedSlotNames.Num())
    {
        OutError = TEXT("slot selection arrays must be nonempty and aligned");
        return false;
    }
    const TArray<FSkeletalMaterial>& Materials = Mesh.GetMaterials();
    int32 PreviousIndex = INDEX_NONE;
    for (int32 Position = 0; Position < SlotIndices.Num(); ++Position)
    {
        const int32 SlotIndex = SlotIndices[Position];
        if (SlotIndex <= PreviousIndex || !Materials.IsValidIndex(SlotIndex))
        {
            OutError = TEXT(
                "slot indices must be unique, ascending, and valid"
            );
            return false;
        }
        if (
            ExpectedSlotNames[Position].IsEmpty()
            || !Materials[SlotIndex].MaterialSlotName.ToString().Equals(
                ExpectedSlotNames[Position],
                ESearchCase::CaseSensitive
            )
        )
        {
            OutError = TEXT("Skeletal Mesh material slot name drifted");
            return false;
        }
        PreviousIndex = SlotIndex;
    }
    return true;
}

bool ValidateVehicleMaterialPath(const FString& MaterialPath)
{
    return MaterialPath.IsEmpty()
        || IsCanonicalGeneratedObjectPath(
            MaterialPath,
            GeneratedVehicleInstanceRootPrefix
        );
}

bool ResolveVehicleReplacementMaterials(
    const TArray<FString>& ReplacementMaterialPaths,
    TArray<UMaterialInterface*>& OutMaterials,
    FString& OutError
)
{
    OutMaterials.Reset();
    OutError.Reset();
    for (const FString& MaterialPath : ReplacementMaterialPaths)
    {
        if (!ValidateVehicleMaterialPath(MaterialPath))
        {
            OutError = TEXT("replacement material path is not canonical");
            return false;
        }
        UMaterialInstanceConstant* Material = MaterialPath.IsEmpty()
            ? nullptr
            : FindOrLoadGeneratedObject<UMaterialInstanceConstant>(
                MaterialPath
            );
        if (!MaterialPath.IsEmpty() && Material == nullptr)
        {
            OutError = TEXT(
                "replacement vehicle Material Instance does not exist"
            );
            return false;
        }
        OutMaterials.Add(Material);
    }
    return true;
}

bool ReadVehicleMaterialPaths(
    const USkeletalMesh& Mesh,
    const TArray<int32>& SlotIndices,
    TArray<FString>& OutMaterialPaths
)
{
    OutMaterialPaths.Reset();
    const TArray<FSkeletalMaterial>& Materials = Mesh.GetMaterials();
    for (const int32 SlotIndex : SlotIndices)
    {
        if (!Materials.IsValidIndex(SlotIndex))
        {
            OutMaterialPaths.Reset();
            return false;
        }
        OutMaterialPaths.Add(
            MaterialObjectPath(Materials[SlotIndex].MaterialInterface)
        );
    }
    return true;
}
} // namespace UE::SharImportEditor::Private

FString USharVehicleMaterialToolset::CreateSimpleUnlitVehicleMaster(
    const FString& FolderPath,
    const FString& AssetName,
    const FString& ShaderFamily,
    bool bLit,
    int32 BlendMode,
    bool bAlphaTest,
    int32 AlphaCompare,
    bool bTwoSided
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    FSharSimpleUnlitVehicleMasterRecipe Recipe;
    if (!ResolveSimpleUnlitVehicleMasterRecipe(
            ShaderFamily,
            bLit,
            BlendMode,
            bAlphaTest,
            AlphaCompare,
            bTwoSided,
            Recipe,
            Error
        ))
    {
        RaiseVehicleMaterialError(Error);
        return {};
    }
    FString PackagePath;
    FString ObjectPath;
    if (!ValidateVehicleMaterialDestination(
            FolderPath,
            AssetName,
            PackagePath,
            ObjectPath,
            Error
        ))
    {
        RaiseVehicleMaterialError(Error);
        return {};
    }

    UMaterialFactoryNew* Factory = NewObject<UMaterialFactoryNew>();
    UMaterial* Material = Cast<UMaterial>(
        FAssetToolsModule::GetModule().Get().CreateAsset(
            AssetName,
            FolderPath,
            UMaterial::StaticClass(),
            Factory,
            NAME_None,
            false
        )
    );
    if (Material == nullptr)
    {
        RaiseVehicleMaterialError(
            TEXT("failed to create vehicle master material")
        );
        return {};
    }
    if (!BuildSimpleUnlitVehicleMaster(*Material, Recipe, Error))
    {
        DiscardCreatedVehicleMaterial(Material);
        RaiseVehicleMaterialError(Error);
        return {};
    }
    UMaterialEditingLibrary::LayoutMaterialExpressions(Material);
    const TArray<FString> CompileErrors =
        UMaterialEditingLibrary::RecompileMaterial(Material);
    if (!CompileErrors.IsEmpty())
    {
        DiscardCreatedVehicleMaterial(Material);
        RaiseVehicleMaterialError(
            TEXT("vehicle master material did not compile")
        );
        return {};
    }
    if (!ReadBackSimpleUnlitVehicleMaster(*Material, Recipe, Error))
    {
        DiscardCreatedVehicleMaterial(Material);
        RaiseVehicleMaterialError(Error);
        return {};
    }
    Material->MarkPackageDirty();
    return ObjectPath;
}

FString USharVehicleMaterialToolset::CreateSimpleLitVehicleMaster(
    const FString& FolderPath,
    const FString& AssetName,
    const FString& ShaderFamily,
    bool bLit,
    int32 BlendMode,
    bool bAlphaTest,
    int32 AlphaCompare,
    bool bTwoSided,
    FLinearColor SourceAmbient,
    FLinearColor SourceSpecular,
    FLinearColor SourceEmissive,
    float SourceShininess
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    FSharSimpleLitVehicleMasterRecipe Recipe;
    if (!ResolveSimpleLitVehicleMasterRecipe(
            ShaderFamily,
            bLit,
            BlendMode,
            bAlphaTest,
            AlphaCompare,
            bTwoSided,
            SourceAmbient,
            SourceSpecular,
            SourceEmissive,
            SourceShininess,
            Recipe,
            Error
        ))
    {
        RaiseVehicleMaterialError(Error);
        return {};
    }
    FString PackagePath;
    FString ObjectPath;
    if (!ValidateVehicleMaterialDestination(
            FolderPath,
            AssetName,
            PackagePath,
            ObjectPath,
            Error
        ))
    {
        RaiseVehicleMaterialError(Error);
        return {};
    }

    UMaterialFactoryNew* Factory = NewObject<UMaterialFactoryNew>();
    UMaterial* Material = Cast<UMaterial>(
        FAssetToolsModule::GetModule().Get().CreateAsset(
            AssetName,
            FolderPath,
            UMaterial::StaticClass(),
            Factory,
            NAME_None,
            false
        )
    );
    if (Material == nullptr)
    {
        RaiseVehicleMaterialError(
            TEXT("failed to create simple-lit vehicle master material")
        );
        return {};
    }
    if (!BuildSimpleLitVehicleMaster(*Material, Recipe, Error))
    {
        DiscardCreatedVehicleMaterial(Material);
        RaiseVehicleMaterialError(Error);
        return {};
    }
    UMaterialEditingLibrary::LayoutMaterialExpressions(Material);
    const TArray<FString> CompileErrors =
        UMaterialEditingLibrary::RecompileMaterial(Material);
    if (!CompileErrors.IsEmpty())
    {
        DiscardCreatedVehicleMaterial(Material);
        RaiseVehicleMaterialError(
            TEXT("simple-lit vehicle master material did not compile")
        );
        return {};
    }
    if (!ReadBackSimpleLitVehicleMaster(*Material, Recipe, Error))
    {
        DiscardCreatedVehicleMaterial(Material);
        RaiseVehicleMaterialError(Error);
        return {};
    }
    Material->MarkPackageDirty();
    return ObjectPath;
}

bool USharVehicleMaterialToolset::VerifySimpleUnlitVehicleMaster(
    const FString& ObjectPath,
    const FString& ShaderFamily,
    bool bLit,
    int32 BlendMode,
    bool bAlphaTest,
    int32 AlphaCompare,
    bool bTwoSided
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    FSharSimpleUnlitVehicleMasterRecipe Recipe;
    if (!ResolveSimpleUnlitVehicleMasterRecipe(
            ShaderFamily,
            bLit,
            BlendMode,
            bAlphaTest,
            AlphaCompare,
            bTwoSided,
            Recipe,
            Error
        ))
    {
        RaiseVehicleMaterialError(Error);
        return false;
    }
    if (!IsCanonicalGeneratedObjectPath(
            ObjectPath,
            GeneratedVehicleMasterRootPrefix
        ))
    {
        RaiseVehicleMaterialError(
            TEXT("master is not a canonical generated vehicle material")
        );
        return false;
    }
    UMaterial* Material = FindOrLoadGeneratedObject<UMaterial>(ObjectPath);
    return Material != nullptr
        && ReadBackSimpleUnlitVehicleMaster(*Material, Recipe, Error);
}

bool USharVehicleMaterialToolset::VerifySimpleLitVehicleMaster(
    const FString& ObjectPath,
    const FString& ShaderFamily,
    bool bLit,
    int32 BlendMode,
    bool bAlphaTest,
    int32 AlphaCompare,
    bool bTwoSided,
    FLinearColor SourceAmbient,
    FLinearColor SourceSpecular,
    FLinearColor SourceEmissive,
    float SourceShininess
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    FSharSimpleLitVehicleMasterRecipe Recipe;
    if (!ResolveSimpleLitVehicleMasterRecipe(
            ShaderFamily,
            bLit,
            BlendMode,
            bAlphaTest,
            AlphaCompare,
            bTwoSided,
            SourceAmbient,
            SourceSpecular,
            SourceEmissive,
            SourceShininess,
            Recipe,
            Error
        ))
    {
        RaiseVehicleMaterialError(Error);
        return false;
    }
    if (!IsCanonicalGeneratedObjectPath(
            ObjectPath,
            GeneratedVehicleMasterRootPrefix
        ))
    {
        RaiseVehicleMaterialError(
            TEXT("master is not a canonical generated vehicle material")
        );
        return false;
    }
    UMaterial* Material = FindOrLoadGeneratedObject<UMaterial>(ObjectPath);
    return Material != nullptr
        && ReadBackSimpleLitVehicleMaster(*Material, Recipe, Error);
}

bool USharVehicleMaterialToolset::UpgradeSimpleUnlitVehicleMasterSkeletalUsage(
    const FString& ObjectPath,
    const FString& ShaderFamily,
    bool bLit,
    int32 BlendMode,
    bool bAlphaTest,
    int32 AlphaCompare,
    bool bTwoSided
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    FSharSimpleUnlitVehicleMasterRecipe Recipe;
    if (!ResolveSimpleUnlitVehicleMasterRecipe(
            ShaderFamily,
            bLit,
            BlendMode,
            bAlphaTest,
            AlphaCompare,
            bTwoSided,
            Recipe,
            Error
        )
        || !IsCanonicalGeneratedObjectPath(
            ObjectPath,
            GeneratedVehicleMasterRootPrefix
        ))
    {
        if (Error.IsEmpty())
        {
            Error = TEXT(
                "master is not a canonical generated vehicle material"
            );
        }
        RaiseVehicleMaterialError(Error);
        return false;
    }
    UMaterial* Material = FindOrLoadGeneratedObject<UMaterial>(ObjectPath);
    if (Material == nullptr)
    {
        RaiseVehicleMaterialError(TEXT("vehicle master does not exist"));
        return false;
    }
    const FSharSimpleUnlitMaterialGraphRecipe GraphRecipe{
        Recipe.Blend,
        Recipe.bAlphaTest,
        Recipe.bTwoSided,
    };
    const auto ReadGraph = [Material, &GraphRecipe](FString& GraphError)
    {
        return ReadBackSimpleUnlitMaterialGraph(
            *Material,
            GraphRecipe,
            GraphError
        );
    };
    if (!UpgradeVehicleMasterSkeletalUsage(*Material, ReadGraph, Error))
    {
        RaiseVehicleMaterialError(Error);
        return false;
    }
    return ReadBackSimpleUnlitVehicleMaster(*Material, Recipe, Error);
}

bool USharVehicleMaterialToolset::UpgradeSimpleLitVehicleMasterSkeletalUsage(
    const FString& ObjectPath,
    const FString& ShaderFamily,
    bool bLit,
    int32 BlendMode,
    bool bAlphaTest,
    int32 AlphaCompare,
    bool bTwoSided,
    FLinearColor SourceAmbient,
    FLinearColor SourceSpecular,
    FLinearColor SourceEmissive,
    float SourceShininess
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    FSharSimpleLitVehicleMasterRecipe Recipe;
    if (!ResolveSimpleLitVehicleMasterRecipe(
            ShaderFamily,
            bLit,
            BlendMode,
            bAlphaTest,
            AlphaCompare,
            bTwoSided,
            SourceAmbient,
            SourceSpecular,
            SourceEmissive,
            SourceShininess,
            Recipe,
            Error
        )
        || !IsCanonicalGeneratedObjectPath(
            ObjectPath,
            GeneratedVehicleMasterRootPrefix
        ))
    {
        if (Error.IsEmpty())
        {
            Error = TEXT(
                "master is not a canonical generated vehicle material"
            );
        }
        RaiseVehicleMaterialError(Error);
        return false;
    }
    UMaterial* Material = FindOrLoadGeneratedObject<UMaterial>(ObjectPath);
    if (Material == nullptr)
    {
        RaiseVehicleMaterialError(TEXT("vehicle master does not exist"));
        return false;
    }
    const FSharSimpleLitMaterialGraphRecipe GraphRecipe{
        Recipe.SourceShininess,
        Recipe.bTwoSided,
    };
    const auto ReadGraph = [Material, &GraphRecipe](FString& GraphError)
    {
        return ReadBackSimpleLitMaterialGraph(
            *Material,
            GraphRecipe,
            GraphError
        );
    };
    if (!UpgradeVehicleMasterSkeletalUsage(*Material, ReadGraph, Error))
    {
        RaiseVehicleMaterialError(Error);
        return false;
    }
    return ReadBackSimpleLitVehicleMaster(*Material, Recipe, Error);
}

FString USharVehicleMaterialToolset::CreateVehicleMaterialInstance(
    const FString& FolderPath,
    const FString& AssetName,
    const FString& ParentMaterialPath,
    const FString& BaseColorTexturePath,
    FLinearColor BaseColorTint,
    bool bSetAlphaReference,
    float AlphaReference
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    FString PackagePath;
    FString ObjectPath;
    if (!ValidateVehicleMaterialDestination(
            FolderPath,
            AssetName,
            PackagePath,
            ObjectPath,
            Error
        ))
    {
        RaiseVehicleMaterialError(Error);
        return {};
    }
    UMaterial* Parent = nullptr;
    UTexture2D* Texture = nullptr;
    if (!ResolveVehicleMaterialInstanceInputs(
            ParentMaterialPath,
            BaseColorTexturePath,
            BaseColorTint,
            bSetAlphaReference,
            AlphaReference,
            Parent,
            Texture,
            Error
        ))
    {
        RaiseVehicleMaterialError(Error);
        return {};
    }

    UMaterialInstanceConstantFactoryNew* Factory =
        NewObject<UMaterialInstanceConstantFactoryNew>();
    Factory->InitialParent = Parent;
    UMaterialInstanceConstant* Instance = Cast<UMaterialInstanceConstant>(
        FAssetToolsModule::GetModule().Get().CreateAsset(
            AssetName,
            FolderPath,
            UMaterialInstanceConstant::StaticClass(),
            Factory,
            NAME_None,
            false
        )
    );
    if (Instance == nullptr)
    {
        RaiseVehicleMaterialError(
            TEXT("failed to create vehicle material instance")
        );
        return {};
    }

    const FMaterialParameterInfo TintInfo(BaseColorTintParameter);
    Instance->SetVectorParameterValueEditorOnly(TintInfo, BaseColorTint);
    if (Texture != nullptr)
    {
        const FMaterialParameterInfo TextureInfo(BaseColorTextureParameter);
        Instance->SetTextureParameterValueEditorOnly(TextureInfo, Texture);
    }
    if (bSetAlphaReference)
    {
        const FMaterialParameterInfo AlphaInfo(AlphaReferenceParameter);
        Instance->SetScalarParameterValueEditorOnly(AlphaInfo, AlphaReference);
    }
    Instance->PostEditChange();

    const FLinearColor ReadTint =
        UMaterialEditingLibrary::GetMaterialInstanceVectorParameterValue(
            Instance,
            BaseColorTintParameter
        );
    const UTexture* ReadTexture = Texture == nullptr
        ? nullptr
        : UMaterialEditingLibrary::GetMaterialInstanceTextureParameterValue(
            Instance,
            BaseColorTextureParameter
        );
    const float ReadAlpha = bSetAlphaReference
        ? UMaterialEditingLibrary::GetMaterialInstanceScalarParameterValue(
            Instance,
            AlphaReferenceParameter
        )
        : AlphaReference;
    if (
        !ReadTint.Equals(BaseColorTint)
        || (Texture != nullptr && ReadTexture != Texture)
        || (
            bSetAlphaReference
            && !FMath::IsNearlyEqual(ReadAlpha, AlphaReference)
        )
    )
    {
        DiscardCreatedVehicleMaterial(Instance);
        RaiseVehicleMaterialError(
            TEXT("vehicle material instance read-back drifted")
        );
        return {};
    }
    Instance->MarkPackageDirty();
    return ObjectPath;
}
TArray<FString> USharVehicleMaterialToolset::ReadVehicleMaterialSlots(
    const FString& SkeletalMeshPath,
    const TArray<int32>& SlotIndices,
    const TArray<FString>& ExpectedSlotNames
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    USkeletalMesh* Mesh = nullptr;
    if (
        !ResolveVehicleSkeletalMesh(SkeletalMeshPath, Mesh, Error)
        || !ValidateVehicleSlotSelection(
            *Mesh,
            SlotIndices,
            ExpectedSlotNames,
            Error
        )
    )
    {
        RaiseVehicleMaterialError(Error);
        return {};
    }
    TArray<FString> MaterialPaths;
    if (!ReadVehicleMaterialPaths(*Mesh, SlotIndices, MaterialPaths))
    {
        RaiseVehicleMaterialError(
            TEXT("failed to read vehicle material slots")
        );
        return {};
    }
    return MaterialPaths;
}

TArray<FString>
USharVehicleMaterialToolset::CompareExchangeVehicleMaterialSlots(
    const FString& SkeletalMeshPath,
    const TArray<int32>& SlotIndices,
    const TArray<FString>& ExpectedSlotNames,
    const TArray<FString>& ExpectedMaterialPaths,
    const TArray<FString>& ReplacementMaterialPaths
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    USkeletalMesh* Mesh = nullptr;
    if (
        !ResolveVehicleSkeletalMesh(SkeletalMeshPath, Mesh, Error)
        || !ValidateVehicleSlotSelection(
            *Mesh,
            SlotIndices,
            ExpectedSlotNames,
            Error
        )
        || SlotIndices.Num() != ExpectedMaterialPaths.Num()
        || SlotIndices.Num() != ReplacementMaterialPaths.Num()
    )
    {
        if (Error.IsEmpty())
        {
            Error = TEXT("material compare-exchange arrays are not aligned");
        }
        RaiseVehicleMaterialError(Error);
        return {};
    }
    for (const FString& MaterialPath : ExpectedMaterialPaths)
    {
        if (!ValidateVehicleMaterialPath(MaterialPath))
        {
            RaiseVehicleMaterialError(
                TEXT("expected material path is not canonical")
            );
            return {};
        }
    }
    TArray<FString> CurrentMaterialPaths;
    if (!ReadVehicleMaterialPaths(*Mesh, SlotIndices, CurrentMaterialPaths))
    {
        RaiseVehicleMaterialError(
            TEXT("failed to read current material slots")
        );
        return {};
    }
    if (CurrentMaterialPaths != ExpectedMaterialPaths)
    {
        RaiseVehicleMaterialError(
            TEXT("vehicle material compare-exchange expectation drifted")
        );
        return {};
    }
    TArray<UMaterialInterface*> Replacements;
    if (!ResolveVehicleReplacementMaterials(
            ReplacementMaterialPaths,
            Replacements,
            Error
        ))
    {
        RaiseVehicleMaterialError(Error);
        return {};
    }

    TArray<FSkeletalMaterial> Materials = Mesh->GetMaterials();
    Mesh->Modify();
    for (int32 Position = 0; Position < SlotIndices.Num(); ++Position)
    {
        Materials[SlotIndices[Position]].MaterialInterface =
            Replacements[Position];
    }
    Mesh->SetMaterials(Materials);
    Mesh->PostEditChange();
    Mesh->MarkPackageDirty();

    TArray<FString> ReadBackPaths;
    if (
        !ReadVehicleMaterialPaths(*Mesh, SlotIndices, ReadBackPaths)
        || ReadBackPaths != ReplacementMaterialPaths
    )
    {
        RaiseVehicleMaterialError(
            TEXT("vehicle material slot read-back drifted after mutation")
        );
        return {};
    }
    return ReadBackPaths;
}
