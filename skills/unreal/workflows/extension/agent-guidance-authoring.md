# Agent guidance authoring

Read the [central Unreal MCP index](../../index.md), the
[workflow map](../README.md), and
[manual guidance maintenance](../maintenance/manual-guidance-maintenance.md)
before using this workflow.

## Goal

Create or revise reusable Unreal guidance in the correct SHAR-owned surface
without confusing generated capability pages, manual workflow runbooks, or
runtime Unreal AgentSkills.

## Three distinct guidance surfaces

SHAR uses three different forms of guidance. Choose exactly one owner before
writing.

### Generated capability skill

A generated capability page documents one native tool identity under
`skills/unreal/capabilities/**`.

- the generated shell comes from the live Toolset Registry;
- only protected manual fields are editable;
- the live schema remains authoritative;
- one native identity owns one generated page;
- regeneration controls names and paths.

Use this surface for reproduced SHAR-specific use cases, prerequisites,
arguments, verification evidence, caveats, and review revision.

### Manual workflow runbook

A workflow under `skills/unreal/workflows/**` owns a reusable procedure that
spans capabilities or defines a stable operating stage.

- the complete Markdown file is manually maintained;
- one file owns one lifecycle responsibility;
- the workflow must remain useful across native schema refreshes;
- exact volatile schemas belong in generated capability pages;
- the root workflow map and central index own navigation.

Use this surface for editor readiness, planning, execution, assurance,
maintenance, connection, or extension procedure.

### Runtime Unreal AgentSkill

A runtime AgentSkill is registered with Unreal's AgentSkill registry and loaded
inside the editor context. It can be native, plugin-backed, or asset-backed.

Use the live `ToolsetRegistry.AgentSkillToolset` capabilities to list, inspect,
create, or update these skills. Runtime AgentSkills are not substitutes for
repository workflow files and do not own general SHAR operating rules.

## Routing decision

Before authoring, answer:

1. Is the content about one native tool identity?
1. Is it a reusable procedure across several capabilities?
1. Must Unreal load it dynamically at runtime?
1. Is the content repository-specific or plugin-specific?
1. Does an existing guidance surface already own the topic?
1. Can the useful rule remain stable across tool renames and schema changes?

Route to:

- protected capability fields for one native tool;
- a workflow runbook for repository operating procedure;
- a runtime AgentSkill for editor-loaded project or plugin guidance.

Do not duplicate the same rule across all three surfaces.

## Discovery before creation

Before adding guidance:

1. search the workflow map and all workflow headings;
1. search generated capability identities and protected fields;
1. run `ToolsetRegistry.AgentSkillToolset.ListSkills` when runtime guidance is
   relevant;
1. load only plausible runtime skills with `GetSkills`;
1. identify the current owner and missing knowledge;
1. distinguish missing guidance from missing native capability;
1. define the exact audience and activation condition;
1. record the target surface before drafting.

A missing native operation requires toolset design, not more instructions.

## Guidance quality principles

Reusable guidance should be:

- **novel**: it contributes knowledge not recoverable from live schema alone;
- **evidenced**: project facts come from reproduced editor or repository state;
- **durable**: it survives routine tool and schema evolution;
- **bounded**: it covers one responsibility and one activation condition;
- **actionable**: it gives a clear sequence, evidence route, and stop condition;
- **context-efficient**: every section earns its place;
- **neutral**: it avoids harness, model, role, or client-specific assumptions;
- **recoverable**: mutating guidance includes verification and ambiguity
  handling.

Do not fill guidance with generic Unreal explanations or signatures already
available through `describe`.

## Generated capability guidance

When the target is a generated capability page:

1. read the live toolset schema;
1. reproduce the selected behavior in the canonical project;
1. capture independent verification;
1. edit only the complete protected field set;
1. use project-safe identities and examples;
1. advance `manual-review-revision` only after every field is reviewed;
1. hash protected fields before regeneration;
1. regenerate the complete catalog;
1. prove byte-for-byte preservation;
1. run scoped validation and the complete MCP tests.

Never edit generated purpose, schema, invocation, path, or navigation text by
hand.

## Workflow runbook authoring

A new workflow is justified only when the procedure:

- spans more than one native capability or toolset;
- has stable steps independent of one schema;
- owns a lifecycle stage not covered by another runbook;
- requires reusable stop conditions or evidence rules;
- has a clear folder in the workflow taxonomy.

Before creating a workflow:

1. identify the single stable responsibility;
1. compare every existing workflow for overlap;
1. choose connection, planning, execution, assurance, maintenance, or extension;
1. define entry conditions and deterministic handoffs;
1. define authority order and evidence requirements;
1. define failure classes and stop conditions;
1. update the workflow map and central index renderer;
1. add or update structural regression tests.

Do not create nested `index.md` files. Use `workflows/README.md` as the only
manual workflow map.

## Runtime AgentSkill authoring

Runtime AgentSkills should capture editor-loaded knowledge that is genuinely
useful at dispatch time, such as:

- project naming and folder rules;
- a project-specific multi-step editor sequence;
- plugin-specific domain constraints;
- required setup that native schemas cannot express;
- caveats that must travel with a runtime plugin or asset.

They should not contain:

- a copied native tool catalog;
- repository Git procedure;
- generated capability documentation;
- generic instructions to discover or call tools;
- workstation paths or client setup;
- transient test evidence;
- broad policy unrelated to the editor task.

## Runtime description and instructions

A runtime skill has two conceptual layers.

### Routing description

The routing description must let an agent decide whether to load the skill
without reading its full body.

It should state:

- the domain or workflow covered;
- the concrete task conditions that activate it;
- important exclusions when a neighboring skill owns similar work.

Keep it short. Do not place the complete procedure in the routing description.

### Instruction body

The instruction body should contain only the durable knowledge needed after
activation:

- prerequisites not visible in schemas;
- required sequence and ordering;
- project identities or naming patterns that are stable;
- verification and recovery expectations;
- limits and known unsupported cases;
- handoffs to other guidance when responsibility ends.

Do not hardcode a full toolset list. Runtime tool discovery remains live.

## Runtime implementation path

Choose asset-backed or plugin-backed runtime guidance from ownership and
lifecycle.

Use an asset-backed skill when guidance is specific to the project and should be
managed as project content.

Use a plugin-backed skill when guidance belongs to a maintained plugin and must
version with its implementation.

For either path, define:

- stable identity;
- registration or asset discovery;
- update mechanism;
- load and unload behavior;
- exact verification through `ListSkills` and `GetSkills`;
- removal or rollback path;
- source ownership and persistence.

Do not create both forms for the same responsibility.

## Creation and update procedure

For a runtime skill:

1. complete editor readiness;
1. list current runtime skills;
1. inspect every plausible existing owner;
1. refresh the live AgentSkill toolset schema;
1. choose one exact asset or plugin-backed identity;
1. write a concise routing description;
1. write bounded instructions with verification and stop conditions;
1. create or update exactly once;
1. inspect the returned identity and result;
1. list skills again;
1. load the exact skill through `GetSkills`;
1. compare persisted description and instructions with the intended text;
1. verify save or plugin reload behavior separately;
1. remove the disposable fixture when the operation was a test.

Do not treat a successful create or update response as the only evidence.

## Review questions

Review the completed guidance as the consuming agent will encounter it.

For routing:

- Can the activation condition be decided from the description alone?
- Does it distinguish neighboring domains?
- Is it short enough to scan in an inventory?
- Does it avoid generic phrases that match every Unreal task?

For instructions:

- Does every rule add information beyond the live tools?
- Are project-specific claims reproduced?
- Are identity forms exact and stable?
- Is the sequence deterministic?
- Are mutation verification and recovery explicit?
- Are unsupported cases and responsibility handoffs clear?
- Can any paragraph be removed without losing behavior?

For ownership:

- Is the content in exactly one guidance surface?
- Will regeneration overwrite any part of it?
- Does a runtime skill duplicate a repository workflow?
- Does the guidance belong with a plugin rather than project content?

## Testing and verification

Guidance requires tests appropriate to its surface.

For generated capability fields:

- marker completeness;
- protected-field preservation;
- current revision status;
- scoped Markdown validation;
- live argument and postcondition evidence.

For workflow runbooks:

- exact taxonomy path;
- required sections;
- minimum depth;
- local link resolution;
- central-index routing;
- absence of competing workflow maps.

For runtime AgentSkills:

- exact inventory identity;
- routing description round-trip;
- instruction-body round-trip;
- plugin reload or asset persistence;
- duplicate handling;
- missing identity behavior;
- cleanup or rollback.

The test should prove the guidance can be discovered and consumed, not merely
that a source file exists.

## Context budget

Runtime instruction bodies can be large. Load only the guidance relevant to the
current task and avoid creating oversized skills that combine unrelated
procedures.

Split guidance when two responsibilities activate under different conditions or
change for different reasons. Do not split only to satisfy an arbitrary line
count.

A workflow runbook can be detailed because it is opened deliberately. A runtime
routing description must remain concise because inventories can include many
skills.

## Maintenance and retirement

When guidance becomes stale:

1. identify the stronger current authority;
1. migrate durable evidence to the correct surface;
1. update links and routing descriptions;
1. remove duplicate or superseded runtime guidance;
1. regenerate capability pages when the native interface changed;
1. rerun discovery and link tests;
1. prove no stale identity remains in the registry or repository.

Do not leave two active files claiming the same responsibility.

## Completion criteria

Guidance authoring is complete only when:

- the target surface is correct;
- no current guidance already owns the responsibility;
- content adds durable knowledge beyond live schemas;
- routing and instructions are bounded and distinct;
- project claims are evidenced;
- navigation or runtime discovery resolves the exact identity;
- persistence and round-trip verification pass;
- duplicate authority has not been introduced;
- generated content remains generator-owned;
- no YAML frontmatter or harness-specific metadata was added to workflows.

## Stop conditions

Stop when:

- the target guidance surface is ambiguous;
- the request is actually for a missing native capability;
- existing guidance already covers the responsibility;
- project facts cannot be reproduced;
- a runtime skill would duplicate repository workflow authority;
- a workflow would duplicate one generated capability page;
- the content depends on one client, model, or workstation path;
- the only implementation source is installed or untracked code;
- round-trip discovery or persistence cannot be verified;
- the draft contains unrelated responsibilities that need separate owners.
