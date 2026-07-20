// File: SharProgression.Build.cs
// Path: src/uproject/Source/SharProgression/SharProgression.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: profile identity, progression catalogs, deterministic mutations, and immutable projections only; save I/O, platform accounts, UI, and gameplay execution remain external.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

using UnrealBuildTool;

public class SharProgression : ModuleRules
{
    public SharProgression(ReadOnlyTargetRules target) : base(target)
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
