// File: SharFrontendCatalogDefinition.h
// Path: src/uproject/Source/SharUI/Public/UI/SharFrontendCatalogDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable frontend screen definitions and readiness policy only; widgets, domain snapshots, application transitions, persistence, and platform behavior remain external.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharFrontendCatalogDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharFrontendLayer : uint8
{
    Boot,
    Primary,
    Modal,
    Notification,
};

UENUM(BlueprintType)
enum class ESharFrontendHistoryPolicy : uint8
{
    Preserve,
    Push,
    Replace,
    Pop,
};

UENUM(BlueprintType)
enum class ESharFrontendReadinessKind : uint8
{
    DomainSnapshot,
    AssetBundles,
    ViewModel,
    LayerReservation,
    WidgetActivation,
    Focus,
    ActionRouting,
};

USTRUCT(BlueprintType)
struct SHARUI_API FSharFrontendScreenDefinition
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Identity")
    FName ScreenId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Layer")
    ESharFrontendLayer Layer = ESharFrontendLayer::Primary;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "View Model")
    FName ViewModelSchemaId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Input")
    FName SemanticActionSetId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Policy")
    FName EntryPredicateId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Policy")
    FName ExitPolicyId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Focus")
    FName FocusPolicyId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Assets")
    TArray<FName> RequiredBundleIds;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Navigation")
    TArray<FName> AllowedDestinationScreenIds;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Readiness")
    TArray<ESharFrontendReadinessKind> PreCommitRequirements;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Readiness")
    TArray<ESharFrontendReadinessKind> PostCommitRequirements;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Readiness")
    bool bAllowDegradedReadiness = false;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Navigation")
    bool bSupportsBack = true;
};

UCLASS(BlueprintType)
class SHARUI_API USharFrontendCatalogDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Frontend")
    FName InitialScreenId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Frontend")
    TArray<FSharFrontendScreenDefinition> Screens;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

    [[nodiscard]] const FSharFrontendScreenDefinition* FindScreen(
        const FName& ScreenId
    ) const;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
