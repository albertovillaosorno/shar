// File: SharProgressionStateTests.cpp
// Path: src/uproject/Source/SharProgression/Private/Tests/SharProgressionStateTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient progression and idempotent reward tests only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#if WITH_DEV_AUTOMATION_TESTS

#include "Progression/SharProgressionState.h"

#include "Misc/AutomationTest.h"

static constexpr int32 CurrencyRewardQuantity = 50;
static constexpr int32 RedundantUnlockQuantity = 5;

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharProgressionIdempotencyTest,
    "SHAR.Progression.State.Idempotency",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharProgressionIdempotencyTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Progression = NewObject<USharProgressionState>();

    FSharRewardRequest Currency;
    Currency.TransactionId = FName(TEXT("mission_01_currency"));
    Currency.OperationId = FName(TEXT("grant_currency"));
    Currency.TargetId = FName(TEXT("coins"));
    Currency.Quantity = CurrencyRewardQuantity;
    TestTrue(
        TEXT("Permanent reward applies"),
        Progression->ApplyReward(Currency) == ESharRewardApplyResult::Applied
    );
    TestTrue(
        TEXT("Permanent reward is idempotent"),
        Progression->ApplyReward(Currency)
            == ESharRewardApplyResult::AlreadyApplied
    );
    TestTrue(
        TEXT("Currency is not duplicated"),
        Progression->GetQuantity(Currency.OperationId, Currency.TargetId)
            == CurrencyRewardQuantity
    );

    FSharRewardRequest Unlock;
    Unlock.TransactionId = FName(TEXT("temporary_character_unlock"));
    Unlock.OperationId = FName(TEXT("unlock_character"));
    Unlock.TargetId = FName(TEXT("bart"));
    Unlock.Quantity = RedundantUnlockQuantity;
    Unlock.bPermanent = false;
    TestTrue(
        TEXT("Temporary unlock applies"),
        Progression->ApplyReward(Unlock) == ESharRewardApplyResult::Applied
    );
    TestTrue(
        TEXT("Unlock operation stores a boolean quantity"),
        Progression->GetQuantity(Unlock.OperationId, Unlock.TargetId) == 1
    );
    return true;
}

#endif
