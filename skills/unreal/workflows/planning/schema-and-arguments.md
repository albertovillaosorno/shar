# Schema and arguments

Read the [central Unreal MCP index](../../index.md), the
[workflow map](../README.md), and the selected per-tool skill before
using this workflow.

## Goal

Build one JSON argument object that matches the current live schema exactly,
uses validated SHAR project identities, and keeps the requested scope as narrow
as possible.

## Authority order

Use this order when sources disagree:

1. current live `describe` schema;
1. native tool error and structured validation output;
1. completed protected SHAR guidance reproduced for the current version;
1. generated per-tool schema summary;
1. generic workflow examples.

Never preserve an old argument merely because it appeared in a generated file.

## Refresh the schema

Run:

```text
shar-unreal-mcp describe TOOLSET_IDENTITY
```

Locate the exact native tool identity and inspect:

- `inputSchema.type`;
- `inputSchema.properties`;
- `inputSchema.required`;
- nested object properties;
- array item schemas;
- defaults, enums, minimums, maximums, and patterns;
- output fields required for verification.

The per-tool skill summarizes top-level fields. The live schema is authoritative
for nested structures and interface changes.

## Protected validated example

When the selected skill's `Validated argument example` field is complete, treat
it as reviewed project evidence, not a copy-and-run command.

Before adapting it:

1. Run live `describe`.
1. Compare every field name and type.
1. Replace task-specific identities.
1. Reconfirm target scope and state-changing effects.
1. Remove optional fields whose behavior is not needed.

When the field still contains `[FILL_ME]`, build arguments from the live schema.
Do not replace the placeholder until the call succeeds and its postcondition is
independently verified.

## Build the top-level object

1. Start from `{}`.
1. Add every required field.
1. Preserve native JSON types.
1. Replace all generated placeholders.
1. Add optional fields only when their behavior is understood.
1. Apply the narrowest filters and limits available.
1. Reject unknown fields copied from old schemas.

Do not use `null`, an empty string, zero, or an empty array as a generic
substitute
for a missing required value.

## String identities

Different tools can require different string identity forms:

- asset paths;
- object paths;
- package paths;
- actor labels or names;
- class paths;
- graph, node, pin, track, section, channel, or control names;
- Gameplay Tags;
- plugin or Game Feature names.

Do not infer one identity form from another. Use a read tool to obtain the exact
native value when possible.

## Nested objects

For each nested object:

1. Inspect its own `properties` and `required` fields.
1. Preserve the nested structure.
1. Validate every enum and numeric constraint.
1. Avoid additional fields not accepted by the schema.
1. Keep vectors, transforms, colors, ranges, and settings in native structure.

Do not flatten nested fields or serialize nested JSON as a string unless the
schema explicitly requires a string.

## Arrays and target sets

For arrays:

- confirm the item type;
- preserve order when semantically meaningful;
- deduplicate identities when duplicate effects are unsafe;
- enforce the approved maximum target count;
- reject an empty array when the native operation expects at least one target;
- inspect whether omission means “all targets.”

For batch mutations, record the final ordered target set before invocation.

## Numbers and units

Confirm:

- integer versus number;
- frame number versus seconds;
- local versus global coordinates;
- degrees versus radians;
- normalized versus absolute values;
- inclusive versus exclusive ranges;
- Unreal unit assumptions.

Do not rely on parameter names alone to infer units.

## Booleans and defaults

An omitted optional boolean can differ from explicit `false`. Inspect the schema
and native description before deciding whether to omit or set it.

Treat defaults as interface facts, not necessarily project-safe choices. Include
a default explicitly only when doing so makes intent clearer and the value is
still accepted by the live schema.

## Enums and constrained strings

Use only values present in the live enum or pattern. Preserve spelling and case.
When the generated skill contains a long enum list, refresh `describe` instead
of
assuming the checked-in list is current.

## Command construction

Generic shape:

```text
shar-unreal-mcp call \
  TOOLSET_IDENTITY \
  TOOL_IDENTITY \
  --arguments '{
    "requiredField": "validated value"
  }'
```

The shell must pass one valid JSON object to `--arguments`. Keep shell quoting
separate from JSON validity.

### Git Bash

Use single quotes around JSON when values do not require shell interpolation.

### PowerShell

Construct or quote the JSON so the resulting process receives one exact JSON
object. Inspect the final serialized value instead of changing field types to
work around quoting.

## Pre-invocation review

Confirm:

- every required field is present;
- every field exists in the live schema;
- types and nesting are correct;
- no generated placeholder remains;
- identities belong to the intended project;
- scope and cardinality are approved;
- optional fields do not silently broaden behavior;
- expected output fields are understood;
- verification and recovery are defined.

## Validation failure handling

When the native tool rejects arguments:

1. Preserve the exact error.
1. Re-run `describe`.
1. Compare the rejected path and expected type.
1. Correct only the invalid field.
1. Do not remove required validation or add broad defaults.
1. Recheck scope before retrying.

A validation error before mutation is safer than guessing a correction during a
partially completed operation.

## Promotion to manual guidance

After successful independent verification, record a repository-safe example in
the protected `Validated argument example` field. Include only the minimum
fields
needed to demonstrate the reproduced SHAR use case.

## Stop conditions

Stop before invocation when:

- the schema cannot be refreshed;
- a required identity cannot be obtained reliably;
- a placeholder remains;
- nested or array semantics are unknown;
- units are ambiguous;
- omission may broaden scope to all assets or objects;
- the argument set exceeds the declared target scope;
- the example conflicts with the current schema.
