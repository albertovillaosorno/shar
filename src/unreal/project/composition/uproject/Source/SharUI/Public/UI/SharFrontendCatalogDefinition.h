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
//   - Shar frontend catalog definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar frontend catalog definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar frontend catalog definition composition module.

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
