// File: SharMessageSchemaCatalogTests.cpp
// Path: src/uproject/Source/SharMessaging/Private/Tests/SharMessageSchemaCatalogTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient semantic-channel and schema-policy validation tests only.
// Specification: docs/technical/unreal/typed-event-and-observation-routing-runtime.md

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
