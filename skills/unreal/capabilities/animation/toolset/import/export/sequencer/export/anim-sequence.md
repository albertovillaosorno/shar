# Export anim sequence

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Execution or transient mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.import_export.SequencerImportExportTools.export_anim_sequence
```

Toolset:

```text
animation_toolset.toolsets.import_export.SequencerImportExportTools
```

## What this tool does

Export animation from a sequence binding to an AnimSequence asset.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Confirm execution scope, cancellation behavior, and expected side effects
before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to bake one Sequencer skeletal binding into a task-owned
AnimSequence asset.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a disposable sequence, world, matching skeletal binding, and writable
  task-owned destination.
- Capture file metadata or destination AnimSequence metadata before invocation
  and delete all output afterward.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "anim_sequence": {
    "refPath": "/Game/SHAR_MCP_Validation_Round50_260718/A_MCP_Round50_Export.A_MCP_Round50_Export"
  },
  "binding": {
    "bindingId": "5F6BCAB5-4277-73A7-F101-82B3FD263512",
    "sequence": {
      "refPath": "/Game/SHAR_MCP_Validation_Round50_260718/LS_MCP_Round50.LS_MCP_Round50"
    }
  },
  "create_link": true,
  "sequence": {
    "refPath": "/Game/SHAR_MCP_Validation_Round50_260718/LS_MCP_Round50.LS_MCP_Round50"
  },
  "world": {
    "refPath": "/Game/Untitled.Untitled"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The destination animation changed from 56 keys and 1.833 seconds to 151 keys
and 5 seconds, while preserving the exact tutorial skeleton.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Export mutates external files or task-owned assets and can overwrite
  existing destinations.
- Binding, world, sequence, skeleton, and destination identities must remain
  compatible for the complete operation.
- This overwrites animation data in the destination asset. Use only a
  task-owned copy and compare sample count, length, and skeleton afterward.
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
shar-unreal-mcp describe animation_toolset.toolsets.import_export.SequencerImportExportTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `anim_sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `binding`

- Required: **yes**
- Type: `object`
- Purpose:

MovieSceneBindingProxy

### `create_link`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

Whether to create a link between the sequence and the exported AnimSequence.

### `sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `world`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.import_export.SequencerImportExportTools \
  animation_toolset.toolsets.import_export.SequencerImportExportTools.export_anim_sequence \
  --arguments '
{
  "anim_sequence": {},
  "binding": {},
  "sequence": {},
  "world": {}
}
'
```

## Expected output

True if the export succeeded, False otherwise.

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
