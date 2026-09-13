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
//   - Deterministic FBX scene-unit import policy for SHAR native assets.
// - Must-Not:
//   - Scale source vertices, invent asset transforms, or own gameplay policy.
// - Allows:
//   - Configure Unreal FBX import data from authored scene-unit metadata.
// - Split-When:
//   - Split when another FBX policy gains an independent lifecycle.
// - Merge-When:
//   - Merge when another import policy owns identical scene-unit settings.
// - Summary:
//   - SHAR FBX scene-unit import policy.
// - Description:
//   - Preserves identity import scale while allowing Unreal to convert the
//     source FBX system unit to its native centimeter unit.
// - Usage:
//   - Applied by static, world, and skeletal FBX import transactions.
// - Defaults:
//   - Uniform import scale is one and FBX scene-unit conversion is required.
//

//! SHAR FBX scene-unit import policy.

#pragma once

#include "Factories/FbxAssetImportData.h"

namespace UE::SharImportEditor::Private
{
inline void ApplyFbxSceneUnitPolicy(UFbxAssetImportData& ImportData)
{
    ImportData.ImportUniformScale = 1.0F;
    ImportData.bConvertSceneUnit = true;
}
}
