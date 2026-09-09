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
//   - Native construction of reviewed vehicle simple/unlit master materials.
// - Must-Not:
//   - Parse source catalogs, save packages, overwrite assets, or promote
//   - special vehicle presentation semantics.
// - Allows:
//   - Exact verified plan fields classified as simple/unlit graph candidates.
// - Split-When:
//   - Vehicle instances or another shader family gain independent construction.
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
#include "Kismet/KismetSystemLibrary.h"
#include "MaterialEditingLibrary.h"
#include "Materials/Material.h"
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
