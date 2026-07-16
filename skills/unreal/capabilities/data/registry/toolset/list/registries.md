# List registries

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
DataRegistryToolset.DataRegistryTools.ListRegistries
```

Toolset:

```text
DataRegistryToolset.DataRegistryTools
```

## What this tool does

Returns the names of all registered Data Registries.

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
Use this tool as the first Data Registry preflight before SHAR attempts to
inspect registry schemas, sources, item identities, or cached values. An empty
inventory proves that registry-dependent work must stop instead of guessing a
registry name.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- The Data Registry subsystem and any plugins that register project registries
  must be loaded.
- Omit `structFilter` for the complete inventory.
- When filtering, supply a reflected `UScriptStruct` path, not a class or object
  path.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive unfiltered calls returned an empty array. Explicit `null` and an
empty struct reference produced the same empty inventory. Supplying
`/Script/CoreUObject.Object` failed because that identity is a class rather than
the required script struct.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- An empty array is a valid result and means no Data Registry is currently
  registered in the editor session.
- Registry-dependent tools cannot be validated truthfully until this inventory
  returns a real name.
- Availability can change when plugins, project settings, or registry assets are
  loaded or reconfigured; repeat this preflight in the same session as later
  reads.
- `structFilter` checks item-struct inheritance and accepts a `UScriptStruct`
  identity only.
- Registry names are runtime subsystem identities, not asset paths or config
  filenames.
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
shar-unreal-mcp describe DataRegistryToolset.DataRegistryTools
```

1. Confirm every required input against the current schema.

## Inputs

### `structFilter`

- Required: **no**
- Type: `object`
- Purpose:

If non-null, only registries whose item struct inherits from this struct are
returned.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  DataRegistryToolset.DataRegistryTools \
  DataRegistryToolset.DataRegistryTools.ListRegistries \
  --arguments '
{}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

A list of registry names.

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
