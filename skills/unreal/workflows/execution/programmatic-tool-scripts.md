# Programmatic tool scripts

Read the [central Unreal MCP index](../../index.md), the
[workflow map](../README.md), the programmatic execution capability,
and every per-tool skill referenced by the script.

## Goal

Use native programmatic execution only when a bounded script is more
deterministic and reviewable than a sequence of individual terminal calls.

## When this workflow applies

Use this workflow for a tool that executes Python or another native editor-side
script capable of calling multiple Toolset Registry capabilities.

Do not choose programmatic execution merely because it is shorter to type.
Prefer
individual calls when they provide clearer state boundaries, per-step
verification, or simpler recovery.

## Admission criteria

A script is justified only when all conditions hold:

- the target set is explicit and bounded;
- the same operation would otherwise require repetitive deterministic calls;
- every called native tool has been selected from the central index;
- every input and output schema has been refreshed live;
- the execution environment and allowed modules were discovered;
- side effects and rollback are understood;
- the script can return structured JSON-compatible evidence.

## Required environment discovery

Before writing a script:

1. Open the exact programmatic execution skill.
1. Invoke the environment-discovery capability named by its live documentation.
1. Record the allowed modules, helper objects, execution limits, and required
   entry-point signature.
1. Stop when the returned environment differs from the generated skill.
1. Do not import filesystem, process, network, or operating-system modules
   unless
   the live environment explicitly permits and the task requires them.

## Script design

A safe script should:

- define the exact required entry point, normally `run()`;
- use a literal bounded target list or a narrow validated query;
- validate required preconditions before the first mutation;
- call native tools through the supported programmatic interface;
- serialize mutations in deterministic order;
- capture a per-item result;
- stop or classify failures explicitly;
- return a JSON-compatible dictionary containing counts and identities.

Avoid hidden global state, nondeterministic iteration, sleeps used as readiness
checks, broad exception swallowing, and unbounded editor searches.

## Change boundary

Treat the script as the union of all operations it can perform. The declared
scope must cover:

- every mutation class;
- every target family;
- maximum target count;
- save, compile, activate, delete, or overwrite effects;
- recovery and rollback behavior.

A read-only wrapper around mutating tools is still mutating.

## Dry-run strategy

When the native environment supports it, separate discovery from mutation:

1. Read and return the exact target set.
1. Review identities and counts.
1. Freeze the reviewed target set.
1. Execute the mutation in a new bounded call.
1. Verify each resulting state independently.

When no dry-run mode exists, perform the smallest representative operation first
and verify it before expanding the batch.

## Invocation preparation

Before calling the programmatic executor:

- run `doctor`;
- refresh every participating toolset schema;
- verify no placeholder values remain;
- set an appropriate bounded timeout;
- capture pre-state evidence;
- confirm no overlapping terminal process is mutating the editor;
- decide how an ambiguous timeout will be investigated.

## Result contract

Return structured evidence such as:

```json
{
  "requested": 3,
  "completed": 3,
  "failed": 0,
  "targets": [
    "/Game/Example/A",
    "/Game/Example/B",
    "/Game/Example/C"
  ]
}
```

Do not return only free-form log text when counts and identities are available.
The script response is evidence of execution, not final proof of editor state.

## Failure handling

Classify failures as:

- environment discovery failure;
- schema or argument validation failure;
- precondition failure before mutation;
- partial item failure;
- script exception after partial mutation;
- timeout with unknown completion state;
- verification-only failure.

After any partial or ambiguous failure:

1. Stop dependent operations.
1. Read current editor state with fresh native queries.
1. Do not rerun the complete script until duplicate effects are ruled out.
1. Apply a reviewed repair or inverse operation item by item.
1. Record the failed target and evidence.

## Verification

Use separate read or validation tools to confirm:

- the exact intended targets changed;
- no extra target changed;
- requested and completed counts agree;
- saves, compilation, activation, or registration completed;
- the editor log contains no deferred failure.

## Manual guidance promotion

After a script pattern is reproduced successfully, record only reusable facts in
the protected fields of the participating per-tool skills. Do not paste the full
programmatic script into unrelated skills. Keep script-specific orchestration in
an owning technical document or test fixture when it becomes durable behavior.

## Stop conditions

Do not execute when:

- environment discovery was skipped;
- any participating tool is unindexed or stale;
- the target set is unbounded;
- the script would bypass normal scope declaration or verification;
- recovery depends on deleting unknown state;
- the script requires modules or access absent from environment discovery;
- multiple editor mutation processes would overlap.
