// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Production Asset Manager loading for root runtime definitions.
// - Must-Not:
//   - Parse source formats, invent missing assets, or own application state.
// - Allows:
//   - Discovering and asynchronously loading the root catalog and mode family.
// - Split-When:
//   - Another Primary Asset family requires an independent loading lifecycle.
// - Merge-When:
//   - Another composition object owns the identical startup loading contract.
// - Summary:
//   - Loads root runtime definitions through Unreal Asset Manager.
// - Description:
//   - Fails closed unless one catalog and its complete mode family resolve.
// - Usage:
//   - Owned by the project game instance during native runtime startup.
// - Defaults:
//   - Starts idle with no loaded definitions and no synthetic fallback.
//

//! Asset Manager backed loading for root runtime definitions.

#pragma once

#include "CoreMinimal.h"
#include "UObject/Object.h"

#include "SharRuntimeAssetLoader.generated.h"

class UAssetManager;
class USharApplicationModeDefinition;
class USharGameplayCatalog;
class USharGameplayCatalogSubsystem;

UENUM(BlueprintType)
enum class ESharRuntimeAssetLoadStartResult : uint8
{
    Started,
    AlreadyStarted,
    InvalidDependencies,
    CatalogNotUnique,
};

UENUM(BlueprintType)
enum class ESharRuntimeAssetLoadStatus : uint8
{
    Idle,
    LoadingCatalog,
    LoadingApplicationModes,
    Ready,
    Failed,
};

UENUM(BlueprintType)
enum class ESharRuntimeAssetLoadFailure : uint8
{
    None,
    AssetManagerUnavailable,
    RootCatalogUnavailable,
    CatalogNotUnique,
    CatalogLoadFailed,
    CatalogActivationRejected,
    ApplicationModeFamilyMissing,
    ApplicationModeLoadFailed,
};

DECLARE_MULTICAST_DELEGATE_TwoParams(
    FSharRuntimeAssetLoadTerminalDelegate,
    ESharRuntimeAssetLoadStatus,
    ESharRuntimeAssetLoadFailure
);

UCLASS()
class SHAR_API USharRuntimeAssetLoader final : public UObject
{
    GENERATED_BODY()

public:
    ESharRuntimeAssetLoadStartResult Start(
        UAssetManager* InAssetManager,
        USharGameplayCatalogSubsystem* InRootCatalog
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|Runtime")
    [[nodiscard]] ESharRuntimeAssetLoadStatus GetStatus() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Runtime")
    [[nodiscard]] ESharRuntimeAssetLoadFailure GetFailure() const;

    [[nodiscard]] const USharGameplayCatalog* GetLoadedCatalog() const;

    [[nodiscard]] TArray<USharApplicationModeDefinition*>
    GetLoadedApplicationModes() const;

    [[nodiscard]] FSharRuntimeAssetLoadTerminalDelegate& OnTerminal();

private:
    void HandleCatalogLoaded();
    void HandleApplicationModesLoaded();
    void PublishTerminal();
    void Fail(ESharRuntimeAssetLoadFailure Reason);

    UPROPERTY(Transient)
    TObjectPtr<UAssetManager> AssetManager;

    UPROPERTY(Transient)
    TObjectPtr<USharGameplayCatalogSubsystem> RootCatalog;

    UPROPERTY(Transient)
    TObjectPtr<USharGameplayCatalog> LoadedCatalog;

    UPROPERTY(Transient)
    TArray<TObjectPtr<USharApplicationModeDefinition>> LoadedApplicationModes;

    FPrimaryAssetId CatalogAssetId;
    TArray<FPrimaryAssetId> ApplicationModeAssetIds;
    ESharRuntimeAssetLoadStatus Status = ESharRuntimeAssetLoadStatus::Idle;
    ESharRuntimeAssetLoadFailure Failure = ESharRuntimeAssetLoadFailure::None;
    bool bTerminalPublished = false;
    FSharRuntimeAssetLoadTerminalDelegate TerminalDelegate;
};
