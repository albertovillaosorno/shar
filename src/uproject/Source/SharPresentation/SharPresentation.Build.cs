// File: SharPresentation.Build.cs
// Path: src/uproject/Source/SharPresentation/SharPresentation.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: presentation definitions and world-scoped lifecycle coordination only; adapters remain in their owning modules.
// Specification: docs/technical/unreal/presentation-playback-runtime.md

using UnrealBuildTool;

public class SharPresentation : ModuleRules
{
    public SharPresentation(ReadOnlyTargetRules target) : base(target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
        PublicDependencyModuleNames.AddRange(
            new[]
            {
                "Core",
                "CoreUObject",
                "Engine",
                "SharContent",
            }
        );
    }
}
