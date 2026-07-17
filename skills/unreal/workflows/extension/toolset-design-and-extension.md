# Toolset design and extension

Read the [central Unreal MCP index](../../index.md), the
[workflow map](../README.md), and
[capability selection](../planning/capability-selection.md) before using this
workflow.

## Goal

Design or extend a native Unreal MCP toolset so the new callable surface is
narrow, typed, composable, discoverable, testable, and projected into the SHAR
skill catalog without duplicating an existing capability.

## When this workflow applies

Use this workflow when a task explicitly requires:

- a new native tool identity;
- a new toolset domain;
- a new operation inside an existing toolset;
- a schema redesign for an existing native tool;
- registration or discovery changes;
- an async result or cancellation contract;
- a converter required for a real schema problem.

Do not use this workflow merely to invoke, document, or troubleshoot an
existing tool. Use the generated capability skill and normal operating
workflows instead.

## Installed-source boundary

SHAR normally consumes installed native toolsets through the inbound server. Do
not patch installed engine or plugin source as a convenience during an ordinary
editor task.

Proceed with implementation only when the task identifies a writable,
repository-owned plugin or another explicitly maintained source boundary. When
only installed source exists, record the missing capability and stop rather than
creating an untracked local patch.

## Discovery before design

Before proposing an API:

1. run editor readiness;
1. list the complete live toolset inventory;
1. describe every plausible owning toolset;
1. search generated capability identities and descriptions;
1. search repository source for equivalent behavior;
1. inspect generic object, asset, scene, and editor tools;
1. identify whether the requested outcome is already composable;
1. record the exact gap that remains.

A domain-specific request can still belong to a broader toolset. Prefer the
owner whose abstraction remains valid for other assets or objects of the same
kind.

## Responsibility and ownership

One toolset should own one coherent domain. Add to an existing toolset when the
new operation shares its target types, lifecycle, and vocabulary.

Create a new toolset only when:

- no current toolset owns the domain cleanly;
- the domain has several related operations rather than one isolated helper;
- plugin and module ownership are explicit;
- registration and test placement are clear;
- the new boundary does not duplicate generic capabilities.

Do not create a toolset named after one immediate task when the stable domain is
broader.

## API design principles

A strong callable surface is:

- **narrow**: one operation has one observable outcome;
- **complete**: lifecycle gaps are intentional and documented;
- **composable**: outputs can feed related tools without translation tricks;
- **typed**: structured values use real reflected types;
- **bounded**: target count, payload size, and duration are controlled;
- **deterministic**: ordering and no-op behavior are defined;
- **inspectable**: reads exist for important mutation postconditions;
- **recoverable**: destructive actions expose enough identity for verification.

Do not mirror a large Unreal API mechanically. Shape the smallest stable
contract that expresses useful editor work.

## Lifecycle symmetry

Review the complete lifecycle before implementing one method:

- discovery or listing;
- identity lookup;
- state inspection;
- creation or attachment;
- update or movement;
- removal or deletion;
- compilation, save, or activation where applicable;
- independent verification.

A missing inverse can make a mutation unsuitable for automated use. Getter-only
surfaces are valid when mutation is unsafe or unsupported. Mutations without a
read path require an explicit verification design before implementation.

## Schema design

Use reflected native types for parameters and return values. Structured data
must remain structured through the live schema.

Prefer:

- exact UObject, class, asset, graph, node, or component references;
- native structs for vectors, transforms, colors, ranges, and settings;
- arrays with explicit item types;
- enums for closed value sets;
- dedicated result structs for stable multi-field reads;
- optional fields only when omission has a defined meaning.

Reject:

- JSON encoded inside a string merely to bypass schema design;
- labels used where stable object identities exist;
- status booleans that hide useful result data;
- generic dictionaries when a stable reflected struct exists;
- fields whose omission silently means every target;
- return wrappers that mix data, warnings, and failure ambiguously.

Document units, coordinate space, ranges, empty-result meaning, ordering, and
non-obvious identity forms. Do not repeat information already visible in names
and schemas.

## Error model

A synchronous tool should return normally only when its contract completed. A
validation or execution failure should surface as a native tool error with a
specific, actionable message.

Define error branches for:

- missing or invalid identity;
- incompatible target class;
- missing editor context;
- unsupported lifecycle state;
- duplicate or conflicting state;
- target count beyond the bound;
- persistence or compilation failure;
- partial native result that cannot satisfy the contract.

Do not convert failure into an empty success value unless empty is the
documented successful result.

## Synchronous and asynchronous work

Use synchronous execution only for bounded work that can complete promptly on
the editor thread.

Use an asynchronous contract when the operation must wait for:

- compilation;
- rendering or capture;
- asset processing;
- test execution;
- editor state transition;
- multi-frame or externally completed work.

An async design must define:

- terminal success data;
- terminal error data;
- progress or polling evidence when exposed;
- cancellation behavior;
- resource and callback cleanup;
- timeout consequences;
- post-timeout state inspection.

Do not classify a long operation as synchronous merely to simplify the schema.

## Implementation language decision

Choose the implementation language from actual API coverage and source
ownership, not iteration preference alone.

For each candidate language, record:

- required Unreal APIs;
- reflected type availability;
- editor-thread constraints;
- registration mechanism;
- reload or compilation path;
- test framework and fixtures;
- packaging and plugin ownership;
- missing APIs that would force a workaround.

Prefer the language that expresses the complete typed contract without hidden
bridges. Stop when neither supported language can expose the required behavior
cleanly.

## Registration lifecycle

A toolset is not complete until registration is deterministic.

Define:

- the owning plugin or module;
- startup registration;
- shutdown unregistration where supported;
- reload behavior for iterative development;
- duplicate-registration handling;
- the exact toolset identity exposed by live discovery;
- the plugin state required for availability.

After registration, the live registry must expose the intended toolset exactly
once. A source file existing on disk is not registration evidence.

## Documentation contract

Document the toolset domain in a short description that remains useful before
individual tools are loaded.

For each tool, document only information not obvious from the live schema:

- semantic purpose;
- units and coordinate space;
- ordering;
- empty or null meaning;
- important editor-state requirements;
- destructive or persistent effects;
- cancellation or compile behavior.

Avoid usage essays, obvious examples, implementation rationale, and copied
engine documentation. The generated capability skill will project the current
native description and schema after registration.

## Test design

Every new or changed tool requires evidence for:

- one normal success path;
- every validation branch;
- empty-result behavior;
- duplicate or no-op behavior when applicable;
- target identity and count boundaries;
- persistence, compilation, or activation when applicable;
- cancellation and cleanup for async work;
- registration and live schema discovery.

Use disposable fixtures whenever mutation is involved. Capture pre-state,
perform one bounded mutation, verify through an independent read, restore or
delete the fixture, and prove no residue remains.

Tests must verify contract behavior rather than internal helper calls. A schema
snapshot alone does not prove the editor outcome.

## Live verification sequence

After implementation:

1. compile or reload through the supported project path;
1. confirm registration completed without duplication;
1. run `shar-unreal-mcp doctor`;
1. list toolsets and confirm the exact owner identity;
1. describe the toolset and inspect the new live schema;
1. invoke bounded success and error fixtures;
1. verify postconditions independently;
1. run relevant native automation tests;
1. run translator tests and canonical repository validation;
1. regenerate the Unreal skill catalog;
1. inspect the new or changed per-tool skill;
1. leave SHAR manual fields review-required until evidence is reproduced.

## Catalog integration

A live interface change requires complete regeneration, not a hand-authored
capability page.

Confirm:

- one generated page exists for each native tool identity;
- taxonomy ownership resolves without collision;
- removed identities remove obsolete generated pages;
- protected fields follow unchanged identities;
- changed schemas update the interface digest;
- review counts reflect the new current revision;
- all central-index and local links resolve.

Do not advance manual review tokens merely because implementation tests passed.
Project-specific arguments and caveats must be reproduced through the live
interface.

## Review pass

Before completion, review the toolset as one API rather than isolated methods.
Check for:

- duplicated generic functionality;
- inconsistent identity forms;
- missing reads or inverses;
- hidden broad target behavior;
- unnecessary wrapper types;
- structured data encoded as strings;
- mismatched parameter naming;
- error paths without tests;
- async work without cancellation cleanup;
- descriptions that repeat schemas;
- registration that depends on import accident;
- generated skills that remain stale.

Fix the surface, not only the immediate test failure.

## Completion criteria

The extension is complete only when:

- ownership and implementation source are explicit;
- no existing tool already covers the outcome;
- the API is typed, bounded, and composable;
- success and failure behavior are deterministic;
- registration is visible in the live registry;
- success, error, and cleanup tests pass;
- live `describe` exposes the intended schema;
- generated capabilities and digest are current;
- no installed or ignored source became the only implementation record.

## Stop conditions

Stop when:

- only installed engine or plugin source can be changed;
- an equivalent capability already exists;
- ownership between toolsets is unresolved;
- the API requires JSON strings to represent stable structured data;
- no independent verification route exists for a mutation;
- required behavior cannot be expressed through supported reflected APIs;
- async work has no bounded completion or cancellation design;
- registration or test ownership is unclear;
- implementation would broaden an existing tool incompatibly;
- unrelated editor or repository work overlaps the target files.
