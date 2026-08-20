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
//   - Shar import request validation.
// - Must-Not:
//   - Own runtime gameplay policy or mutate content outside generated roots.
// - Allows:
//   - Editor-only inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one imported asset family gains an independent lifecycle.
// - Merge-When:
//   - Merge when another editor module owns the identical responsibility.
// - Summary:
//   - Shar import request validation.
// - Description:
//   - Owns private destination and media payload validation contracts.
// - Usage:
//   - Used through the SharImportEditor module and its native toolset boundary.
// - Defaults:
//   - Invalid, ambiguous, or replacement requests fail explicitly.
//

//! Shar import request validation.

#pragma once

#include "CoreMinimal.h"

namespace UE::SharImportEditor::Private
{
struct FFileMediaSourcePaths
{
    FString PackagePath;
    FString ObjectPath;
    FString RelativePayloadPath;
    FString FullPayloadPath;
};

struct FSkeletalMeshImportPaths
{
    FString MeshPackagePath;
    FString MeshObjectPath;
    FString SkeletonPackagePath;
    FString SkeletonObjectPath;
};

bool ValidateStaticMeshRequest(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName,
    FString& OutError
);

bool ValidateSkeletalMeshRequest(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName,
    FSkeletalMeshImportPaths& OutPaths,
    FString& OutError
);

bool ValidateSoundWaveRequest(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName,
    FString& OutError
);

bool ValidateFileMediaSourceRequest(
    const FString& SourceFile,
    const FString& FolderPath,
    const FString& AssetName,
    FFileMediaSourcePaths& OutPaths,
    FString& OutError
);

bool BuildFileMediaSourcePathsFromObjectPath(
    const FString& AssetPath,
    FFileMediaSourcePaths& OutPaths,
    FString& OutError
);
}
