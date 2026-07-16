# Capability selection

Read [`../index.md`](../index.md) before using this workflow.

## Goal

Choose the narrowest native tool that expresses the requested outcome without
guessing names, loading unrelated skills, or selecting a broad mutation when a
specific read or focused mutation exists.

## Inputs

Before selecting a tool, state:

- requested outcome;
- target project object or asset family;
- expected read or mutation posture;
- maximum target count;
- required postcondition evidence;
- declared change boundary for persistent state.

When any of these are unknown, perform discovery before selection.

## Selection procedure

1. Translate the operator request into one observable postcondition.
1. Find the closest domain heading in the central index.
1. Compare native tool names under the relevant toolsets.
1. Open one candidate per-tool skill.
1. Read purpose, safety posture, inputs, output, and verification sections.
1. Review protected SHAR fields without treating placeholders as evidence.
1. Compare alternatives only when ownership or scope remains ambiguous.
1. Choose the narrowest tool with a verifiable postcondition.
1. Refresh the selected toolset schema before invocation.

## Manual guidance status

A completed protected field is reviewed SHAR-specific evidence. `[TODO]` and
`[FILL_ME]` mean no project guidance has been established. They are not
a validated example, current evidence, or a schema guarantee.

The live schema remains authoritative even when manual guidance is complete.
Stop and update guidance when it conflicts with the current interface.

## Capability preference order

Prefer, in order:

1. dedicated `get`, `list`, `find`, `inspect`, `query`, or `validate` tools;
1. a focused create or update tool for one object or asset;
1. a native batch tool with explicit bounded targets and per-item results;
1. sequential individual calls over a reviewed target set;
1. programmatic execution only when it is more deterministic and reviewable.

Do not choose programmatic execution merely because it can call many APIs.

## Ownership distinctions

Similar names can belong to materially different contexts. Compare:

- toolset identity;
- accepted native object or asset type;
- required editor or graph context;
- world versus asset ownership;
- sequence, track, section, channel, or key ownership;
- class defaults versus instance state;
- persistent asset mutation versus transient editor state;
- output schema and available verification tools.

Examples that require deliberate distinction:

- actor operations versus generic UObject operations;
- Material assets versus Material Instances;
- Static Mesh versus Skeletal Mesh tools;
- Sequencer tracks versus Control Rig channels;
- Blueprint graph editing versus Blueprint asset metadata;
- Game Feature discovery versus activation and deactivation.

## Read versus mutation classification

A read-like verb is useful evidence but not a safety guarantee. Inspect the live
description and schema for:

- save, compile, refresh, activate, load, or cache effects;
- selection or editor-state changes;
- lazy discovery or registration;
- hidden batch scope;
- asynchronous work.

Classify the operation by actual side effects, not only by its name.

## Scope comparison

For each candidate, compare:

- required identifiers;
- optional filters and limits;
- target cardinality;
- whether omitted fields broaden scope;
- whether output identifies every affected item;
- available inverse or repair tools;
- independent postcondition reads.

Reject a tool whose minimum scope is broader than the approved operation.

## Selection record

Before a persistent mutation, retain:

- selected toolset and tool identities;
- reason narrower alternatives were insufficient;
- live schema timestamp or digest;
- approved target scope;
- expected output and postcondition;
- verification tool;
- recovery or inverse operation.

## Common selection mistakes

- Selecting from memory without reading the index.
- Matching only the final verb while ignoring the owning toolset.
- Treating a generated invocation example as a project-valid example.
- Choosing a broad batch tool for one target.
- Using a mutating tool to discover whether a target exists.
- Selecting by documentation prose after the live schema changed.
- Assuming a completed manual field applies to another Unreal version.

## Decision outcomes

### One clear capability

Open the exact skill, refresh its schema, and continue to the appropriate read
or
mutation workflow.

### Two plausible capabilities

Run read-only discovery or schema comparison. Do not test both mutations on the
project merely to learn which one applies.

### No indexed capability

Regenerate the catalog. If the live tool exists but lacks taxonomy ownership,
stop for reviewed taxonomy assignment.

### No native capability

Document the gap. Do not silently fall back to custom plugin code or arbitrary
editor scripting without a separate architecture decision.

## Stop conditions

Stop selection when:

- the live editor lists an unindexed toolset or tool;
- skill digest and live catalog disagree;
- schema fields no longer match the skill;
- two tools normalize to one path;
- target scope cannot be bounded;
- declared change scope does not cover the selected side effects;
- independent verification is unavailable for a persistent mutation.
