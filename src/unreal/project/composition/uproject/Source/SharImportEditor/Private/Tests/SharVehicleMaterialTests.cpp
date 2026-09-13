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
//   - Native automation tests for reviewed vehicle simple material graphs.
// - Must-Not:
//   - Save assets, parse source catalogs, or promote presentation-special
//   - slots.
// - Allows:
//   - Transient graph policy and create-only unsaved vehicle master materials.
// - Split-When:
//   - Vehicle Material Instance or additional shader-family tests gain
//   - lifecycle.
// - Merge-When:
//   - Another test owns the identical vehicle simple/unlit graph contract.
// - Summary:
//   - Vehicle simple material automation tests.
// - Description:
//   - Verifies exact PDDI candidate fields, culling, graph read-back, and
//   - no-save AssetTools creation without claiming presentation readiness.
// - Usage:
//   - Runs in editor or commandlet automation contexts.
// - Defaults:
//   - Unsupported lit states, blends, and compare requests fail closed.
//

//! Vehicle simple material automation tests.

// CSpell:ignore MATUSAGE

#if WITH_DEV_AUTOMATION_TESTS

#include "Materials/SharVehicleMaterialPolicy.h"
#include "Materials/SharVehicleMaterialToolset.h"

#include "Engine/SkeletalMesh.h"
#include "Engine/Texture2D.h"
#include "HAL/FileManager.h"
#include "MaterialEditingLibrary.h"
#include "Materials/Material.h"
#include "Materials/MaterialExpressionMultiply.h"
#include "Materials/MaterialInstanceConstant.h"
#include "Misc/AutomationTest.h"
#include "Misc/Guid.h"
#include "Misc/PackageName.h"
#include "ToolsetRegistry/UToolsetRegistry.h"

namespace
{
using UE::SharImportEditor::Private::BuildSimpleLitVehicleMaster;
using UE::SharImportEditor::Private::BuildSimpleUnlitVehicleMaster;
using UE::SharImportEditor::Private::ReadBackSimpleLitVehicleMaster;
using UE::SharImportEditor::Private::ReadBackSimpleUnlitVehicleMaster;
using UE::SharImportEditor::Private::ResolveSimpleLitVehicleMasterRecipe;
using UE::SharImportEditor::Private::ResolveSimpleUnlitVehicleMasterRecipe;
using UE::SharImportEditor::Private::SourceShininessToRoughness;

bool ResolveVehicleRecipe(
    int32 BlendMode,
    bool bAlphaTest,
    bool bTwoSided,
    FSharSimpleUnlitVehicleMasterRecipe& OutRecipe,
    FString& OutError
)
{
    return ResolveSimpleUnlitVehicleMasterRecipe(
        TEXT("simple"),
        false,
        BlendMode,
        bAlphaTest,
        4,
        bTwoSided,
        OutRecipe,
        OutError
    );
}

FString TransientTestRunId()
{
    return FGuid::NewGuid().ToString(EGuidFormats::Digits);
}
} // namespace

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleMaterialToolsetRegistrationTest,
    "SHAR.Import.VehicleMaterials.ToolsetRegistration",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharVehicleMaterialToolsetRegistrationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    TestTrue(
        TEXT("Vehicle material toolset is registered after engine ")
        TEXT("initialization"),
        UToolsetRegistry::IsToolsetClassRegistered(
            USharVehicleMaterialToolset::StaticClass()
        )
    );
    return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleMaterialPolicyTest,
    "SHAR.Import.VehicleMaterials.SimpleUnlitMasterPolicy",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharVehicleMaterialPolicyTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    struct FCase
    {
        int32 BlendMode;
        EBlendMode ExpectedBlend;
        bool bAlphaTest;
        bool bTwoSided;
    };
    const FCase Cases[] = {
        {0, BLEND_Opaque, false, false},
        {0, BLEND_Opaque, true, true},
        {1, BLEND_Translucent, false, true},
        {1, BLEND_Translucent, true, false},
        {2, BLEND_Additive, false, false},
        {2, BLEND_Additive, true, true},
    };
    for (const FCase& Case : Cases)
    {
        FString Error;
        FSharSimpleUnlitVehicleMasterRecipe Recipe;
        TestTrue(
            TEXT("Reviewed vehicle graph candidate resolves"),
            ResolveVehicleRecipe(
                Case.BlendMode,
                Case.bAlphaTest,
                Case.bTwoSided,
                Recipe,
                Error
            )
        );
        UMaterial* Material = NewObject<UMaterial>(GetTransientPackage());
        TestNotNull(TEXT("Transient vehicle material exists"), Material);
        if (Material == nullptr)
        {
            continue;
        }
        TestTrue(
            TEXT("Reviewed vehicle master graph builds"),
            BuildSimpleUnlitVehicleMaster(*Material, Recipe, Error)
        );
        TestTrue(
            TEXT("Vehicle master graph read-back matches request"),
            ReadBackSimpleUnlitVehicleMaster(*Material, Recipe, Error)
        );
        TestEqual(
            TEXT("Vehicle blend mode matches PDDI plan"),
            Material->BlendMode.GetValue(),
            Case.ExpectedBlend
        );
        TestEqual(
            TEXT("Vehicle two-sided state matches PDDI plan"),
            Material->TwoSided != 0,
            Case.bTwoSided
        );
        TestTrue(
            TEXT("Vehicle master declares SkeletalMesh material usage"),
            Material->GetUsageByFlag(MATUSAGE_SkeletalMesh)
        );
        TestTrue(
            TEXT("Vehicle candidate remains unlit"),
            Material->GetShadingModels().HasShadingModel(MSM_Unlit)
        );
    }

    FString Error;
    FSharSimpleUnlitVehicleMasterRecipe Recipe;
    TestFalse(
        TEXT("Sphere-map family is not a simple/unlit candidate"),
        ResolveSimpleUnlitVehicleMasterRecipe(
            TEXT("spheremap"),
            false,
            1,
            false,
            4,
            false,
            Recipe,
            Error
        )
    );
    TestFalse(
        TEXT("Lit simple material remains outside reviewed policy"),
        ResolveSimpleUnlitVehicleMasterRecipe(
            TEXT("simple"),
            true,
            1,
            false,
            4,
            false,
            Recipe,
            Error
        )
    );
    TestFalse(
        TEXT("PDDI subtract blend remains outside reviewed policy"),
        ResolveSimpleUnlitVehicleMasterRecipe(
            TEXT("simple"),
            false,
            3,
            false,
            4,
            false,
            Recipe,
            Error
        )
    );
    TestFalse(
        TEXT("Non-Greater alpha compare remains outside reviewed policy"),
        ResolveSimpleUnlitVehicleMasterRecipe(
            TEXT("simple"),
            false,
            1,
            false,
            5,
            false,
            Recipe,
            Error
        )
    );
    return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleSimpleLitMaterialPolicyTest,
    "SHAR.Import.VehicleMaterials.SimpleLitMasterPolicy",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharVehicleSimpleLitMaterialPolicyTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    struct FCase
    {
        float Shininess;
        bool bTwoSided;
    };
    const FCase Cases[] = {
        {0.0F, false},
        {10.0F, true},
    };
    for (const FCase& Case : Cases)
    {
        FString Error;
        FSharSimpleLitVehicleMasterRecipe Recipe;
        TestTrue(
            TEXT("Reviewed simple-lit source state resolves"),
            ResolveSimpleLitVehicleMasterRecipe(
                TEXT("simple"),
                true,
                0,
                false,
                4,
                Case.bTwoSided,
                FLinearColor::Black,
                FLinearColor::Black,
                FLinearColor::Black,
                Case.Shininess,
                Recipe,
                Error
            )
        );
        UMaterial* Material = NewObject<UMaterial>(GetTransientPackage());
        TestNotNull(TEXT("Transient simple-lit material exists"), Material);
        if (Material == nullptr)
        {
            continue;
        }
        TestTrue(
            TEXT("Reviewed simple-lit graph builds"),
            BuildSimpleLitVehicleMaster(*Material, Recipe, Error)
        );
        TestTrue(
            TEXT("Simple-lit graph read-back matches source state"),
            ReadBackSimpleLitVehicleMaster(*Material, Recipe, Error)
        );
        TestTrue(
            TEXT("Simple-lit graph uses DefaultLit"),
            Material->GetShadingModels().HasShadingModel(MSM_DefaultLit)
        );
        TestTrue(
            TEXT("Simple-lit master declares SkeletalMesh material usage"),
            Material->GetUsageByFlag(MATUSAGE_SkeletalMesh)
        );
        TestTrue(
            TEXT("Source shininess maps to finite normalized roughness"),
            FMath::IsWithinInclusive(
                SourceShininessToRoughness(Case.Shininess),
                0.0F,
                1.0F
            )
        );
    }

    TestTrue(
        TEXT("PDDI shininess 10 maps to the reviewed Unreal roughness"),
        FMath::IsNearlyEqual(
            SourceShininessToRoughness(10.0F),
            0.6389431F,
            0.000001F
        )
    );

    {
        FString MutationError;
        FSharSimpleLitVehicleMasterRecipe MutationRecipe;
        TestTrue(
            TEXT("Mutation fixture resolves simple-lit policy"),
            ResolveSimpleLitVehicleMasterRecipe(
                TEXT("simple"),
                true,
                0,
                false,
                4,
                false,
                FLinearColor::Black,
                FLinearColor::Black,
                FLinearColor::Black,
                10.0F,
                MutationRecipe,
                MutationError
            )
        );
        UMaterial* MutationMaterial =
            NewObject<UMaterial>(GetTransientPackage());
        TestTrue(
            TEXT("Mutation fixture builds simple-lit graph"),
            MutationMaterial != nullptr
                && BuildSimpleLitVehicleMaster(
                    *MutationMaterial,
                    MutationRecipe,
                    MutationError
                )
        );
        if (MutationMaterial != nullptr)
        {
            for (
                UMaterialExpression* Expression :
                    MutationMaterial->GetExpressions()
            )
            {
                auto* Multiply = Cast<UMaterialExpressionMultiply>(Expression);
                if (Multiply != nullptr)
                {
                    Multiply->B.Expression = nullptr;
                    break;
                }
            }
            TestFalse(
                TEXT("Broken texture/tint modulation fails read-back"),
                ReadBackSimpleLitVehicleMaster(
                    *MutationMaterial,
                    MutationRecipe,
                    MutationError
                )
            );
            TestTrue(
                TEXT("Mutation fixture rebuilds before usage drift"),
                BuildSimpleLitVehicleMaster(
                    *MutationMaterial,
                    MutationRecipe,
                    MutationError
                )
            );
            MutationMaterial->SetUsageByFlag(MATUSAGE_SkeletalMesh, false);
            TestFalse(
                TEXT("Missing SkeletalMesh usage fails vehicle read-back"),
                ReadBackSimpleLitVehicleMaster(
                    *MutationMaterial,
                    MutationRecipe,
                    MutationError
                )
            );
        }
    }

    FString Error;
    FSharSimpleLitVehicleMasterRecipe Recipe;
    TestFalse(
        TEXT("Translucent lit policy remains separate"),
        ResolveSimpleLitVehicleMasterRecipe(
            TEXT("simple"),
            true,
            1,
            false,
            4,
            true,
            FLinearColor::Black,
            FLinearColor::Black,
            FLinearColor::Black,
            10.0F,
            Recipe,
            Error
        )
    );
    TestFalse(
        TEXT("Nonblack source ambient remains fail-closed"),
        ResolveSimpleLitVehicleMasterRecipe(
            TEXT("simple"),
            true,
            0,
            false,
            4,
            false,
            FLinearColor::White,
            FLinearColor::Black,
            FLinearColor::Black,
            10.0F,
            Recipe,
            Error
        )
    );
    TestFalse(
        TEXT("Coloured source specular remains fail-closed"),
        ResolveSimpleLitVehicleMasterRecipe(
            TEXT("simple"),
            true,
            0,
            false,
            4,
            false,
            FLinearColor::Black,
            FLinearColor::White,
            FLinearColor::Black,
            10.0F,
            Recipe,
            Error
        )
    );
    TestFalse(
        TEXT("Out-of-range source shininess remains fail-closed"),
        ResolveSimpleLitVehicleMasterRecipe(
            TEXT("simple"),
            true,
            0,
            false,
            4,
            false,
            FLinearColor::Black,
            FLinearColor::Black,
            FLinearColor::Black,
            129.0F,
            Recipe,
            Error
        )
    );
    return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleSimpleLitMaterialAssetCreationTest,
    "SHAR.Import.VehicleMaterials.CreateSimpleLitMasterAsset",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharVehicleSimpleLitMaterialAssetCreationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FString Folder =
        TEXT("/Game/Generated/SHAR/Materials/Vehicles/Masters/Automation");
    const FString RunId = TransientTestRunId();
    const FString AssetName = FString::Printf(
        TEXT("TransientVehicleLitMaster_%s"),
        *RunId
    );
    const FString ObjectPath = FString::Printf(
        TEXT("%s/%s.%s"),
        *Folder,
        *AssetName,
        *AssetName
    );
    const FString Created =
        USharVehicleMaterialToolset::CreateSimpleLitVehicleMaster(
            Folder,
            AssetName,
            TEXT("simple"),
            true,
            0,
            false,
            4,
            true,
            FLinearColor::Black,
            FLinearColor::Black,
            FLinearColor::Black,
            10.0F
        );
    TestEqual(
        TEXT("Simple-lit master creation returns planned object path"),
        Created,
        ObjectPath
    );
    UMaterial* Material = FindObject<UMaterial>(nullptr, *ObjectPath);
    TestNotNull(TEXT("Created simple-lit vehicle master exists"), Material);
    if (Material != nullptr)
    {
        FString Error;
        FSharSimpleLitVehicleMasterRecipe Recipe;
        TestTrue(
            TEXT("Simple-lit asset request resolves for read-back"),
            ResolveSimpleLitVehicleMasterRecipe(
                TEXT("simple"),
                true,
                0,
                false,
                4,
                true,
                FLinearColor::Black,
                FLinearColor::Black,
                FLinearColor::Black,
                10.0F,
                Recipe,
                Error
            )
        );
        TestTrue(
            TEXT("Created simple-lit master passes read-back"),
            ReadBackSimpleLitVehicleMaster(*Material, Recipe, Error)
        );
        TestTrue(
            TEXT("Public simple-lit verifier accepts the exact recipe"),
            USharVehicleMaterialToolset::VerifySimpleLitVehicleMaster(
                ObjectPath,
                TEXT("simple"),
                true,
                0,
                false,
                4,
                true,
                FLinearColor::Black,
                FLinearColor::Black,
                FLinearColor::Black,
                10.0F
            )
        );
        TestFalse(
            TEXT("Public simple-lit verifier rejects culling drift"),
            USharVehicleMaterialToolset::VerifySimpleLitVehicleMaster(
                ObjectPath,
                TEXT("simple"),
                true,
                0,
                false,
                4,
                false,
                FLinearColor::Black,
                FLinearColor::Black,
                FLinearColor::Black,
                10.0F
            )
        );
        UPackage* Package = Material->GetPackage();
        Material->SetUsageByFlag(MATUSAGE_SkeletalMesh, false);
        Package->SetDirtyFlag(false);
        TestFalse(
            TEXT("Simple-lit verifier detects missing SkeletalMesh usage"),
            USharVehicleMaterialToolset::VerifySimpleLitVehicleMaster(
                ObjectPath,
                TEXT("simple"),
                true,
                0,
                false,
                4,
                true,
                FLinearColor::Black,
                FLinearColor::Black,
                FLinearColor::Black,
                10.0F
            )
        );
        TestTrue(
            TEXT("Simple-lit usage upgrade accepts exact clean master"),
            USharVehicleMaterialToolset::
                UpgradeSimpleLitVehicleMasterSkeletalUsage(
                    ObjectPath,
                    TEXT("simple"),
                    true,
                    0,
                    false,
                    4,
                    true,
                    FLinearColor::Black,
                    FLinearColor::Black,
                    FLinearColor::Black,
                    10.0F
                )
        );
        TestTrue(
            TEXT("Simple-lit usage upgrade restores verifier"),
            USharVehicleMaterialToolset::VerifySimpleLitVehicleMaster(
                ObjectPath,
                TEXT("simple"),
                true,
                0,
                false,
                4,
                true,
                FLinearColor::Black,
                FLinearColor::Black,
                FLinearColor::Black,
                10.0F
            )
        );
        TestTrue(
            TEXT("Created simple-lit package is dirty before save"),
            Package->IsDirty()
        );
        const FString PackageFilename =
            FPackageName::LongPackageNameToFilename(
                Package->GetName(),
                FPackageName::GetAssetPackageExtension()
            );
        TestFalse(
            TEXT("Simple-lit master creation does not save implicitly"),
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
    FSharVehicleMaterialAssetCreationTest,
    "SHAR.Import.VehicleMaterials.CreateSimpleUnlitMasterAsset",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharVehicleMaterialAssetCreationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FString Folder =
        TEXT("/Game/Generated/SHAR/Materials/Vehicles/Masters/Automation");
    const FString RunId = TransientTestRunId();
    const FString AssetName = FString::Printf(
        TEXT("TransientVehicleMaster_%s"),
        *RunId
    );
    const FString ObjectPath = FString::Printf(
        TEXT("%s/%s.%s"),
        *Folder,
        *AssetName,
        *AssetName
    );
    const FString Created =
        USharVehicleMaterialToolset::CreateSimpleUnlitVehicleMaster(
            Folder,
            AssetName,
            TEXT("simple"),
            false,
            1,
            true,
            4,
            false
        );
    TestEqual(
        TEXT("Vehicle master creation returns planned object path"),
        Created,
        ObjectPath
    );
    UMaterial* Material = FindObject<UMaterial>(nullptr, *ObjectPath);
    TestNotNull(TEXT("Created vehicle master exists in memory"), Material);
    if (Material != nullptr)
    {
        FString Error;
        FSharSimpleUnlitVehicleMasterRecipe Recipe;
        TestTrue(
            TEXT("Created vehicle master request resolves for read-back"),
            ResolveVehicleRecipe(1, true, false, Recipe, Error)
        );
        TestTrue(
            TEXT("Created vehicle master passes independent read-back"),
            ReadBackSimpleUnlitVehicleMaster(*Material, Recipe, Error)
        );
        TestTrue(
            TEXT("Public simple-unlit verifier accepts the exact recipe"),
            USharVehicleMaterialToolset::VerifySimpleUnlitVehicleMaster(
                ObjectPath,
                TEXT("simple"),
                false,
                1,
                true,
                4,
                false
            )
        );
        TestFalse(
            TEXT("Public simple-unlit verifier rejects culling drift"),
            USharVehicleMaterialToolset::VerifySimpleUnlitVehicleMaster(
                ObjectPath,
                TEXT("simple"),
                false,
                1,
                true,
                4,
                true
            )
        );
        UPackage* Package = Material->GetPackage();
        Material->SetUsageByFlag(MATUSAGE_SkeletalMesh, false);
        Package->SetDirtyFlag(false);
        TestFalse(
            TEXT("Simple-unlit verifier detects missing SkeletalMesh usage"),
            USharVehicleMaterialToolset::VerifySimpleUnlitVehicleMaster(
                ObjectPath,
                TEXT("simple"),
                false,
                1,
                true,
                4,
                false
            )
        );
        TestTrue(
            TEXT("Simple-unlit usage upgrade accepts exact clean master"),
            USharVehicleMaterialToolset::
                UpgradeSimpleUnlitVehicleMasterSkeletalUsage(
                    ObjectPath,
                    TEXT("simple"),
                    false,
                    1,
                    true,
                    4,
                    false
                )
        );
        TestTrue(
            TEXT("Simple-unlit usage upgrade restores verifier"),
            USharVehicleMaterialToolset::VerifySimpleUnlitVehicleMaster(
                ObjectPath,
                TEXT("simple"),
                false,
                1,
                true,
                4,
                false
            )
        );
        TestEqual(
            TEXT("Created vehicle master keeps source-alpha blend"),
            Material->BlendMode.GetValue(),
            BLEND_Translucent
        );
        TestFalse(
            TEXT("Created vehicle master does not inherit world two-sided"),
            Material->TwoSided != 0
        );
        TestTrue(
            TEXT("Created vehicle master package is dirty before save"),
            Package->IsDirty()
        );
        const FString PackageFilename =
            FPackageName::LongPackageNameToFilename(
                Package->GetName(),
                FPackageName::GetAssetPackageExtension()
            );
        TestFalse(
            TEXT("Vehicle master creation does not save implicitly"),
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
    FSharVehicleMaterialInstanceCreationTest,
    "SHAR.Import.VehicleMaterials.CreateSimpleUnlitMaterialInstance",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharVehicleMaterialInstanceCreationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FString RunId = TransientTestRunId();
    const FString MaterialFolder =
        TEXT("/Game/Generated/SHAR/Materials/Vehicles/Automation");
    const FString MasterName = FString::Printf(
        TEXT("TransientVehicleInstanceMaster_%s"),
        *RunId
    );
    const FString MasterPath = FString::Printf(
        TEXT("%s/%s.%s"),
        *MaterialFolder,
        *MasterName,
        *MasterName
    );
    const FString CreatedMaster =
        USharVehicleMaterialToolset::CreateSimpleUnlitVehicleMaster(
            MaterialFolder,
            MasterName,
            TEXT("simple"),
            false,
            1,
            true,
            4,
            false
        );
    TestEqual(
        TEXT("Vehicle instance test creates reviewed parent master"),
        CreatedMaster,
        MasterPath
    );
    UMaterial* Master = FindObject<UMaterial>(nullptr, *MasterPath);
    TestNotNull(TEXT("Vehicle instance parent master exists"), Master);
    if (Master == nullptr)
    {
        return false;
    }

    const FString TextureName = FString::Printf(
        TEXT("TransientVehicleTexture_%s"),
        *RunId
    );
    const FString TexturePackagePath = FString::Printf(
        TEXT("/Game/Generated/SHAR/Textures/Vehicles/Automation/%s"),
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
    TestNotNull(TEXT("Vehicle texture fixture exists in memory"), Texture);
    if (Texture == nullptr)
    {
        return false;
    }

    const FString InstanceName = FString::Printf(
        TEXT("TransientVehicleInstance_%s"),
        *RunId
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
        USharVehicleMaterialToolset::CreateVehicleMaterialInstance(
            MaterialFolder,
            InstanceName,
            MasterPath,
            TexturePath,
            Tint,
            true,
            AlphaReference
        );
    TestEqual(
        TEXT("Vehicle Material Instance returns planned object path"),
        CreatedInstance,
        InstancePath
    );
    UMaterialInstanceConstant* Instance =
        FindObject<UMaterialInstanceConstant>(nullptr, *InstancePath);
    TestNotNull(TEXT("Created vehicle Material Instance exists"), Instance);
    if (Instance != nullptr)
    {
        TestTrue(
            TEXT("Vehicle instance keeps the reviewed parent master"),
            Instance->Parent == Master
        );
        TestTrue(
            TEXT("Vehicle instance keeps base-color tint"),
            UMaterialEditingLibrary::GetMaterialInstanceVectorParameterValue(
                Instance,
                TEXT("BaseColorTint")
            ).Equals(Tint)
        );
        TestTrue(
            TEXT("Vehicle instance keeps reviewed texture"),
            UMaterialEditingLibrary::GetMaterialInstanceTextureParameterValue(
                Instance,
                TEXT("BaseColorTexture")
            ) == Texture
        );
        TestTrue(
            TEXT("Vehicle instance keeps alpha reference"),
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
            TEXT("Vehicle Material Instance is dirty before explicit save"),
            InstancePackage->IsDirty()
        );
        const FString Filename = FPackageName::LongPackageNameToFilename(
            InstancePackage->GetName(),
            FPackageName::GetAssetPackageExtension()
        );
        TestFalse(
            TEXT("Vehicle Material Instance does not save implicitly"),
            IFileManager::Get().FileExists(*Filename)
        );
        InstancePackage->SetDirtyFlag(false);
        Instance->ClearFlags(RF_Public | RF_Standalone);
        Instance->MarkAsGarbage();
        InstancePackage->MarkAsGarbage();
    }

    const FString LitMasterName = FString::Printf(
        TEXT("TransientVehicleLitInstanceMaster_%s"),
        *RunId
    );
    const FString LitMasterPath = FString::Printf(
        TEXT("%s/%s.%s"),
        *MaterialFolder,
        *LitMasterName,
        *LitMasterName
    );
    const FString CreatedLitMaster =
        USharVehicleMaterialToolset::CreateSimpleLitVehicleMaster(
            MaterialFolder,
            LitMasterName,
            TEXT("simple"),
            true,
            0,
            false,
            4,
            false,
            FLinearColor::Black,
            FLinearColor::Black,
            FLinearColor::Black,
            10.0F
        );
    TestEqual(
        TEXT("Vehicle instance test creates reviewed lit parent master"),
        CreatedLitMaster,
        LitMasterPath
    );
    UMaterial* LitMaster = FindObject<UMaterial>(nullptr, *LitMasterPath);
    TestNotNull(TEXT("Vehicle lit instance parent master exists"), LitMaster);
    if (LitMaster != nullptr)
    {
        const FString LitInstanceName = FString::Printf(
            TEXT("TransientVehicleLitInstance_%s"),
            *RunId
        );
        const FString LitInstancePath = FString::Printf(
            TEXT("%s/%s.%s"),
            *MaterialFolder,
            *LitInstanceName,
            *LitInstanceName
        );
        const FLinearColor LitTint(0.75F, 0.5F, 0.25F, 1.0F);
        const FString CreatedLitInstance =
            USharVehicleMaterialToolset::CreateVehicleMaterialInstance(
                MaterialFolder,
                LitInstanceName,
                LitMasterPath,
                TexturePath,
                LitTint,
                false,
                0.0F
            );
        TestEqual(
            TEXT("Lit vehicle Material Instance returns planned path"),
            CreatedLitInstance,
            LitInstancePath
        );
        UMaterialInstanceConstant* LitInstance =
            FindObject<UMaterialInstanceConstant>(nullptr, *LitInstancePath);
        TestNotNull(
            TEXT("Created lit vehicle Material Instance exists"),
            LitInstance
        );
        if (LitInstance != nullptr)
        {
            TestTrue(
                TEXT("Lit vehicle instance keeps reviewed parent master"),
                LitInstance->Parent == LitMaster
            );
            TestTrue(
                TEXT("Lit vehicle instance keeps diffuse-backed tint"),
                UMaterialEditingLibrary::
                    GetMaterialInstanceVectorParameterValue(
                        LitInstance,
                        TEXT("BaseColorTint")
                    )
                    .Equals(LitTint)
            );
            TestTrue(
                TEXT("Lit vehicle instance keeps reviewed texture"),
                UMaterialEditingLibrary::
                    GetMaterialInstanceTextureParameterValue(
                        LitInstance,
                        TEXT("BaseColorTexture")
                    ) == Texture
            );
            UPackage* LitInstancePackage = LitInstance->GetPackage();
            LitInstancePackage->SetDirtyFlag(false);
            LitInstance->ClearFlags(RF_Public | RF_Standalone);
            LitInstance->MarkAsGarbage();
            LitInstancePackage->MarkAsGarbage();
        }
        UPackage* LitMasterPackage = LitMaster->GetPackage();
        LitMasterPackage->SetDirtyFlag(false);
        LitMaster->ClearFlags(RF_Public | RF_Standalone);
        LitMaster->MarkAsGarbage();
        LitMasterPackage->MarkAsGarbage();
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


IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleMaterialSlotCompareExchangeTest,
    "SHAR.Import.VehicleMaterials.MaterialSlotCompareExchange",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharVehicleMaterialSlotCompareExchangeTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FString RunId = TransientTestRunId();
    const FString MeshName = FString::Printf(
        TEXT("TransientVehicleSlotMesh_%s"),
        *RunId
    );
    const FString MeshPackagePath = FString::Printf(
        TEXT("/Game/Generated/SHAR/cars/Automation/%s"),
        *MeshName
    );
    const FString MeshObjectPath = FString::Printf(
        TEXT("%s.%s"),
        *MeshPackagePath,
        *MeshName
    );
    UPackage* MeshPackage = CreatePackage(*MeshPackagePath);
    USkeletalMesh* Mesh = NewObject<USkeletalMesh>(
        MeshPackage,
        *MeshName,
        RF_Public | RF_Standalone
    );
    TestNotNull(TEXT("Vehicle slot fixture mesh exists"), Mesh);
    if (Mesh == nullptr)
    {
        return false;
    }
    TArray<FSkeletalMaterial> Materials;
    Materials.Emplace(
        nullptr,
        FName(TEXT("slot_a")),
        FName(TEXT("slot_a"))
    );
    Materials.Emplace(
        nullptr,
        FName(TEXT("slot_b")),
        FName(TEXT("slot_b"))
    );
    Materials.Emplace(
        nullptr,
        FName(TEXT("slot_c")),
        FName(TEXT("slot_c"))
    );
    Mesh->SetMaterials(Materials);
    MeshPackage->SetDirtyFlag(false);

    const FString InstanceName = FString::Printf(
        TEXT("TransientVehicleSlotInstance_%s"),
        *RunId
    );
    const FString InstancePackagePath = FString::Printf(
        TEXT("/Game/Generated/SHAR/Materials/Vehicles/Instances/")
        TEXT("Automation/%s"),
        *InstanceName
    );
    const FString InstanceObjectPath = FString::Printf(
        TEXT("%s.%s"),
        *InstancePackagePath,
        *InstanceName
    );
    UPackage* InstancePackage = CreatePackage(*InstancePackagePath);
    UMaterialInstanceConstant* Instance = NewObject<UMaterialInstanceConstant>(
        InstancePackage,
        *InstanceName,
        RF_Public | RF_Standalone
    );
    TestNotNull(TEXT("Vehicle slot fixture instance exists"), Instance);
    if (Instance == nullptr)
    {
        return false;
    }

    const TArray<int32> SlotIndices{0, 2};
    const TArray<FString> SlotNames{TEXT("slot_a"), TEXT("slot_c")};
    const TArray<FString> EmptyPaths{TEXT(""), TEXT("")};
    const TArray<FString> InstancePaths{
        InstanceObjectPath,
        InstanceObjectPath,
    };
    TestEqual(
        TEXT("Vehicle selected slots initially have null materials"),
        USharVehicleMaterialToolset::ReadVehicleMaterialSlots(
            MeshObjectPath,
            SlotIndices,
            SlotNames
        ),
        EmptyPaths
    );
    TestEqual(
        TEXT("Vehicle slot compare-exchange applies exact instance paths"),
        USharVehicleMaterialToolset::CompareExchangeVehicleMaterialSlots(
            MeshObjectPath,
            SlotIndices,
            SlotNames,
            EmptyPaths,
            InstancePaths
        ),
        InstancePaths
    );
    TestTrue(
        TEXT("Vehicle slot compare-exchange marks mesh dirty"),
        MeshPackage->IsDirty()
    );
    TestTrue(
        TEXT("Vehicle slot names are preserved"),
        Mesh->GetMaterials()[0].MaterialSlotName == TEXT("slot_a")
            && Mesh->GetMaterials()[2].MaterialSlotName == TEXT("slot_c")
    );
    TestNull(
        TEXT("Vehicle unselected material slot stays unchanged"),
        Mesh->GetMaterials()[1].MaterialInterface.Get()
    );
    TestEqual(
        TEXT("Vehicle slot compare-exchange rolls back to null materials"),
        USharVehicleMaterialToolset::CompareExchangeVehicleMaterialSlots(
            MeshObjectPath,
            SlotIndices,
            SlotNames,
            InstancePaths,
            EmptyPaths
        ),
        EmptyPaths
    );
    TestFalse(
        TEXT("Vehicle slot transaction does not save mesh implicitly"),
        IFileManager::Get().FileExists(
            *FPackageName::LongPackageNameToFilename(
                MeshPackagePath,
                FPackageName::GetAssetPackageExtension()
            )
        )
    );

    MeshPackage->SetDirtyFlag(false);
    Mesh->ClearFlags(RF_Public | RF_Standalone);
    Mesh->MarkAsGarbage();
    MeshPackage->MarkAsGarbage();
    InstancePackage->SetDirtyFlag(false);
    Instance->ClearFlags(RF_Public | RF_Standalone);
    Instance->MarkAsGarbage();
    InstancePackage->MarkAsGarbage();
    return true;
}

#endif // WITH_DEV_AUTOMATION_TESTS
