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
//   - Native transient vehicle Physics Asset construction automation.
// - Must-Not:
//   - Save generated assets or create live world physics state.
// - Allows:
//   - Load reviewed preview meshes and construct transient analytic bodies.
// - Split-When:
//   - Persisted Physics Asset publication gains its own automation lifecycle.
// - Merge-When:
//   - Another test owns the same transient vehicle collision contract.
// - Summary:
//   - Vehicle Physics Asset construction automation.
// - Description:
//   - Proves source magnitudes and imported root scale compose exactly once.
// - Usage:
//   - Runs in editor or commandlet automation contexts.
// - Defaults:
//   - Missing or scale-invalid representative assets fail explicitly.
//

//! Vehicle Physics Asset construction automation.


#if WITH_DEV_AUTOMATION_TESTS

#include "Import/SharVehiclePhysicsAssetBuilder.h"

#include "Engine/SkeletalMesh.h"
#include "Misc/AutomationTest.h"
#include "PhysicsEngine/PhysicsAsset.h"
#include "PhysicsEngine/SkeletalBodySetup.h"

namespace
{
using UE::SharImportEditor::Private::ESharVehiclePhysicsShapeKind;
using UE::SharImportEditor::Private::FSharVehiclePhysicsShapeRecipe;

FSharVehiclePhysicsShapeRecipe SedanaBox(
    const FVector& Center,
    const FVector& Extents
)
{
    FSharVehiclePhysicsShapeRecipe Shape;
    Shape.Kind = ESharVehiclePhysicsShapeKind::Box;
    Shape.BoneName = TEXT("sedanA");
    Shape.Center = Center;
    Shape.AxisX = FVector(-4.371139e-8, 0.0, 1.0);
    Shape.AxisY = FVector(-1.0, 4.371139e-8, -4.371139e-8);
    Shape.AxisZ = FVector(-4.371139e-8, -1.0, -1.9106855e-15);
    Shape.BoxExtents = Extents;
    return Shape;
}

FSharVehiclePhysicsShapeRecipe SedanaWheel(
    const TCHAR* BoneName,
    const FVector& Center,
    const float Radius
)
{
    FSharVehiclePhysicsShapeRecipe Shape;
    Shape.Kind = ESharVehiclePhysicsShapeKind::Sphere;
    Shape.BoneName = FName(BoneName);
    Shape.Center = Center;
    Shape.Radius = Radius;
    return Shape;
}

TArray<FSharVehiclePhysicsShapeRecipe> SedanaShapes()
{
    return {
        SedanaBox(
            FVector(0.008986959, -0.063300475, -0.5765616),
            FVector(5.1482176, 2.1956174, 0.8035346)
        ),
        SedanaBox(
            FVector(0.008986959, -0.7825054, -0.72709286),
            FVector(2.6254544, 2.1956174, 0.62056834)
        ),
        SedanaWheel(
            TEXT("w3"),
            FVector(7.6293944e-8, 3.8146972e-8, 0.0),
            0.42785335F
        ),
        SedanaWheel(
            TEXT("w2"),
            FVector(0.0, 3.8146972e-8, 0.0),
            0.42785335F
        ),
        SedanaWheel(
            TEXT("w1"),
            FVector(7.6293944e-8, 0.0, 0.0),
            0.42785338F
        ),
        SedanaWheel(
            TEXT("w0"),
            FVector(0.0, 3.8146972e-8, 0.0),
            0.42785335F
        ),
    };
}
} // namespace

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehiclePhysicsAssetTest,
    "SHAR.Import.VehiclePhysics.TransientSedana",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharVehiclePhysicsAssetTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USkeletalMesh* Mesh = LoadObject<USkeletalMesh>(
        nullptr,
        TEXT(
            "/Game/Generated/SHAR/PortPreviewUnits/Vehicle_Sedana."
            "Vehicle_Sedana"
        )
    );
    if (!TestNotNull(TEXT("Scene-unit sedana mesh loads"), Mesh))
    {
        return false;
    }
    TestEqual(
        TEXT("Unreal retains sixteen imported sedana bones"),
        Mesh->GetRefSkeleton().GetNum(),
        16
    );
    TestEqual(
        TEXT("Semantic sedana rig remains the imported root"),
        Mesh->GetRefSkeleton().GetBoneName(0),
        FName(TEXT("sedanA"))
    );
    UPhysicsAsset* Asset = nullptr;
    FString Error;
    TestTrue(
        TEXT("Scene-unit sedana recipes construct transient Physics Asset"),
        UE::SharImportEditor::Private::BuildTransientVehiclePhysicsAsset(
            *Mesh,
            TEXT("sedanA"),
            19,
            SedanaShapes(),
            Asset,
            Error
        )
    );
    if (!TestNotNull(TEXT("Transient Physics Asset exists"), Asset))
    {
        AddError(Error);
        return false;
    }
    TestEqual(TEXT("Sedana body count"), Asset->SkeletalBodySetups.Num(), 5);
    const TArray<FName> ExpectedBodyOrder = {
        FName(TEXT("sedanA")),
        FName(TEXT("w3")),
        FName(TEXT("w2")),
        FName(TEXT("w1")),
        FName(TEXT("w0")),
    };
    for (int32 Index = 0; Index < ExpectedBodyOrder.Num(); ++Index)
    {
        TestEqual(
            *FString::Printf(TEXT("Sedana body order %d"), Index),
            Asset->SkeletalBodySetups[Index]->BoneName,
            ExpectedBodyOrder[Index]
        );
    }
    const int32 RootBodyIndex = Asset->FindBodyIndex(TEXT("sedanA"));
    TestTrue(TEXT("Sedana root body resolves"), RootBodyIndex != INDEX_NONE);
    if (RootBodyIndex != INDEX_NONE)
    {
        const USkeletalBodySetup* RootBody =
            Asset->SkeletalBodySetups[RootBodyIndex];
        TestEqual(TEXT("Root box count"), RootBody->AggGeom.BoxElems.Num(), 2);
        if (RootBody->AggGeom.BoxElems.Num() == 2)
        {
            TestTrue(
                TEXT("Root box keeps source-unit numeric extent"),
                FMath::IsNearlyEqual(
                    RootBody->AggGeom.BoxElems[0].X,
                    5.1482176F,
                    1.0e-5F
                )
            );
        }
    }
    for (const FName Wheel : {
             FName(TEXT("w0")),
             FName(TEXT("w1")),
             FName(TEXT("w2")),
             FName(TEXT("w3")),
         })
    {
        const int32 BodyIndex = Asset->FindBodyIndex(Wheel);
        TestTrue(
            *FString::Printf(TEXT("Wheel body %s resolves"), *Wheel.ToString()),
            BodyIndex != INDEX_NONE
        );
        if (BodyIndex != INDEX_NONE)
        {
            TestEqual(
                *FString::Printf(
                    TEXT("Wheel body %s sphere count"),
                    *Wheel.ToString()
                ),
                Asset->SkeletalBodySetups[BodyIndex]->AggGeom.SphereElems.Num(),
                1
            );
        }
    }

    TArray<FSharVehiclePhysicsShapeRecipe> InvalidShapes = SedanaShapes();
    InvalidShapes[0].BoxExtents.X = 0.0;
    UPhysicsAsset* InvalidGeometryAsset = nullptr;
    TestFalse(
        TEXT("Non-positive source box extent is rejected"),
        UE::SharImportEditor::Private::BuildTransientVehiclePhysicsAsset(
            *Mesh,
            TEXT("sedanA"),
            19,
            InvalidShapes,
            InvalidGeometryAsset,
            Error
        )
    );
    TestNull(
        TEXT("Invalid geometry publishes no Physics Asset"),
        InvalidGeometryAsset
    );

    InvalidShapes = SedanaShapes();
    InvalidShapes[0].AxisZ = InvalidShapes[0].AxisY;
    InvalidGeometryAsset = nullptr;
    TestFalse(
        TEXT("Improper source box basis is rejected"),
        UE::SharImportEditor::Private::BuildTransientVehiclePhysicsAsset(
            *Mesh,
            TEXT("sedanA"),
            19,
            InvalidShapes,
            InvalidGeometryAsset,
            Error
        )
    );
    TestNull(
        TEXT("Invalid box basis publishes no Physics Asset"),
        InvalidGeometryAsset
    );

    UPhysicsAsset* InvalidCountAsset = nullptr;
    TestFalse(
        TEXT("Source joint count cannot understate imported bones"),
        UE::SharImportEditor::Private::BuildTransientVehiclePhysicsAsset(
            *Mesh,
            TEXT("sedanA"),
            15,
            SedanaShapes(),
            InvalidCountAsset,
            Error
        )
    );
    TestNull(
        TEXT("Invalid source joint count publishes no Physics Asset"),
        InvalidCountAsset
    );

    USkeletalMesh* OldScaleMesh = LoadObject<USkeletalMesh>(
        nullptr,
        TEXT(
            "/Game/Generated/SHAR/PortPreview/Vehicle_Sedana."
            "Vehicle_Sedana"
        )
    );
    if (TestNotNull(TEXT("Old-scale sedana mesh loads"), OldScaleMesh))
    {
        UPhysicsAsset* RejectedAsset = nullptr;
        TestFalse(
            TEXT("Old-scale skeletal import fails Physics Asset construction"),
            UE::SharImportEditor::Private::BuildTransientVehiclePhysicsAsset(
                *OldScaleMesh,
                TEXT("sedanA"),
                19,
                SedanaShapes(),
                RejectedAsset,
                Error
            )
        );
        TestNull(
            TEXT("Rejected request publishes no Physics Asset"),
            RejectedAsset
        );
    }
    return true;
}

#endif // WITH_DEV_AUTOMATION_TESTS
