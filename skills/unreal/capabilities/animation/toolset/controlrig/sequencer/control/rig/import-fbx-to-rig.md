# Import fbx to rig

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.import_fbx_to_rig
```

Toolset:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

## What this tool does

Import an FBX file onto a Control Rig track.

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
Use this tool for a Control Rig FBX import only in a disposable sequence with
independent channel-level verification.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a valid FBX whose hierarchy and mapping match the target rig.
- Resolve the exact Control Rig track, section, and intended controls.
- Snapshot all target channels and keys before import and define rollback.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "sequence": {
    "refPath": "/Game/Cinematics/LS_Example.LS_Example"
  },
  "control_rig_asset_path": "/Game/ControlRigs/CR_Example.CR_Example",
  "import_file_path": "D:/SHAR-Validation/control-rig-import.fbx",
  "selected_controls": [
    "Root_CTRL"
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
With selected controls, UE 5.8 reported that the import settings object has no
`import_onto_selected_controls` attribute. With an empty selection the wrapper
returned `true`, but nine independently read `Root_CTRL` channels remained
empty, so the result was rejected.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The selected-controls branch targets a settings field absent from UE 5.8.
- An empty control list can return `true` without importing channel data.
- Verify controls, channels, key counts, and values after every import.
- Remove the disposable FBX and sequence after validation.
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
shar-unreal-mcp describe animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `control_rig_asset_path`

- Required: **yes**
- Type: `string`
- Purpose:

Path to identify which Control Rig.

### `import_file_path`

- Required: **yes**
- Type: `string`
- Purpose:

File system path to the FBX file to import.

### `selected_controls`

- Required: **yes**
- Type: `array<string>`
- Purpose:

Optional list of control names to import onto. If provided, only these controls
will be affected.

### `sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.import_fbx_to_rig \
  --arguments '
{
  "control_rig_asset_path": "<value>",
  "import_file_path": "<value>",
  "selected_controls": [],
  "sequence": {}
}
'
```

## Expected output

True if the import succeeded.

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
