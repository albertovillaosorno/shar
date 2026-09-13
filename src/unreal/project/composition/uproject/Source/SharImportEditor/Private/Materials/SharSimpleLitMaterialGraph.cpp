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
//   - Native expression construction and read-back for reviewed simple/lit
//   - opaque material graphs.
// - Must-Not:
//   - Select source families, infer translucency, or save assets.
// - Allows:
//   - Pure3D diffuse texture modulation and source shininess conversion.
// - Split-When:
//   - A lit source subset requires coloured specular or translucent response.
// - Merge-When:
//   - Another private adapter owns identical expression construction.
// - Summary:
//   - Shared simple/lit material graph implementation.
// - Description:
//   - Uses DefaultLit with a dielectric zero-specular subset and the Phong
//   - power-to-roughness relation retained in Unreal Engine source.
// - Usage:
//   - Invoked only after source-state policy validation.
// - Defaults:
//   - Metallic and source specular response are both zero.
//

//! Shared simple/lit material graph implementation.

#include "Materials/SharSimpleLitMaterialGraph.h"

#include "MaterialEditingLibrary.h"
#include "Materials/Material.h"
#include "Materials/MaterialExpressionMultiply.h"
#include "Materials/MaterialExpressionScalarParameter.h"
#include "Materials/MaterialExpressionTextureSampleParameter2D.h"
#include "Materials/MaterialExpressionVectorParameter.h"
#include "Materials/MaterialExpressionVertexColor.h"

namespace UE::SharImportEditor::Private
{
namespace
{
constexpr TCHAR BaseColorTextureParameter[] = TEXT("BaseColorTexture");
constexpr TCHAR BaseColorTintParameter[] = TEXT("BaseColorTint");
constexpr TCHAR RoughnessParameter[] = TEXT("SourceRoughness");
constexpr TCHAR SpecularParameter[] = TEXT("SourceSpecularStrength");

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
        OutError = TEXT("failed to create simple-lit material expression");
    }
    return Expression;
}

const UMaterialExpressionScalarParameter* FindScalar(
    const UMaterial& Material,
    const FName Name
)
{
    for (UMaterialExpression* Expression : Material.GetExpressions())
    {
        const auto* Parameter =
            Cast<UMaterialExpressionScalarParameter>(Expression);
        if (Parameter != nullptr && Parameter->ParameterName == Name)
        {
            return Parameter;
        }
    }
    return nullptr;
}

bool HasNamedTextureParameter(const UMaterial& Material)
{
    for (UMaterialExpression* Expression : Material.GetExpressions())
    {
        const auto* Parameter =
            Cast<UMaterialExpressionTextureSampleParameter2D>(Expression);
        if (
            Parameter != nullptr
            && Parameter->ParameterName == BaseColorTextureParameter
        )
        {
            return true;
        }
    }
    return false;
}

bool HasNeutralTintParameter(const UMaterial& Material)
{
    for (UMaterialExpression* Expression : Material.GetExpressions())
    {
        const auto* Parameter = Cast<UMaterialExpressionVectorParameter>(
            Expression
        );
        if (
            Parameter != nullptr
            && Parameter->ParameterName == BaseColorTintParameter
            && Parameter->DefaultValue.Equals(FLinearColor::White)
        )
        {
            return true;
        }
    }
    return false;
}


bool HasTextureTintModulation(const UMaterial& Material)
{
    for (UMaterialExpression* Expression : Material.GetExpressions())
    {
        const auto* Multiply = Cast<UMaterialExpressionMultiply>(Expression);
        if (Multiply == nullptr)
        {
            continue;
        }
        const auto IsPair = [](
            const FExpressionInput& TextureInput,
            const FExpressionInput& TintInput
        )
        {
            const auto* Texture =
                Cast<UMaterialExpressionTextureSampleParameter2D>(
                    TextureInput.Expression
                );
            const auto* Tint = Cast<UMaterialExpressionVectorParameter>(
                TintInput.Expression
            );
            return Texture != nullptr
                && Texture->ParameterName == BaseColorTextureParameter
                && TextureInput.OutputIndex == 0
                && Tint != nullptr
                && Tint->ParameterName == BaseColorTintParameter
                && TintInput.OutputIndex == 0;
        };
        if (
            IsPair(Multiply->A, Multiply->B)
            || IsPair(Multiply->B, Multiply->A)
        )
        {
            return true;
        }
    }
    return false;
}
} // namespace

float SourceShininessToRoughness(float SourceShininess)
{
    return FMath::Pow(SourceShininess * 0.5F + 1.0F, -0.25F);
}

bool BuildSimpleLitMaterialGraph(
    UMaterial& Material,
    const FSharSimpleLitMaterialGraphRecipe& Recipe,
    FString& OutError
)
{
    OutError.Reset();
    Material.MaterialDomain = MD_Surface;
    Material.BlendMode = BLEND_Opaque;
    Material.TwoSided = Recipe.bTwoSided;
    Material.SetShadingModel(MSM_DefaultLit);

    auto* Texture = AddExpression<UMaterialExpressionTextureSampleParameter2D>(
        Material,
        -650,
        -100,
        OutError
    );
    auto* Tint = AddExpression<UMaterialExpressionVectorParameter>(
        Material,
        -650,
        80,
        OutError
    );
    auto* BaseColor = AddExpression<UMaterialExpressionMultiply>(
        Material,
        -360,
        -80,
        OutError
    );
    auto* Roughness = AddExpression<UMaterialExpressionScalarParameter>(
        Material,
        -360,
        140,
        OutError
    );
    auto* Specular = AddExpression<UMaterialExpressionScalarParameter>(
        Material,
        -360,
        260,
        OutError
    );
    if (
        Texture == nullptr
        || Tint == nullptr
        || BaseColor == nullptr
        || Roughness == nullptr
        || Specular == nullptr
    )
    {
        return false;
    }

    Texture->ParameterName = BaseColorTextureParameter;
    Texture->SetDefaultTexture();
    Texture->AutoSetSampleType();
    Tint->ParameterName = BaseColorTintParameter;
    Tint->DefaultValue = FLinearColor::White;
    Roughness->ParameterName = RoughnessParameter;
    Roughness->DefaultValue = SourceShininessToRoughness(
        Recipe.SourceShininess
    );
    Roughness->SliderMin = 0.0F;
    Roughness->SliderMax = 1.0F;
    Specular->ParameterName = SpecularParameter;
    Specular->DefaultValue = 0.0F;
    Specular->SliderMin = 0.0F;
    Specular->SliderMax = 1.0F;

    // Pure3D's fixed-function lit path disables the mesh colour array. Its
    // base texture is therefore modulated by material diffuse, not vertex
    // colour. BaseColorTint carries that verified diffuse value per instance.
    BaseColor->A.Expression = Texture;
    BaseColor->B.Expression = Tint;

    if (
        !UMaterialEditingLibrary::ConnectMaterialProperty(
            BaseColor,
            TEXT(""),
            MP_BaseColor
        )
        || !UMaterialEditingLibrary::ConnectMaterialProperty(
            Roughness,
            TEXT(""),
            MP_Roughness
        )
        || !UMaterialEditingLibrary::ConnectMaterialProperty(
            Specular,
            TEXT(""),
            MP_Specular
        )
    )
    {
        OutError = TEXT("failed to connect simple-lit material properties");
        return false;
    }
    return true;
}

bool ReadBackSimpleLitMaterialGraph(
    const UMaterial& Material,
    const FSharSimpleLitMaterialGraphRecipe& Recipe,
    FString& OutError
)
{
    OutError.Reset();
    if (
        Material.MaterialDomain != MD_Surface
        || Material.BlendMode.GetValue() != BLEND_Opaque
        || !Material.GetShadingModels().HasShadingModel(MSM_DefaultLit)
    )
    {
        OutError = TEXT("simple-lit surface policy drifted");
        return false;
    }
    if ((Material.TwoSided != 0) != Recipe.bTwoSided)
    {
        OutError = TEXT("simple-lit culling state drifted");
        return false;
    }
    if (
        !HasNamedTextureParameter(Material)
        || !HasNeutralTintParameter(Material)
        || !HasTextureTintModulation(Material)
    )
    {
        OutError = TEXT("simple-lit base parameters drifted");
        return false;
    }
    for (UMaterialExpression* Expression : Material.GetExpressions())
    {
        if (Cast<UMaterialExpressionVertexColor>(Expression) != nullptr)
        {
            OutError = TEXT("simple-lit graph unexpectedly uses vertex colour");
            return false;
        }
    }
    const auto* Roughness = FindScalar(Material, RoughnessParameter);
    const auto* Specular = FindScalar(Material, SpecularParameter);
    if (
        Roughness == nullptr
        || Specular == nullptr
        || !FMath::IsNearlyEqual(
            Roughness->DefaultValue,
            SourceShininessToRoughness(Recipe.SourceShininess)
        )
        || !FMath::IsNearlyZero(Specular->DefaultValue)
    )
    {
        OutError = TEXT("simple-lit specular response drifted");
        return false;
    }
    return true;
}
} // namespace UE::SharImportEditor::Private
