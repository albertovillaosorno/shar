// File: SharWorldClock.h
// Path: src/uproject/Source/SharWorld/Public/World/SharWorldClock.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic world-time state and Blueprint projection only.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#pragma once

#include "CoreMinimal.h"

#include "SharWorldClock.generated.h"

UCLASS(BlueprintType)
class SHARWORLD_API USharWorldClock final : public UObject
{
    GENERATED_BODY()

public:
    static constexpr float HoursPerDay = 24.0F;
    static constexpr float DefaultDayLengthSeconds = 1440.0F;

    UFUNCTION(BlueprintCallable, Category = "SHAR|World Clock")
    bool Configure(float InDayLengthSeconds, float InitialHour);

    UFUNCTION(BlueprintCallable, Category = "SHAR|World Clock")
    bool AdvanceRealSeconds(float RealSeconds);

    UFUNCTION(BlueprintCallable, Category = "SHAR|World Clock")
    void SetPaused(bool bInPaused);

    UFUNCTION(BlueprintPure, Category = "SHAR|World Clock")
    [[nodiscard]] float GetWorldHour() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|World Clock")
    [[nodiscard]] float GetDayFraction() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|World Clock")
    [[nodiscard]] bool IsPaused() const;

private:
    UPROPERTY(Transient)
    float DayLengthSeconds = DefaultDayLengthSeconds;

    UPROPERTY(Transient)
    float WorldTimeSeconds = 0.0F;

    UPROPERTY(Transient)
    bool bPaused = false;

    [[nodiscard]] float WrapWorldSeconds(float CandidateSeconds) const;
};
