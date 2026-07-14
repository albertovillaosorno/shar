# Get niagara script digest

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_Assets.GetNiagaraScriptDigest
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_Assets
```

## What this tool does

Returns the decoded asset-registry tag metadata for a Niagara script asset.

Looks up the asset by object path in the asset registry and reads its tags; no
LoadObject is performed. Returned fields reflect the exposed (published)
version when the script uses FVersionedNiagaraScriptData versioning - the
registry never carries non-exposed-version metadata.

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
Use this tool to inspect a discovered Niagara script's usage, visibility,
deprecation, category, description, and compatibility bitmask before SHAR adds
or audits a module in an effects stack.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain a full Niagara script object path from registry discovery.
- The path must identify a `NiagaraScript`, not only its package or folder.
- The Asset Registry must contain the script's exposed-version tags.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "objectPath": "/Niagara/Modules/Spawn/Initialization/InitializeParticle.InitializeParticle"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Repeated calls returned identical decoded metadata: asset name
`InitializeParticle`, usage `Module`, visibility `Hidden`, bitmask `90`, category
`Initialization`, deprecated true, and suggested false. AssetTools independently
returned class `NiagaraScript` and matching raw registry tags.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The tool reads Asset Registry tags and does not load the UObject.
- Metadata represents the exposed published version, not hidden script versions.
- Raw AssetTools tags encode booleans and integers as strings; the digest returns
  typed values.
- A missing object path raises `No asset found` rather than returning an empty
  digest.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b
<!-- END MANUAL FIELD: manual-review-revision -->

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

### `objectPath`

- Required: **yes**
- Type: `string`
- Purpose:

Full object path of the script (e.g. "/Niagara/Modules/Spawn/Initialize
Particle.Initialize Particle").

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_Assets \
  NiagaraToolsets.NiagaraToolset_Assets.GetNiagaraScriptDigest \
  --arguments '
{
  "objectPath": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Decoded digest. If the path is invalid or not a UNiagaraScript, an error is
raised and a default-initialized digest is returned.

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
