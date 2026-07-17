# Set content browser path

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.EditorAppToolset.SetContentBrowserPath
```

Toolset:

```text
EditorToolset.EditorAppToolset
```

## What this tool does

Navigates the active content browser to the specified folder path.

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
Use this tool to place the active Content Browser at one canonical virtual
folder before bounded SHAR asset discovery, inspection, selection, or visual
capture.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project must be open with an active Content Browser.
- Capture the current virtual path with `GetContentBrowserPath` before changing
  it.
- Use an Unreal virtual folder such as `/Game`, not a workstation filesystem
  path.
- Define the exact restoration path before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "path": "/Game"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Starting from `/BaseMaterial/Materials/Functions`, the call returned
`returnValue: null` and a separate `GetContentBrowserPath` call returned
`/Game`. A second mutation restored the captured path, and another independent
read returned `/BaseMaterial/Materials/Functions`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The tool requires an active Content Browser; global editor and MCP readiness
  alone are insufficient.
- In a session without an active browser, navigation failed closed and reported
  that the path remained empty.
- The mutation changes transient editor navigation only; it does not create,
  move, load, or save an asset.
- The mutation response does not contain the applied path, so verify every call
  with `GetContentBrowserPath`.
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
shar-unreal-mcp describe EditorToolset.EditorAppToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `path`

- Required: **yes**
- Type: `string`
- Purpose:

The internal path to navigate to, e.g. '/Game/Meshes'.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.EditorAppToolset \
  EditorToolset.EditorAppToolset.SetContentBrowserPath \
  --arguments '
{
  "path": "<value>"
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
