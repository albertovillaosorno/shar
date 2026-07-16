# Add emitter

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.AddEmitter
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Adds an emitter to a Niagara System. The new emitter will be based on the
template emitter, inheriting its configuration and modules. Returns the full
emitter topology (no input values — call GetEmitterInputValues for values).

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
Use this mutation to add a named emitter instance to a disposable or authored
SHAR NiagaraSystem from a known emitter template.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a mutable NiagaraSystem and a NiagaraEmitter template.
- Choose an emitter instance name not already present in the system.
- Use a disposable system for validation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "system": {
    "refPath": (
        "/Game/NS_SHAR_MCP_EmitterProbe_3."
        "NS_SHAR_MCP_EmitterProbe_3"
    )
},
    "templateEmitter": {
        "refPath": (
            "/Niagara/DefaultAssets/Templates/Emitters/"
            "Minimal.Minimal"
        )
    },
    "emitterName": "SHARExtra",
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two systems added `SHARExtra` beside `Minimal`. The new CPUSim emitter was
enabled, had one sprite renderer, and reported module counts 0, 1, 1, and 1
across its four stacks.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a persistent system mutation and returns a large parsed topology
  object.
- Input values are not included in the returned topology.
- Verify the emitter inventory with `GetSystemSummary`.
- Emitter templates are copied; the source asset is not modified.
- Wrong system and template asset types fail parameter translation.
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

### `emitterName`

- Required: **yes**
- Type: `string`
- Purpose:

Name for the new emitter instance

### `system`

- Required: **yes**
- Type: `object`
- Purpose:

The Niagara System to add the emitter to

### `templateEmitter`

- Required: **yes**
- Type: `object`
- Purpose:

The emitter asset to use as a template for the new emitter

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.AddEmitter \
  --arguments '
{
  "emitterName": "<value>",
  "system": {},
  "templateEmitter": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Topology of the newly added emitter with all scripts and modules walked

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
