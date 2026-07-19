// File: sharServer.Target.cs
// Path: src/uproject/Source/sharServer.Target.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: self-hosted dedicated-server target declaration only; no operated infrastructure.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

using UnrealBuildTool;

[SupportedPlatforms(UnrealPlatformClass.Server)]
public class sharServerTarget : TargetRules
{
    public sharServerTarget(TargetInfo target) : base(target)
    {
        Type = TargetType.Server;
        SharTargetDefaults.Apply(this);
    }
}
