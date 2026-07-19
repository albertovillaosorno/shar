// File: SharPlatformProfileDefinition.h
// Path: src/uproject/Source/SharContent/Public/Platform/SharPlatformProfileDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: native SHAR platform-profile contract; no local SDK claims or runtime packaging mutation.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

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
