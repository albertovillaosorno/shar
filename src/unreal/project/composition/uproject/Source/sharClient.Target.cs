// File: sharClient.Target.cs
// Path: src/unreal/project/composition/uproject/Source/sharClient.Target.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// jig-ignore-next-line: exact syntax is indivisible
// Boundary: desktop client target declaration only; no hosted-service assumptions.
// jig-ignore-next-line: exact syntax is indivisible
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

using UnrealBuildTool;

[SupportedPlatforms(UnrealPlatformClass.Desktop)]
public class sharClientTarget : TargetRules
{
    public sharClientTarget(TargetInfo target) : base(target)
    {
        Type = TargetType.Client;
        SharTargetDefaults.Apply(this);
    }
}
