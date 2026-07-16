# Construct niagara bp wrapper from component

[Return to the central Unreal MCP index](../../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_Blueprint.ConstructNiagaraBPWrapperFromComponent
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_Blueprint
```

## What this tool does

Creates a Blueprint actor wrapper from a Niagara Component. This generates a
new Blueprint actor and preserves all component property values and user
variable overrides.

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
Use this tool to generate a Blueprint actor wrapper from an already configured
Niagara Component when SHAR must preserve the component's system and user
variable overrides rather than starting from the Niagara System defaults.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Obtain a live `UNiagaraComponent` reference from a compiled actor or Blueprint
  class default object.
- The source component must have a Niagara System assigned.
- The target `/Game/...` package must not already exist.
- `parentClass` must resolve to `AActor` or a subclass.
- Capture a cleanup or save plan before invoking this persistent mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "newAssetPath": "/Game/B_SHAR_MCP_Fountain_FromComponent",
  "component": {
    "refPath": "/Game/B_SHAR_MCP_Fountain_FromSystem.B_SHAR_MCP_Fountain_FromSystem_C:Niagara_GEN_VARIABLE"
  },
  "parentClass": {
    "refPath": "/Script/Engine.Actor"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
In two complete cycles, the source component was independently discovered from
the compiled system-wrapper CDO and confirmed as
`/Script/Niagara.NiagaraComponent`. Each call created the same component-wrapper
Blueprint identity, reported the package dirty, and produced a compiled CDO with
exactly one Niagara Component. Cleanup returned `true`, and final existence was
`false` for both source and copied wrappers.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This tool creates and registers a persistent Blueprint package and marks it
  dirty, but the verified operation did not save it automatically.
- The component reference can be an in-memory generated-component path whose
  lifetime is tied to the loaded Blueprint and editor session.
- The source component must have a Niagara System assigned; otherwise native
  construction fails.
- The copied wrapper preserves component property values and user overrides,
  while still generating Blueprint variables only for supported Niagara user
  parameter types.
- ObjectTools returned no enumerable properties for the generated component
  template in the verified session; component class, successful wrapper
  construction, compiled CDO structure, and cleanup were used as independent
  evidence.
- Delete disposable output and verify final absence when the wrapper is only a
  test fixture.
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
shar-unreal-mcp describe NiagaraToolsets.NiagaraToolset_Blueprint
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `component`

- Required: **yes**
- Type: `object`
- Purpose:

The Niagara Component with configured properties and overrides to preserve

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

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_Blueprint \
  NiagaraToolsets.NiagaraToolset_Blueprint.ConstructNiagaraBPWrapperFromComponent \
  --arguments '
{
  "component": {},
  "newAssetPath": "<value>",
  "parentClass": {}
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
