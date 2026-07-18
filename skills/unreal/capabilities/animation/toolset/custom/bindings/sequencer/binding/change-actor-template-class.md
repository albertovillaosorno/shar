# Change actor template class

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.change_actor_template_class
```

Toolset:

```text
animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools
```

## What this tool does

Set the actor class for a spawnable or replaceable template.

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
Use this tool to change one spawnable actor template to an exact replacement
class.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Open the exact disposable sequence and confirm the source binding type,
  custom binding objects, and resolved world objects.
- Treat the returned binding proxy as the new authority after every
  conversion.
- Capture custom type and bound-object readers before mutation and define
  whole-folder cleanup.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "actor_class": "/Script/Engine.CameraActor",
  "binding": {
    "bindingId": "653BACC2-4172-2058-B979-D7858B2E7518",
    "sequence": {
      "refPath": "/Game/SHAR_MCP_Validation_Round50_260718/LS_MCP_Round50.LS_MCP_Round50"
    }
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned true; custom-binding readers then resolved a CameraActor
template and CameraActor live object.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Conversion can invalidate the input binding proxy and create one or more
  replacement proxies.
- Custom binding template objects and resolved world objects are different
  identities; inspect both.
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
shar-unreal-mcp describe animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `actor_class`

- Required: **yes**
- Type: `string`
- Purpose:

Full class path of the new actor class.

### `binding`

- Required: **yes**
- Type: `object`
- Purpose:

MovieSceneBindingProxy

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools \
  animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.change_actor_template_class \
  --arguments '
{
  "actor_class": "<value>",
  "binding": {}
}
'
```

## Expected output

True if the class was changed successfully.

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
