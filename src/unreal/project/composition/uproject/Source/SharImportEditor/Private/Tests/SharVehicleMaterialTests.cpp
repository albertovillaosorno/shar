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
//   - Native automation tests for reviewed vehicle simple/unlit material
//   - graphs.
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
//   - Vehicle simple/unlit material automation tests.
// - Description:
//   - Verifies exact PDDI candidate fields, culling, graph read-back, and
//   - no-save AssetTools creation without claiming presentation readiness.
// - Usage:
//   - Runs in editor or commandlet automation contexts.
// - Defaults:
//   - Lit, unsupported blend, and non-Greater compare requests fail closed.
//

//! Vehicle simple/unlit material automation tests.

#if WITH_DEV_AUTOMATION_TESTS

#include "Materials/SharVehicleMaterialPolicy.h"
#include "Materials/SharVehicleMaterialToolset.h"

#include "Engine/Texture2D.h"
#include "HAL/FileManager.h"
#include "HAL/PlatformProcess.h"
#include "MaterialEditingLibrary.h"
#include "Materials/Material.h"
#include "Materials/MaterialInstanceConstant.h"
#include "Misc/AutomationTest.h"
#include "Misc/PackageName.h"

namespace
{
using UE::SharImportEditor::Private::BuildSimpleUnlitVehicleMaster;
using UE::SharImportEditor::Private::ReadBackSimpleUnlitVehicleMaster;
using UE::SharImportEditor::Private::ResolveSimpleUnlitVehicleMasterRecipe;

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
} // namespace

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
        TEXT("/Game/Generated/SHAR/Materials/Vehicles/Automation");
    const FString AssetName = FString::Printf(
        TEXT("TransientVehicleMaster_%u"),
        FPlatformProcess::GetCurrentProcessId()
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
        TestEqual(
            TEXT("Created vehicle master keeps source-alpha blend"),
            Material->BlendMode.GetValue(),
            BLEND_Translucent
        );
        TestFalse(
            TEXT("Created vehicle master does not inherit world two-sided"),
            Material->TwoSided != 0
        );
        UPackage* Package = Material->GetPackage();
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
    const uint32 ProcessId = FPlatformProcess::GetCurrentProcessId();
    const FString MaterialFolder =
        TEXT("/Game/Generated/SHAR/Materials/Vehicles/Automation");
    const FString MasterName = FString::Printf(
        TEXT("TransientVehicleInstanceMaster_%u"),
        ProcessId
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
        TEXT("TransientVehicleTexture_%u"),
        ProcessId
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
        TEXT("TransientVehicleInstance_%u"),
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
        USharVehicleMaterialToolset::CreateSimpleUnlitVehicleMaterialInstance(
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
