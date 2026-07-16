# Verification and recovery

Read [`../index.md`](../index.md) and the invoked per-tool skill before using
this workflow.

## Goal

Determine whether the requested editor outcome actually occurred and recover
safely when transport success, native output, persistent state, and observable
editor behavior do not agree.

## Verification is a separate operation

The mutation response proves only what the native tool reported. Final evidence
must come from a separate source whenever one exists.

Use the strongest available evidence in this order:

1. a separate native `get`, `list`, `find`, `inspect`, or validation tool;
1. structured output containing resulting identities and state;
1. deterministic compilation, map check, or automation test output;
1. editor log evidence tied to the operation;
1. visible editor inspection when no machine-readable read exists.

Do not use the mutation response as its own only verification source.

## Verification plan

Define before mutation:

- intended postcondition;
- exact target identity or set;
- expected value, count, state, or absence;
- verification tool and arguments;
- persistence requirement;
- maximum time for asynchronous completion;
- recovery or inverse operation.

## Verify identity and scope

Confirm:

- result belongs to the intended project and editor process;
- returned identities match approved targets;
- requested and completed counts agree;
- no unrelated target changed;
- persistent changes survived required save or compile;
- transient editor state did not masquerade as persisted state.

## Verification patterns

### Create

Verify the new object or asset exists, has the expected class and identity, and
survives required save or registry refresh.

### Update

Read the exact property or semantic state through a separate capability and
compare native values.

### Delete or remove

Confirm the exact target is absent and unrelated neighbors remain present.

### Graph mutation

Inspect graph ownership, node identities, pins, links, and compilation state.

### Sequencer or Control Rig mutation

Inspect binding, track, section, channel, key, frame, control, or hierarchy
state
at the exact requested scope.

### Plugin or Game Feature transition

Read final state and inspect logs for deferred activation, dependency, or unload
failure.

### Test or processing operation

Read final status and results, including failures, skipped items, and requested
versus executed counts.

## Persistence verification

Determine whether completion requires:

- explicit asset or package save;
- Blueprint, Material, Niagara, PCG, sequence, or Control Rig compilation;
- editor refresh or reload;
- Asset Registry update;
- plugin or Game Feature state confirmation;
- repository validation.

A correct in-memory state is incomplete when the requested outcome is
persistent.

## Record reusable project evidence

After successful independent verification, update only relevant protected fields
in the per-tool skill. Record reproducible prerequisites, arguments,
verification
steps, and caveats.

Keep `[TODO]` or `[FILL_ME]` when evidence remains incomplete, ambiguous, or not
reproduced for the current Unreal version.

## Ambiguous outcomes

An outcome is ambiguous when:

- the terminal timed out while the editor may still be processing;
- `isError` is false but the postcondition is absent;
- output reports partial item failure;
- the editor log reports a later asynchronous error;
- a fresh read conflicts with visible editor state;
- a save or compile result is unknown;
- the connection closed before final response delivery.

## Ambiguity procedure

1. Stop dependent operations.
1. Do not retry the mutation.
1. Open a fresh MCP session.
1. Inspect current target state.
1. Query status or results when available.
1. Inspect relevant editor logs.
1. Compare state with captured pre-state and intended postcondition.
1. Classify completed, failed, partial, or still unknown.

Do not select the most convenient classification.

## Failure classification

### Validation failure before mutation

Correct arguments against the live schema. Reconfirm scope before retrying.

### Execution failure before persistent change

Inspect logs and prerequisites. Retry only when no mutation occurred.

### Partial mutation

Record completed, failed, and unattempted targets. Recover item by item.

### Persistence failure

The editor state changed but save, compile, activation, or registration failed.
Repair persistence without blindly repeating the mutation.

### Verification-only failure

The mutation may be correct, but the evidence source is unavailable or stale.
Use another independent verification route.

### Unexpected target change

Stop immediately. Capture all affected identities and use the narrowest reviewed
repair or inverse operation.

## Recovery procedure

1. Preserve call, arguments, output, timestamps, and logs.
1. Read current state with a fresh session.
1. Compare against pre-state and intended postcondition.
1. Determine whether duplicate execution is possible.
1. Choose a reviewed inverse or repair tool.
1. Apply the smallest repair scope.
1. Verify repaired state independently.
1. Run required compilation, save, map check, tests, or repository validation.
1. Record unresolved uncertainty explicitly.

## Rollback principles

- Prefer restoring captured values over broad reset commands.
- Prefer one-target repairs over recursive cleanup.
- Do not delete unknown assets or editor state.
- Do not assume undo history exists or remains valid.
- Do not modify Epic plugin source to bypass a native failure.
- Do not weaken protocol or schema validation during recovery.

## Recovery evidence

Retain:

- original target set;
- pre-state;
- mutation result;
- observed current state;
- failure classification;
- repair operation;
- post-repair verification;
- remaining unknowns.

## Promotion to manual guidance

Record a caveat only when reproduced. Include:

- triggering state;
- Unreal or plugin version;
- observable failure;
- safe detection method;
- reviewed recovery or stop condition.

After promoting evidence, update `manual-review-revision` only when every
populated protected field in that skill has been rechecked against the printed
current revision. Otherwise leave the skill marked **Review required**.

Do not record speculative explanations as project caveats.

## Stop conditions

Stop and escalate when recovery would require:

- modifying Epic's native plugin source;
- disabling transport, protocol, schema, or safety validation;
- deleting unknown assets or broad editor state;
- enabling an outbound MCP client not present in the current project
  configuration;
- guessing whether an asynchronous mutation completed;
- operating beyond approved target scope.
