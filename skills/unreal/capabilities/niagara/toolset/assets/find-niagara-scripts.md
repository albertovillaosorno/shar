# Find niagara scripts

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_Assets.FindNiagaraScripts
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_Assets
```

## What this tool does

Searches for UNiagaraScript assets matching the given filters.

Reads filterable metadata from asset registry tags only - no LoadObject
required.

Tags reflect the exposed (published) version of versioned scripts; this
function does not need a version filter and there is no way to discover non-
exposed versions through the asset registry.

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
Use this tool to locate Niagara modules for SHAR effects work without loading
the assets. The verified query resolves the engine initialization module before
a digest review or a controlled stack operation.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR editor and Niagara Asset Registry data must be ready.
- Supply all seven fields required by the live schema.
- Choose usage, visibility, deprecation, folder, and bitmask filters explicitly.
- Use the returned object path with `GetNiagaraScriptDigest` before mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "bIncludeDeprecated": true,
  "bRecursive": false,
  "folderPath": "/Niagara/Modules/Spawn/Initialization",
  "moduleUsageBitmask": 0,
  "name": "InitializeParticle",
  "usages": [
    "Module"
  ],
  "visibilities": [
    "Hidden"
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Repeated calls returned exactly one `InitializeParticle` row in the requested
folder. Its digest reported a Hidden, deprecated Module with usage bitmask `90`.
AssetTools independently confirmed that the object exists, has class
`NiagaraScript`, and exposes matching registry tags. A missing name returned
`[]`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Result rows contain object paths but not the registry tag map.
- An empty `visibilities` array defaults to `Library`; it excluded this Hidden
  fixture.
- `bIncludeDeprecated: false` excluded this deprecated fixture.
- Bitmask filtering is any-match: `2` matched bitmask `90`, while `1` did not.
- Searches reflect only the exposed version recorded in the Asset Registry.
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
shar-unreal-mcp describe NiagaraToolsets.NiagaraToolset_Assets
```

1. Confirm every required input against the current schema.

## Inputs

### `bIncludeDeprecated`

- Required: **yes**
- Type: `boolean`
- Purpose:

If false (default usage), assets whose exposed version has bDeprecated=true are
excluded. Pass true only when the caller explicitly wants to surface deprecated
assets (e.g. searching for the canonical replacement of a known deprecated
module).

### `bRecursive`

- Required: **yes**
- Type: `boolean`
- Purpose:

Whether to search subfolders. Ignored when FolderPath is empty (whole-project
scan is always exhaustive).

### `folderPath`

- Required: **yes**
- Type: `string`
- Purpose:

The folder to search within. Pass an empty string to search the entire project.

### `moduleUsageBitmask`

- Required: **yes**
- Type: `integer`
- Purpose:

If non-zero, restricts results to Module scripts whose ModuleUsageBitmask
shares at least one bit with this argument (any-match). Non-Module scripts are
excluded when this is non-zero. Pass 0 to disable the bitmask gate.

### `name`

- Required: **yes**
- Type: `string`
- Purpose:

If non-empty, only return assets whose name contains this substring (case-
insensitive).

### `usages`

- Required: **yes**
- Type: `array<string>`
- Purpose:

If non-empty, only return assets whose Usage matches one of the listed values.
Empty array = all usages allowed.

### `visibilities`

- Required: **yes**
- Type: `array<string>`
- Purpose:

If non-empty, only return assets whose LibraryVisibility matches one of the
listed values. Empty array defaults to {Library} (the editor's "Exposed");
explicit override required to surface Unexposed or Hidden scripts.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_Assets \
  NiagaraToolsets.NiagaraToolset_Assets.FindNiagaraScripts \
  --arguments '
{
  "bIncludeDeprecated": false,
  "bRecursive": false,
  "folderPath": "<value>",
  "moduleUsageBitmask": 0,
  "name": "<value>",
  "usages": [],
  "visibilities": []
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

Array of AssetData entries for matching scripts. Use GetNiagaraScriptDigest to
decode tag metadata for any row.

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
