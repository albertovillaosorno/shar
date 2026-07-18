# Get node connections

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
conversation_toolset.toolsets.conversation.ConversationTools.get_node_connections
```

Toolset:

```text
conversation_toolset.toolsets.conversation.ConversationTools
```

## What this tool does

Returns output connection GUIDs for a conversation node.

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
Use this tool to enumerate outgoing connection GUIDs from a resolved
ConversationNodeWithLinks.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the node through `get_node_by_guid` from the same database.
- Confirm the returned node is a ConversationNodeWithLinks subtype.
- Treat returned GUIDs as database-local identities and resolve them before
  traversal.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "node": {
    "refPath": "/Game/Conversations/DA_ConversationExample.DA_ConversationExample:ConversationNode_0"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A disposable ConversationDatabase was intentionally passed in place of a node.
Parameter translation failed with `not valid ConversationNodeWithLinks for
property node`, confirming the strict subtype boundary.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- A ConversationDatabase asset is not a node argument.
- Not every conversation node exposes outgoing links.
- The result contains GUIDs, not hydrated destination node objects.
- Resolve every destination against the same compiled database before use.
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
shar-unreal-mcp describe conversation_toolset.toolsets.conversation.ConversationTools
```

1. Confirm every required input against the current schema.

## Inputs

### `node`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  conversation_toolset.toolsets.conversation.ConversationTools \
  conversation_toolset.toolsets.conversation.ConversationTools.get_node_connections \
  --arguments '
{
  "node": {}
}
'
```

## Expected output

A list of FGuid output connections.

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

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
