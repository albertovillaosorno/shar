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
//   - Create-only publication of review vehicle Animation Blueprints.
// - Must-Not:
//   - Save packages, mutate existing assets, or define vehicle handling.
// - Allows:
//   - Create one generated-root AnimBlueprint from a validated Skeleton.
// - Split-When:
//   - Production vehicle animation publication gains independent policy.
// - Merge-When:
//   - Another editor adapter owns identical vehicle AnimBlueprint creation.
// - Summary:
//   - Creates canonical review-only vehicle Animation Blueprints.
// - Description:
//   - Uses Unreal's AnimBlueprint factory with VehicleAnimationInstance.
// - Usage:
//   - Called before graph tooling wires a review Wheel Controller graph.
// - Defaults:
//   - Existing outputs and non-generated destinations fail closed.
//

//! Create-only vehicle drive-test Animation Blueprint toolset.

#pragma once

#include "ToolsetRegistry/ToolsetDefinition.h"

#include "SharVehicleDriveTestAnimToolset.generated.h"

UCLASS(BlueprintType)
class SHARIMPORTEDITOR_API USharVehicleDriveTestAnimToolset
    : public UToolsetDefinition
{
    GENERATED_BODY()

public:
    /**
     * Create one unsaved vehicle AnimBlueprint with a validated Skeleton.
     * The result uses VehicleAnimationInstance and is intentionally dirty.
     */
    UFUNCTION(
        meta = (AICallable),
        Category = "SharVehicleDriveTestAnimToolset"
    )
    static FString CreateVehicleDriveTestAnimBlueprint(
        const FString& FolderPath,
        const FString& AssetName,
        const FString& SkeletonPath
    );
};
