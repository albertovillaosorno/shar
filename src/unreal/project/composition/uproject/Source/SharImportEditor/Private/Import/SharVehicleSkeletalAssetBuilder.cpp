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
//   - Transient native vehicle Skeleton and SkeletalMesh shell construction.
// - Must-Not:
//   - Save packages, publish content, or repeat source-basis conversion.
// - Allows:
//   - Construct transient UObjects and render data from a validated recipe.
// - Split-When:
//   - Render geometry publication gains its own construction lifecycle.
// - Merge-When:
//   - Another builder owns identical transient skeletal UObject construction.
// - Summary:
//   - Native vehicle skeletal UObject shell builder implementation.
// - Description:
//   - Materializes rig identity, rest transforms, and material slots natively.
// - Usage:
//   - Runs after normalized model decoding and before package publication.
// - Defaults:
//   - Invalid recipes fail before output objects are returned.
//

//! Native vehicle skeletal UObject shell builder implementation.

#include "Import/SharVehicleSkeletalAssetBuilder.h"

#include "Import/SharVehicleSkeletalModelBuilder.h"

#include "Animation/Skeleton.h"
#include "AssetRegistry/AssetRegistryModule.h"
#include "BoneWeights.h"
#include "Engine/SkeletalMesh.h"
#include "MeshDescription.h"
#include "Misc/PackageName.h"
#include "ReferenceSkeleton.h"
#include "Rendering/SkeletalMeshLODModel.h"
#include "Rendering/SkeletalMeshModel.h"
#include "Rendering/SkeletalMeshRenderData.h"
#include "SkeletalMeshAttributes.h"

namespace UE::SharImportEditor::Private
{
namespace
{
bool Fail(FString &OutError, const TCHAR *Message)
{
    OutError = Message;
    return false;
}

constexpr const TCHAR *GeneratedRoot = TEXT("/Game/Generated/SHAR/");

bool BuildCreateOnlyDestination(const FString &FolderPath,
                                const FString &AssetName,
                                FString &OutPackagePath, FString &OutObjectPath,
                                FString &OutError)
{
    if (!FolderPath.StartsWith(GeneratedRoot, ESearchCase::CaseSensitive))
    {
        return Fail(
            OutError,
            TEXT("vehicle skeletal output must be beneath generated root"));
    }
    if (AssetName.IsEmpty() || AssetName.Contains(TEXT("/")) ||
        AssetName.Contains(TEXT(".")))
    {
        return Fail(OutError, TEXT("vehicle skeletal asset name is invalid"));
    }
    OutPackagePath = FolderPath + TEXT("/") + AssetName;
    if (!FPackageName::IsValidLongPackageName(OutPackagePath))
    {
        return Fail(OutError, TEXT("vehicle skeletal package path is invalid"));
    }
    OutObjectPath = FString::Printf(TEXT("%s.%s"), *OutPackagePath, *AssetName);
    if (FindObject<UObject>(nullptr, *OutObjectPath) != nullptr ||
        FindPackage(nullptr, *OutPackagePath) != nullptr ||
        FPackageName::DoesPackageExist(OutPackagePath))
    {
        return Fail(OutError, TEXT("vehicle skeletal output already exists"));
    }
    return true;
}

void DiscardPublishedObject(UObject *Object)
{
    if (Object == nullptr)
    {
        return;
    }
    UPackage *Package = Object->GetPackage();
    Object->ClearFlags(RF_Public | RF_Standalone);
    Object->MarkAsGarbage();
    if (Package != nullptr)
    {
        Package->ClearDirtyFlag();
        Package->MarkAsGarbage();
    }
}

bool HasExpectedPublishedState(const FSharNormalizedVehicleSkeletalModel &Model,
                               const USkeletalMesh &Mesh,
                               const USkeleton &Skeleton)
{
    if (Mesh.GetSkeleton() != &Skeleton ||
        Mesh.GetRefSkeleton().GetNum() != Model.Bones.Num() ||
        Skeleton.GetReferenceSkeleton().GetNum() != Model.Bones.Num())
    {
        return false;
    }
    const FSkeletalMeshRenderData *RenderData = Mesh.GetResourceForRendering();
    if (RenderData == nullptr || RenderData->LODRenderData.Num() != 1)
    {
        return false;
    }
    return RenderData->LODRenderData[0].GetNumVertices() > 0 &&
           !RenderData->LODRenderData[0].RenderSections.IsEmpty();
}

bool IsFiniteMatrix(const std::array<double, 16> &Matrix)
{
    for (const double Value : Matrix)
    {
        if (!FMath::IsFinite(Value))
        {
            return false;
        }
    }
    return true;
}

FTransform TransformFromRowMajorMatrix(const std::array<double, 16> &Matrix)
{
    const FMatrix UnrealMatrix(
        FPlane(Matrix[0], Matrix[1], Matrix[2], Matrix[3]),
        FPlane(Matrix[4], Matrix[5], Matrix[6], Matrix[7]),
        FPlane(Matrix[8], Matrix[9], Matrix[10], Matrix[11]),
        FPlane(Matrix[12], Matrix[13], Matrix[14], Matrix[15]));
    return FTransform(UnrealMatrix);
}

bool ValidateModelRecipe(const FSharNormalizedVehicleSkeletalModel &Model,
                         FString &OutError)
{
    if (Model.ModelId.IsEmpty() || Model.Bones.IsEmpty())
    {
        return Fail(OutError,
                    TEXT("vehicle skeletal recipe identity is empty"));
    }
    TSet<FName> BoneNames;
    for (int32 Index = 0; Index < Model.Bones.Num(); ++Index)
    {
        const FSharVehicleBoneRecipe &Bone = Model.Bones[Index];
        if (Bone.BoneName.IsNone() || BoneNames.Contains(Bone.BoneName) ||
            !IsFiniteMatrix(Bone.LocalRestMatrix))
        {
            return Fail(OutError,
                        TEXT("vehicle skeletal recipe bone is invalid"));
        }
        if ((Index == 0 && Bone.ParentIndex != INDEX_NONE) ||
            (Index > 0 && (Bone.ParentIndex < 0 || Bone.ParentIndex >= Index)))
        {
            return Fail(OutError,
                        TEXT("vehicle skeletal recipe hierarchy is invalid"));
        }
        BoneNames.Add(Bone.BoneName);
    }
    return true;
}

bool PopulateMeshDescription(const FSharNormalizedVehicleSkeletalModel &Model,
                             USkeletalMesh &Mesh, FString &OutError)
{
    FMeshDescription *Description = Mesh.CreateMeshDescription(0);
    if (Description == nullptr)
    {
        return Fail(OutError, TEXT("vehicle mesh description creation failed"));
    }
    FSkeletalMeshAttributes Attributes(*Description);
    Attributes.Register();
    Attributes.ReserveNewBones(Model.Bones.Num());
    auto BoneNames = Attributes.GetBoneNames();
    auto BoneParents = Attributes.GetBoneParentIndices();
    auto BonePoses = Attributes.GetBonePoses();
    for (const FSharVehicleBoneRecipe &Bone : Model.Bones)
    {
        const FBoneID BoneID = Attributes.CreateBone();
        BoneNames[BoneID] = Bone.BoneName;
        BoneParents[BoneID] = Bone.ParentIndex;
        BonePoses[BoneID] = TransformFromRowMajorMatrix(Bone.LocalRestMatrix);
    }

    auto VertexPositions = Attributes.GetVertexPositions();
    auto VertexNormals = Attributes.GetVertexInstanceNormals();
    auto VertexColors = Attributes.GetVertexInstanceColors();
    auto VertexUVs = Attributes.GetVertexInstanceUVs();
    auto MaterialSlotNames = Attributes.GetPolygonGroupMaterialSlotNames();
    auto SkinWeights = Attributes.GetVertexSkinWeights(NAME_None);
    VertexUVs.SetNumChannels(1);

    for (const FSharVehiclePartRecipe &Part : Model.Parts)
    {
        for (const FSharVehiclePrimitiveGroupRecipe &Group : Part.Groups)
        {
            const FPolygonGroupID PolygonGroup =
                Description->CreatePolygonGroup();
            MaterialSlotNames[PolygonGroup] = FName(*Group.MaterialId);
            TArray<FVertexID> Vertices;
            Vertices.Reserve(Group.PositionsCm.Num());
            for (int32 Index = 0; Index < Group.PositionsCm.Num(); ++Index)
            {
                const FVertexID Vertex = Description->CreateVertex();
                VertexPositions[Vertex] = Group.PositionsCm[Index];
                Vertices.Add(Vertex);

                TArray<UE::AnimationCore::FBoneWeight> Weights;
                for (const FSharVehicleSkinInfluenceRecipe &Influence :
                     Group.SkinInfluences)
                {
                    if (Influence.VertexIndex == Index)
                    {
                        Weights.Emplace(Influence.BoneIndex, Influence.Weight);
                    }
                }
                if (Weights.IsEmpty())
                {
                    return Fail(
                        OutError,
                        TEXT("vehicle mesh vertex has no skin weights"));
                }
                SkinWeights.SetRaw(Vertex, Weights);
            }
            for (const FIntVector &Triangle : Group.Triangles)
            {
                TArray<FVertexInstanceID> Instances;
                Instances.Reserve(3);
                for (int32 Corner = 0; Corner < 3; ++Corner)
                {
                    const int32 LocalVertex = Triangle[Corner];
                    const FVertexInstanceID Instance =
                        Description->CreateVertexInstance(
                            Vertices[LocalVertex]);
                    if (!Group.Normals.IsEmpty())
                    {
                        VertexNormals[Instance] = Group.Normals[LocalVertex];
                    }
                    if (!Group.Colors.IsEmpty())
                    {
                        VertexColors[Instance] = Group.Colors[LocalVertex];
                    }
                    if (!Group.UV0.IsEmpty())
                    {
                        VertexUVs.Set(Instance, 0, Group.UV0[LocalVertex]);
                    }
                    Instances.Add(Instance);
                }
                Description->CreateTriangle(PolygonGroup, Instances, nullptr);
            }
        }
    }
    if (!Mesh.CommitMeshDescription(
            0, USkeletalMesh::FCommitMeshDescriptionParams()))
    {
        return Fail(OutError, TEXT("vehicle mesh description commit failed"));
    }
    return true;
}

TArray<FName> MaterialSlots(const FSharNormalizedVehicleSkeletalModel &Model)
{
    TArray<FName> Slots;
    TSet<FName> Seen;
    for (const FSharVehiclePartRecipe &Part : Model.Parts)
    {
        for (const FSharVehiclePrimitiveGroupRecipe &Group : Part.Groups)
        {
            const FName Slot(*Group.MaterialId);
            if (!Slot.IsNone() && !Seen.Contains(Slot))
            {
                Seen.Add(Slot);
                Slots.Add(Slot);
            }
        }
    }
    return Slots;
}
} // namespace

bool BuildTransientVehicleSkeletalAssetShell(
    const FSharNormalizedVehicleSkeletalModel &Model, USkeletalMesh *&OutMesh,
    USkeleton *&OutSkeleton, FString &OutError)
{
    OutMesh = nullptr;
    OutSkeleton = nullptr;
    OutError.Reset();
    if (!ValidateModelRecipe(Model, OutError))
    {
        return false;
    }

    USkeleton *Skeleton =
        NewObject<USkeleton>(GetTransientPackage(), NAME_None, RF_Transient);
    if (Skeleton == nullptr)
    {
        return Fail(OutError,
                    TEXT("vehicle transient Skeleton creation failed"));
    }

    FReferenceSkeleton ReferenceSkeleton;
    {
        FReferenceSkeletonModifier Modifier(ReferenceSkeleton, Skeleton);
        for (const FSharVehicleBoneRecipe &Bone : Model.Bones)
        {
            const FTransform LocalTransform =
                TransformFromRowMajorMatrix(Bone.LocalRestMatrix);
            if (!LocalTransform.IsValid())
            {
                return Fail(OutError,
                            TEXT("vehicle skeletal rest transform is invalid"));
            }
            Modifier.Add(FMeshBoneInfo(Bone.BoneName, Bone.BoneName.ToString(),
                                       Bone.ParentIndex),
                         LocalTransform);
        }
    }

    USkeletalMesh *Mesh = NewObject<USkeletalMesh>(GetTransientPackage(),
                                                   NAME_None, RF_Transient);
    if (Mesh == nullptr)
    {
        return Fail(OutError,
                    TEXT("vehicle transient SkeletalMesh creation failed"));
    }
    Mesh->SetRefSkeleton(ReferenceSkeleton);
    Mesh->SetSkeleton(Skeleton);
    Skeleton->MergeAllBonesToBoneTree(Mesh);
    Mesh->SetNumSourceModels(1);
    FSkeletalMeshModel *ImportedModel = Mesh->GetImportedModel();
    if (ImportedModel == nullptr)
    {
        return Fail(OutError, TEXT("vehicle imported model creation failed"));
    }
    ImportedModel->LODModels.Add(new FSkeletalMeshLODModel());
    if (!PopulateMeshDescription(Model, *Mesh, OutError))
    {
        return false;
    }

    TArray<FSkeletalMaterial> Materials;
    for (const FName Slot : MaterialSlots(Model))
    {
        Materials.Emplace(nullptr, Slot);
    }
    Mesh->SetMaterials(Materials);
    Mesh->InvalidateDeriveDataCacheGUID();
    Mesh->Build();

    OutMesh = Mesh;
    OutSkeleton = Skeleton;
    return true;
}

bool PublishVehicleSkeletalAssetsCreateOnly(
    const FSharNormalizedVehicleSkeletalModel &Model, const FString &FolderPath,
    const FString &MeshAssetName, const FString &SkeletonAssetName,
    FSharPublishedVehicleSkeletalAssets &OutAssets, FString &OutError)
{
    OutAssets = {};
    OutError.Reset();
    FString MeshPackagePath;
    FString MeshObjectPath;
    FString SkeletonPackagePath;
    FString SkeletonObjectPath;
    if (!BuildCreateOnlyDestination(FolderPath, MeshAssetName, MeshPackagePath,
                                    MeshObjectPath, OutError) ||
        !BuildCreateOnlyDestination(FolderPath, SkeletonAssetName,
                                    SkeletonPackagePath, SkeletonObjectPath,
                                    OutError) ||
        MeshPackagePath == SkeletonPackagePath)
    {
        if (OutError.IsEmpty())
        {
            OutError = TEXT("vehicle skeletal destinations collide");
        }
        return false;
    }

    USkeletalMesh *CandidateMesh = nullptr;
    USkeleton *CandidateSkeleton = nullptr;
    if (!BuildTransientVehicleSkeletalAssetShell(Model, CandidateMesh,
                                                 CandidateSkeleton, OutError))
    {
        return false;
    }

    UPackage *SkeletonPackage = CreatePackage(*SkeletonPackagePath);
    USkeleton *PublishedSkeleton = DuplicateObject<USkeleton>(
        CandidateSkeleton, SkeletonPackage, FName(*SkeletonAssetName));
    if (PublishedSkeleton == nullptr)
    {
        return Fail(OutError, TEXT("vehicle Skeleton publication failed"));
    }
    PublishedSkeleton->ClearFlags(RF_Transient);
    PublishedSkeleton->SetFlags(RF_Public | RF_Standalone | RF_Transactional);

    UPackage *MeshPackage = CreatePackage(*MeshPackagePath);
    USkeletalMesh *PublishedMesh = DuplicateObject<USkeletalMesh>(
        CandidateMesh, MeshPackage, FName(*MeshAssetName));
    if (PublishedMesh == nullptr)
    {
        DiscardPublishedObject(PublishedSkeleton);
        return Fail(OutError, TEXT("vehicle SkeletalMesh publication failed"));
    }
    PublishedMesh->ClearFlags(RF_Transient);
    PublishedMesh->SetFlags(RF_Public | RF_Standalone | RF_Transactional);
    PublishedMesh->SetSkeleton(PublishedSkeleton);
    PublishedSkeleton->MergeAllBonesToBoneTree(PublishedMesh);
    if (!HasExpectedPublishedState(Model, *PublishedMesh, *PublishedSkeleton))
    {
        DiscardPublishedObject(PublishedMesh);
        DiscardPublishedObject(PublishedSkeleton);
        return Fail(OutError, TEXT("vehicle skeletal publication drifted"));
    }

    FAssetRegistryModule::AssetCreated(PublishedSkeleton);
    FAssetRegistryModule::AssetCreated(PublishedMesh);
    SkeletonPackage->MarkPackageDirty();
    MeshPackage->MarkPackageDirty();
    OutAssets.Mesh = PublishedMesh;
    OutAssets.Skeleton = PublishedSkeleton;
    OutAssets.MeshObjectPath = MoveTemp(MeshObjectPath);
    OutAssets.SkeletonObjectPath = MoveTemp(SkeletonObjectPath);
    return true;
}
} // namespace UE::SharImportEditor::Private
