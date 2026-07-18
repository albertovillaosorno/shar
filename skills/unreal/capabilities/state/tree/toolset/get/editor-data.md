# Get editor data

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
state_tree_toolset.toolsets.state_tree.StateTreeTools.get_editor_data
```

Toolset:

```text
state_tree_toolset.toolsets.state_tree.StateTreeTools
```

## What this tool does

Returns the editor data for a StateTree asset.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Use the returned structured evidence directly, but still confirm the live
schema because names do not prove side effects.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to resolve the StateTreeEditorData object before any editor-state,
node, task, evaluator, or transition inspection.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a valid StateTree asset authored or initialized with editor data.
- Do not assume a newly constructed StateTree contains StateTreeEditorData.
- Treat missing editor data as a hard boundary for every editor-only reader.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "state_tree": {
    "refPath": "/Game/AI/ST_Example.ST_Example"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`DataAssetTools.create` successfully created a disposable StateTree with the
exact `/Script/StateTreeModule.StateTree` class. This reader then failed closed
with `StateTree has no editor data`, and the fixture was deleted.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- A valid StateTree UObject can still lack editor data.
- Runtime-compiled data is not a substitute for StateTreeEditorData.
- Do not call dependent editor readers after this prerequisite fails.
- Use an editor-authored or correctly factory-initialized StateTree for positive
  inspection.
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
shar-unreal-mcp describe state_tree_toolset.toolsets.state_tree.StateTreeTools
```

1. Confirm every required input against the current schema.

## Inputs

### `state_tree`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  state_tree_toolset.toolsets.state_tree.StateTreeTools \
  state_tree_toolset.toolsets.state_tree.StateTreeTools.get_editor_data \
  --arguments '
{
  "state_tree": {}
}
'
```

## Expected output

The editor data object.

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Verification

- Check the returned `isError` state and structured output.
- Compare returned identities and counts with the requested scope.
- Treat transport success as insufficient evidence by itself.
- Confirm the response belongs to the open editor project.
- Reject evidence derived from stale discovery state.

## Common failure modes

- The skill may be stale; run `describe` and regenerate the catalog.
- A required editor object or asset may not be loaded.
- Placeholder values are not valid project identities.
- Native validation may reject semantically invalid JSON values.
