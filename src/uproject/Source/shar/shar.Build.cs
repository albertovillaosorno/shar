// File:
//   - shar.Build.cs
// Path:
//   - src/uproject/Source/shar/shar.Build.cs
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
//   - UnrealBuildTool dependency rules for the SHAR runtime module.
// - Must-Not:
//   - Reference private engine modules, editor-only modules, or unrelated
//   - third-party dependencies.
// - Allows:
//   - The minimal public Unreal modules required by authored SHAR runtime
//   - code.
// - Split-When:
//   - A dependency group belongs to an independently buildable SHAR module.
// - Merge-When:
//   - Another module rules file owns the identical runtime dependency
//   - boundary.
// - Summary:
//   - Declares the SHAR runtime module dependencies.
// - Description:
//   - Uses explicit or shared precompiled headers and lists only the public
//   - modules required by SHAR.
// - Usage:
//   - Loaded by UnrealBuildTool while constructing the shar runtime module
//   - build graph.
// - Defaults:
//   - Depends on Core, CoreUObject, Engine, InputCore, and EnhancedInput.
//
// ADRs:
// - docs/adr/unreal/project/cpp-primary-blueprint-compatible-project.md
//
// Large file:
//   - false
//

// Defines only the public module dependencies required by the authored SHAR
// runtime and leaves editor-only dependencies in separate future modules.

using UnrealBuildTool;

public class shar : ModuleRules
{
    public shar(ReadOnlyTargetRules target) : base(target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
        PublicDependencyModuleNames.AddRange(
            new[]
            {
                "Core",
                "CoreUObject",
                "Engine",
                "InputCore",
                "EnhancedInput",
            }
        );
    }
}
