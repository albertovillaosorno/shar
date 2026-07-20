// File: SharCompatibilityHandshake.h
// Path: src/uproject/Source/SharNetworking/Public/Networking/SharCompatibilityHandshake.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: fail-closed client/server compatibility comparison only; no sockets, travel, or admission mutation.
// ADR: docs/adr/modding/mod-owned-multiplayer-adapters-and-community-servers.md
// LARGE-FILE owner=SharNetworking; reason=cohesive reflected handshake snapshot and result schema;
// split=extract optional compatibility fields if package-defined schemas expand;
// validation=validate.sh SharNetworking plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"
#include "Networking/SharMultiplayerAdapterDefinition.h"
#include "Platform/SharPlatformProfileDefinition.h"

#include "SharCompatibilityHandshake.generated.h"

UENUM(BlueprintType)
enum class ESharCompatibilityResult : uint8
{
    Compatible,
    InvalidSnapshot,
    ProtocolMismatch,
    RuntimeMismatch,
    TargetMismatch,
    PackageSetMismatch,
    CatalogMismatch,
    CapabilityMismatch,
    AuthorityMismatch,
    SavePolicyMismatch,
    AchievementPolicyMismatch,
};

USTRUCT(BlueprintType)
struct SHARNETWORKING_API FSharCompatibilitySnapshot
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Session")
    FName SessionId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Session")
    FName SessionRoleId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Protocol")
    FName ProtocolId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Protocol")
    FString ProtocolRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Compatibility")
    FString RuntimeContractRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Compatibility")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Compatibility")
    FString PackageSetDigest;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Target")
    ESharTargetPlatform Platform = ESharTargetPlatform::Windows;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Target")
    ESharCpuArchitecture Architecture = ESharCpuArchitecture::X8664;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Authority")
    ESharNetworkAuthorityModel AuthorityModel =
        ESharNetworkAuthorityModel::DedicatedServer;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Persistence")
    ESharNetworkSavePolicy SavePolicy = ESharNetworkSavePolicy::None;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Achievements")
    ESharNetworkAchievementPolicy AchievementPolicy =
        ESharNetworkAchievementPolicy::BaseIncompatible;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Capabilities")
    TArray<FName> CapabilityIds;
};

UCLASS(BlueprintType)
class SHARNETWORKING_API USharCompatibilityHandshake final : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Networking")
    static ESharCompatibilityResult Evaluate(
        const FSharCompatibilitySnapshot& Client,
        const FSharCompatibilitySnapshot& Server,
        FName& OutMismatchField
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|Networking")
    [[nodiscard]] static bool IsValidSnapshot(
        const FSharCompatibilitySnapshot& Snapshot
    );

private:
    [[nodiscard]] static ESharCompatibilityResult EvaluateRevisions(
        const FSharCompatibilitySnapshot& Client,
        const FSharCompatibilitySnapshot& Server,
        FName& OutMismatchField
    );
    [[nodiscard]] static ESharCompatibilityResult EvaluatePolicy(
        const FSharCompatibilitySnapshot& Client,
        const FSharCompatibilitySnapshot& Server,
        FName& OutMismatchField
    );
    [[nodiscard]] static bool HasRequiredCapabilities(
        const TArray<FName>& ClientCapabilities,
        const TArray<FName>& ServerCapabilities
    );
};
