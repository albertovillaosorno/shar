// File: SharMultiplayerAdapterDefinition.h
// Path: src/uproject/Source/SharNetworking/Public/Networking/SharMultiplayerAdapterDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deferred mod-owned multiplayer declaration only; no transport or base-campaign multiplayer implementation.
// ADR: docs/adr/modding/mod-owned-multiplayer-adapters-and-community-servers.md
// LARGE-FILE owner=SharNetworking; reason=cohesive reflected multiplayer declaration schema;
// split=extract package and target definitions if independently versioned assets appear;
// validation=validate.sh SharNetworking plus Unreal automation; review=2027-01.

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
