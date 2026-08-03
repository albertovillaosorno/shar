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
//   - Shar load plan tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar load plan tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar load plan tests composition module.

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
    // jig-ignore-next-line: exact syntax is indivisible
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
    // jig-ignore-next-line: exact syntax is indivisible
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
