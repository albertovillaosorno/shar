# Apply stack issue fix

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.ApplyStackIssueFix
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Applies a Fix-style stack issue fix identified by IssueId and FixId. Link-style
fixes are rejected. The fix is undoable via the editor undo stack. Applying a
fix may trigger a recompile; the result waits for that compile to complete so
post-fix state is valid.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Capture pre-state, bound the target set, and verify the resulting editor or
asset state through an independent read.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this mutation to apply a native Fix-style Niagara stack repair after SHAR
issue inspection identifies a specific current issue and fix pair.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Call GetStackIssues after compilation is settled.
- Select an issue whose fix has `style: Fix`.
- Use the returned issue and fix IDs immediately.
- Capture stack topology and issue counts before mutation.
- Use a disposable system for validation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "system": {
    "refPath": (
      "/Game/NS_SHAR_MCP_StackIssueProbe_6."
      "NS_SHAR_MCP_StackIssueProbe_6"
    )
  },
  "issueId": "d7c39a94ec04737bebfbdaf2809a395d",
  "fixId": "c482ba4e3e10d3406e09fcb3d0e65676"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles applied the `Add new dependency module SolveForcesAndVelocity` fix to
a GravityForce dependency error. After compilation settled, the solver module
was appended and the error count changed from one to zero.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a persistent stack mutation.
- In both cycles the fix was applied, but the call returned `Cannot collect
  stack issues while a compile is in flight` during its post-fix verification.
- Treat that error as ambiguous: poll GetSystemCompileState, then verify
  topology and GetStackIssues before retrying.
- Reusing the old issue ID after repair raises `No stack issue with IssueId`.
- Issue IDs changed across rebuilt fixtures; rediscover them immediately before
  invocation.
- Only a Fix-style action was validated. The live description says Link-style
  actions are rejected.
- A StaticMesh system argument fails NiagaraSystem type validation.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b
<!-- END MANUAL FIELD: manual-review-revision -->

<!-- markdownlint-disable-next-line MD013 -->
- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Current**

## Before invocation

1. Run `shar-unreal-mcp doctor` and require `ready: true`.
1. Select this skill from the central index, not from memory.
1. Refresh the live schema:

```text
shar-unreal-mcp describe NiagaraToolsets.NiagaraToolset_System
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `fixId`

- Required: **yes**
- Type: `string`
- Purpose:

FixId from a prior FNiagaraExt_StackIssueFix.

### `issueId`

- Required: **yes**
- Type: `string`
- Purpose:

IssueId from a prior FNiagaraExt_StackIssue.

### `system`

- Required: **yes**
- Type: `object`
- Purpose:

The Niagara System to apply the fix to.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.ApplyStackIssueFix \
  --arguments '
{
  "fixId": "<value>",
  "issueId": "<value>",
  "system": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Async result carrying the fix outcome and a post-fix stack-issues snapshot.

## Verification

- Check the returned `isError` state and structured output.
- Compare returned identities and counts with the requested scope.
- Treat transport success as insufficient evidence by itself.
- Verify changed state through a separate read or inspection.
- Use another capability to confirm the postcondition.
- Inspect editor logs when state is not directly observable.

## Common failure modes

- The skill may be stale; run `describe` and regenerate the catalog.
- A required editor object or asset may not be loaded.
- Placeholder values are not valid project identities.
- Native validation may reject semantically invalid JSON values.
