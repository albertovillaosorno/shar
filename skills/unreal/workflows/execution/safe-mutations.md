# Safe mutations

Read the [central Unreal MCP index](../../index.md), the
[workflow map](../README.md), the exact per-tool skill, and
[schema and arguments](../planning/schema-and-arguments.md) before any
persistent or state-removing operation.

## Goal

Apply one approved editor mutation with bounded scope, deterministic ordering,
independent verification, and a defined recovery path.

## Mutation classes

Treat a tool as mutating when it can change any of the following:

- assets, packages, graphs, nodes, pins, tracks, sections, channels, or keys;
- actors, components, transforms, tags, hierarchy, or world state;
- project configuration, plugin state, or Game Feature state;
- compilation, save, activation, registration, or import state;
- editor selection or transient state when later operations depend on it.

A tool name beginning with `get` or `validate` does not override actual side
effects described by the live interface.

## Change boundary

Before invocation, the declared change scope must identify:

- mutation class;
- exact target or bounded target set;
- intended new value or resulting state;
- whether save, compile, activate, delete, overwrite, or import occurs;
- maximum item count;
- expected verification;
- recovery or inverse operation.

Do not expand target scope from one reproduced example.

## Pre-state capture

Use independent read tools to capture applicable:

- target existence and identity;
- current value, transform, parent, binding, tag, or configuration;
- dependency and dependent relationships;
- current compile, save, activation, or registration state;
- target count;
- value needed for rollback.

Record enough pre-state to distinguish no-op, duplicate execution, and partial
mutation.

## Mutation procedure

1. Complete editor readiness.
1. Select the narrowest mutating capability.
1. Refresh the live schema.
1. Confirm protected manual prerequisites and caveats.
1. Build validated arguments.
1. Capture pre-state evidence.
1. Confirm the declared scope matches exact targets and effects.
1. Define verification and recovery before invocation.
1. Invoke exactly once.
1. Inspect JSON-RPC, native `isError`, and structured output.
1. Verify the postcondition through an independent read or validation tool.
1. Perform required save, compile, reload, or repository validation.

## Scope control

Prefer:

- one explicit asset or object identity;
- one property or semantic operation;
- one graph, track, section, hierarchy, or plugin;
- explicit target arrays with bounded counts;
- native batch tools only when they return per-item outcomes.

Reject omitted fields that can mean “all,” wildcard searches, broad class scans,
or hidden recursive behavior unless explicitly approved.

## Ordering

The translator serializes calls. Preserve a deterministic sequence:

1. prerequisite reads;
1. minimal mutation;
1. required compile or save;
1. independent verification;
1. dependent mutation only after verification succeeds.

Do not launch overlapping mutation processes to increase throughput.

## No-op and duplicate handling

Before mutation, determine whether the desired state already exists. Prefer a
no-op outcome over rewriting equivalent state when the native tool supports it.

After timeout or connection loss, do not blindly retry. First read current state
to determine whether the original mutation completed.

## Output review

Inspect:

- native error state;
- target identities;
- requested and completed counts;
- per-item outcomes;
- returned new values or states;
- warnings and deferred work;
- compilation, save, or activation status.

Transport success does not prove persistent state changed.

## Postcondition examples

### Asset creation

Confirm the asset exists at the expected native path, has the expected class,
and survives required save or registry refresh.

### Property update

Read the property through a separate capability and compare the native value,
not only formatted text.

### Graph or node mutation

Inspect node identity, pins, links, graph ownership, and compilation state.

### Sequencer or Control Rig mutation

Inspect the exact binding, track, section, channel, key, frame, or control
state.

### Plugin or Game Feature mutation

Read the resulting state and inspect logs for deferred activation failures.

### Delete or remove

Confirm the exact target is absent and no unrelated target was removed.

## Save and compilation

Determine whether the native tool:

- saves automatically;
- changes only in-memory editor state;
- requires an explicit save tool;
- requires Blueprint, Material, Niagara, PCG, sequence, or asset compilation;
- requires a refresh or reload before reads become current.

Do not report completion until the required persistence step and validation have
succeeded.

## Failure classification

Classify failure as:

- rejected before mutation;
- failed before mutation after validation;
- partially completed;
- completed but failed to save or compile;
- timed out with unknown state;
- completed but verification failed;
- changed an unexpected target.

The recovery response depends on this classification.

## Recovery

1. Stop dependent operations.
1. Read current state through a fresh session.
1. Compare with captured pre-state and intended postcondition.
1. Rule out duplicate execution before retrying.
1. Apply a reviewed inverse or repair operation when available.
1. Verify the repaired state independently.
1. Run project and repository validation when files changed.
1. Record unresolved ambiguity explicitly.

Do not delete unknown assets or reset broad editor state to simplify recovery.

## Promotion to manual guidance

After a reproduced successful mutation, update only protected fields with:

- SHAR-specific use case;
- exact prerequisites;
- minimal validated JSON example;
- independent verification step;
- reproduced caveats.

Do not place blanket authorization inside a manual field.

## Stop conditions

Stop before or during mutation when:

- declared scope is missing or broader effects are discovered;
- target identity or count is uncertain;
- pre-state cannot be captured;
- no independent postcondition check exists;
- another process is mutating overlapping editor state;
- timeout state is ambiguous;
- recovery requires deletion or reset beyond the captured target set;
- live schema conflicts with generated or manual guidance.
