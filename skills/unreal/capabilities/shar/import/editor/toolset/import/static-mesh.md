# Import static mesh

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `106cd0a0cc2273be4d9a633ac49dee0857958117831071949e38f01cec179e92`

## Native identities

Tool:

```text
SharImportEditor.SharImportToolset.ImportStaticMesh
```

Toolset:

```text
SharImportEditor.SharImportToolset
```

## What this tool does

Imports one reviewed FBX as a StaticMesh under /Game/Generated/SHAR. Authored
normals are preserved and auxiliary assets are not generated. The caller
remains responsible for saving and postcondition read-back.

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
[TODO]
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
[TODO]
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
[FILL_ME]
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
[TODO]
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
[TODO]
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
[REVIEW_REQUIRED]
<!-- END MANUAL FIELD: manual-review-revision -->

<!-- markdownlint-disable-next-line MD013 -->
- Current revision: `1.0.0/106cd0a0cc2273be4d9a633ac49dee0857958117831071949e38f01cec179e92`
- Manual guidance status: **Review required**

## Before invocation

1. Run `shar-unreal-mcp doctor` and require `ready: true`.
1. Select this skill from the central index, not from memory.
1. Refresh the live schema:

```text
shar-unreal-mcp describe SharImportEditor.SharImportToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `assetName`

- Required: **yes**
- Type: `string`
- Purpose:

Exact destination asset name.

### `folderPath`

- Required: **yes**
- Type: `string`
- Purpose:

Generated Unreal content folder.

### `sourceFile`

- Required: **yes**
- Type: `string`
- Purpose:

Absolute verified FBX source path.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  SharImportEditor.SharImportToolset \
  SharImportEditor.SharImportToolset.ImportStaticMesh \
  --arguments '
{
  "assetName": "<value>",
  "folderPath": "<value>",
  "sourceFile": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

The single StaticMesh object path produced by the import task.

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
