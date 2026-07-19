// File: SharMissions.Build.cs
// Path: src/uproject/Source/SharMissions/SharMissions.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: mission module dependencies only; no sideways gameplay implementation dependencies.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

using UnrealBuildTool;

public class SharMissions : ModuleRules
{
    public SharMissions(ReadOnlyTargetRules target) : base(target)
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
