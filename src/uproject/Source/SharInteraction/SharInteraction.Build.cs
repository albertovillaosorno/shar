// File: SharInteraction.Build.cs
// Path: src/uproject/Source/SharInteraction/SharInteraction.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: interaction definition, candidate, reservation, and transaction dependencies only.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md

using UnrealBuildTool;

public class SharInteraction : ModuleRules
{
    public SharInteraction(ReadOnlyTargetRules target) : base(target)
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
