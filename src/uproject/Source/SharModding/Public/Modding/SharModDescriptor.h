// File: SharModDescriptor.h
// Path: src/uproject/Source/SharModding/Public/Modding/SharModDescriptor.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: mod identity, compatibility, trust, replacement, save, and network contracts; no false sandbox claims.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"
#include "Engine/DataAsset.h"

#include "SharModDescriptor.generated.h"

UENUM(BlueprintType)
enum class ESharModTrustTier : uint8
{
    DataOnly,
    Blueprint,
    Native,
    ServerRequired,
};

UENUM(BlueprintType)
enum class ESharModSavePolicy : uint8
{
    NoPersistentState,
    NamespacedOptional,
    NamespacedRequired,
};

UENUM(BlueprintType)
enum class ESharModNetworkPolicy : uint8
{
    SinglePlayerOnly,
    CosmeticOptional,
    PackageMatchRequired,
};

USTRUCT(BlueprintType)
struct SHARMODDING_API FSharModReplacementDefinition
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Replacement")
    FPrimaryAssetId TargetAssetId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Replacement")
    FName ScopeId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Replacement")
    int32 Priority = 0;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Replacement")
    FName RollbackPolicyId;
};

UCLASS(BlueprintType)
class SHARMODDING_API USharModDescriptor final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mod")
    FName NamespaceId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mod")
    FString Version;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Mod")
    FString PackageSetDigest;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Compatibility")
    int32 MinimumGameSchemaVersion = 1;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Trust")
    ESharModTrustTier TrustTier = ESharModTrustTier::DataOnly;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Trust")
    bool bExplicitUserApprovalRequired = false;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Save")
    ESharModSavePolicy SavePolicy = ESharModSavePolicy::NoPersistentState;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Network")
    ESharModNetworkPolicy NetworkPolicy = ESharModNetworkPolicy::SinglePlayerOnly;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Compatibility")
    TArray<FName> RequiredModNamespaces;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Compatibility")
    TArray<FName> ConflictingModNamespaces;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Activation")
    TArray<FName> ActivationActionIds;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Replacement")
    TArray<FSharModReplacementDefinition> Replacements;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
