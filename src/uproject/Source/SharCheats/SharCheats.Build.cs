// File: SharCheats.Build.cs
// Path: src/uproject/Source/SharCheats/SharCheats.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: semantic cheat recognition, deterministic effect arbitration, and lifetime state only; physical input, gameplay execution, progression mutation, save I/O, UI, and presentation remain external.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

using UnrealBuildTool;

public class SharCheats : ModuleRules
{
    public SharCheats(ReadOnlyTargetRules target) : base(target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
        PublicDependencyModuleNames.AddRange(
            new[]
            {
                "Core",
                "CoreUObject",
                "Engine",
                "SharMeta",
            }
        );
    }
}
