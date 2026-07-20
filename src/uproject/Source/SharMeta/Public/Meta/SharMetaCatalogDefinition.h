// File: SharMetaCatalogDefinition.h
// Path: src/uproject/Source/SharMeta/Public/Meta/SharMetaCatalogDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable semantic cheat definitions within the shared meta catalog only; physical input mapping, runtime state, effect execution, credits playback, and calendar selection remain external.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharMeta; reason=cohesive reflected meta-catalog and cheat-definition contract;
// split=extract credits and calendar rows when their runtimes are implemented;
// validation=validate.sh SharMeta plus Unreal automation; review=2027-01.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharMetaCatalogDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharCheatInputToken : uint8
{
    Up,
    Down,
    Left,
    Right,
    Accept,
    Cancel,
    ShoulderLeft,
    ShoulderRight,
    ActionPrimary,
    ActionSecondary,
    Horn,
};

UENUM(BlueprintType)
enum class ESharCheatPrerequisite : uint8
{
    None,
    LoadedProfile,
    CompletedStory,
    DeveloperBuild,
};

UENUM(BlueprintType)
enum class ESharCheatActivationMode : uint8
{
    EnableOnly,
    Toggle,
    ImmediateCommand,
};

UENUM(BlueprintType)
enum class ESharCheatLifetime : uint8
{
    Session,
    Chapter,
    Mission,
    PersistentTransaction,
};

UENUM(BlueprintType)
enum class ESharCheatEffectKind : uint8
{
    UnlockOverlay,
    VehicleModifier,
    HudOverlay,
    PresentationOverride,
    ProgressionTransaction,
    ApplicationTransition,
    CreditsDialogue,
    Diagnostic,
};

USTRUCT(BlueprintType)
struct SHARMETA_API FSharCheatEffectParameters
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Effect")
    FName OperationId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Effect")
    FName TargetId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Effect")
    int64 Quantity = 0;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Effect")
    double Scalar = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Effect")
    FName TransitionId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Effect")
    bool bRepeatable = false;
};

USTRUCT(BlueprintType)
struct SHARMETA_API FSharCheatDefinition
{
    GENERATED_BODY()

    static constexpr int32 RequiredInputTokenCount = 4;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Identity")
    FName CheatId;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Input")
    TArray<ESharCheatInputToken> InputTokens;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Policy")
    ESharCheatPrerequisite Prerequisite = ESharCheatPrerequisite::None;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Policy")
    ESharCheatActivationMode ActivationMode =
        ESharCheatActivationMode::EnableOnly;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Policy")
    ESharCheatLifetime Lifetime = ESharCheatLifetime::Session;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Effect")
    ESharCheatEffectKind EffectKind = ESharCheatEffectKind::UnlockOverlay;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Effect")
    FSharCheatEffectParameters EffectParameters;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Feedback")
    FName SuccessFeedbackEvent;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Feedback")
    FName UnavailableFeedbackEvent;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Feedback")
    FName DisabledFeedbackEvent;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Feedback")
    FName InvalidSequenceFeedbackEvent;
};

UCLASS(BlueprintType)
class SHARMETA_API USharMetaCatalogDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Cheats")
    TArray<FSharCheatDefinition> Cheats;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

    [[nodiscard]] const FSharCheatDefinition* FindCheat(
        const FName& CheatId
    ) const;

    [[nodiscard]] const FSharCheatDefinition* FindCheatBySequence(
        const TArray<ESharCheatInputToken>& InputTokens
    ) const;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
