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
//   - Shar message schema catalog tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar message schema catalog tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar message schema catalog tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharMessagingTestFixtures.h"

#include "Messaging/SharMessageSchemaCatalog.h"
#include "Misc/AutomationTest.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharMessageSchemaCatalogValidationTest,
    "SHAR.Messaging.SchemaCatalog.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharMessageSchemaCatalogValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Catalog = NewObject<USharMessageSchemaCatalog>();
    const FSharMessageSchemaDefinition Valid = MakeMessageSchema({
        .SchemaId = FName(TEXT("world_ready_v1")),
        .ChannelId = FName(TEXT("shar.lifecycle.world.ready")),
        .DeliveryPhase = ESharMessageDeliveryPhase::ImmediateReadOnly,
    });
    TestTrue(
        TEXT("Valid semantic channel schema registers"),
        Catalog->RegisterSchema(Valid)
    );

    const FSharMessageSchemaDefinition DuplicateChannel = MakeMessageSchema({
        .SchemaId = FName(TEXT("world_ready_v2")),
        .ChannelId = FName(TEXT("shar.lifecycle.world.ready")),
        .DeliveryPhase = ESharMessageDeliveryPhase::NextWorldFrame,
    });
    TestFalse(
        TEXT("Duplicate canonical channel is rejected"),
        Catalog->RegisterSchema(DuplicateChannel)
    );

    const FSharMessageSchemaDefinition InvalidChannel = MakeMessageSchema({
        .SchemaId = FName(TEXT("invalid_channel_v1")),
        .ChannelId = FName(TEXT("shar..world.ready")),
    });
    TestFalse(
        TEXT("Malformed semantic channel is rejected"),
        Catalog->RegisterSchema(InvalidChannel)
    );

    FSharMessageSchemaDefinition InvalidDurable = MakeMessageSchema({
        .SchemaId = FName(TEXT("durable_pointer_v1")),
        .ChannelId = FName(TEXT("shar.world.durable_pointer")),
    });
    InvalidDurable.bDurable = true;
    InvalidDurable.bAllowsTransientObjectReferences = true;
    TestFalse(
        TEXT("Durable schema cannot permit transient object references"),
        Catalog->RegisterSchema(InvalidDurable)
    );
    return true;
}

#endif
