// File: SharCheats.Build.cs
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharCheats/SharCheats.Build.cs
// Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// jig-ignore-next-line: exact syntax is indivisible
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
