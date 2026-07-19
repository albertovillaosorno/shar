// File:
//   - SharCharacters.Build.cs
// Path:
//   - src/uproject/Source/SharCharacters/SharCharacters.Build.cs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - UnrealBuildTool dependencies for character content and character tests.
// - Must-Not:
//   - Depend on missions, vehicles, world assembly, UI, or editor-only modules.
// - Allows:
//   - Native character definitions, presentations, tags, and automation tests.
// - Split-When:
//   - Character runtime behavior requires an independent module lifecycle.
// - Merge-When:
//   - Another module owns the same character content boundary.
// - Summary:
//   - Declares the SHAR character-family module dependencies.
// - Description:
//   - Keeps imported character assets independent from higher gameplay systems.
// - Usage:
//   - Loaded by the game bootstrap and character-consuming runtime services.
// - Defaults:
//   - Depends on SharContent and public Unreal runtime modules only.
//
// ADRs:
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
//
// Large file:
//   - false
//

using UnrealBuildTool;

public class SharCharacters : ModuleRules
{
    public SharCharacters(ReadOnlyTargetRules target) : base(target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
        PublicDependencyModuleNames.AddRange(
            new[]
            {
                "Core",
                "CoreUObject",
                "Engine",
                "GameplayTags",
                "SharContent",
            }
        );
    }
}
