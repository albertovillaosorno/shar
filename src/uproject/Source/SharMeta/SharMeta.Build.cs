// File: SharMeta.Build.cs
// Path: src/uproject/Source/SharMeta/SharMeta.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable meta-catalog schemas and activation only; cheat execution, credits playback, calendar selection, UI, and persistence remain external.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

using UnrealBuildTool;

public class SharMeta : ModuleRules
{
    public SharMeta(ReadOnlyTargetRules target) : base(target)
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
