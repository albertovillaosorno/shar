# Long-running and batch operations

Read the [central Unreal MCP index](../../index.md), the
[workflow map](../README.md), and every participating per-tool skill
before using this workflow.

## Goal

Run tests, imports, compilation, processing, or repeated calls without losing
determinism, exceeding bounded execution assumptions, or hiding partial failure.

## Operations in this class

Examples include:

- automation test discovery and execution;
- large imports, exports, or asset processing;
- Blueprint, sequence, Niagara, PCG, Dataflow, or Control Rig compilation;
- repeated mutations over an approved target set;
- plugin or Game Feature transitions;
- programmatic tool scripts;
- asynchronous operations with status or cancellation tools.

## Batch admission checklist

Before choosing a batch:

- the target set is explicit and bounded;
- individual operation semantics are understood;
- declared scope covers every target and side effect;
- a native batch tool is narrower than manual looping;
- per-item or aggregate completion evidence exists;
- partial failure can be classified and recovered;
- timeout and cancellation behavior are understood.

Use sequential individual calls when a batch hides item identity or recovery.

## Preparation

1. Complete editor readiness.
1. Refresh every participating schema.
1. Capture the ordered target set and count.
1. Estimate duration and output volume.
1. Identify status, result, stop, or cancellation tools.
1. Capture pre-state for mutations.
1. Define success, partial success, and failure thresholds.
1. Choose timeout and polling strategy.
1. Confirm no overlapping editor mutation process exists.

## Serialization rule

The translator intentionally serializes native MCP operations. Do not bypass
that behavior with parallel terminal processes.

Unreal can track asynchronous requests, but shared editor state still requires a
deterministic mutation order. Parallel reads are allowed only after proving they
do not trigger loading, selection, discovery, or cache side effects.

## Native batch versus sequential calls

Prefer a native batch tool when it provides:

- explicit target input;
- deterministic target ordering;
- per-item outcomes or complete aggregate evidence;
- bounded filters;
- cancellation or status inspection;
- clear rollback semantics.

Prefer sequential calls when:

- one failure should stop later items;
- verification is required after every item;
- targets require different arguments;
- batch output does not identify failed items;
- recovery is item-specific.

## Chunking

When the approved set is large:

1. Choose a deterministic chunk size.
1. Preserve target order.
1. Record each chunk's identities.
1. Verify one representative chunk before continuing.
1. Stop on the first unexpected failure unless independent failures were
   explicitly approved.
1. Retain completed and pending sets separately.

Do not let chunking silently expand the original target set.

## Progress and polling

Treat progress notifications as informational until a final result or separate
status tool confirms completion.

Polling should:

- use a stable operation or result identity;
- avoid creating duplicate work;
- use bounded intervals;
- stop on terminal success, terminal failure, or reviewed timeout;
- record the final status response.

Do not use arbitrary sleep as proof that processing completed.

## Translator timeout cancellation

The terminal translator treats timeout as an incomplete operation, not as proof
of failure or success. When one native tool call exceeds `--timeout`, it sends
`notifications/cancelled` with the original request ID before returning the
timeout error.

After a timeout:

1. Preserve the timeout message and request scope.
1. Do not immediately retry the mutation.
1. Inspect the editor through an independent read capability.
1. Confirm whether the native tool completed, partially changed state, or
   honored cancellation.
1. Increase `--timeout` only after the operation's expected duration and
   verification path are understood.

Cancellation is best-effort at the native tool boundary. A timeout remains an
ambiguous-state event until post-state verification completes.

## Timeout handling

A timeout does not imply rollback. After timeout:

1. Stop dependent operations.
1. Open a fresh session.
1. Query operation status when available.
1. Inspect current editor and asset state.
1. Compare completed counts and identities.
1. Rule out duplicate effects before retrying.
1. Cancel only through a reviewed native stop tool.

Never blindly resubmit the complete batch.

## Cancellation

Before cancellation, determine:

- whether completed items remain committed;
- whether the operation is atomic or incremental;
- whether cancellation itself mutates state;
- how to identify unfinished targets;
- whether save, compile, or cleanup remains required.

Verify state after cancellation exactly as after a partial failure.

## Partial failure

Record:

- requested target count;
- completed targets;
- failed targets and messages;
- unattempted targets;
- current persistent state;
- whether recovery is safe per item.

Do not report a batch as failed without preserving successful item evidence, and
do not report success while any item outcome is unknown.

## Programmatic execution

Use the dedicated
[programmatic tool scripts](programmatic-tool-scripts.md) workflow when one
native script coordinates multiple tools. Programmatic execution does not bypass
serialization, schema, target-scope, or verification requirements.

## Completion evidence

Retain:

- exact ordered target set;
- operation and chunk order;
- arguments or stable argument digest;
- per-call or per-item output;
- final status;
- editor logs for warnings or failures;
- independent postcondition evidence;
- remaining pending or failed targets.

## Validation after completion

Run the applicable editor and repository checks:

- asset or graph reads;
- compilation;
- map check;
- automation tests;
- plugin or Game Feature state reads;
- canonical `validate.sh` when repository files changed.

## Stop conditions

Stop when:

- target set or count changes unexpectedly;
- status cannot distinguish running, completed, and failed;
- timeout leaves completion ambiguous;
- partial failure cannot be mapped to targets;
- cancellation semantics are unknown;
- a batch would overlap another editor mutation;
- recovery would require broad or unbounded state cleanup;
- output volume is too large to review reliably.
