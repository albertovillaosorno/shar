# Rebind component

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.sequencer.SequencerTools.rebind_component
```

Toolset:

```text
animation_toolset.toolsets.sequencer.SequencerTools
```

## What this tool does

Rebind component bindings to a named component.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

The native identity does not establish side effects. Review the live schema and
editor context before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to redirect existing component possessable bindings to a named
component only with independent bound-object verification.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Open the target Level Sequence and obtain child component bindings from an
  actor binding.
- Confirm the target actor owns exactly one component with the requested name.
- Capture `get_bound_objects` for every component binding before mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "component_bindings": [
    {
      "bindingId": "11111111-2222-3333-4444-555555555555",
      "sequence": {
        "refPath": "/Game/Cinematics/LS_Example.LS_Example"
      }
    }
  ],
  "component_name": "CameraComponent"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A disposable camera binding resolved its original `CameraComponent`. A second
camera component named `MCP_RebindTarget56` was added, and the wrapper returned
`true`; thirty independent `get_bound_objects` polls still resolved the original
component. The call was rejected as a false positive.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The wrapper always returns `true` after delegating to the editor subsystem.
- A successful transport does not prove the component binding changed.
- Verify exact resolved object paths and restore the original component binding
  on failure.
- Component names are actor-local and can be ambiguous across different bound
  actors.
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
shar-unreal-mcp describe animation_toolset.toolsets.sequencer.SequencerTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `component_bindings`

- Required: **yes**
- Type: `array<object>`
- Purpose:

List of SequencerBindingProxy for the component bindings to rebind.

### `component_name`

- Required: **yes**
- Type: `string`
- Purpose:

Name of the target component.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.sequencer.SequencerTools \
  animation_toolset.toolsets.sequencer.SequencerTools.rebind_component \
  --arguments '
{
  "component_bindings": [],
  "component_name": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

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
