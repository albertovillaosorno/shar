# Select assets

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.EditorAppToolset.SelectAssets
```

Toolset:

```text
EditorToolset.EditorAppToolset
```

## What this tool does

Selects the specified assets in the content browser. Completes once the content
browser has applied the selection.

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
Use this tool to select one or more known assets in an active SHAR Content
Browser before an asset-editor, inspection, or batch operation.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve each target through the Asset Registry and keep an active Content
  Browser visible while selection is applied.
- Capture `GetSelectedAssets` and `GetContentBrowserPath` before invocation.
- For the current UE 5.8 implementation, use a full object path when a package
  path times out, and require an independent package-selection read afterward.
- Define cleanup that navigates away from the selected asset and closes any
  Content Browser tab opened by the task.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "assetPaths": [
    "/Engine/BasicShapes/Cube.Cube"
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Pre-state had no active Content Browser path and no selected assets. After
opening and docking the Content Browser at `/Engine/BasicShapes`, the full
object-path call returned `null` without timing out. An independent
`GetSelectedAssets` read returned the package
`/Engine/BasicShapes/Cube`. Cleanup navigated to `/Game`, which cleared the
selection, then closed the task-opened Content Browser tab. Final reads returned
an empty selection and an empty active-browser path.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The live schema describes package paths, but the UE 5.8 implementation also
  passes each string directly to `SyncBrowserToObjects`. The package path
  `/Engine/BasicShapes/Cube` timed out and left selection empty; the full object
  path `/Engine/BasicShapes/Cube.Cube` selected the asset successfully.
- A full object path produces no `ExpectedPackages` entry in the implementation,
  so the async result can complete before selection settles. Always verify with
  `GetSelectedAssets`; never trust the `null` return alone.
- `SelectAssets([])` completes immediately but does not clear an existing
  selection. Navigating the browser to another folder cleared the tested
  selection.
- Content Browser refs and layout state are transient. Refresh Slate refs after
  opening, docking, navigating, or closing the browser.
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

### `assetPaths`

- Required: **yes**
- Type: `array<string>`
- Purpose:

The package paths of the assets to select.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.EditorAppToolset \
  EditorToolset.EditorAppToolset.SelectAssets \
  --arguments '
{
  "assetPaths": []
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `null`
- Purpose:

Always null

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
