// File: SharWorldSpatialObservationSubsystem.h
// Path: src/uproject/Source/SharWorld/Public/Spatial/SharWorldSpatialObservationSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: per-world spatial registration and accepted observation state only; no mission, camera, audio, or streaming mutation.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharWorld; reason=cohesive reflected registration and occupancy contract;
// split=extract diagnostic observations if retention policy expands;
// validation=validate.sh SharWorld plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/WorldSubsystem.h"

#include "SharWorldSpatialObservationSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharSpatialObservationKind : uint8
{
    Enter,
    Stay,
    Exit,
};

UENUM(BlueprintType)
enum class ESharSpatialObservationResult : uint8
{
    Accepted,
    InvalidObservation,
    StaleWorld,
    UnknownPlacement,
    StaleDefinition,
    Duplicate,
    InvalidTransition,
};

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharSpatialObservation
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName PlacementId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName VolumeId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ParticipantId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString WorldRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString DefinitionRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Observation")
    ESharSpatialObservationKind Kind = ESharSpatialObservationKind::Enter;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Observation")
    int32 SequenceNumber = 0;
};

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharSpatialRegistrationState
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName PlacementId;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString DefinitionRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Registration")
    bool bActive = false;
};

USTRUCT(BlueprintType)
struct SHARWORLD_API FSharSpatialOccupancyState
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName PlacementId;

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName VolumeId;

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName ParticipantId;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString DefinitionRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Observation")
    int32 LastSequenceNumber = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Observation")
    bool bOccupied = false;
};

UCLASS()
class SHARWORLD_API USharWorldSpatialObservationSubsystem final
    : public UWorldSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|World|Spatial")
    bool ConfigureWorld(const FName& InWorldId, const FString& InWorldRevision);

    UFUNCTION(BlueprintCallable, Category = "SHAR|World|Spatial")
    bool RegisterPlacement(
        const FName& PlacementId,
        const FString& DefinitionRevision
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|World|Spatial")
    bool ReleasePlacement(const FName& PlacementId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|World|Spatial")
    ESharSpatialObservationResult Observe(
        const FSharSpatialObservation& Observation
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Spatial")
    [[nodiscard]] bool IsOccupied(
        const FName& PlacementId,
        const FName& VolumeId,
        const FName& ParticipantId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Spatial")
    [[nodiscard]] int32 GetActiveRegistrationCount() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|World|Spatial")
    [[nodiscard]] int32 GetActiveOccupancyCount() const;

private:
    UPROPERTY(Transient)
    FName WorldId;

    UPROPERTY(Transient)
    FString WorldRevision;

    UPROPERTY(Transient)
    TArray<FSharSpatialRegistrationState> Registrations;

    UPROPERTY(Transient)
    TArray<FSharSpatialOccupancyState> Occupancies;

    [[nodiscard]] FSharSpatialRegistrationState* FindRegistration(
        const FName& PlacementId
    );
    [[nodiscard]] const FSharSpatialRegistrationState* FindRegistration(
        const FName& PlacementId
    ) const;
    [[nodiscard]] FSharSpatialOccupancyState* FindOccupancy(
        const FSharSpatialObservation& Observation
    );
    [[nodiscard]] const FSharSpatialOccupancyState* FindOccupancy(
        const FName& PlacementId,
        const FName& VolumeId,
        const FName& ParticipantId
    ) const;
    [[nodiscard]] static bool IsValidObservation(
        const FSharSpatialObservation& Observation
    );
    [[nodiscard]] static ESharSpatialObservationResult ApplyTransition(
        const FSharSpatialObservation& Observation,
        FSharSpatialOccupancyState& Occupancy
    );
};
