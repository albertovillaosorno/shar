// File: SharMissions.Build.cs
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharMissions/SharMissions.Build.cs
// Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// jig-ignore-next-line: exact syntax is indivisible
// Boundary: mission and save composition only; shared progression authority comes from SharProgression.
// jig-ignore-next-line: exact syntax is indivisible
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
                "SharProgression",
            }
        );
    }
}
