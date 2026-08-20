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
//   - Shar multiplayer adapter definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar multiplayer adapter definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar multiplayer adapter definition composition module.

#pragma once

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"
#include "Platform/SharPlatformProfileDefinition.h"

#include "SharMultiplayerAdapterDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharNetworkAuthorityModel : uint8
{
    DedicatedServer,
    ListenServer,
};

UENUM(BlueprintType)
enum class ESharNetworkSavePolicy : uint8
{
    None,
    EphemeralSession,
    NamespacedModOwned,
};

UENUM(BlueprintType)
enum class ESharNetworkAchievementPolicy : uint8
{
    BaseCompatible,
    BaseIncompatible,
    CustomProvider,
};

UENUM(BlueprintType)
enum class ESharNetworkDiscoveryPolicy : uint8
{
    DirectAddress,
    Lan,
    ModOwnedDirectory,
};

UENUM(BlueprintType)
enum class ESharNetworkNativeCodePolicy : uint8
{
    ContentOnly,
    ExplicitlyTrustedNative,
};

USTRUCT(BlueprintType)
struct SHARNETWORKING_API FSharNetworkServerTarget
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Target")
    ESharTargetPlatform Platform = ESharTargetPlatform::Windows;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Target")
    ESharCpuArchitecture Architecture = ESharCpuArchitecture::X8664;
};

USTRUCT(BlueprintType)
struct SHARNETWORKING_API FSharNetworkRequiredPackage
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Package")
    FName NamespaceId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Package")
    FString Version;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Package")
    FString PackageDigest;
};

UCLASS(BlueprintType)
class SHARNETWORKING_API USharMultiplayerAdapterDefinition final
    : public UDataAsset
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Identity")
    FName MultiplayerModeId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Protocol")
    FName ProtocolId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Protocol")
    FString ProtocolRevision;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Compatibility")
    FString RuntimeContractRevision;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Compatibility")
    FString RequiredCatalogRevision;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Compatibility")
    FString PackageSetDigest;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Authority")
    ESharNetworkAuthorityModel AuthorityModel =
        ESharNetworkAuthorityModel::DedicatedServer;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Roles")
    TArray<FName> ClientRoleIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Targets")
    TArray<FSharNetworkServerTarget> ServerTargets;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Packages")
    TArray<FSharNetworkRequiredPackage> RequiredPackages;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Capabilities")
    TArray<FName> RequiredCapabilityIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Trust")
    ESharNetworkNativeCodePolicy NativeCodePolicy =
        ESharNetworkNativeCodePolicy::ContentOnly;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Trust")
    bool bExplicitUserApprovalRequired = false;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Persistence")
    ESharNetworkSavePolicy SavePolicy = ESharNetworkSavePolicy::None;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Achievements")
    ESharNetworkAchievementPolicy AchievementPolicy =
        ESharNetworkAchievementPolicy::BaseIncompatible;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Discovery")
    ESharNetworkDiscoveryPolicy DiscoveryPolicy =
        ESharNetworkDiscoveryPolicy::DirectAddress;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Teardown")
    FName TeardownPolicyId;

    UFUNCTION(BlueprintCallable, Category = "SHAR|Networking")
    void GatherValidationErrors(TArray<FText>& OutErrors) const;
};
