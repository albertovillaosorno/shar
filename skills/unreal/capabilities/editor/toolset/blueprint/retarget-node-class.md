# Retarget node class

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.retarget_node_class
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Replaces a node's baked-in class reference from old_class to new_class in
place.

If the node's current class reference matches old_class it is replaced with
new_class and the node is reconstructed so its pins reflect the new type. If
the node already references new_class the call is a no-op.

When a Blueprint is duplicated, nodes in the copied graph retain class
references pointing to the original Blueprint. Calling this on each node with
the original and duplicate classes retargets them without manual delete-
recreate-rewire cycles.

Handles cast, function call, event, and multicast delegate nodes. The Blueprint
must be compiled after all retargeting is complete.

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
Use this mutation to retarget one reviewed SHAR cast, call, event, or delegate
node from an exact source class to an exact replacement class without deleting
and rebuilding the node.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact Blueprint graph and node in the current editor session.
- Confirm the node kind is supported by the live tool description.
- Capture the original class as the exact inverse.
- Verify class-dependent pins before mutation; do not rely only on the node type
  ID.
- Define strict compilation, reverse retargeting, and disposable-asset cleanup
  before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "new_class": {
    "refPath": "/Script/Engine.Pawn"
  },
  "node": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_RetargetLifecycle.BP_MCP_RetargetLifecycle:EventGraph.K2Node_DynamicCast_0"
  },
  "old_class": {
    "refPath": "/Script/Engine.Actor"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A disposable `Utilities|Casting|CastToActor` node initially exposed an
`AsActor` output with type `Actor Object Reference` and compiled strictly.
Retargeting Actor to Pawn returned `null`; a fresh node read exposed `AsPawn`
with type `Pawn Object Reference`, preserved position `(480, 240)`, and strict
compilation passed. Reversing Pawn to Actor returned `null`; the `AsActor`
output and Actor reference type were restored at the same position, and strict
compilation passed again. Deleting the disposable validation folder removed all
virtual and physical assets.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `get_node_infos.type_id` remained `Utilities|Casting|CastToActor` after the
  Actor-to-Pawn retarget and after compilation.
- Verify the class-dependent pin name and type, not only the potentially stale
  node type ID.
- The supplied `old_class` must match the node's current underlying class.
- Connected pins may become incompatible when source and replacement classes
  expose different types or signatures.
- Retarget all required nodes before compiling a duplicated Blueprint.
- The operation has no structured return value and does not save automatically.
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
shar-unreal-mcp describe editor_toolset.toolsets.blueprint.BlueprintTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `new_class`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `node`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `old_class`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.retarget_node_class \
  --arguments '
{
  "new_class": {},
  "node": {},
  "old_class": {}
}
'
```

## Expected output

The live interface does not declare a structured output schema.

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
