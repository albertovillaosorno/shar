// File: SharAction.Build.cs
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharAction/SharAction.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// jig-ignore-next-line: exact syntax is indivisible
// Boundary: action definitions and resource arbitration dependencies only; StateTree remains the scheduler.
// ADR: docs/adr/unreal/runtime/typed-state-tree-action-sequences.md

using UnrealBuildTool;

public class SharAction : ModuleRules
{
    public SharAction(ReadOnlyTargetRules target) : base(target)
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
