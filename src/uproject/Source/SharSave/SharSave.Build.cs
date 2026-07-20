// File: SharSave.Build.cs
// Path: src/uproject/Source/SharSave/SharSave.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: portable save schema, migration, slot, and transaction coordination only; domain snapshots and platform storage remain external.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md

using UnrealBuildTool;

public class SharSave : ModuleRules
{
    public SharSave(ReadOnlyTargetRules target) : base(target)
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
