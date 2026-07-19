// File: SharPlatformProfileDefinition.cpp
// Path: src/uproject/Source/SharContent/Private/Platform/SharPlatformProfileDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: platform capability validation only; no SDK probing or packaging mutation.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#include "Platform/SharPlatformProfileDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddPlatformError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static void AppendBudgetErrors(
    const USharPlatformProfileDefinition& Profile,
    TArray<FText>& OutErrors
)
{
    if (Profile.TargetFrameRate <= 0)
    {
        AddPlatformError(OutErrors, TEXT("Target frame rate must be positive."));
    }
    if (Profile.MemoryBudgetMegabytes
        < USharPlatformProfileDefinition::MinimumMemoryBudgetMegabytes)
    {
        AddPlatformError(
            OutErrors,
            TEXT("Memory budget is below the supported minimum.")
        );
    }
}

static void AppendRendererErrors(
    const USharPlatformProfileDefinition& Profile,
    TArray<FText>& OutErrors
)
{
    const bool bIsAndroid =
        Profile.TargetPlatform == ESharTargetPlatform::Android;
    const bool bUsesMobileRenderer =
        Profile.RendererProfile == ESharRendererProfile::MobileForward;
    if (bIsAndroid != bUsesMobileRenderer)
    {
        AddPlatformError(
            OutErrors,
            TEXT("Android requires the mobile renderer; desktop requires deferred.")
        );
    }
    if (Profile.bSupportsHardwareRayTracing && bUsesMobileRenderer)
    {
        AddPlatformError(
            OutErrors,
            TEXT("Hardware ray tracing requires the desktop renderer profile.")
        );
    }
}

static void AppendNetworkingErrors(
    const USharPlatformProfileDefinition& Profile,
    TArray<FText>& OutErrors
)
{
    if (Profile.TargetPlatform == ESharTargetPlatform::Android
        && Profile.bSupportsDedicatedServer)
    {
        AddPlatformError(
            OutErrors,
            TEXT("Android profiles cannot claim dedicated-server packaging.")
        );
    }
}

bool USharPlatformProfileDefinition::IsSupportedTarget(
    const ESharTargetPlatform Platform,
    const ESharCpuArchitecture Architecture
)
{
    const bool bKnownArchitecture =
        Architecture == ESharCpuArchitecture::X8664
        || Architecture == ESharCpuArchitecture::Arm64;
    switch (Platform)
    {
        case ESharTargetPlatform::Windows:
        case ESharTargetPlatform::Linux:
        case ESharTargetPlatform::Android:
            return bKnownArchitecture;
        default:
            return false;
    }
}

void USharPlatformProfileDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    if (!IsSupportedTarget(TargetPlatform, CpuArchitecture))
    {
        AddPlatformError(
            OutErrors,
            TEXT("The platform and CPU combination is unsupported.")
        );
    }
    AppendBudgetErrors(*this, OutErrors);
    AppendRendererErrors(*this, OutErrors);
    AppendNetworkingErrors(*this, OutErrors);
}

FPrimaryAssetType
USharPlatformProfileDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharPlatformProfile")};
}
