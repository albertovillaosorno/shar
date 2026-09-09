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
//   - Native construction of reviewed vehicle simple/unlit materials.
// - Must-Not:
//   - Parse source catalogs, save packages, overwrite assets, or promote
//   - special vehicle presentation semantics.
// - Allows:
//   - Exact verified plan fields classified as simple/unlit graph candidates.
// - Split-When:
//   - Another shader family or runtime material mutation gains construction.
// - Merge-When:
//   - Another editor adapter owns identical vehicle master construction.
// - Summary:
//   - Vehicle simple/unlit material construction adapter.
// - Description:
//   - Applies vehicle-specific policy to the shared reviewed graph kernel and
//   - reads back the result before returning an unsaved generated asset.
// - Usage:
//   - Exposed through the Shar vehicle-material ToolsetRegistry toolset.
// - Defaults:
//   - Only graph candidates are accepted; presentation remains separately
//   - gated.
//

//! Vehicle simple/unlit material construction adapter.

#include "Materials/SharVehicleMaterialToolset.h"

#include "Materials/SharVehicleMaterialPolicy.h"

#include "AssetToolsModule.h"
#include "Factories/MaterialFactoryNew.h"
#include "Factories/MaterialInstanceConstantFactoryNew.h"
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

bool BuildSimpleUnlitVehicleMaster(
    UMaterial& Material,
    const FSharSimpleUnlitVehicleMasterRecipe& Recipe,
    FString& OutError
)
{
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
    return ReadBackSimpleUnlitMaterialGraph(Material, GraphRecipe, OutError);
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

FString USharVehicleMaterialToolset::CreateSimpleUnlitVehicleMaterialInstance(
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
