// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar import editor build rules.
// - Must-Not:
//   - Own runtime gameplay policy or mutate content outside generated roots.
// - Allows:
//   - Editor-only inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one imported asset family gains an independent lifecycle.
// - Merge-When:
//   - Merge when another editor module owns the identical responsibility.
// - Summary:
//   - Shar import editor build rules.
// - Description:
//   - Declares editor-only module dependencies for generated content imports.
// - Usage:
//   - Used through the SharImportEditor module and its native toolset boundary.
// - Defaults:
//   - Invalid, ambiguous, or replacement requests fail explicitly.
//
using UnrealBuildTool;

public class SharImportEditor : ModuleRules
{
    public SharImportEditor(ReadOnlyTargetRules target) : base(target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
        PublicDependencyModuleNames.AddRange(
            new[]
            {
                "Core",
                "CoreUObject",
                "Engine",
                "ToolsetRegistry",
            }
        );
        PrivateDependencyModuleNames.AddRange(
            new[]
            {
                "AssetRegistry",
                "AssetTools",
                "AudioEditor",
                "MediaAssets",
                "UnrealEd",
            }
        );
    }
}
