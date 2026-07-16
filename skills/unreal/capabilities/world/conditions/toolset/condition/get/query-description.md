# Get query description

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
WorldConditionsToolset.WorldConditionTools.GetQueryDescription
```

Toolset:

```text
WorldConditionsToolset.WorldConditionTools
```

## What this tool does

Returns a human-readable description of a world condition query.

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
Use this tool to produce a bounded diagnostic summary of a World Condition
query before SHAR reviews Smart Object availability, StateTree transitions, or
other conditional gameplay interactions. Keep the original query definition as
the behavioral authority.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Obtain the complete query definition from live editor data or a safely
  equivalent reflected fixture.
- Every editable condition must contain a loaded reflected condition struct.
- Preserve operator, depth, inversion, schema, and shared-definition data
  separately because the description is not lossless.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "queryDefinition": {
    "sharedDefinition": {},
    "schemaClass": {
      "refPath": "/Script/WorldConditionsTestSuite.WorldConditionTestSchema"
    },
    "editableConditions": [
      {
        "expressionDepth": 0,
        "operator": "Copy",
        "bInvert": false,
        "condition": {
          "_structType": "/Script/WorldConditionsTestSuite.WorldConditionTest"
        }
      },
      {
        "expressionDepth": 0,
        "operator": "And",
        "bInvert": false,
        "condition": {
          "_structType": "/Script/WorldConditionsTestSuite.WorldConditionTest"
        }
      }
    ]
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive calls returned
`IF [Value == 0] AND [Value == 0]`. A grouped `Or` fixture produced
`IF ([Value == 0] OR [Value == 0])`, and an empty definition returned `Empty`.
Changing `bInvert` did not change the rendered description in the verified
editor session.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The output is diagnostic text, not a lossless query serialization and not a
  truth-value evaluation.
- The verified renderer represented operator order and expression depth but did
  not display `bInvert`; never infer complete query semantics from the text.
- An empty definition returns `Empty` rather than an error.
- Reflected condition descriptions inherit the limitations of
  `GetConditionDescription`, including empty text for some invalid structs.
- Description wording can change across engine or plugin versions and must not
  be used as a stable query identity.
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
shar-unreal-mcp describe WorldConditionsToolset.WorldConditionTools
```

1. Confirm every required input against the current schema.

## Inputs

### `queryDefinition`

- Required: **yes**
- Type: `object`
- Purpose:

The query definition to describe.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  WorldConditionsToolset.WorldConditionTools \
  WorldConditionsToolset.WorldConditionTools.GetQueryDescription \
  --arguments '
{
  "queryDefinition": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

Text description of all conditions in the query.

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
