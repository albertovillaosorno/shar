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
//   - Transient editor-only vehicle drive-test actions in an active PIE world.
// - Must-Not:
//   - Save maps, publish assets, or mutate the editor world.
// - Allows:
//   - Spawning one validated drive-test pawn in GEditor PlayWorld.
// - Split-When:
//   - Split when another transient runtime diagnostic gains its own lifecycle.
// - Merge-When:
//   - Merge when another toolset owns the identical PIE vehicle boundary.
// - Summary:
//   - Vehicle drive-test PIE diagnostics.
// - Description:
//   - Implements bounded transient vehicle spawning for live PIE validation.
// - Usage:
//   - Registered by SharImportEditor and called through MCP while PIE runs.
// - Defaults:
//   - Missing PIE, non-generated classes, or non-drive-test pawns fail closed.
//

//! Vehicle drive-test PIE diagnostic implementation.

#include "Import/SharVehicleDriveTestToolset.h"

#include "Editor.h"
#include "Engine/World.h"
#include "GameFramework/Pawn.h"
#include "Misc/PackageName.h"
#include "UObject/Class.h"
#include "UObject/UObjectGlobals.h"

namespace UE::SharImportEditor::Private
{
namespace
{
constexpr TCHAR GeneratedDriveTestRoot[] =
    TEXT("/Game/Generated/SHAR/DriveTests/");
constexpr TCHAR NativeDriveTestPawnPath[] =
    TEXT("/Script/SharVehicles.SharVehicleDriveTestPawn");

void RaiseDriveTestError(const FString& Message)
{
    FFrame::KismetExecutionMessage(
        *FString::Printf(TEXT("SharVehicleDriveTestToolset: %s"), *Message),
        ELogVerbosity::Warning
    );
}

bool IsGeneratedDriveTestClassPath(const FString& ClassPath)
{
    const FString PackagePath =
        FPackageName::ObjectPathToPackageName(ClassPath);
    return !PackagePath.IsEmpty()
        && PackagePath.StartsWith(GeneratedDriveTestRoot)
        && ClassPath.EndsWith(TEXT("_C"));
}
} // namespace
} // namespace UE::SharImportEditor::Private

FString USharVehicleDriveTestToolset::SpawnDriveTestPawnInPIE(
    const FString& PawnClassPath,
    const FVector Location,
    const FRotator Rotation
)
{
    using namespace UE::SharImportEditor::Private;
    if (GEditor == nullptr || GEditor->PlayWorld == nullptr)
    {
        RaiseDriveTestError(TEXT("PIE is not running"));
        return {};
    }
    if (!IsGeneratedDriveTestClassPath(PawnClassPath))
    {
        RaiseDriveTestError(
            TEXT("pawn class is not under generated drive-test root")
        );
        return {};
    }
    UClass* PawnClass = LoadObject<UClass>(nullptr, *PawnClassPath);
    UClass* DriveTestClass = LoadObject<UClass>(
        nullptr,
        NativeDriveTestPawnPath
    );
    if (PawnClass == nullptr || DriveTestClass == nullptr
        || !PawnClass->IsChildOf(DriveTestClass)
        || !PawnClass->IsChildOf(APawn::StaticClass()))
    {
        RaiseDriveTestError(TEXT("pawn class is not a SHAR drive-test pawn"));
        return {};
    }

    FActorSpawnParameters Parameters;
    Parameters.SpawnCollisionHandlingOverride =
        ESpawnActorCollisionHandlingMethod::AlwaysSpawn;
    APawn* Pawn = GEditor->PlayWorld->SpawnActor<APawn>(
        PawnClass,
        Location,
        Rotation,
        Parameters
    );
    if (Pawn == nullptr)
    {
        RaiseDriveTestError(TEXT("PIE drive-test pawn spawn failed"));
        return {};
    }
    if (!Pawn->GetActorScale3D().Equals(FVector::OneVector))
    {
        Pawn->Destroy();
        RaiseDriveTestError(
            TEXT("drive-test pawn class has non-unit authored scale")
        );
        return {};
    }
    return Pawn->GetPathName();
}
