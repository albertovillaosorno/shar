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
//   - Gives automation a bounded way to spawn generated drive-test pawns in
//   - the active PIE world without persisting experimental level state.
// - Usage:
//   - Call only after PIE has started; StopPIE owns cleanup.
// - Defaults:
//   - Missing PIE, non-generated classes, or non-drive-test pawns fail closed.
//

//! Vehicle drive-test PIE diagnostic toolset.

#pragma once

#include "CoreMinimal.h"
#include "ToolsetRegistry/ToolsetDefinition.h"
#include "SharVehicleDriveTestToolset.generated.h"

UCLASS()
class SHARIMPORTEDITOR_API USharVehicleDriveTestToolset
    : public UToolsetDefinition
{
    GENERATED_BODY()

public:
    /**
     * Spawn one generated SHAR drive-test pawn directly into active PIE.
     * The supplied transform is used exactly; no ground snapping or scaling
     * adjustment is performed. StopPIE destroys the transient actor.
     */
    UFUNCTION(meta = (AICallable), Category = "SharVehicleDriveTestToolset")
    static FString SpawnDriveTestPawnInPIE(
        const FString& PawnClassPath,
        FVector Location,
        FRotator Rotation
    );
};
