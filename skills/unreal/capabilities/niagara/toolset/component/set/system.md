# Set system

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_Component.SetSystem
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_Component
```

## What this tool does

Sets the Niagara System for a component. Use this instead of setting the Asset
property directly to ensure proper initialization.

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
Use this mutation to assign or replace the NiagaraSystem used by a SHAR
NiagaraComponent while preserving component initialization semantics.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a live NiagaraComponent and a valid NiagaraSystem asset.
- Capture current `asset` and `instanceParameterOverrides` first.
- Decide explicitly whether compatible overrides should be preserved.
- Use a disposable actor or component for validation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "niagaraComponent": {
    "refPath": (
        "/Temp/Untitled_1.Untitled_1:PersistentLevel."
        "NiagaraActor_1.NiagaraComponent0"
    )
},
    "system": {
    "refPath": (
        "/Game/NS_SHAR_MCP_ComponentA_5."
        "NS_SHAR_MCP_ComponentA_5"
    )
},
    "bResetExistingOverrideParameters": True,
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles assigned system A to an empty NiagaraComponent, switched to
compatible system B with reset false, and switched back to A with reset true.
Every successful call returned JSON null.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Component and spawned actor paths are transient world identities.
- Assigning the same system again raises `New Niagara System is the same as the
  existing System`.
- In this fixture, compatible `User.SHARSpeed` overrides survived both reset
  false and reset true system switches.
- Do not assume the reset flag cleared overrides; re-read
  `instanceParameterOverrides`.
- A StaticMesh component argument fails NiagaraComponent type validation.
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
shar-unreal-mcp describe NiagaraToolsets.NiagaraToolset_Component
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `bResetExistingOverrideParameters`

- Required: **yes**
- Type: `boolean`
- Purpose:

If true, reset all user variables to system defaults; if false, preserve
matching overrides

### `niagaraComponent`

- Required: **yes**
- Type: `object`
- Purpose:

The Niagara component to set the system on

### `system`

- Required: **yes**
- Type: `object`
- Purpose:

The Niagara system asset to assign to the component

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_Component \
  NiagaraToolsets.NiagaraToolset_Component.SetSystem \
  --arguments '
{
  "bResetExistingOverrideParameters": false,
  "niagaraComponent": {},
  "system": {}
}
'
```

## Expected output

The live interface does not declare a structured output schema.

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
