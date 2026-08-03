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
//   - Shar world clock composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar world clock composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar world clock composition module.

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
