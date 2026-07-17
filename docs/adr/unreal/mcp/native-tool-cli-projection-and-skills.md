# Native MCP tool projection and protected skill guidance

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Unreal MCP tool projection, generated skills, and human guidance

## Context

The native Unreal MCP interface is a live external contract. Toolsets, tools,
input schemas, output schemas, descriptions, and supported capabilities can
change when Unreal Engine or Toolset plugins change. Repository documentation
must therefore be regenerated from the live interface instead of maintained as a
copied static catalog.

Live metadata alone is not enough for a useful operational skill. SHAR-specific
use cases, prerequisites, validated examples, verification evidence, known
caveats, and review currency require deliberate human review. Those facts are
maintained only in protected fields and survive live-interface refreshes.

The repository needs one deterministic update process that can replace generated
metadata while preserving explicitly human-owned content.

The translator uses the repository's exact current stable Python patch and the
current stable Hatchling build backend. Runtime upgrades follow the canonical
version-currency gate; generated skill content does not encode toolchain pins.

## Decision

Every discovered native Unreal tool receives exactly one name-derived Markdown
skill in the generated Unreal capability catalog. The central Unreal skill index
remains fully generated.

Each per-tool skill contains two ownership regions:

1. A generated shell derived from live MCP metadata.
1. Six protected human-authored fields delimited by stable marker pairs.

The generated shell owns:

- tool and toolset identities;
- interface digest and protocol metadata;
- native purpose and documentation;
- technical execution posture inferred from the native identity;
- current input and output schema summaries;
- generated invocation examples;
- generic verification and failure guidance;
- navigation and taxonomy links.

The protected human fields are:

- `project-use-cases`: SHAR-specific use cases;
- `project-prerequisites`: project and editor prerequisites;
- `validated-arguments`: a reproduced argument example;
- `project-verification`: project-specific postcondition evidence;
- `known-caveats`: known limitations, hazards, or version constraints; and
- `manual-review-revision`: the exact Unreal MCP plugin version and live
  interface digest against which the five guidance fields were last reviewed.

New tools initialize the five guidance fields with their documented sentinel
values and initialize `manual-review-revision` with `[REVIEW_REQUIRED]`. The
generator derives visible review status from whether the protected revision
token matches the generated current revision; it does not invent review
completion.

The current revision token is `<unreal-mcp-version>/<interface-digest>`. The
version is read from the installed `ModelContextProtocol.uplugin` `VersionName`
and normalized to three-part SemVer (`1.0` becomes `1.0.0`). The Python
translator package version is not part of this token.

## Marker contract

Each manual field uses one stable pair:

```text
<!-- BEGIN MANUAL FIELD: field-identity -->
human-authored content
<!-- END MANUAL FIELD: field-identity -->
```

Operators and agents may edit only the content between matching markers. Marker
lines, field identities, generated headings, and all content outside the marker
pairs remain generator-owned.

A file containing any protected markers must contain the complete recognized
field set. Duplicate, missing, reordered, malformed, or unknown markers fail
regeneration before stale-file cleanup or generated writes begin.

A legacy generated file with no markers receives the current empty manual-field
template during its first regeneration. A legacy five-field file preserves those
five values and receives `manual-review-revision` as `[REVIEW_REQUIRED]`; review
status remains `Review required` until that protected value exactly matches the
generated current revision.

## Name-derived taxonomy

The complete toolset identity defines the base directory. Within one toolset,
the generator tokenizes every sibling tool leaf and extracts the longest prefix
shared by at least two tools. Shared prefix words become nested directories and
the remaining unique suffix becomes the Markdown filename.

For example:

```text
SetYAlpha -> set/y/alpha.md
SetYBeta  -> set/y/beta.md
```

```text
SetSectionRange     -> set/section/range.md
SetSectionBlendType -> set/section/blend-type.md
SetPlaybackRange    -> set/playback-range.md
```

An unshared compound name remains one readable hyphenated filename. Every live
tool must normalize to one unique path; collisions fail generation before any
filesystem mutation.

## Regeneration algorithm

The update process performs these steps in order:

1. Discover the complete live native Toolset Registry catalog.
1. Validate explicit taxonomy ownership for every toolset.
1. Derive and validate one unique generated path per native tool.
1. Scan existing generated capability files and extract one native Tool identity
   from each file.
1. Reject duplicate existing files that claim the same native identity.
1. Validate and extract protected human field contents by native identity.
1. Render a fresh central index and one fresh per-tool skill shell.
1. Inject the exact retained fields into the shell for the same native identity,
   regardless of its previous filesystem path.
1. Abort without filesystem mutation when any identity, path, or marker contract
   is invalid.
1. Remove generated files whose native tools no longer exist.
1. Atomically replace retained and newly created generated documents.
1. Remove empty taxonomy directories.

A taxonomy-only path change preserves human content automatically because field
ownership follows the complete native tool identity. A native identity rename is
still treated as removal plus creation; the new identity starts with
placeholders unless a separate technical migration proves semantic equivalence.

## Manual workflow taxonomy

Manual Unreal MCP workflows are maintained separately from generated capability
pages. They use one root workflow map and lifecycle folders for connection,
planning, execution, assurance, maintenance, and extension.

The generated central index links the root map and every workflow under grouped
lifecycle headings. Workflow paths are owned by the index renderer and guarded
by recursive link and structure tests. Workflow moves must update the map,
renderer, tests, local links, and checked-in generated index together.

No nested workflow indexes or flat compatibility copies are retained. Generated
capability taxonomy remains native-name-derived; manual workflow taxonomy is
responsibility-derived.

## Technical metadata boundary

Generated metadata is limited to interoperability information exposed by the
live MCP interface. The generator consumes tool identities, descriptions, input
schemas, output schemas, and toolset metadata; it does not inspect engine source
when constructing skill documents.

Generated technical documentation does not define general agent guardrails.
Those remain owned exclusively by the workspace agent authority.

## Consequences

### Positive

- Live schemas can be refreshed without deleting accumulated project knowledge.
- New capabilities become visible immediately with honest empty fields.
- Removed capabilities do not leave misleading dead skills.
- Human review is explicit and incremental rather than simulated by generation.
- Malformed ownership markers cannot silently destroy manual work.
- Tests can verify both live projection and preservation behavior.

### Negative

- Per-tool files are larger because they contain both generated and manual
  regions.
- Native tool identity becomes the stable manual-content compatibility key.
- Native identity renames require a separate semantic migration when prior
  guidance remains useful.
- Complete validation must cover hundreds of generated Markdown documents.

## Rejected alternatives

### Fully generated skills

Rejected because generic native metadata cannot provide verified SHAR-specific
examples, prerequisites, or caveats.

### Fully manual skills

Rejected because 830 independent copies of live tool schemas would drift and
would be impractical to maintain.

### One unstructured manual block

Rejected because field-specific markers provide clearer ownership, better
validation, and safer schema evolution.

### Preserve arbitrary edits anywhere in the file

Rejected because a three-way Markdown merge would make generated authority
ambiguous and could retain obsolete schema text.

### Keep removed tool files as archives

Rejected because the active skill tree must represent the current live native
interface. Historical retention belongs in version control, not the active
capability taxonomy.

## Validation

The implementation must prove:

- one central index and one skill per live tool;
- unique name-derived paths;
- all six protected marker pairs in every per-tool skill;
- sentinel initialization for newly created tools and migrated legacy files;
- derived manual-review status from exact revision-token equality;
- exact multiline human-content preservation across refreshes;
- fail-closed behavior before mutation for malformed markers;
- exactly one documented `[TODO]` sentinel and one documented `[FILL_ME]`
  sentinel in this ADR, with no unresolved work-marker tokens elsewhere in the
  active documentation surface;
- deletion of stale tool files;
- stable digest and resolving local links;
- canonical Python, Markdown, and repository validation.
