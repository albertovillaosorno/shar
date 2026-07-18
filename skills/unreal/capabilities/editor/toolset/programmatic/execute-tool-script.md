# Execute tool script

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Execution or transient mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.programmatic.ProgrammaticToolset.execute_tool_script
```

Toolset:

```text
editor_toolset.toolsets.programmatic.ProgrammaticToolset
```

## What this tool does

Execute a Python script against the toolset APIs.

Use this to batch multiple tool calls into a single script execution, reducing
round-trips and context usage.

IMPORTANT: Available modules and usage instructions are described by the value
returned by `get_execution_environment`. You MUST call
`get_execution_environment` once in the conversation before using this tool.
Read the value in the `instructions` field in the returned environment info
prior to calling this function, so that you understand what APIs are available
and how to use them.

Before writing a script that calls multiple tools, look up the output schemas
(if available) for any tools you plan to use. This returns the JSON schema
describing each tool's return value, so you know how to parse results and pass
data between calls.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Confirm execution scope, cancellation behavior, and expected side effects
before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to compose multiple bounded read-only Unreal MCP calls into one
SHAR programmatic script.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Call `get_execution_environment` in the current conversation and read its
  instructions completely.
- Refresh the live schema and output schema for every registered tool the
  script calls.
- Keep the script read-only or fully disposable, and independently repeat its
  returned reads through direct tool calls.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "script": "import json

def get_priority():
    return execute_tool(
        \"animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_priority_order\",
        json.dumps({\"sequence\": {\"refPath\": \"/Game/SHAR_MCP_Validation_ControlRig_Large_260718/LS_MCP_Large_260718.LS_MCP_Large_260718\"}, \"control_rig_asset_path\": \"/Game/SHAR_MCP_Validation_ControlRig_Large_260718/CR_MCP_Large_260718\"})
    )[\"returnValue\"]

def get_visible():
    return execute_tool(
        \"animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_controls_mask\",
        json.dumps({\"section\": {\"refPath\": \"/Game/SHAR_MCP_Validation_ControlRig_Large_260718/LS_MCP_Large_260718.LS_MCP_Large_260718:MovieScene_0.MovieSceneControlRigParameterTrack_0.MovieSceneControlRigParameterSection_0\"}, \"control_name\": \"TransformControl\"})
    )[\"returnValue\"]

def run():
    return {\"priority\": get_priority(), \"visible\": get_visible()}
"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The script returned JSON with priority 17 and visibility true. Independent
direct calls to the same two native getters returned the same values.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The programmatic environment is runtime authority and may change with the
  live toolset revision.
- Registered tool calls remain subject to their own editor-state, mutation,
  and verification contracts.
- The return is a JSON-encoded string and requires a second parse.
- Only modules listed by `get_execution_environment` are allowed; raw Unreal
  Python and filesystem access are not implied.
- Inspect every called tool output schema before parsing its result inside the
  script.
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
shar-unreal-mcp describe editor_toolset.toolsets.programmatic.ProgrammaticToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `script`

- Required: **yes**
- Type: `string`
- Purpose:

Python script to execute. Must define a `run()` function that returns a
`Dict[str, Any]`.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.programmatic.ProgrammaticToolset \
  editor_toolset.toolsets.programmatic.ProgrammaticToolset.execute_tool_script \
  --arguments '
{
  "script": "<value>"
}
'
```

## Expected output

JSON-encoded dict returned by the script's `run()` function.

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

Value of the result.

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

Native failure guidance:

SyntaxError: If the script has invalid syntax. ValueError: If the script
imports a disallowed module or does not define a `run()` function. TypeError:
If `run()` does not return a dict. Exception: Any unhandled exception raised by
the script.
