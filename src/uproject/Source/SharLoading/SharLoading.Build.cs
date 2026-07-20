// File: SharLoading.Build.cs
// Path: src/uproject/Source/SharLoading/SharLoading.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: typed load-plan coordination and world-readiness barriers only; Asset Manager, package, and streaming adapters remain external.
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
