# Export fbx from rig

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Execution or transient mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.export_fbx_from_rig
```

Toolset:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

## What this tool does

Export an FBX file from a Control Rig section.

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
Use this tool to export one Control Rig section to a review FBX only after
confirming the installed export-settings wrapper is compatible.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve a Control Rig track with at least one section in the open sequence.
- Use an approved temporary output path outside committed repository content.
- Capture the expected file format, size, and independent import check before
  export.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "sequence": {
    "refPath": "/Game/Cinematics/LS_Example.LS_Example"
  },
  "control_rig_asset_path": "/Game/ControlRigs/CR_Example.CR_Example",
  "export_file_path": "D:/SHAR-Validation/control-rig-export.fbx",
  "ascii": false
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A disposable Control Rig sequence reached export setup, but UE 5.8 reported that
`MovieSceneUserExportFBXControlRigSettings` has no `export_file_path` attribute.
No FBX was accepted as output.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The installed wrapper targets an export-settings field absent from UE 5.8.
- A returned file must be checked for existence, non-zero size, FBX format, and
  successful reimport.
- The wrapper exports the first Control Rig section; resolve multiple-section
  intent first.
- Remove disposable output after verification and never commit local evidence
  files.
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

### `ascii`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If True, export in ASCII format instead of binary.

### `control_rig_asset_path`

- Required: **yes**
- Type: `string`
- Purpose:

Path to identify which Control Rig.

### `export_file_path`

- Required: **yes**
- Type: `string`
- Purpose:

File system path for the output FBX file.

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
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.export_fbx_from_rig \
  --arguments '
{
  "control_rig_asset_path": "<value>",
  "export_file_path": "<value>",
  "sequence": {}
}
'
```

## Expected output

True if the export succeeded.

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
