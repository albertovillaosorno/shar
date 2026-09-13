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
//   - Validated native vehicle skeletal-model decoding and basis conversion.
// - Must-Not:
//   - Create assets, save packages, or infer missing normalized evidence.
// - Allows:
//   - Decode the versioned normalized model and convert source basis once.
// - Split-When:
//   - Asset publication or animation construction gains its own lifecycle.
// - Merge-When:
//   - Another builder owns identical normalized skeletal-model decoding.
// - Summary:
//   - Vehicle normalized skeletal-model construction input implementation.
// - Description:
//   - Validates source-basis JSON and emits Unreal-basis centimeter recipes.
// - Usage:
//   - Consumed by native vehicle SkeletalMesh and Skeleton construction.
// - Defaults:
//   - Invalid schema, basis, topology, rig, or skin evidence fails closed.
//

//! Vehicle normalized skeletal-model construction input implementation.

#include "Import/SharVehicleSkeletalModelBuilder.h"

#include "Dom/JsonObject.h"
#include "Dom/JsonValue.h"
#include "Misc/FileHelper.h"
#include "Serialization/JsonReader.h"
#include "Serialization/JsonSerializer.h"

namespace UE::SharImportEditor::Private
{
namespace
{
constexpr TCHAR NormalizedModelSchema[] =
    TEXT("shar.normalized-skeletal-model.v1");
constexpr double CentimetersPerMeter = 100.0;
constexpr double WeightSumTolerance = 1.0e-3;

using FMatrixArray = std::array<double, 16>;

bool Fail(FString &OutError, const TCHAR *Message)
{
    OutError = Message;
    return false;
}

bool IsFinite(const FVector3f &Value)
{
    return FMath::IsFinite(Value.X) && FMath::IsFinite(Value.Y) &&
           FMath::IsFinite(Value.Z);
}

FMatrixArray Multiply(const FMatrixArray &Left, const FMatrixArray &Right)
{
    FMatrixArray Result{};
    for (int32 Row = 0; Row < 4; ++Row)
    {
        for (int32 Column = 0; Column < 4; ++Column)
        {
            double Value = 0.0;
            for (int32 Inner = 0; Inner < 4; ++Inner)
            {
                Value += Left[Row * 4 + Inner] * Right[Inner * 4 + Column];
            }
            Result[Row * 4 + Column] = Value;
        }
    }
    return Result;
}

const FMatrixArray &SourceToUnrealBasis()
{
    static const FMatrixArray Basis = {
        0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    };
    return Basis;
}

const FMatrixArray &UnrealToSourceBasis()
{
    static const FMatrixArray Basis = {
        0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    };
    return Basis;
}

bool ReadNumberArray(const TSharedPtr<FJsonValue> &Value, const int32 Count,
                     TArray<double> &OutNumbers)
{
    if (!Value.IsValid() || Value->Type != EJson::Array)
    {
        return false;
    }
    const TArray<TSharedPtr<FJsonValue>> &Values = Value->AsArray();
    if (Values.Num() != Count)
    {
        return false;
    }
    OutNumbers.Reset(Count);
    for (const TSharedPtr<FJsonValue> &Item : Values)
    {
        double Number = 0.0;
        if (!Item.IsValid() || !Item->TryGetNumber(Number) ||
            !FMath::IsFinite(Number))
        {
            return false;
        }
        OutNumbers.Add(Number);
    }
    return true;
}

bool ReadVector3(const TSharedPtr<FJsonValue> &Value, FVector3f &OutVector)
{
    TArray<double> Numbers;
    if (!ReadNumberArray(Value, 3, Numbers))
    {
        return false;
    }
    OutVector = FVector3f(static_cast<float>(Numbers[0]),
                          static_cast<float>(Numbers[1]),
                          static_cast<float>(Numbers[2]));
    return IsFinite(OutVector);
}

bool ReadVector2(const TSharedPtr<FJsonValue> &Value, FVector2f &OutVector)
{
    TArray<double> Numbers;
    if (!ReadNumberArray(Value, 2, Numbers))
    {
        return false;
    }
    OutVector = FVector2f(static_cast<float>(Numbers[0]),
                          static_cast<float>(Numbers[1]));
    return FMath::IsFinite(OutVector.X) && FMath::IsFinite(OutVector.Y);
}

bool ReadVector4(const TSharedPtr<FJsonValue> &Value, FVector4f &OutVector)
{
    TArray<double> Numbers;
    if (!ReadNumberArray(Value, 4, Numbers))
    {
        return false;
    }
    OutVector = FVector4f(
        static_cast<float>(Numbers[0]), static_cast<float>(Numbers[1]),
        static_cast<float>(Numbers[2]), static_cast<float>(Numbers[3]));
    return FMath::IsFinite(OutVector.X) && FMath::IsFinite(OutVector.Y) &&
           FMath::IsFinite(OutVector.Z) && FMath::IsFinite(OutVector.W);
}

bool ReadMatrix(const TSharedPtr<FJsonValue> &Value, FMatrixArray &OutMatrix)
{
    TArray<double> Numbers;
    if (!ReadNumberArray(Value, 16, Numbers))
    {
        return false;
    }
    for (int32 Index = 0; Index < 16; ++Index)
    {
        OutMatrix[Index] = Numbers[Index];
    }
    return true;
}

bool ReadStringField(const TSharedPtr<FJsonObject> &Object, const TCHAR *Field,
                     FString &OutValue)
{
    return Object.IsValid() && Object->TryGetStringField(Field, OutValue) &&
           !OutValue.IsEmpty();
}

bool ValidateRootContract(const TSharedPtr<FJsonObject> &Root,
                          FString &OutError)
{
    FString Schema;
    FString ModelId;
    if (!ReadStringField(Root, TEXT("schema"), Schema) ||
        Schema != NormalizedModelSchema ||
        !ReadStringField(Root, TEXT("model_id"), ModelId))
    {
        return Fail(OutError,
                    TEXT("normalized vehicle model identity is invalid"));
    }
    const TSharedPtr<FJsonObject> *Coordinates = nullptr;
    if (!Root->TryGetObjectField(TEXT("coordinate_system"), Coordinates) ||
        Coordinates == nullptr || !Coordinates->IsValid())
    {
        return Fail(
            OutError,
            TEXT("normalized vehicle model has no coordinate contract"));
    }
    FString Handedness;
    FString Right;
    FString Up;
    FString Forward;
    FString Unit;
    if (!ReadStringField(*Coordinates, TEXT("handedness"), Handedness) ||
        !ReadStringField(*Coordinates, TEXT("right_axis"), Right) ||
        !ReadStringField(*Coordinates, TEXT("up_axis"), Up) ||
        !ReadStringField(*Coordinates, TEXT("forward_axis"), Forward) ||
        !ReadStringField(*Coordinates, TEXT("unit"), Unit) ||
        Handedness != TEXT("right-handed") || Right != TEXT("+X") ||
        Up != TEXT("+Y") || Forward != TEXT("+Z") || Unit != TEXT("meter"))
    {
        return Fail(OutError,
                    TEXT("normalized vehicle coordinate contract is invalid"));
    }
    const TSharedPtr<FJsonObject> *Normalization = nullptr;
    bool TargetBasisApplied = true;
    if (!Root->TryGetObjectField(TEXT("normalization"), Normalization) ||
        Normalization == nullptr || !Normalization->IsValid() ||
        !(*Normalization)
             ->TryGetBoolField(TEXT("target_basis_applied"),
                               TargetBasisApplied) ||
        TargetBasisApplied)
    {
        return Fail(OutError,
                    TEXT("normalized vehicle target basis is invalid"));
    }
    return true;
}

bool ParseBones(const TSharedPtr<FJsonObject> &Root,
                FSharNormalizedVehicleSkeletalModel &OutModel,
                TMap<FString, int32> &OutBoneIndices, FString &OutError)
{
    const TArray<TSharedPtr<FJsonValue>> *Bones = nullptr;
    if (!Root->TryGetArrayField(TEXT("bones"), Bones) || Bones == nullptr ||
        Bones->IsEmpty())
    {
        return Fail(OutError, TEXT("normalized vehicle model has no bones"));
    }
    OutModel.Bones.Reserve(Bones->Num());
    for (int32 Index = 0; Index < Bones->Num(); ++Index)
    {
        const TSharedPtr<FJsonObject> Bone = (*Bones)[Index]->AsObject();
        FString BoneId;
        if (!ReadStringField(Bone, TEXT("bone_id"), BoneId) ||
            OutBoneIndices.Contains(BoneId))
        {
            return Fail(OutError,
                        TEXT("normalized vehicle bone identity is invalid"));
        }
        int32 ParentIndex = INDEX_NONE;
        const TSharedPtr<FJsonValue> ParentValue =
            Bone->TryGetField(TEXT("parent_bone_id"));
        if (ParentValue.IsValid() && ParentValue->Type != EJson::Null)
        {
            FString ParentId;
            if (!ParentValue->TryGetString(ParentId))
            {
                return Fail(OutError,
                            TEXT("normalized vehicle bone parent is invalid"));
            }
            const int32 *Parent = OutBoneIndices.Find(ParentId);
            if (Parent == nullptr)
            {
                return Fail(
                    OutError,
                    TEXT("normalized vehicle bone parent is not ordered"));
            }
            ParentIndex = *Parent;
        }
        else if (Index != 0)
        {
            return Fail(OutError,
                        TEXT("normalized vehicle has multiple root bones"));
        }
        FMatrixArray SourceMatrix{};
        if (!ReadMatrix(Bone->TryGetField(TEXT("rest_matrix_row_major")),
                        SourceMatrix))
        {
            return Fail(OutError,
                        TEXT("normalized vehicle rest matrix is invalid"));
        }
        FSharVehicleBoneRecipe Recipe;
        Recipe.BoneName = FName(BoneId);
        Recipe.ParentIndex = ParentIndex;
        Recipe.LocalRestMatrix =
            ConvertSharVehicleRestMatrixToUnreal(SourceMatrix);
        OutBoneIndices.Add(BoneId, Index);
        OutModel.Bones.Add(MoveTemp(Recipe));
    }
    return true;
}

bool ParseGroup(const TSharedPtr<FJsonObject> &Group,
                const TMap<FString, int32> &BoneIndices,
                FSharVehiclePrimitiveGroupRecipe &OutGroup, FString &OutError)
{
    if (!ReadStringField(Group, TEXT("material_id"), OutGroup.MaterialId))
    {
        return Fail(OutError,
                    TEXT("normalized vehicle material identity is invalid"));
    }
    const TArray<TSharedPtr<FJsonValue>> *Positions = nullptr;
    const TArray<TSharedPtr<FJsonValue>> *Normals = nullptr;
    const TArray<TSharedPtr<FJsonValue>> *UVs = nullptr;
    const TArray<TSharedPtr<FJsonValue>> *Colors = nullptr;
    const TArray<TSharedPtr<FJsonValue>> *Triangles = nullptr;
    const TArray<TSharedPtr<FJsonValue>> *Influences = nullptr;
    if (!Group->TryGetArrayField(TEXT("positions"), Positions) ||
        Positions == nullptr || Positions->IsEmpty() ||
        !Group->TryGetArrayField(TEXT("normals"), Normals) ||
        Normals == nullptr || !Group->TryGetArrayField(TEXT("uv0"), UVs) ||
        UVs == nullptr || !Group->TryGetArrayField(TEXT("colors"), Colors) ||
        Colors == nullptr ||
        !Group->TryGetArrayField(TEXT("triangles"), Triangles) ||
        Triangles == nullptr || Triangles->IsEmpty() ||
        !Group->TryGetArrayField(TEXT("skin_influences"), Influences) ||
        Influences == nullptr)
    {
        return Fail(OutError,
                    TEXT("normalized vehicle primitive arrays are invalid"));
    }
    const int32 VertexCount = Positions->Num();
    if ((!Normals->IsEmpty() && Normals->Num() != VertexCount) ||
        (!UVs->IsEmpty() && UVs->Num() != VertexCount) ||
        (!Colors->IsEmpty() && Colors->Num() != VertexCount))
    {
        return Fail(OutError,
                    TEXT("normalized vehicle vertex channels disagree"));
    }
    OutGroup.PositionsCm.Reserve(VertexCount);
    for (const TSharedPtr<FJsonValue> &Value : *Positions)
    {
        FVector3f Source;
        if (!ReadVector3(Value, Source))
        {
            return Fail(OutError,
                        TEXT("normalized vehicle position is invalid"));
        }
        OutGroup.PositionsCm.Add(ConvertSharVehiclePositionToUnrealCm(Source));
    }
    OutGroup.Normals.Reserve(Normals->Num());
    for (const TSharedPtr<FJsonValue> &Value : *Normals)
    {
        FVector3f Source;
        if (!ReadVector3(Value, Source))
        {
            return Fail(OutError, TEXT("normalized vehicle normal is invalid"));
        }
        OutGroup.Normals.Add(ConvertSharVehicleDirectionToUnreal(Source));
    }
    OutGroup.UV0.Reserve(UVs->Num());
    for (const TSharedPtr<FJsonValue> &Value : *UVs)
    {
        FVector2f UV;
        if (!ReadVector2(Value, UV))
        {
            return Fail(OutError, TEXT("normalized vehicle UV is invalid"));
        }
        OutGroup.UV0.Add(UV);
    }
    OutGroup.Colors.Reserve(Colors->Num());
    for (const TSharedPtr<FJsonValue> &Value : *Colors)
    {
        FVector4f Color;
        if (!ReadVector4(Value, Color))
        {
            return Fail(OutError, TEXT("normalized vehicle color is invalid"));
        }
        OutGroup.Colors.Add(Color);
    }
    OutGroup.Triangles.Reserve(Triangles->Num());
    for (const TSharedPtr<FJsonValue> &Value : *Triangles)
    {
        TArray<double> Indices;
        if (!ReadNumberArray(Value, 3, Indices))
        {
            return Fail(OutError,
                        TEXT("normalized vehicle triangle is invalid"));
        }
        FIntVector Triangle;
        for (int32 Corner = 0; Corner < 3; ++Corner)
        {
            const double Raw = Indices[Corner];
            if (Raw < 0.0 || Raw >= static_cast<double>(VertexCount) ||
                Raw != FMath::FloorToDouble(Raw))
            {
                return Fail(
                    OutError,
                    TEXT("normalized vehicle triangle index is invalid"));
            }
            Triangle[Corner] = static_cast<int32>(Raw);
        }
        OutGroup.Triangles.Add(Triangle);
    }
    OutGroup.SkinInfluences.Reserve(Influences->Num());
    TArray<double> WeightSums;
    WeightSums.Init(0.0, VertexCount);
    for (const TSharedPtr<FJsonValue> &Value : *Influences)
    {
        const TSharedPtr<FJsonObject> Influence = Value->AsObject();
        double VertexRaw = -1.0;
        double WeightRaw = 0.0;
        FString BoneId;
        if (!Influence.IsValid() ||
            !Influence->TryGetNumberField(TEXT("vertex_index"), VertexRaw) ||
            VertexRaw < 0.0 || VertexRaw >= static_cast<double>(VertexCount) ||
            VertexRaw != FMath::FloorToDouble(VertexRaw) ||
            !ReadStringField(Influence, TEXT("bone_id"), BoneId) ||
            !Influence->TryGetNumberField(TEXT("weight"), WeightRaw) ||
            !FMath::IsFinite(WeightRaw) || WeightRaw <= 0.0 ||
            WeightRaw > 1.0 + WeightSumTolerance)
        {
            return Fail(OutError,
                        TEXT("normalized vehicle skin influence is invalid"));
        }
        const int32 *BoneIndex = BoneIndices.Find(BoneId);
        if (BoneIndex == nullptr)
        {
            return Fail(OutError,
                        TEXT("normalized vehicle skin bone is unknown"));
        }
        FSharVehicleSkinInfluenceRecipe Recipe;
        Recipe.VertexIndex = static_cast<int32>(VertexRaw);
        Recipe.BoneIndex = *BoneIndex;
        Recipe.Weight = static_cast<float>(WeightRaw);
        OutGroup.SkinInfluences.Add(Recipe);
        WeightSums[Recipe.VertexIndex] += WeightRaw;
    }
    for (const double Sum : WeightSums)
    {
        if (FMath::Abs(Sum - 1.0) > WeightSumTolerance)
        {
            return Fail(
                OutError,
                TEXT("normalized vehicle vertex weights are not normalized"));
        }
    }
    return true;
}

bool ParseParts(const TSharedPtr<FJsonObject> &Root,
                const TMap<FString, int32> &BoneIndices,
                FSharNormalizedVehicleSkeletalModel &OutModel,
                FString &OutError)
{
    const TArray<TSharedPtr<FJsonValue>> *Parts = nullptr;
    if (!Root->TryGetArrayField(TEXT("parts"), Parts) || Parts == nullptr ||
        Parts->IsEmpty())
    {
        return Fail(OutError, TEXT("normalized vehicle model has no parts"));
    }
    TSet<FString> MeshIds;
    OutModel.Parts.Reserve(Parts->Num());
    for (const TSharedPtr<FJsonValue> &Value : *Parts)
    {
        const TSharedPtr<FJsonObject> Part = Value->AsObject();
        FSharVehiclePartRecipe Recipe;
        if (!ReadStringField(Part, TEXT("mesh_id"), Recipe.MeshId) ||
            MeshIds.Contains(Recipe.MeshId))
        {
            return Fail(OutError,
                        TEXT("normalized vehicle mesh identity is invalid"));
        }
        const TArray<TSharedPtr<FJsonValue>> *Groups = nullptr;
        if (!Part->TryGetArrayField(TEXT("primitive_groups"), Groups) ||
            Groups == nullptr || Groups->IsEmpty())
        {
            return Fail(OutError,
                        TEXT("normalized vehicle part has no groups"));
        }
        Recipe.Groups.Reserve(Groups->Num());
        for (const TSharedPtr<FJsonValue> &GroupValue : *Groups)
        {
            FSharVehiclePrimitiveGroupRecipe GroupRecipe;
            if (!ParseGroup(GroupValue->AsObject(), BoneIndices, GroupRecipe,
                            OutError))
            {
                return false;
            }
            Recipe.Groups.Add(MoveTemp(GroupRecipe));
        }
        MeshIds.Add(Recipe.MeshId);
        OutModel.Parts.Add(MoveTemp(Recipe));
    }
    return true;
}
} // namespace

FVector3f ConvertSharVehiclePositionToUnrealCm(const FVector3f &Source)
{
    return FVector3f(Source.Z, Source.X, Source.Y) *
           static_cast<float>(CentimetersPerMeter);
}

FVector3f ConvertSharVehicleDirectionToUnreal(const FVector3f &Source)
{
    return FVector3f(Source.Z, Source.X, Source.Y);
}

FMatrixArray ConvertSharVehicleRestMatrixToUnreal(const FMatrixArray &Source)
{
    FMatrixArray Converted = Multiply(Multiply(UnrealToSourceBasis(), Source),
                                      SourceToUnrealBasis());
    Converted[12] *= CentimetersPerMeter;
    Converted[13] *= CentimetersPerMeter;
    Converted[14] *= CentimetersPerMeter;
    return Converted;
}

bool ParseNormalizedVehicleSkeletalModel(
    const FString &JsonText, FSharNormalizedVehicleSkeletalModel &OutModel,
    FString &OutError)
{
    OutModel = {};
    OutError.Reset();
    if (JsonText.IsEmpty())
    {
        return Fail(OutError, TEXT("normalized vehicle model JSON is empty"));
    }
    TSharedPtr<FJsonObject> Root;
    const TSharedRef<TJsonReader<>> Reader =
        TJsonReaderFactory<>::Create(JsonText);
    if (!FJsonSerializer::Deserialize(Reader, Root) || !Root.IsValid())
    {
        return Fail(OutError, TEXT("normalized vehicle model JSON is invalid"));
    }
    if (!ValidateRootContract(Root, OutError) ||
        !ReadStringField(Root, TEXT("model_id"), OutModel.ModelId))
    {
        return false;
    }
    TMap<FString, int32> BoneIndices;
    if (!ParseBones(Root, OutModel, BoneIndices, OutError) ||
        !ParseParts(Root, BoneIndices, OutModel, OutError))
    {
        OutModel = {};
        return false;
    }
    const TArray<TSharedPtr<FJsonValue>> *Animations = nullptr;
    if (!Root->TryGetArrayField(TEXT("animations"), Animations) ||
        Animations == nullptr)
    {
        OutModel = {};
        return Fail(OutError,
                    TEXT("normalized vehicle animations are invalid"));
    }
    OutModel.AnimationCount = Animations->Num();
    return true;
}

bool ParseNormalizedVehicleSkeletalModelFile(
    const FString &SourceFile, FSharNormalizedVehicleSkeletalModel &OutModel,
    FString &OutError)
{
    FString JsonText;
    if (SourceFile.IsEmpty() ||
        !FFileHelper::LoadFileToString(JsonText, *SourceFile))
    {
        OutModel = {};
        return Fail(OutError,
                    TEXT("normalized vehicle model file is unreadable"));
    }
    return ParseNormalizedVehicleSkeletalModel(JsonText, OutModel, OutError);
}
} // namespace UE::SharImportEditor::Private
