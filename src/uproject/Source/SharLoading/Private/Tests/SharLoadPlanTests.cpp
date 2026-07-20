// File: SharLoadPlanTests.cpp
// Path: src/uproject/Source/SharLoading/Private/Tests/SharLoadPlanTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable load-plan identity, dependency, and cycle validation tests only.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharLoadingTestFixtures.h"

#include "Loading/SharLoadCoordinatorSubsystem.h"
#include "Misc/AutomationTest.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharLoadPlanValidationTest,
    "SHAR.Loading.Plan.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharLoadPlanValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharLoadCoordinatorSubsystem* Coordinator = MakeEmptyCoordinator();
    const FSharLoadPlan Valid = MakeRequiredPlan();
    TestTrue(TEXT("Valid acyclic plan registers"), Coordinator->RegisterPlan(Valid));

    FSharLoadPlan Cyclic = MakeRequiredPlan();
    Cyclic.PlanId = FName(TEXT("cyclic_plan"));
    for (FSharLoadPlanNode& Node : Cyclic.Nodes)
    {
        if (Node.NodeId == FName(TEXT("package_ready")))
        {
            Node.DependsOn = {FName(TEXT("world_ready"))};
        }
    }
    TestFalse(TEXT("Dependency cycle is rejected"), Coordinator->RegisterPlan(Cyclic));

    FSharLoadPlan Missing = MakeRequiredPlan();
    Missing.PlanId = FName(TEXT("missing_dependency_plan"));
    for (FSharLoadPlanNode& Node : Missing.Nodes)
    {
        if (Node.NodeId == FName(TEXT("world_ready")))
        {
            Node.DependsOn = {FName(TEXT("unknown_node"))};
        }
    }
    TestFalse(
        TEXT("Missing dependency node is rejected"),
        Coordinator->RegisterPlan(Missing)
    );
    return true;
}

#endif
