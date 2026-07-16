# Add renderer

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.AddRenderer
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Adds a renderer to an emitter. Creates a new renderer of the specified type and
adds it to the emitter's renderer list. Returns an FNiagaraExt_RendererRef with
the new renderer's Index and RendererClass, usable directly with
SetRendererData / GetRendererData without a follow-up topology call.

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
Use this mutation to add a renderer implementation to a SHAR Niagara emitter
before configuring sprite, mesh, ribbon, light, or component presentation.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a live emitter location in a mutable NiagaraSystem.
- Choose a concrete NiagaraRendererProperties class.
- Inspect the emitter topology before and after insertion.
- Use a disposable system for validation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "newRendererLocation": {
    "system": {
    "refPath": (
        "/Game/NS_SHAR_MCP_RendererProbe_3."
        "NS_SHAR_MCP_RendererProbe_3"
    )
},
    "emitterName": "SHARExtra",
    "scriptName": "",
    "moduleName": "",
    "rendererIndex": -1,
    "inputNameStack": [],
},
    "rendererClass": {
        "refPath": "/Script/Niagara.NiagaraSpriteRendererProperties"
    },
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two systems added a second sprite renderer to `SHARExtra`. The returned index
was 1, and the emitter topology increased from one renderer to two.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a persistent emitter mutation.
- The return contains only `rendererIndex` and `rendererClass`; it is not a
  complete stack reference.
- Merge the returned index with the original system and emitter location before
  Get, Set, or Remove calls.
- A StaticMesh system fails as an invalid NiagaraSystem.
- A StaticMesh renderer class fails as an invalid Class.
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

### `newRendererLocation`

- Required: **yes**
- Type: `object`
- Purpose:

Reference specifying which emitter to add the renderer to

### `rendererClass`

- Required: **yes**
- Type: `object`
- Purpose:

The class of renderer to create (e.g., UNiagaraSpriteRendererProperties)

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.AddRenderer \
  --arguments '
{
  "newRendererLocation": {},
  "rendererClass": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Renderer reference (Index + RendererClass) for the newly added renderer.

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
