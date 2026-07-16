# Add key

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.curve_table.CurveTableTools.add_key
```

Toolset:

```text
editor_toolset.toolsets.curve_table.CurveTableTools
```

## What this tool does

Adds a key to a row.

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
Use this mutation to append one time/value sample to a SHAR CurveTable row while
retaining the row’s existing samples.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a mutable CurveTable and an existing row name.
- Inspect current key times before insertion.
- Decide explicitly whether duplicate times are acceptable.
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
    "key": {"time": 2.0, "value": 20.0},
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles added times 2 and 1 and read them back sorted as 1 then 2. Adding
another key at time 1 succeeded and produced values 15 then 10 at that same
time.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This mutation returns Boolean true on success.
- Keys are read back in numeric time order.
- Duplicate times are allowed; a newer duplicate appeared before the older key
  at the same time.
- Use SetKeys when replacement or de-duplication is required.
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

### `key`

- Required: **yes**
- Type: `object`
- Purpose:

SimpleCurveKey

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
  editor_toolset.toolsets.curve_table.CurveTableTools.add_key \
  --arguments '
{
  "curve_table": {},
  "key": {},
  "row_name": "<value>"
}
'
```

## Expected output

True if the key was added successfully.

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
