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
