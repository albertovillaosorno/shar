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
//   - Shar cheat identity composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar cheat identity composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar cheat identity composition module.

#pragma once

#include "CoreMinimal.h"

namespace SharCheatIdentity
{
inline bool HasValidBoundary(const FString& Value)
{
    return !Value.IsEmpty() && !Value.StartsWith(TEXT("_"))
        && !Value.EndsWith(TEXT("_"));
}

inline bool IsCanonicalCharacter(const TCHAR Character)
{
    const bool bIsLowercaseAscii = Character >= 'a' && Character <= 'z';
    const bool bIsDigit = Character >= '0' && Character <= '9';
    return bIsLowercaseAscii || bIsDigit || Character == '_';
}

inline bool HasCanonicalCharacters(const FString& Value)
{
    bool bPreviousWasUnderscore = false;
    for (const TCHAR Character : Value)
    {
        const bool bIsUnderscore = Character == '_';
        if (!IsCanonicalCharacter(Character)
            || (bIsUnderscore && bPreviousWasUnderscore))
        {
            return false;
        }
        bPreviousWasUnderscore = bIsUnderscore;
    }
    return true;
}

inline bool IsCanonical(const FName& Candidate)
{
    if (Candidate.IsNone())
    {
        return false;
    }
    const FString Value = Candidate.ToString();
    return HasValidBoundary(Value) && HasCanonicalCharacters(Value);
}
} // namespace SharCheatIdentity
