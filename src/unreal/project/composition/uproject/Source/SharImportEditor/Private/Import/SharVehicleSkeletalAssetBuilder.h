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
//   - Save packages, build render LODs, or repeat source-basis conversion.
// - Allows:
//   - Construct UObjects from an already validated Unreal-basis recipe.
// - Split-When:
//   - Render geometry publication gains its own construction lifecycle.
// - Merge-When:
//   - Another builder owns identical transient skeletal UObject construction.
// - Summary:
//   - Native vehicle skeletal UObject shell builder.
// - Description:
//   - Materializes rig identity, rest transforms, and material slots natively.
// - Usage:
//   - Runs after normalized model decoding and before render LOD construction.
// - Defaults:
//   - Invalid recipes fail before output objects are returned.
//

//! Native vehicle skeletal UObject shell builder.

#pragma once

#include "CoreMinimal.h"

class USkeletalMesh;
class USkeleton;

namespace UE::SharImportEditor::Private
{
struct FSharNormalizedVehicleSkeletalModel;

bool BuildTransientVehicleSkeletalAssetShell(
    const FSharNormalizedVehicleSkeletalModel &Model, USkeletalMesh *&OutMesh,
    USkeleton *&OutSkeleton, FString &OutError);
} // namespace UE::SharImportEditor::Private
