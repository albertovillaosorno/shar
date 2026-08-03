// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar platform profile definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar platform profile definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar platform profile definition composition module.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharPlatformProfileDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharTargetPlatform : uint8
{
    Windows,
    Linux,
    Android,
};

UENUM(BlueprintType)
enum class ESharCpuArchitecture : uint8
{
    X8664,
    Arm64,
};

UENUM(BlueprintType)
enum class ESharRendererProfile : uint8
{
    DesktopDeferred,
    MobileForward,
};

UCLASS(BlueprintType)
class SHARCONTENT_API USharPlatformProfileDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    static constexpr int32 DefaultTargetFrameRate = 60;
    static constexpr int32 MinimumMemoryBudgetMegabytes = 512;
    static constexpr int32 DefaultMemoryBudgetMegabytes = 4096;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Platform")
    ESharTargetPlatform TargetPlatform = ESharTargetPlatform::Windows;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Platform")
    ESharCpuArchitecture CpuArchitecture = ESharCpuArchitecture::X8664;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Rendering")
    // jig-ignore-next-line: exact syntax is indivisible
    ESharRendererProfile RendererProfile = ESharRendererProfile::DesktopDeferred;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Platform")
    int32 TargetFrameRate = DefaultTargetFrameRate;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Platform")
    int32 MemoryBudgetMegabytes = DefaultMemoryBudgetMegabytes;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Networking")
    bool bSupportsListenServer = true;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Networking")
    bool bSupportsDedicatedServer = true;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Rendering")
    bool bSupportsHardwareRayTracing = false;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Rendering")
    bool bSupportsNanite = true;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Rendering")
    bool bSupportsLumen = true;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

    [[nodiscard]] static bool IsSupportedTarget(
        ESharTargetPlatform Platform,
        ESharCpuArchitecture Architecture
    );

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
