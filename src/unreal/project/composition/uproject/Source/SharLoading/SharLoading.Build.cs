// File: SharLoading.Build.cs
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharLoading/SharLoading.Build.cs
// Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// jig-ignore-next-line: exact syntax is indivisible
// Boundary: typed load-plan coordination and world-readiness barriers only; Asset Manager, package, and streaming adapters remain external.
// jig-ignore-next-line: exact syntax is indivisible
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md

using UnrealBuildTool;

public class SharLoading : ModuleRules
{
    public SharLoading(ReadOnlyTargetRules target) : base(target)
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
