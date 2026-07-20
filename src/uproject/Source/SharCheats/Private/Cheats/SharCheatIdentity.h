// File: SharCheatIdentity.h
// Path: src/uproject/Source/SharCheats/Private/Cheats/SharCheatIdentity.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: dependency-free canonical identity validation for the cheat control plane only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

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
