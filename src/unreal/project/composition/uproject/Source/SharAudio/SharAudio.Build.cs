// File: SharAudio.Build.cs
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharAudio/SharAudio.Build.cs
// Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: audio definition and lease-state dependencies only.
// jig-ignore-next-line: exact syntax is indivisible
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

using UnrealBuildTool;

public class SharAudio : ModuleRules
{
    public SharAudio(ReadOnlyTargetRules target) : base(target)
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
