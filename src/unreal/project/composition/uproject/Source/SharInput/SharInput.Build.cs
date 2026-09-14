// File: SharInput.Build.cs
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharInput/SharInput.Build.cs
// Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: per-local-player input leases and Enhanced Input adaptation.
// jig-ignore-next-line: specification path is indivisible
// Specification: docs/technical/unreal/semantic-input-device-and-haptics-runtime.md

using UnrealBuildTool;

public class SharInput : ModuleRules
{
    public SharInput(ReadOnlyTargetRules target) : base(target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
        PublicDependencyModuleNames.AddRange(
            new[]
            {
                "Core",
                "CoreUObject",
                "Engine",
                "EnhancedInput",
                "SharContent",
            }
        );
    }
}
