# Read-only operations

Read [`../index.md`](../index.md), then open the exact read-oriented per-tool
skill.

## Goal

Collect current editor evidence without changing persistent project state and
without treating a read-like name as an absolute safety guarantee.

## Read posture is inferred, not declared

Generated skills classify common prefixes such as `get`, `list`, `find`,
`discover`, `inspect`, `search`, `query`, `check`, or `validate` as expected
read-only. This is a conservative routing aid, not native MCP safety metadata.

Inspect the live description for possible side effects such as:

- lazy discovery or registry refresh;
- loading assets or worlds;
- editor selection changes;
- cache population;
- test-worker initialization;
- status polling that advances native state.

## Preparation

1. Complete editor readiness.
1. Select the narrowest read capability.
1. Refresh its live schema.
1. Identify the exact evidence needed.
1. Bound filters, result limits, and pagination.
1. Record the editor context that makes the result meaningful.

## Query procedure

1. Invoke once with the narrowest validated arguments.
1. Check JSON-RPC and native `isError` state.
1. Inspect structured output before prose or logs.
1. Validate returned identities and counts.
1. Follow pagination until complete or until the approved evidence is sufficient.
1. Record whether the result is a snapshot, cache, or live editor query.

## Scope control

Large worlds, registries, logs, graphs, test catalogs, and hierarchies can create
unreviewable output. Prefer:

- exact asset or object identity;
- narrow name, class, tag, or path filters;
- one graph, track, hierarchy, or component subtree;
- bounded limits;
- server-provided cursors;
- explicit status or result identifiers.

Do not repeatedly request a complete catalog when a focused query exists.

## Pagination

When a result includes a cursor:

- preserve server ordering;
- stop when no next cursor is returned;
- reject repeated cursors as protocol failure;
- deduplicate only when native identity proves duplication;
- retain page counts when completeness matters.

Do not infer completion from a short page unless the schema documents that rule.

## Evidence quality

Useful evidence is:

- current for the intended project and editor instance;
- traceable to exact tool and arguments;
- complete enough for the decision;
- bounded enough to review;
- reproducible when used as regression evidence;
- independent from the mutation it verifies.

## Result verification

Verify applicable:

- returned project, world, asset, object, graph, or plugin identity;
- item count versus requested scope;
- absence of unexpected targets;
- status timestamp or lifecycle state;
- live schema interpretation;
- editor log consistency.

A non-error response containing the wrong project or target is invalid evidence.

## Comparing reads

When two reads disagree:

1. Confirm they query the same editor instance.
1. Refresh both schemas.
1. Identify cache or loading differences.
1. Re-run the more direct identity-based read.
1. Inspect the editor log for deferred updates.
1. Record ambiguity instead of selecting the convenient result.

## Reads before mutations

Use read operations to capture:

- target existence;
- current property or configuration;
- target count;
- dependencies and dependents;
- compilation or activation state;
- a rollback value;
- the postcondition verification route.

Do not use a mutation as discovery when an equivalent read exists.

## Reads after mutations

Use a fresh session when practical. Confirm the postcondition with a capability
that does not merely echo the mutation response.

For saved assets or project configuration, also confirm persistence through the
appropriate compile, save, reload, validation, or repository check.

## Recording manual guidance

A read result may support protected fields when it establishes a reproducible
prerequisite, verification step, or caveat. Record the tool and observable fact,
not a volatile full output dump.

## Common failure modes

- Unbounded result size.
- Querying the wrong open editor project.
- Treating cached discovery as current state.
- Stopping pagination early.
- Comparing labels instead of native identities.
- Treating transport success as evidence completeness.
- Assuming a read-like verb has no transient side effects.

## Stop conditions

Stop when:

- the intended editor or project cannot be confirmed;
- the query scope cannot be bounded;
- pagination repeats or becomes ambiguous;
- output lacks identities needed for review;
- evidence depends on stale discovery state;
- a read unexpectedly mutates persistent state;
- the result conflicts with a stronger direct evidence source.
