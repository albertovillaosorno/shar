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
//   - Shar compatibility handshake composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar compatibility handshake composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar compatibility handshake composition module.

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
