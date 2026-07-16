# Manual guidance maintenance

Read [`../index.md`](../index.md) and the exact per-tool skill before editing
any
protected field.

## Goal

Turn reproduced SHAR project evidence into durable guidance without changing the
live-generated interface description or weakening regeneration safety.

## Ownership boundary

Humans may edit only text between a matching marker pair:

```text
<!-- BEGIN MANUAL FIELD: project-use-cases -->
[TODO]
<!-- END MANUAL FIELD: project-use-cases -->
```

Do not edit:

- marker lines or field identities;
- generated headings, tool identities, schemas, digests, or examples;
- the central index;
- taxonomy paths;
- generic generated safety and verification sections.

Regeneration replaces all generated content and preserves exact text inside a
complete recognized marker set. The generated revision and review status remain
outside human ownership.

## Field responsibilities

### SHAR-specific use cases

Record concrete project outcomes for which the tool has been proven useful.
Name the asset, system, phase, or workflow category without embedding private
local paths.

Good evidence:

- a repeatable import-review step;
- a verified editor inspection used by the pipeline;
- a bounded repair operation for one SHAR asset class.

Do not write vague claims such as “useful for Unreal work.”

### Project prerequisites

Record state that must already be true before invocation, including:

- required project, world, asset, graph, plugin, or editor mode;
- compilation, save, or discovery state;
- required change-scope declarations;
- ordering dependencies on another tool.

Use a checklist when more than one condition applies.

### Validated argument example

Replace `[FILL_ME]` only after one exact argument object has succeeded against
the current live schema and its postcondition was independently verified.

Prefer a JSON block containing repository-safe example identities:

```json
{
  "assetPath": "/Game/Example/Asset"
}
```

Do not include:

- workstation paths;
- unverified placeholders;
- values copied from a different project without reproduction.

### Project verification notes

Record the independent read, validation, compilation, map check, test, or editor
observation that proves success. State what evidence distinguishes success from
transport completion.

### Known project caveats

Record reproduced limitations such as:

- Unreal version constraints;
- required editor restart or refresh;
- stale-cache behavior;
- unsupported asset states;
- ambiguous timeout behavior;
- known unsafe combinations with other operations.

Do not convert speculation into a caveat. Keep `[TODO]` until evidence exists.

### Manual guidance reviewed revision

Leave `[REVIEW_REQUIRED]` until every populated manual field has been checked
against the exact `Current revision` printed below the marker. After
that review, copy the complete current revision token into this protected field.

The token combines the installed Unreal MCP plugin version and live interface
digest. The generator reads `VersionName` from the associated engine plugin and
normalizes `1.0` to `1.0.0`; the Python translator CalVer is not part of this
token. A plugin-version or schema change therefore makes an older token stale.
Regeneration preserves the token but derives **Review required** whenever it no
longer matches. Never advance the token merely to silence the status.

## Evidence threshold

A field may be completed only when all applicable evidence exists:

1. The current live toolset schema was inspected.
1. The tool was invoked against the intended SHAR project state.
1. The returned error state and structured output were reviewed.
1. The postcondition was checked through a separate evidence source.
1. The guidance is reproducible and does not depend on a private local path.
1. Version-specific conditions are stated explicitly.

A successful HTTP or MCP response alone is insufficient.

## Editing procedure

1. Open the per-tool skill selected from the central index.
1. Run `describe` for the current toolset.
1. Reproduce the operation and verification evidence.
1. Edit only the smallest relevant protected fields.
1. Preserve marker lines exactly.
1. Keep unresolved fields as `[TODO]`, `[FILL_ME]`, or
   `[REVIEW_REQUIRED]`.
1. When all populated guidance is verified, copy the exact current revision
   token into `manual-review-revision`.
1. Regenerate the complete skill tree.
1. Confirm the authored text remains byte-for-byte equivalent.
1. Run the generated-tree and canonical validation gates.

## Preservation check

After regeneration, compare the protected region rather than the complete file:

- generated descriptions or schemas may legitimately change;
- the central digest may change;
- taxonomy paths may change only after reviewed taxonomy updates;
- protected field contents must remain exact;
- the review token must remain unchanged unless a human completed the review;
- generated status may change from **Current** to **Review required**.

Regeneration must stop before filesystem mutation when any marker is missing,
duplicated, malformed, unknown, or out of order.

## Tool additions, removals, and renames

- New tool: create five content placeholders plus `[REVIEW_REQUIRED]`.
- Removed tool: delete its active generated skill.
- Renamed tool: remove the old path and create the new path with placeholders.

A rename does not automatically migrate human text. Move it only after proving
the new native tool has equivalent semantics and schema.

## Review checklist

- The field contains project-specific evidence, not copied native prose.
- The live schema still matches the argument and prerequisite claims.
- The example uses stable project identities rather than workstation-specific
  values.
- The example scope is narrow and safe to adapt.
- Verification uses a separate evidence source.
- Marker identities and order are unchanged.
- Unresolved uncertainty remains visible as a placeholder.
- The reviewed revision equals the current revision only after live
  revalidation.
- The central index counts match the per-skill status values.

## Stop conditions

Stop and repair before regeneration when:

- a marker line was edited;
- a protected field is missing or duplicated;
- the tool identity or schema no longer matches the recorded guidance;
- the generated status says **Review required** and the task depends on the
  stale guidance;
- the evidence came from another project and was not reproduced;
- the proposed text would imply a broader state change than the reproduced
  evidence;
- a rename would require guessing whether prior guidance still applies.
