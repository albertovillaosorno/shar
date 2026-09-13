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
//   - Normalized vehicle skeletal-model decoder automation coverage.
// - Must-Not:
//   - Create packages, read proprietary assets, or exercise FBX import.
// - Allows:
//   - Synthetic JSON fixtures and exact source-to-Unreal conversion checks.
// - Split-When:
//   - Native SkeletalMesh publication gains independent automation coverage.
// - Merge-When:
//   - Another test owns the same normalized vehicle model contract.
// - Summary:
//   - Normalized vehicle skeletal-model decoder tests.
// - Description:
//   - Proves one exact basis/unit conversion before native asset construction.
// - Usage:
//   - Runs in editor or commandlet automation contexts.
// - Defaults:
//   - Malformed or pre-rebased normalized evidence must fail closed.
//

//! Normalized vehicle skeletal-model decoder tests.

#if WITH_DEV_AUTOMATION_TESTS

#include "Import/SharVehicleSkeletalModelBuilder.h"

#include "HAL/FileManager.h"
#include "Misc/AutomationTest.h"
#include "Misc/CommandLine.h"
#include "Misc/Parse.h"

namespace
{
FString ValidModelJson()
{
    return TEXT(R"JSON({
  "schema":"shar.normalized-skeletal-model.v1",
  "model_id":"asymmetric-car",
  "coordinate_system":{
    "handedness":"right-handed",
    "right_axis":"+X",
    "up_axis":"+Y",
    "forward_axis":"+Z",
    "unit":"meter"
  },
  "normalization":{"target_basis_applied":false},
  "bones":[{
    "bone_id":"root",
    "parent_bone_id":null,
    "rest_matrix_row_major":[
      1,0,0,0, 0,1,0,0, 0,0,1,0, 1,2,3,1
    ]
  },{
    "bone_id":"wheel-left-front",
    "parent_bone_id":"root",
    "rest_matrix_row_major":[
      1,0,0,0, 0,1,0,0, 0,0,1,0, -4,5,6,1
    ]
  }],
  "parts":[{
    "mesh_id":"body",
    "primitive_groups":[{
      "material_id":"body_m",
      "positions":[[1,2,3],[4,5,6],[7,8,9]],
      "normals":[[1,0,0],[0,1,0],[0,0,1]],
      "colors":[[1,0,0,1],[0,1,0,1],[0,0,1,1]],
      "uv0":[[0,0],[1,0],[0,1]],
      "triangles":[[0,1,2]],
      "skin_influences":[
        {"vertex_index":0,"bone_id":"root","weight":1},
        {"vertex_index":1,"bone_id":"root","weight":1},
        {"vertex_index":2,"bone_id":"wheel-left-front","weight":1}
      ]
    }]
  }],
  "animations":[]
})JSON");
}
} // namespace

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleNormalizedModelDecodeTest,
    "SHAR.Import.VehicleSkeletalModel.DecodeNormalizedSourceBasis",
    EAutomationTestFlags::EditorContext |
        EAutomationTestFlags::CommandletContext |
        EAutomationTestFlags::EngineFilter)

bool FSharVehicleNormalizedModelDecodeTest::RunTest(const FString &Parameters)
{
    (void)Parameters;
    using namespace UE::SharImportEditor::Private;
    FSharNormalizedVehicleSkeletalModel Model;
    FString Error;
    if (!TestTrue(TEXT("Synthetic normalized model decodes"),
                  ParseNormalizedVehicleSkeletalModel(ValidModelJson(), Model,
                                                      Error)))
    {
        AddError(Error);
        return false;
    }
    TestEqual(TEXT("Model identity"), Model.ModelId, TEXT("asymmetric-car"));
    TestEqual(TEXT("Bone count"), Model.Bones.Num(), 2);
    TestEqual(TEXT("Part count"), Model.Parts.Num(), 1);
    TestEqual(TEXT("Animation count"), Model.AnimationCount, 0);
    TestEqual(TEXT("Root parent"), Model.Bones[0].ParentIndex, INDEX_NONE);
    TestEqual(TEXT("Child parent"), Model.Bones[1].ParentIndex, 0);
    const auto &RootMatrix = Model.Bones[0].LocalRestMatrix;
    TestTrue(TEXT("Root translation X is source forward in cm"),
             FMath::IsNearlyEqual(RootMatrix[12], 300.0));
    TestTrue(TEXT("Root translation Y is source right in cm"),
             FMath::IsNearlyEqual(RootMatrix[13], 100.0));
    TestTrue(TEXT("Root translation Z is source up in cm"),
             FMath::IsNearlyEqual(RootMatrix[14], 200.0));
    const FSharVehiclePrimitiveGroupRecipe &Group = Model.Parts[0].Groups[0];
    TestEqual(TEXT("Material identity"), Group.MaterialId, TEXT("body_m"));
    TestEqual(TEXT("Triangle count"), Group.Triangles.Num(), 1);
    TestEqual(TEXT("Influence count"), Group.SkinInfluences.Num(), 3);
    TestTrue(TEXT("Position converts exactly once"),
             Group.PositionsCm[0].Equals(FVector3f(300.0F, 100.0F, 200.0F)));
    TestTrue(TEXT("Normal uses the same proper basis"),
             Group.Normals[0].Equals(FVector3f(0.0F, 1.0F, 0.0F)));
    return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleNormalizedModelRejectsTargetBasisTest,
    "SHAR.Import.VehicleSkeletalModel.RejectsPreRebasedEvidence",
    EAutomationTestFlags::EditorContext |
        EAutomationTestFlags::CommandletContext |
        EAutomationTestFlags::EngineFilter)

bool FSharVehicleNormalizedModelRejectsTargetBasisTest::RunTest(
    const FString &Parameters)
{
    (void)Parameters;
    using namespace UE::SharImportEditor::Private;
    FString Json = ValidModelJson();
    Json.ReplaceInline(TEXT("\"target_basis_applied\":false"),
                       TEXT("\"target_basis_applied\":true"));
    FSharNormalizedVehicleSkeletalModel Model;
    FString Error;
    TestFalse(TEXT("Pre-rebased normalized model fails closed"),
              ParseNormalizedVehicleSkeletalModel(Json, Model, Error));
    TestTrue(TEXT("Failure identifies target basis"),
             Error.Contains(TEXT("target basis")));
    return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleNormalizedModelRejectsWeightDriftTest,
    "SHAR.Import.VehicleSkeletalModel.RejectsUnnormalizedWeights",
    EAutomationTestFlags::EditorContext |
        EAutomationTestFlags::CommandletContext |
        EAutomationTestFlags::EngineFilter)

bool FSharVehicleNormalizedModelRejectsWeightDriftTest::RunTest(
    const FString &Parameters)
{
    (void)Parameters;
    using namespace UE::SharImportEditor::Private;
    FString Json = ValidModelJson();
    Json.ReplaceInline(
        TEXT("{\"vertex_index\":0,\"bone_id\":\"root\",\"weight\":1}"),
        TEXT("{\"vertex_index\":0,\"bone_id\":\"root\",\"weight\":0.5}"));
    FSharNormalizedVehicleSkeletalModel Model;
    FString Error;
    TestFalse(TEXT("Unnormalized skin weights fail closed"),
              ParseNormalizedVehicleSkeletalModel(Json, Model, Error));
    TestTrue(TEXT("Failure identifies normalized weights"),
             Error.Contains(TEXT("weights are not normalized")));
    return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(FSharVehicleNormalizedModelCorpusProbeTest,
                                 "SHAR.Import.VehicleSkeletalModel.CorpusProbe",
                                 EAutomationTestFlags::EditorContext |
                                     EAutomationTestFlags::CommandletContext |
                                     EAutomationTestFlags::EngineFilter)

bool FSharVehicleNormalizedModelCorpusProbeTest::RunTest(
    const FString &Parameters)
{
    (void)Parameters;
    FString SourceFile;
    if (!FParse::Value(FCommandLine::Get(), TEXT("SharNormalizedVehicleModel="),
                       SourceFile))
    {
        AddInfo(TEXT("No normalized vehicle corpus probe path was supplied"));
        return true;
    }
    using namespace UE::SharImportEditor::Private;
    FSharNormalizedVehicleSkeletalModel Model;
    FString Error;
    if (!TestTrue(
            TEXT("Normalized vehicle corpus model decodes"),
            ParseNormalizedVehicleSkeletalModelFile(SourceFile, Model, Error)))
    {
        AddError(Error);
        return false;
    }
    TestEqual(TEXT("Snake corpus identity"), Model.ModelId, TEXT("snake-v"));
    TestEqual(TEXT("Snake retained bone count"), Model.Bones.Num(), 29);
    TestEqual(TEXT("Snake part count"), Model.Parts.Num(), 39);
    TestEqual(TEXT("Snake animation count"), Model.AnimationCount, 1);
    const FName WheelName(TEXT("w0"));
    const FSharVehicleBoneRecipe *Wheel = Model.Bones.FindByPredicate(
        [&WheelName](const FSharVehicleBoneRecipe &Bone) {
            return Bone.BoneName == WheelName;
        });
    if (!TestNotNull(TEXT("Snake w0 bone survives"), Wheel))
    {
        return false;
    }
    TestTrue(
        TEXT("Snake w0 source forward becomes Unreal X cm"),
        FMath::IsNearlyEqual(Wheel->LocalRestMatrix[12], -146.750593, 1.0e-3));
    TestTrue(
        TEXT("Snake w0 source right becomes Unreal Y cm"),
        FMath::IsNearlyEqual(Wheel->LocalRestMatrix[13], 85.299999, 1.0e-3));
    TestTrue(
        TEXT("Snake w0 source up becomes Unreal Z cm"),
        FMath::IsNearlyEqual(Wheel->LocalRestMatrix[14], -29.503202, 1.0e-3));
    return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleNormalizedModelCorpusDirectoryProbeTest,
    "SHAR.Import.VehicleSkeletalModel.CorpusDirectoryProbe",
    EAutomationTestFlags::EditorContext |
        EAutomationTestFlags::CommandletContext |
        EAutomationTestFlags::EngineFilter)

bool FSharVehicleNormalizedModelCorpusDirectoryProbeTest::RunTest(
    const FString &Parameters)
{
    (void)Parameters;
    FString CorpusRoot;
    if (!FParse::Value(FCommandLine::Get(),
                       TEXT("SharNormalizedVehicleCorpus="), CorpusRoot))
    {
        AddInfo(TEXT("No normalized vehicle corpus directory was supplied"));
        return true;
    }
    TArray<FString> Files;
    IFileManager::Get().FindFilesRecursive(
        Files, *CorpusRoot, TEXT("model.normalized.json"), true, false);
    Files.Sort();
    if (!TestEqual(TEXT("Standalone normalized vehicle count"), Files.Num(),
                   88))
    {
        return false;
    }
    using namespace UE::SharImportEditor::Private;
    TSet<FString> ModelIds;
    for (const FString &SourceFile : Files)
    {
        FSharNormalizedVehicleSkeletalModel Model;
        FString Error;
        if (!ParseNormalizedVehicleSkeletalModelFile(SourceFile, Model, Error))
        {
            AddError(FString::Printf(TEXT("Corpus decode failed for %s: %s"),
                                     *SourceFile, *Error));
            return false;
        }
        if (ModelIds.Contains(Model.ModelId))
        {
            AddError(
                FString::Printf(TEXT("Duplicate normalized model identity: %s"),
                                *Model.ModelId));
            return false;
        }
        ModelIds.Add(Model.ModelId);
    }
    TestEqual(TEXT("Unique normalized vehicle identities"), ModelIds.Num(), 88);
    return true;
}

#endif // WITH_DEV_AUTOMATION_TESTS
