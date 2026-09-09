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
//   - Native expression construction and read-back for reviewed simple/unlit
//   - material graphs.
// - Must-Not:
//   - Select world or vehicle policy, save assets, or infer source values.
// - Allows:
//   - Family-neutral Unreal graph mutation after policy validation.
// - Split-When:
//   - A reviewed shader family requires graph behavior beyond this kernel.
// - Merge-When:
//   - Another private adapter owns identical expression construction.
// - Summary:
//   - Shared simple/unlit material graph implementation.
// - Description:
//   - Reproduces source texture/vertex-colour modulation, reviewed blend modes,
//   - Greater alpha discard, and explicitly supplied culling state.
// - Usage:
//   - Invoked only through a family-specific material policy.
// - Defaults:
//   - Base-color tint is neutral white and does not alter faithful output.
//

//! Shared simple/unlit material graph implementation.

#include "Materials/SharSimpleUnlitMaterialGraph.h"

#include "MaterialEditingLibrary.h"
#include "Materials/Material.h"
#include "Materials/MaterialExpressionCustom.h"
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
constexpr TCHAR AlphaReferenceParameter[] = TEXT("AlphaReference");

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
        OutError = TEXT("failed to create simple-unlit material expression");
    }
    return Expression;
}

EBlendMode NativeBlendMode(ESharSimpleUnlitBlend Blend)
{
    switch (Blend)
    {
    case ESharSimpleUnlitBlend::Opaque:
        return BLEND_Opaque;
    case ESharSimpleUnlitBlend::SourceAlpha:
        return BLEND_Translucent;
    case ESharSimpleUnlitBlend::Additive:
        return BLEND_Additive;
    }
    return BLEND_Opaque;
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

bool HasTextureVertexModulation(
    const UMaterial& Material,
    int32 TextureOutput,
    int32 VertexOutput
)
{
    for (UMaterialExpression* Expression : Material.GetExpressions())
    {
        const auto* Multiply = Cast<UMaterialExpressionMultiply>(Expression);
        if (Multiply == nullptr)
        {
            continue;
        }
        const auto IsPair = [TextureOutput, VertexOutput](
            const FExpressionInput& TextureInput,
            const FExpressionInput& VertexInput
        )
        {
            return Cast<UMaterialExpressionTextureSampleParameter2D>(
                       TextureInput.Expression
                   ) != nullptr
                && TextureInput.OutputIndex == TextureOutput
                && Cast<UMaterialExpressionVertexColor>(
                       VertexInput.Expression
                   ) != nullptr
                && VertexInput.OutputIndex == VertexOutput;
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

bool HasAlphaDiscard(const UMaterial& Material)
{
    for (UMaterialExpression* Expression : Material.GetExpressions())
    {
        const auto* Custom = Cast<UMaterialExpressionCustom>(Expression);
        if (
            Custom != nullptr
            && Custom->Code.Contains(TEXT("clip("))
            && Custom->HasPixelDiscard()
        )
        {
            return true;
        }
    }
    return false;
}

bool HasAlphaReference(const UMaterial& Material)
{
    for (UMaterialExpression* Expression : Material.GetExpressions())
    {
        const auto* Parameter = Cast<UMaterialExpressionScalarParameter>(
            Expression
        );
        if (
            Parameter != nullptr
            && Parameter->ParameterName == AlphaReferenceParameter
            && FMath::IsNearlyEqual(Parameter->DefaultValue, 0.5F)
        )
        {
            return true;
        }
    }
    return false;
}
} // namespace

bool BuildSimpleUnlitMaterialGraph(
    UMaterial& Material,
    const FSharSimpleUnlitMaterialGraphRecipe& Recipe,
    FString& OutError
)
{
    OutError.Reset();
    Material.MaterialDomain = MD_Surface;
    Material.BlendMode = NativeBlendMode(Recipe.Blend);
    Material.TwoSided = Recipe.bTwoSided;
    Material.SetShadingModel(MSM_Unlit);

    UMaterialExpressionTextureSampleParameter2D* Texture =
        AddExpression<UMaterialExpressionTextureSampleParameter2D>(
            Material,
            -800,
            -120,
            OutError
        );
    UMaterialExpressionVertexColor* VertexColor =
        AddExpression<UMaterialExpressionVertexColor>(
            Material,
            -800,
            40,
            OutError
        );
    UMaterialExpressionVectorParameter* Tint =
        AddExpression<UMaterialExpressionVectorParameter>(
            Material,
            -800,
            200,
            OutError
        );
    UMaterialExpressionMultiply* SourceModulate =
        AddExpression<UMaterialExpressionMultiply>(
            Material,
            -520,
            -120,
            OutError
        );
    UMaterialExpressionMultiply* TintModulate =
        AddExpression<UMaterialExpressionMultiply>(
            Material,
            -280,
            -120,
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
        || VertexColor == nullptr
        || Tint == nullptr
        || SourceModulate == nullptr
        || TintModulate == nullptr
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

    // Pure3D simple/unlit multiplies texture by authored vertex colour. The
    // source programmable path supplies opaque white when the mesh has no
    // authored colour channel, so the same graph is valid for either case.
    SourceModulate->A.Expression = Texture;
    SourceModulate->B.Expression = VertexColor;
    TintModulate->A.Expression = SourceModulate;
    TintModulate->B.Expression = Tint;
    AlphaModulate->A.Connect(4, Texture);
    AlphaModulate->B.Connect(4, VertexColor);

    UMaterialExpression* FinalColor = TintModulate;
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
        Clip->Code = TEXT("clip(Alpha - AlphaReference); return 1.0;");
        Clip->Inputs.SetNum(2);
        Clip->Inputs[0].InputName = TEXT("Alpha");
        Clip->Inputs[0].Input.Expression = AlphaModulate;
        Clip->Inputs[1].InputName = AlphaReferenceParameter;
        Clip->Inputs[1].Input.Expression = Reference;
        Gate->A.Expression = TintModulate;
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
        OutError = TEXT("failed to connect simple-unlit emissive color");
        return false;
    }
    if (
        Recipe.Blend == ESharSimpleUnlitBlend::SourceAlpha
        && !UMaterialEditingLibrary::ConnectMaterialProperty(
            AlphaModulate,
            TEXT(""),
            MP_Opacity
        )
    )
    {
        OutError = TEXT("failed to connect simple-unlit source alpha");
        return false;
    }
    return true;
}

bool ReadBackSimpleUnlitMaterialGraph(
    const UMaterial& Material,
    const FSharSimpleUnlitMaterialGraphRecipe& Recipe,
    FString& OutError
)
{
    OutError.Reset();
    if (Material.MaterialDomain != MD_Surface)
    {
        OutError = TEXT("simple-unlit material domain drifted");
        return false;
    }
    if (Material.BlendMode.GetValue() != NativeBlendMode(Recipe.Blend))
    {
        OutError = TEXT("simple-unlit blend mode drifted");
        return false;
    }
    if ((Material.TwoSided != 0) != Recipe.bTwoSided)
    {
        OutError = TEXT("simple-unlit culling state drifted");
        return false;
    }
    if (!Material.GetShadingModels().HasShadingModel(MSM_Unlit))
    {
        OutError = TEXT("simple-unlit shading model drifted");
        return false;
    }
    if (
        !HasNamedTextureParameter(Material)
        || !HasNeutralTintParameter(Material)
    )
    {
        OutError = TEXT("simple-unlit base parameters drifted");
        return false;
    }
    if (
        !HasTextureVertexModulation(Material, 0, 0)
        || !HasTextureVertexModulation(Material, 4, 4)
    )
    {
        OutError = TEXT("simple-unlit source modulation drifted");
        return false;
    }
    if (
        HasAlphaDiscard(Material) != Recipe.bAlphaTest
        || HasAlphaReference(Material) != Recipe.bAlphaTest
    )
    {
        OutError = TEXT("simple-unlit alpha-test graph drifted");
        return false;
    }
    return true;
}
} // namespace UE::SharImportEditor::Private
