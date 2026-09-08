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
//   - Native construction of reviewed simple-unlit world master materials.
// - Must-Not:
//   - Parse source catalogs, save packages, overwrite assets, or invent maps.
// - Allows:
//   - Verified plan fields already classified by the conversion pipeline.
// - Split-When:
//   - Material Instances or additional shader families gain separate policy.
// - Merge-When:
//   - Another editor adapter owns identical native material construction.
// - Summary:
//   - Simple-unlit world master material construction adapter.
// - Description:
//   - Builds native material graphs from reviewed world raster-family requests.
// - Usage:
//   - Exposed through the Shar world-material ToolsetRegistry toolset.
// - Defaults:
//   - Regular world presentation is two-sided to reproduce source CullNone.
//

//! Simple-unlit world master material construction adapter.

#include "Materials/SharWorldMaterialToolset.h"

#include "Materials/SharWorldMaterialPolicy.h"

#include "AssetToolsModule.h"
#include "Factories/MaterialFactoryNew.h"
#include "Kismet/KismetSystemLibrary.h"
#include "MaterialEditingLibrary.h"
#include "Materials/Material.h"
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

void DiscardCreatedWorldMaterial(UMaterial* Material)
{
    if (Material == nullptr)
    {
        return;
    }
    TArray<UObject*> Objects{Material};
    (void)ObjectTools::ForceDeleteObjects(Objects, false);
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
