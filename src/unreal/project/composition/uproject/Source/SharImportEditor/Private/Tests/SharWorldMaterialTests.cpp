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
//   - Native automation tests for reviewed simple-unlit world materials.
// - Must-Not:
//   - Save assets, parse source catalogs, or test unsupported shader families.
// - Allows:
//   - Transient Unreal materials and deterministic request validation.
// - Split-When:
//   - Additional shader-family tests gain an independent lifecycle.
// - Merge-When:
//   - Another test owns the identical world-master material contract.
// - Summary:
//   - Simple-unlit world material automation tests.
// - Description:
//   - Verifies native masters and instances without persistent assets.
// - Usage:
//   - Runs in editor or commandlet automation contexts.
// - Defaults:
//   - Unsupported blend-family inputs are rejected.
//

//! Simple-unlit world material automation tests.

#if WITH_DEV_AUTOMATION_TESTS

#include "Materials/SharWorldMaterialPolicy.h"
#include "Materials/SharWorldMaterialToolset.h"

#include "Engine/Texture2D.h"
#include "MaterialEditingLibrary.h"
#include "Materials/Material.h"
#include "Materials/MaterialInstanceConstant.h"
#include "Materials/MaterialExpressionCustom.h"
#include "Materials/MaterialExpressionMultiply.h"
#include "Materials/MaterialExpressionScalarParameter.h"
#include "Materials/MaterialExpressionTextureSampleParameter2D.h"
#include "Materials/MaterialExpressionVectorParameter.h"
#include "Materials/MaterialExpressionVertexColor.h"
#include "HAL/FileManager.h"
#include "HAL/PlatformProcess.h"
#include "Misc/AutomationTest.h"
#include "Misc/PackageName.h"

namespace
{
using UE::SharImportEditor::Private::BuildSimpleUnlitWorldMaster;
using UE::SharImportEditor::Private::ResolveSimpleUnlitWorldMasterRecipe;

bool HasNamedTextureParameter(const UMaterial& Material, const FName Name)
{
    for (UMaterialExpression* Expression : Material.GetExpressions())
    {
        const auto* Parameter =
            Cast<UMaterialExpressionTextureSampleParameter2D>(Expression);
        if (Parameter != nullptr && Parameter->ParameterName == Name)
        {
            return true;
        }
    }
    return false;
}

bool HasNamedVectorParameter(const UMaterial& Material, const FName Name)
{
    for (UMaterialExpression* Expression : Material.GetExpressions())
    {
        const auto* Parameter = Cast<UMaterialExpressionVectorParameter>(
            Expression
        );
        if (Parameter != nullptr && Parameter->ParameterName == Name)
        {
            return true;
        }
    }
    return false;
}

bool HasTextureVertexModulation(
    const UMaterial& Material,
    const int32 TextureOutput,
    const int32 VertexOutput
)
{
    for (UMaterialExpression* Expression : Material.GetExpressions())
    {
        const auto* Multiply = Cast<UMaterialExpressionMultiply>(Expression);
        if (Multiply == nullptr)
        {
            continue;
        }
        const auto IsTextureVertexPair = [TextureOutput, VertexOutput](
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
            IsTextureVertexPair(Multiply->A, Multiply->B)
            || IsTextureVertexPair(Multiply->B, Multiply->A)
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
            && Parameter->ParameterName == TEXT("AlphaReference")
            && Parameter->DefaultValue == 0.5F
        )
        {
            return true;
        }
    }
    return false;
}
} // namespace

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldMaterialTest,
    "SHAR.Import.WorldMaterials.SimpleUnlitMasters",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharWorldMaterialTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    struct FCase
    {
        const TCHAR* Blend;
        EBlendMode ExpectedBlend;
        bool bAlphaTest;
    };
    const FCase Cases[] = {
        {TEXT("opaque"), BLEND_Opaque, false},
        {TEXT("opaque"), BLEND_Opaque, true},
        {TEXT("alpha"), BLEND_Translucent, false},
        {TEXT("alpha"), BLEND_Translucent, true},
        {TEXT("additive"), BLEND_Additive, false},
        {TEXT("additive"), BLEND_Additive, true},
    };
    for (const FCase& Case : Cases)
    {
        FString Error;
        FSharSimpleUnlitWorldMasterRecipe Recipe;
        TestTrue(
            TEXT("Reviewed family resolves"),
            ResolveSimpleUnlitWorldMasterRecipe(
                Case.Blend,
                Case.bAlphaTest,
                Recipe,
                Error
            )
        );
        UMaterial* Material = NewObject<UMaterial>(GetTransientPackage());
        TestNotNull(TEXT("Transient material exists"), Material);
        if (Material == nullptr)
        {
            continue;
        }
        TestTrue(
            TEXT("Reviewed native master graph builds"),
            BuildSimpleUnlitWorldMaster(*Material, Recipe, Error)
        );
        TestEqual(
            TEXT("Native blend mode matches source family"),
            Material->BlendMode.GetValue(),
            Case.ExpectedBlend
        );
        TestTrue(
            TEXT("Regular world master reproduces source CullNone"),
            Material->TwoSided != 0
        );
        TestTrue(
            TEXT("Simple-unlit master keeps unlit shading"),
            Material->GetShadingModels().HasShadingModel(MSM_Unlit)
        );
        TestTrue(
            TEXT("Base-color texture parameter exists"),
            HasNamedTextureParameter(*Material, TEXT("BaseColorTexture"))
        );
        TestTrue(
            TEXT("Base-color tint parameter exists"),
            HasNamedVectorParameter(*Material, TEXT("BaseColorTint"))
        );
        TestTrue(
            TEXT("Unlit colour uses source texture and vertex colour"),
            HasTextureVertexModulation(*Material, 0, 0)
        );
        TestTrue(
            TEXT("Unlit alpha uses source texture and vertex alpha"),
            HasTextureVertexModulation(*Material, 4, 4)
        );
        TestEqual(
            TEXT("Alpha-test custom discard matches source family"),
            HasAlphaDiscard(*Material),
            Case.bAlphaTest
        );
        TestEqual(
            TEXT("Alpha-reference parameter matches alpha-test state"),
            HasAlphaReference(*Material),
            Case.bAlphaTest
        );
    }

    FString Error;
    FSharSimpleUnlitWorldMasterRecipe Recipe;
    TestFalse(
        TEXT("Unreviewed blend family fails closed"),
        ResolveSimpleUnlitWorldMasterRecipe(
            TEXT("modulate"),
            false,
            Recipe,
            Error
        )
    );
    return true;
}


IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldMaterialAssetCreationTest,
    "SHAR.Import.WorldMaterials.CreateSimpleUnlitMasterAsset",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharWorldMaterialAssetCreationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FString Folder =
        TEXT("/Game/Generated/SHAR/Materials/Automation");
    const FString AssetName = FString::Printf(
        TEXT("TransientWorldMaster_%u"),
        FPlatformProcess::GetCurrentProcessId()
    );
    const FString ObjectPath = FString::Printf(
        TEXT("%s/%s.%s"),
        *Folder,
        *AssetName,
        *AssetName
    );
    const FString Created =
        USharWorldMaterialToolset::CreateSimpleUnlitWorldMaster(
            Folder,
            AssetName,
            TEXT("alpha"),
            true
        );
    TestEqual(
        TEXT("World master creation returns planned object path"),
        Created,
        ObjectPath
    );
    UMaterial* Material = FindObject<UMaterial>(nullptr, *ObjectPath);
    TestNotNull(TEXT("Created world master exists in memory"), Material);
    if (Material != nullptr)
    {
        TestEqual(
            TEXT("Created master keeps source-alpha blend"),
            Material->BlendMode.GetValue(),
            BLEND_Translucent
        );
        TestTrue(
            TEXT("Created master keeps alpha-test pixel discard"),
            HasAlphaDiscard(*Material)
        );
        UPackage* Package = Material->GetPackage();
        TestTrue(
            TEXT("Created master package is dirty before explicit save"),
            Package->IsDirty()
        );
        const FString PackageFilename =
            FPackageName::LongPackageNameToFilename(
                Package->GetName(),
                FPackageName::GetAssetPackageExtension()
            );
        TestFalse(
            TEXT("World master creation does not save implicitly"),
            IFileManager::Get().FileExists(*PackageFilename)
        );
        Package->SetDirtyFlag(false);
        Material->ClearFlags(RF_Public | RF_Standalone);
        Material->MarkAsGarbage();
        Package->MarkAsGarbage();
    }
    return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharWorldMaterialInstanceCreationTest,
    "SHAR.Import.WorldMaterials.CreateSimpleUnlitMaterialInstance",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharWorldMaterialInstanceCreationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const uint32 ProcessId = FPlatformProcess::GetCurrentProcessId();
    const FString MaterialFolder =
        TEXT("/Game/Generated/SHAR/Materials/Automation");
    const FString MasterName = FString::Printf(
        TEXT("TransientInstanceMaster_%u"),
        ProcessId
    );
    const FString MasterPath = FString::Printf(
        TEXT("%s/%s.%s"),
        *MaterialFolder,
        *MasterName,
        *MasterName
    );
    const FString CreatedMaster =
        USharWorldMaterialToolset::CreateSimpleUnlitWorldMaster(
            MaterialFolder,
            MasterName,
            TEXT("alpha"),
            true
        );
    TestEqual(
        TEXT("Instance test creates its reviewed parent master"),
        CreatedMaster,
        MasterPath
    );
    UMaterial* Master = FindObject<UMaterial>(nullptr, *MasterPath);
    TestNotNull(TEXT("Instance parent master exists in memory"), Master);
    if (Master == nullptr)
    {
        return false;
    }

    const FString TextureName = FString::Printf(
        TEXT("TransientWorldTexture_%u"),
        ProcessId
    );
    const FString TexturePackagePath = FString::Printf(
        TEXT("/Game/Generated/SHAR/Textures/Automation/%s"),
        *TextureName
    );
    const FString TexturePath = FString::Printf(
        TEXT("%s.%s"),
        *TexturePackagePath,
        *TextureName
    );
    UPackage* TexturePackage = CreatePackage(*TexturePackagePath);
    UTexture2D* Texture = NewObject<UTexture2D>(
        TexturePackage,
        *TextureName,
        RF_Public | RF_Standalone
    );
    TestNotNull(TEXT("Generated texture fixture exists in memory"), Texture);
    if (Texture == nullptr)
    {
        return false;
    }

    const FString InstanceName = FString::Printf(
        TEXT("TransientWorldInstance_%u"),
        ProcessId
    );
    const FString InstancePath = FString::Printf(
        TEXT("%s/%s.%s"),
        *MaterialFolder,
        *InstanceName,
        *InstanceName
    );
    const FLinearColor Tint(0.25F, 0.5F, 0.75F, 0.625F);
    constexpr float AlphaReference = 0.375F;
    const FString CreatedInstance =
        USharWorldMaterialToolset::CreateSimpleUnlitWorldMaterialInstance(
            MaterialFolder,
            InstanceName,
            MasterPath,
            TexturePath,
            Tint,
            true,
            AlphaReference
        );
    TestEqual(
        TEXT("World Material Instance returns planned object path"),
        CreatedInstance,
        InstancePath
    );
    UMaterialInstanceConstant* Instance =
        FindObject<UMaterialInstanceConstant>(nullptr, *InstancePath);
    TestNotNull(TEXT("Created world Material Instance exists"), Instance);
    if (Instance != nullptr)
    {
        TestTrue(
            TEXT("Material Instance keeps the verified parent master"),
            Instance->Parent == Master
        );
        TestTrue(
            TEXT("Material Instance keeps the verified base-color tint"),
            UMaterialEditingLibrary::GetMaterialInstanceVectorParameterValue(
                Instance,
                TEXT("BaseColorTint")
            ).Equals(Tint)
        );
        TestTrue(
            TEXT("Material Instance keeps the verified texture"),
            UMaterialEditingLibrary::GetMaterialInstanceTextureParameterValue(
                Instance,
                TEXT("BaseColorTexture")
            ) == Texture
        );
        TestTrue(
            TEXT("Material Instance keeps the verified alpha reference"),
            FMath::IsNearlyEqual(
                UMaterialEditingLibrary::
                    GetMaterialInstanceScalarParameterValue(
                        Instance,
                        TEXT("AlphaReference")
                    ),
                AlphaReference
            )
        );
        UPackage* InstancePackage = Instance->GetPackage();
        TestTrue(
            TEXT("Material Instance is dirty before explicit save"),
            InstancePackage->IsDirty()
        );
        const FString Filename = FPackageName::LongPackageNameToFilename(
            InstancePackage->GetName(),
            FPackageName::GetAssetPackageExtension()
        );
        TestFalse(
            TEXT("Material Instance creation does not save implicitly"),
            IFileManager::Get().FileExists(*Filename)
        );
        InstancePackage->SetDirtyFlag(false);
        Instance->ClearFlags(RF_Public | RF_Standalone);
        Instance->MarkAsGarbage();
        InstancePackage->MarkAsGarbage();
    }

    TexturePackage->SetDirtyFlag(false);
    Texture->ClearFlags(RF_Public | RF_Standalone);
    Texture->MarkAsGarbage();
    TexturePackage->MarkAsGarbage();
    UPackage* MasterPackage = Master->GetPackage();
    MasterPackage->SetDirtyFlag(false);
    Master->ClearFlags(RF_Public | RF_Standalone);
    Master->MarkAsGarbage();
    MasterPackage->MarkAsGarbage();
    return true;
}

#endif // WITH_DEV_AUTOMATION_TESTS
