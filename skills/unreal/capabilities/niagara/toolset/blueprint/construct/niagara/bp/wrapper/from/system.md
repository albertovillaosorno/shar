# Construct niagara bp wrapper from system

[Return to the central Unreal MCP index](../../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Review required**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_Blueprint.ConstructNiagaraBPWrapperFromSystem
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_Blueprint
```

## What this tool does

Creates a Blueprint actor wrapper around a Niagara System. This generates a new
Blueprint actor with a Niagara component configured to use the specified
system.

Naming convention: NS_MyEffect -&gt; B_MyEffect

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

The native identity does not establish side effects. Review the live schema and
editor context before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to generate a reusable Blueprint actor wrapper for a verified
Niagara System when SHAR needs a placeable effect actor with the system's user
parameters exposed through Blueprint variables. Use a disposable target first
when validating a new system or engine revision.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Confirm the source Niagara System exists and pass its full soft object path,
  including the object name after the package path.
- The target `/Game/...` package must not already exist.
- `parentClass` must resolve to `AActor` or a subclass.
- Capture a cleanup or save plan before invoking this persistent mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "newAssetPath": "/Game/B_SHAR_MCP_Fountain_FromSystem",
  "system": {
    "refPath": "/Niagara/DefaultAssets/Templates/Systems/FountainLightweight.FountainLightweight"
  },
  "parentClass": {
    "refPath": "/Script/Engine.Actor"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two complete create-and-delete cycles returned the same Blueprint identity.
Each created asset existed, was compiled as a generated Blueprint class, and was
reported dirty. Its compiled class default object contained exactly one
component whose class was `/Script/Niagara.NiagaraComponent`. Each cleanup
returned `true`, and both final existence checks returned `false`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This tool creates and registers a persistent Blueprint package and marks it
  dirty, but the verified operation did not save it automatically.
- `find_assets` returned the source as a package-style virtual path; the
  `system` argument required the full soft object path with the repeated object
  name.
- An invalid parent class raised `Parent class is not a child of AActor` and
  created no target.
- Asset Registry dependency inspection failed on the unsaved generated package
  in the verified session; inspect the compiled CDO and components directly or
  save before relying on registry graph reads.
- The wrapper mirrors supported Niagara user parameters into Blueprint
  variables and construction-script assignments; review compilation output for
  unsupported parameter types.
- Delete disposable output and verify final absence when the wrapper is only a
  test fixture.
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
shar-unreal-mcp describe NiagaraToolsets.NiagaraToolset_Blueprint
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `newAssetPath`

- Required: **yes**
- Type: `string`
- Purpose:

Full path for the new Blueprint asset (prefer same directory as System)

### `parentClass`

- Required: **yes**
- Type: `object`
- Purpose:

The parent Actor class for the Blueprint (e.g., AActor)

### `system`

- Required: **yes**
- Type: `object`
- Purpose:

The Niagara System to wrap in the Blueprint

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_Blueprint \
  NiagaraToolsets.NiagaraToolset_Blueprint.ConstructNiagaraBPWrapperFromSystem \
  --arguments '
{
  "newAssetPath": "<value>",
  "parentClass": {},
  "system": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

The newly created Blueprint actor asset

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
