# Set properties

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.object.ObjectTools.set_properties
```

Toolset:

```text
editor_toolset.toolsets.object.ObjectTools
```

## What this tool does

Sets the values of properties on an object.

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
Use this generic mutation only when a more specific SHAR tool does not expose
the required reflected UObject property update.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a disposable or explicitly task-owned UObject.
- Discover exact reflected property names first.
- Read the current values for rollback and verification.
- Encode `values` as a JSON-formatted string, not a nested request object.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "instance": {
    "refPath": (
      "/Game/NS_SHAR_MCP_ObjectPropertyProbe_4."
      "NS_SHAR_MCP_ObjectPropertyProbe_4"
    )
  },
  "values": (
    "{\"bSupportLargeWorldCoordinates\":false,"
    "\"WarmupTime\":2.5,\"WarmupTickDelta\":0.125}"
  )
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles returned true. Large-world support changed to false, tick delta
became 0.125, and Niagara normalized requested warmup time 2.5 to 2.375.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a persistent UObject mutation and returns Boolean true on accepted
  conversion.
- A true return does not prove exact value preservation; Niagara quantized
  WarmupTime.
- Invalid numeric strings and object-shaped numeric values returned true but
  stored zero.
- Boolean string `"false"` converted to false.
- Unknown property names raise and list properties that could not be set.
- An empty JSON object returns true as a no-op.
- Always re-read every changed property and prefer a domain-specific tool when
  available.
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
shar-unreal-mcp describe editor_toolset.toolsets.object.ObjectTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `instance`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `values`

- Required: **yes**
- Type: `string`
- Purpose:

A JSON formatted string of the properties to set and their values. For
instanced sub-object properties, pass a class path as the instance member.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.object.ObjectTools \
  editor_toolset.toolsets.object.ObjectTools.set_properties \
  --arguments '
{
  "instance": {},
  "values": "<value>"
}
'
```

## Expected output

True if the property was set. False otherwise.

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
