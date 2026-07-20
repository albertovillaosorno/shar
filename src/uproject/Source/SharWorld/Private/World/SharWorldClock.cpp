// File: SharWorldClock.cpp
// Path: src/uproject/Source/SharWorld/Private/World/SharWorldClock.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic clock arithmetic only; no actor ticking or lighting mutation.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#include "World/SharWorldClock.h"

float USharWorldClock::WrapWorldSeconds(const float CandidateSeconds) const
{
    const float Wrapped = FMath::Fmod(CandidateSeconds, DayLengthSeconds);
    return Wrapped < 0.0F ? Wrapped + DayLengthSeconds : Wrapped;
}

bool USharWorldClock::Configure(
    const float InDayLengthSeconds,
    const float InitialHour
)
{
    const bool bInvalid =
        !FMath::IsFinite(InDayLengthSeconds)
        || InDayLengthSeconds <= 0.0F
        || !FMath::IsFinite(InitialHour)
        || InitialHour < 0.0F
        || InitialHour >= HoursPerDay;
    if (bInvalid)
    {
        return false;
    }
    DayLengthSeconds = InDayLengthSeconds;
    WorldTimeSeconds = DayLengthSeconds * (InitialHour / HoursPerDay);
    return true;
}

bool USharWorldClock::AdvanceRealSeconds(const float RealSeconds)
{
    if (!FMath::IsFinite(RealSeconds) || RealSeconds < 0.0F)
    {
        return false;
    }
    if (!bPaused)
    {
        WorldTimeSeconds = WrapWorldSeconds(WorldTimeSeconds + RealSeconds);
    }
    return true;
}

void USharWorldClock::SetPaused(const bool bInPaused)
{
    bPaused = bInPaused;
}

float USharWorldClock::GetWorldHour() const
{
    return GetDayFraction() * HoursPerDay;
}

float USharWorldClock::GetDayFraction() const
{
    return WorldTimeSeconds / DayLengthSeconds;
}

bool USharWorldClock::IsPaused() const
{
    return bPaused;
}
