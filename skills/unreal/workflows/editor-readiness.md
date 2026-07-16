# Editor readiness

Read [`../index.md`](../index.md) before using this workflow.

## Goal

Prove that the intended Unreal Editor instance, SHAR project, native MCP server,
and Toolset Registry are ready before discovery, reads, mutations, skill
regeneration, or troubleshooting.

## Readiness is more than an open port

A listening endpoint proves only that one process accepted a socket. Complete
readiness requires agreement between:

- the intended `.uproject` and editor process;
- enabled native plugins and project settings;
- negotiated MCP protocol and session behavior;
- Toolset Registry meta-tools;
- generated skill digest and live catalog;
- operation-specific editor context.

## Required project configuration

Confirm:

- the intended project is open in Unreal Engine 5.8;
- `ModelContextProtocol` is enabled;
- `AllToolsets` is enabled;
- `MCPClientToolset` remains disabled unless separately reviewed;
- the editor was restarted after plugin or MCP configuration changes;
- the endpoint is loopback HTTP, normally
  `http://127.0.0.1:8000/mcp`;
- server auto-start and Tool Search are enabled;
- Asset Manager settings contain the required `GameFeatureData` definition.

The configured architecture uses the installed native plugin; do not patch it
as a readiness workaround.

## Phase 1: identify the intended editor

Before opening an MCP session:

1. Identify the project shown by the interactive editor.
1. Identify running Unreal editor and commandlet processes.
1. Determine which process owns the configured MCP port.
1. Confirm that process belongs to the intended project.
1. Distinguish the operator's interactive editor from commandlets created by the
   current validation or test run.

Never terminate the operator's interactive editor as cleanup. Stop only a
headless process created and owned by the current operation.

## Phase 2: transport and session check

Run:

```text
shar-unreal-mcp doctor
```

Require:

- `ready` is `true`;
- JSON-RPC 2.0 is accepted;
- protocol version `2025-11-25` is negotiated;
- a visible-ASCII session identifier is returned;
- initialization completes before any tool request;
- the session closes cleanly;
- the missing-meta-tool list is empty;
- `toolsetCount` is greater than zero.

Unreal Engine 5.8 can return empty `serverInfo.name`, `serverInfo.title`, and
`serverInfo.version` strings. Do not treat those informational empty strings as
failure. Protocol negotiation, session evidence, and registry meta-tools are the
readiness authority.

## Phase 3: Toolset Registry check

The live server must expose:

- `list_toolsets`;
- `describe_toolset`;
- `call_tool`.

Run a live toolset listing and compare:

- toolset count;
- exact registry identities;
- expected enabled domains;
- generated central-index digest.

A partial registry surface is not ready for normal operation. Do not continue
with a convenient subset when `AllToolsets` failed to load completely.

## Phase 4: operation context check

Readiness for one tool may require more than global MCP readiness. Confirm the
selected skill's prerequisites, including applicable:

- active world or level;
- selected actor, node, track, control, or asset;
- open Blueprint, Material, Sequencer, Niagara, PCG, or Control Rig editor;
- completed discovery, compilation, save, or refresh state;
- loaded plugin or Game Feature state;
- absence of an overlapping mutation or compilation task.

A globally ready editor can still be locally unready for the requested tool.

## Connection failure decision tree

### Connection refused

1. Confirm the intended editor is running.
1. Check whether a different process owns the port.
1. Confirm server auto-start is enabled.
1. Restart the editor after configuration changes.
1. Inspect the editor log for MCP route registration.
1. Retry `doctor` once the intended process is confirmed.

### HTTP or Origin rejection

- Confirm the endpoint is loopback HTTP.
- Confirm the translator sends the canonical loopback Origin.
- Do not weaken Origin or loopback validation.
- Reject remote or wildcard binding as a troubleshooting shortcut.

### Session or protocol rejection

- Confirm both sides use protocol `2025-11-25`.
- Inspect session and protocol headers.
- Confirm the editor was restarted after plugin updates.
- Do not bypass initialization, session, or JSON-RPC validation.

### Missing Toolset Registry meta-tools

- Confirm Tool Search is enabled.
- Confirm `AllToolsets` loaded successfully.
- Inspect plugin-load errors in the editor log.
- Treat the editor as not ready until all three meta-tools are present.

### Catalog or digest mismatch

- Stop capability selection.
- Run live discovery and schema inspection.
- Regenerate the skills.
- Review taxonomy ownership failures.
- Do not invoke a stale or unindexed tool from memory.

## Readiness evidence record

Retain enough evidence to reproduce the conclusion:

- endpoint;
- editor process identity;
- project identity;
- `doctor` JSON output;
- protocol version and session result;
- toolset count and required meta-tools;
- relevant editor log lines;
- plugin configuration;
- whether the editor was restarted;
- operation-specific prerequisite state.

## Readiness completion criteria

Readiness is complete only when:

- the intended interactive editor owns the endpoint;
- `doctor` reports ready;
- all required meta-tools exist;
- live catalog and generated index agree;
- the selected tool's editor context is satisfied;
- no overlapping mutation makes execution nondeterministic.

## Stop conditions

Stop before any tool call when:

- the endpoint belongs to an unknown process or project;
- protocol or session validation fails;
- the registry is partial;
- the generated digest is stale;
- a required editor context is missing;
- completing readiness would require modifying Epic plugin source;
- cleanup would require terminating a process not launched by the current
  operation.
