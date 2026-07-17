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
[TODO]
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and an active Content Browser must be available.
- Resolve every package path through the Asset Registry before selection.
- Capture both `GetSelectedAssets` and `GetContentBrowserPath` pre-state because
  selection can change browser context even when completion is not reported.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
[FILL_ME]
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Selecting the existing package
`/BaseMaterial/Materials/Functions/MF_Rotate2D` timed out while waiting for the
Content Browser to apply the selection. A fresh `GetSelectedAssets` call
returned
an empty array after cleanup, while `GetContentBrowserPath` showed that the
browser had become active at `/BaseMaterial/Materials/Functions`. The attempted
selection therefore did not provide a verified selected-asset postcondition and
its validated argument placeholder remains intentionally unresolved.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- A timeout is ambiguous: the current call activated and navigated the Content
  Browser even though it did not report selection completion.
- Do not retry after timeout until `GetSelectedAssets` and
  `GetContentBrowserPath` establish the current state.
- Passing an empty array was used as cleanup and the final selected-assets read
  was empty, but this session did not prove a successful non-empty selection.
- The available toolset does not expose a capability that restores the absence
  of an active Content Browser, so browser activation may remain transiently
  visible after a failed call.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
[REVIEW_REQUIRED]
<!-- END MANUAL FIELD: manual-review-revision -->

<!-- markdownlint-disable-next-line MD013 -->
- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Review required**

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
