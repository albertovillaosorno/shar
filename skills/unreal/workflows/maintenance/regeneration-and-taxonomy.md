# Regeneration and taxonomy

Read the [central Unreal MCP index](../../index.md) and the
[workflow map](../README.md) before changing generated Unreal MCP
skills or manual workflow routing.

## Goal

Regenerate one central index and one skill per native tool while preserving a
stable name-derived taxonomy and every valid human-authored field.

## Generated and manual ownership

The generator owns:

- `skills/unreal/index.md`;
- capability paths and filenames;
- tool and toolset identities;
- interface digest and schema summaries;
- generated purpose, examples, safety, and verification sections;
- marker lines, protected-field identities, and field headings;
- current revision, derived review status, and central review counts;
- stale-file cleanup and empty-directory cleanup.

Humans own:

- workflow files under `skills/unreal/workflows/**`;
- exact content between recognized marker pairs in per-tool skills, including
  the reviewed revision token.

Do not edit generated content outside protected fields.

## Protected fields

Every per-tool skill contains:

- SHAR-specific use cases: `[TODO]` for a new tool;
- project prerequisites: `[TODO]` for a new tool;
- validated argument example: `[FILL_ME]` for a new tool;
- project verification notes: `[TODO]` for a new tool;
- known project caveats: `[TODO]` for a new tool;
- manual guidance reviewed revision: `[REVIEW_REQUIRED]` for a new or legacy
  tool until a human completes the current-revision review.

Fill fields only from reviewed project evidence. Use the dedicated
[manual guidance maintenance](manual-guidance-maintenance.md) workflow.

The generated current revision is `<unreal-mcp-version>/<interface-digest>`.
Regeneration never copies that value into the protected reviewed-revision field.
It preserves the prior token and derives **Current** only for an exact match.
The Unreal MCP version is read from the associated engine's
`ModelContextProtocol.uplugin` `VersionName` and normalized to three-part
SemVer.
The Python translator package version is a separate release identity.

## Regeneration command

From the SHAR repository root:

```text
PYTHONPATH=src python -m mcp.src.adapters.driving.cli skills
```

The installed command is equivalent:

```text
shar-unreal-mcp skills
```

## Live discovery inputs

Regeneration requires:

- intended editor readiness;
- complete Toolset Registry meta-tools;
- all expected toolsets loaded;
- live descriptions and schemas;
- explicit taxonomy ownership for every toolset;
- no malformed existing manual-field markers.

The live interface is authoritative. The generator consumes only exposed MCP
metadata when constructing skill documents.

## Name-derived taxonomy

Toolset identities produce the base directory. Generic repeated components are
normalized without an `other` or `misc` fallback.

Within one toolset, the generator examines sibling tool names:

- the longest prefix shared by at least two tools becomes nested directories;
- the remaining unique suffix becomes the Markdown filename;
- an unshared compound name remains one readable hyphenated filename;
- every final path must be unique.

Examples:

```text
SetYAlpha -> set/y/alpha.md
SetYBeta  -> set/y/beta.md
```

```text
SetSectionRange     -> set/section/range.md
SetSectionBlendType -> set/section/blend-type.md
SetPlaybackRange    -> set/playback-range.md
```

A single `DiscoverTests` tool remains `discover-tests.md` because no sibling
shares its prefix.

## Manual workflow taxonomy

Manual workflows use lifecycle ownership rather than native-name tokenization.
The only workflow map is `skills/unreal/workflows/README.md`.

The controlled folders are:

- `connection`: project, editor, server, session, and registry readiness;
- `planning`: capability routing and live-schema argument construction;
- `execution`: reads, mutations, batches, and programmatic orchestration;
- `assurance`: postcondition verification, ambiguity, and recovery;
- `maintenance`: protected guidance and generated catalog upkeep;
- `extension`: callable-surface and reusable-guidance authoring.

A workflow move must update:

1. the root workflow map;
1. the generated central-index renderer;
1. workflow regression tests;
1. all local links;
1. recursive manual-document link validation;
1. the checked-in generated central index.

Do not create nested `index.md` files, a flat duplicate of a moved runbook, or a
second workflow map. Folder selection follows stable responsibility, not file
count or source-document origin.

## Identity-based manual-field migration

Manual fields belong to the complete native tool identity, not to the current
filesystem path.

Before mutation, the store:

1. scans existing generated capability files;
1. extracts exactly one native Tool identity from each generated file;
1. rejects duplicate files claiming the same identity;
1. validates protected marker structure;
1. accepts the legacy five-field schema only for one-way migration;
1. maps manual fields by native identity;
1. adds `[REVIEW_REQUIRED]` when the legacy review field is absent;
1. injects those fields into the fresh generated path.

Therefore a taxonomy-only path change preserves manual content exactly.

## Tool lifecycle behavior

### Existing identity, same path

Refresh generated content, preserve manual fields, and recompute review status.

### Existing identity, new path

Move the generated skill to the new path and preserve manual fields by identity.
Delete the obsolete path only after validation succeeds.

### New identity

Create a new skill with four `[TODO]` fields and one `[FILL_ME]` field.

### Removed identity

Delete the obsolete active generated skill.

### Renamed identity

Treat as removal plus creation. The new skill starts with placeholders unless a
separate reviewed migration proves semantic equivalence.

## Fail-before-mutation checks

Regeneration must abort before stale cleanup or writes when:

- a marker is malformed, missing, duplicated, unknown, or out of order;
- a generated capability lacks one native Tool identity;
- two existing files claim one native identity;
- two live tools normalize to one path;
- a toolset lacks explicit taxonomy ownership;
- an output path escapes the owned generated surface;
- the generated set omits the central index.

## Atomic replacement sequence

1. Discover the complete live catalog.
1. Validate taxonomy ownership.
1. Validate unique generated paths.
1. Render fresh index and per-tool shells.
1. Scan existing generated identities.
1. Validate and extract protected manual fields.
1. Merge fields into fresh shells by native identity.
1. Abort without mutation on any validation failure.
1. Remove stale generated files.
1. Atomically write current generated documents.
1. Remove empty taxonomy directories.

Workflow files are never part of generated cleanup.

## Regenerate after

- Unreal Engine upgrade;
- Toolset plugin enablement or disablement;
- toolset or tool identity change;
- input or output schema change;
- generated interface-digest mismatch;
- Unreal MCP plugin `VersionName` change;
- generated capability taxonomy policy change;
- manual workflow path or lifecycle-routing change;
- protected-field contract change.

## Review procedure

1. Run `doctor` and confirm the intended editor.
1. Generate the skills.
1. Confirm toolset, tool, and document counts.
1. Review any taxonomy ownership failure.
1. Confirm exactly one generated `index.md` exists.
1. Confirm the workflow map links every manual runbook.
1. Confirm central-index workflow groups match the lifecycle folders.
1. Confirm one skill exists per live tool.
1. Confirm all six protected marker pairs exist exactly once.
1. Confirm the index Unreal MCP version and manual review revision are current.
1. Confirm review counts equal the per-skill status totals.
1. Confirm all local links resolve.
1. Compare representative path families.
1. Confirm manual content survived moved paths.
1. Run package, workflow, capability-shard, and repository validation.

## Validation expectations

The generated tree must prove:

- one central index;
- one per-tool skill;
- one shared interface digest;
- resolving links;
- bounded Markdown files;
- no nested indexes;
- no trailing whitespace;
- required generated sections;
- complete protected marker sets;
- one current revision shared by the index and every capability;
- exact current and review-required counts in the index;
- exact placeholder counts for untouched new skills.

## Unknown toolsets

Generation fails closed when the live editor exposes a toolset without reviewed
taxonomy ownership. Add the exact identity to the owning SRP taxonomy module,
add coverage, regenerate, and review resulting paths.

Do not create an `other`, `misc`, or unclassified fallback.

## Stop conditions

Stop and repair when:

- the live catalog is partial;
- current files contain invalid markers or duplicate identities;
- path collision occurs;
- a path move loses manual content;
- counts or digest are inconsistent;
- validation requires modifying Epic plugin source;
- generated cleanup would touch workflow or non-owned documentation.
