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
//   - Native construction of reviewed simple-unlit world materials.
// - Must-Not:
//   - Parse source catalogs, save packages, overwrite assets, or invent maps.
// - Allows:
//   - Verified plan fields already classified by the conversion pipeline.
// - Split-When:
//   - Additional shader families gain separate construction policy.
// - Merge-When:
//   - Another editor adapter owns identical native material construction.
// - Summary:
//   - Simple-unlit world material construction adapter.
// - Description:
//   - Builds native masters and instances from reviewed presentation requests.
// - Usage:
//   - Exposed through the Shar world-material ToolsetRegistry toolset.
// - Defaults:
//   - Regular world presentation is two-sided to reproduce source CullNone.
//

//! Simple-unlit world material construction adapter.

#include "Materials/SharWorldMaterialToolset.h"

#include "Materials/SharWorldMaterialPolicy.h"

#include "AssetToolsModule.h"
#include "Factories/MaterialFactoryNew.h"
#include "Factories/MaterialInstanceConstantFactoryNew.h"
#include "Kismet/KismetSystemLibrary.h"
#include "MaterialEditingLibrary.h"
#include "Engine/Texture2D.h"
#include "Materials/Material.h"
#include "Materials/MaterialInstanceConstant.h"
#include "Materials/MaterialExpressionCustom.h"
#include "Materials/MaterialExpressionMultiply.h"
#include "Materials/MaterialExpressionScalarParameter.h"
#include "Materials/MaterialExpressionTextureSampleParameter2D.h"
#include "Materials/MaterialExpressionVectorParameter.h"
#include "Misc/PackageName.h"
#include "ObjectTools.h"

namespace UE::SharImportEditor::Private
{
namespace
{
constexpr TCHAR GeneratedMaterialRoot[] =
    TEXT("/Game/Generated/SHAR/Materials");
constexpr TCHAR GeneratedMaterialRootPrefix[] =
    TEXT("/Game/Generated/SHAR/Materials/");
constexpr TCHAR GeneratedRootPrefix[] = TEXT("/Game/Generated/SHAR/");
constexpr TCHAR BaseColorTextureParameter[] = TEXT("BaseColorTexture");
constexpr TCHAR BaseColorTintParameter[] = TEXT("BaseColorTint");
constexpr TCHAR AlphaReferenceParameter[] = TEXT("AlphaReference");

void RaiseMaterialError(const FString& Message)
{
    UKismetSystemLibrary::RaiseScriptError(
        FString::Printf(TEXT("SharWorldMaterialToolset: %s"), *Message)
    );
}

bool ValidateMaterialDestination(
    const FString& FolderPath,
    const FString& AssetName,
    FString& OutPackagePath,
    FString& OutObjectPath,
    FString& OutError
)
{
    OutError.Reset();
    if (
        !FolderPath.Equals(GeneratedMaterialRoot, ESearchCase::CaseSensitive)
        && !FolderPath.StartsWith(
            GeneratedMaterialRootPrefix,
            ESearchCase::CaseSensitive
        )
    )
    {
        OutError = TEXT(
            "folder_path must be beneath /Game/Generated/SHAR/Materials"
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
        OutError = TEXT("destination package path is invalid");
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
        OutError = TEXT("world master material output already exists");
        return false;
    }
    return true;
}

template <typename TExpression>
TExpression* AddExpression(
    UMaterial& Material,
    int32 X,
    int32 Y,
    FString& OutError
)
{
    TExpression* Expression = Cast<TExpression>(
        UMaterialEditingLibrary::CreateMaterialExpression(
            &Material,
            TExpression::StaticClass(),
            X,
            Y
        )
    );
    if (Expression == nullptr)
    {
        OutError = TEXT("failed to create world master material expression");
    }
    return Expression;
}

void DiscardCreatedWorldMaterial(UObject* Material)
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
            FString::Printf(
                TEXT("%s.%s"),
                *PackagePath,
                *AssetName
            ),
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
    const auto IsNormalized = [](float Value)
    {
        return FMath::IsFinite(Value) && Value >= 0.0F && Value <= 1.0F;
    };
    return IsNormalized(Color.R)
        && IsNormalized(Color.G)
        && IsNormalized(Color.B)
        && IsNormalized(Color.A);
}

bool ResolveWorldMaterialInstanceInputs(
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
            GeneratedMaterialRootPrefix
        ))
    {
        OutError = TEXT("parent material path is not generated and canonical");
        return false;
    }
    OutParent = FindOrLoadGeneratedObject<UMaterial>(ParentMaterialPath);
    if (OutParent == nullptr)
    {
        OutError = TEXT("parent material does not resolve to a UMaterial");
        return false;
    }
    if (!HasParameter(
            *OutParent,
            BaseColorTintParameter,
            EMaterialParameterType::Vector
        ))
    {
        OutError = TEXT("parent material lacks BaseColorTint");
        return false;
    }
    if (!HasParameter(
            *OutParent,
            BaseColorTextureParameter,
            EMaterialParameterType::Texture
        ))
    {
        OutError = TEXT("parent material lacks BaseColorTexture");
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
            GeneratedRootPrefix
        ))
    {
        OutError = TEXT(
            "base color texture path is not generated and canonical"
        );
        return false;
    }
    OutTexture = FindOrLoadGeneratedObject<UTexture2D>(BaseColorTexturePath);
    if (OutTexture == nullptr)
    {
        OutError = TEXT("base color texture does not resolve to a UTexture2D");
        return false;
    }
    return true;
}

EBlendMode NativeBlendMode(ESharWorldBlendFamily Blend)
{
    switch (Blend)
    {
    case ESharWorldBlendFamily::Opaque:
        return BLEND_Opaque;
    case ESharWorldBlendFamily::SourceAlpha:
        return BLEND_Translucent;
    case ESharWorldBlendFamily::Additive:
        return BLEND_Additive;
    }
    return BLEND_Opaque;
}
} // namespace

bool ResolveSimpleUnlitWorldMasterRecipe(
    const FString& BlendFamily,
    bool bAlphaTest,
    FSharSimpleUnlitWorldMasterRecipe& OutRecipe,
    FString& OutError
)
{
    OutError.Reset();
    OutRecipe = {};
    if (BlendFamily.Equals(TEXT("opaque"), ESearchCase::CaseSensitive))
    {
        OutRecipe.Blend = ESharWorldBlendFamily::Opaque;
    }
    else if (BlendFamily.Equals(TEXT("alpha"), ESearchCase::CaseSensitive))
    {
        OutRecipe.Blend = ESharWorldBlendFamily::SourceAlpha;
    }
    else if (
        BlendFamily.Equals(TEXT("additive"), ESearchCase::CaseSensitive)
    )
    {
        OutRecipe.Blend = ESharWorldBlendFamily::Additive;
    }
    else
    {
        OutError = TEXT("blend_family is not a reviewed world blend family");
        return false;
    }
    OutRecipe.bAlphaTest = bAlphaTest;
    return true;
}

bool BuildSimpleUnlitWorldMaster(
    UMaterial& Material,
    const FSharSimpleUnlitWorldMasterRecipe& Recipe,
    FString& OutError
)
{
    OutError.Reset();
    Material.MaterialDomain = MD_Surface;
    Material.BlendMode = NativeBlendMode(Recipe.Blend);
    Material.TwoSided = true;
    Material.SetShadingModel(MSM_Unlit);

    UMaterialExpressionTextureSampleParameter2D* Texture =
        AddExpression<UMaterialExpressionTextureSampleParameter2D>(
            Material,
            -800,
            -120,
            OutError
        );
    UMaterialExpressionVectorParameter* Tint =
        AddExpression<UMaterialExpressionVectorParameter>(
            Material,
            -800,
            120,
            OutError
        );
    UMaterialExpressionMultiply* Modulate =
        AddExpression<UMaterialExpressionMultiply>(
            Material,
            -520,
            -80,
            OutError
        );
    UMaterialExpressionMultiply* AlphaModulate =
        AddExpression<UMaterialExpressionMultiply>(
            Material,
            -520,
            160,
            OutError
        );
    if (
        Texture == nullptr
        || Tint == nullptr
        || Modulate == nullptr
        || AlphaModulate == nullptr
    )
    {
        return false;
    }

    Texture->ParameterName = BaseColorTextureParameter;
    Texture->SetDefaultTexture();
    Texture->AutoSetSampleType();
    Tint->ParameterName = BaseColorTintParameter;
    Tint->DefaultValue = FLinearColor::White;
    Modulate->A.Expression = Texture;
    Modulate->B.Expression = Tint;
    AlphaModulate->A.Connect(4, Texture);
    AlphaModulate->B.Connect(4, Tint);

    UMaterialExpression* FinalColor = Modulate;
    if (Recipe.bAlphaTest)
    {
        UMaterialExpressionScalarParameter* Reference =
            AddExpression<UMaterialExpressionScalarParameter>(
                Material,
                -300,
                300,
                OutError
            );
        UMaterialExpressionCustom* Clip =
            AddExpression<UMaterialExpressionCustom>(
                Material,
                -60,
                80,
                OutError
            );
        UMaterialExpressionMultiply* Gate =
            AddExpression<UMaterialExpressionMultiply>(
                Material,
                180,
                -80,
                OutError
            );
        if (Reference == nullptr || Clip == nullptr || Gate == nullptr)
        {
            return false;
        }
        Reference->ParameterName = AlphaReferenceParameter;
        Reference->DefaultValue = 0.5F;
        Reference->SliderMin = 0.0F;
        Reference->SliderMax = 1.0F;
        Clip->Description = TEXT("Source PDDI alpha compare: Greater");
        Clip->Code = TEXT(
            "clip(Alpha - AlphaReference); return 1.0;"
        );
        Clip->Inputs.SetNum(2);
        Clip->Inputs[0].InputName = TEXT("Alpha");
        Clip->Inputs[0].Input.Expression = AlphaModulate;
        Clip->Inputs[1].InputName = AlphaReferenceParameter;
        Clip->Inputs[1].Input.Expression = Reference;
        Gate->A.Expression = Modulate;
        Gate->B.Expression = Clip;
        FinalColor = Gate;
    }

    if (
        !UMaterialEditingLibrary::ConnectMaterialProperty(
            FinalColor,
            TEXT(""),
            MP_EmissiveColor
        )
    )
    {
        OutError = TEXT("failed to connect world master emissive color");
        return false;
    }
    if (
        Recipe.Blend == ESharWorldBlendFamily::SourceAlpha
        && !UMaterialEditingLibrary::ConnectMaterialProperty(
            AlphaModulate,
            TEXT(""),
            MP_Opacity
        )
    )
    {
        OutError = TEXT("failed to connect world master source alpha");
        return false;
    }
    return true;
}
} // namespace UE::SharImportEditor::Private

FString USharWorldMaterialToolset::CreateSimpleUnlitWorldMaster(
    const FString& FolderPath,
    const FString& AssetName,
    const FString& BlendFamily,
    bool bAlphaTest
)
{
    using namespace UE::SharImportEditor::Private;
    FString Error;
    FSharSimpleUnlitWorldMasterRecipe Recipe;
    if (!ResolveSimpleUnlitWorldMasterRecipe(
            BlendFamily,
            bAlphaTest,
            Recipe,
            Error
        ))
    {
        RaiseMaterialError(Error);
        return {};
    }
    FString PackagePath;
    FString ObjectPath;
    if (!ValidateMaterialDestination(
            FolderPath,
            AssetName,
            PackagePath,
            ObjectPath,
            Error
        ))
    {
        RaiseMaterialError(Error);
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
        RaiseMaterialError(TEXT("failed to create world master material"));
        return {};
    }
    if (!BuildSimpleUnlitWorldMaster(*Material, Recipe, Error))
    {
        DiscardCreatedWorldMaterial(Material);
        RaiseMaterialError(Error);
        return {};
    }
    UMaterialEditingLibrary::LayoutMaterialExpressions(Material);
    const TArray<FString> CompileErrors =
        UMaterialEditingLibrary::RecompileMaterial(Material);
    if (!CompileErrors.IsEmpty())
    {
        DiscardCreatedWorldMaterial(Material);
        RaiseMaterialError(TEXT("world master material did not compile"));
        return {};
    }
    Material->MarkPackageDirty();
    return ObjectPath;
}

FString USharWorldMaterialToolset::CreateSimpleUnlitWorldMaterialInstance(
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
    if (!ValidateMaterialDestination(
            FolderPath,
            AssetName,
            PackagePath,
            ObjectPath,
            Error
        ))
    {
        RaiseMaterialError(Error);
        return {};
    }
    UMaterial* Parent = nullptr;
    UTexture2D* Texture = nullptr;
    if (!ResolveWorldMaterialInstanceInputs(
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
        RaiseMaterialError(Error);
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
        RaiseMaterialError(TEXT("failed to create world material instance"));
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
        DiscardCreatedWorldMaterial(Instance);
        RaiseMaterialError(TEXT("world material instance read-back drifted"));
        return {};
    }
    Instance->MarkPackageDirty();
    return ObjectPath;
}
