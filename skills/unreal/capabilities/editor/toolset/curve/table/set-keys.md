# Set keys

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.curve_table.CurveTableTools.set_keys
```

Toolset:

```text
editor_toolset.toolsets.curve_table.CurveTableTools
```

## What this tool does

Replaces all keys in a row with the provided list.

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
Use this mutation to replace the complete key set of one SHAR CurveTable row
with a reviewed deterministic sequence.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a mutable CurveTable and existing row.
- Capture the current key list for rollback.
- Supply every key that should remain after replacement.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "curve_table": {
    "refPath": (
        "/Game/CT_SHAR_MCP_MutationProbe_2."
        "CT_SHAR_MCP_MutationProbe_2"
    )
},
    "row_name": "Speed",
    "keys": [
        {"time": 4.0, "value": 40.0},
        {"time": 0.0, "value": 0.0},
        {"time": 2.5, "value": 25.0},
    ],
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles replaced all prior Speed keys. Readback contained only times 0, 2.5,
and 4 in sorted order, and the mutation returned Boolean true.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This mutation replaces the entire row rather than appending.
- Input order is normalized to numeric time order on readback.
- Integral numbers may serialize without decimal points.
- Re-read all keys before accepting the mutation.
- A StaticMesh argument fails CurveTable type validation.
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
shar-unreal-mcp describe editor_toolset.toolsets.curve_table.CurveTableTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `curve_table`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `keys`

- Required: **yes**
- Type: `array<object>`
- Purpose:

The keys to set in the row.

### `row_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the row to modify.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.curve_table.CurveTableTools \
  editor_toolset.toolsets.curve_table.CurveTableTools.set_keys \
  --arguments '
{
  "curve_table": {},
  "keys": [],
  "row_name": "<value>"
}
'
```

## Expected output

True if all keys were set successfully.

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
