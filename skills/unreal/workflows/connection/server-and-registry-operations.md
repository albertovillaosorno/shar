# Server and registry operations

Read the [central Unreal MCP index](../../index.md), the
[workflow map](../README.md), and
[editor readiness](editor-readiness.md) before using this workflow.

## Goal

Operate and diagnose the native Unreal MCP server and Toolset Registry without
using restart, refresh, or retry as a substitute for identifying the current
editor, session, request, and mutation state.

## Normal operating surface

Routine work uses the translator:

```text
shar-unreal-mcp doctor
shar-unreal-mcp toolsets
shar-unreal-mcp describe TOOLSET_IDENTITY
shar-unreal-mcp call TOOLSET_IDENTITY TOOL_IDENTITY --arguments '{}'
```

The normal tool-search surface contains only:

- `list_toolsets`;
- `describe_toolset`;
- `call_tool`.

Native leaf tools are dispatched through `call_tool`; they are not expected to
appear as top-level MCP tools. A top-level inventory containing only the three
meta-tools can be healthy when the Toolset Registry itself is complete.

## Operational states

Classify the server before acting.

### Not started

The intended editor is running, but no canonical MCP endpoint accepts a
session. Confirm project identity and tracked auto-start settings before a
manual start is considered.

### Transport reachable, registry unavailable

The endpoint negotiates transport, but required meta-tools or registered
toolsets are missing. Treat this as incomplete plugin or registry startup, not
normal readiness.

### Registry ready

`doctor` reports `ready: true`, the missing-meta-tool list is empty, and the
registry contains one or more toolsets.

### Busy

The server is valid, but the editor is compiling, loading, processing an async
request, running PIE, or performing another serialized operation.

### Ambiguous

A request timed out, the connection dropped, or a response cannot prove whether
native work continued. Do not restart or repeat the mutation until state is
inspected independently.

## Server lifecycle boundary

Starting or stopping the native server is a recovery action, not a routine first
step. Before a lifecycle action:

1. Identify the intended interactive editor.
1. Confirm the current server state.
1. Confirm no mutation or async operation is active.
1. Capture the last request identity and result state.
1. Confirm the action will not terminate the editor.
1. Define the readiness check that follows.

Never stop the server to clear an ambiguous mutation. First inspect the target
state and active request evidence.

A project configuration change can require an operator-controlled editor
restart. Do not automate termination of an interactive process that the current
operation did not launch.

## Tool-search mode

SHAR keeps native tool-search mode enabled. This provides a stable top-level MCP
surface while schemas remain discoverable on demand.

Do not disable tool search merely to make every native tool visible at once. A
full top-level catalog increases context, can invalidate cached assumptions, and
is not the repository's normal dispatch contract.

When a capability is needed:

1. obtain the current toolset inventory;
1. select one candidate toolset;
1. describe that toolset;
1. identify the exact native tool;
1. validate arguments;
1. dispatch through the translator.

## Registry refresh boundary

A registry refresh is appropriate only after a known plugin or registration
change in the current editor session. Before refresh:

- capture the current toolset inventory;
- identify the expected added, removed, or changed toolset;
- confirm plugin loading has completed;
- confirm no tool invocation is active;
- define the exact post-refresh comparison.

After refresh:

1. list toolsets again;
1. compare exact identities and counts;
1. describe the changed toolset;
1. regenerate skills when the live interface changed;
1. validate the new digest and manual review status.

A refresh cannot load a plugin that the running editor never loaded. Plugin
availability changes can require an editor restart.

## Port collision

When the canonical endpoint cannot bind:

1. identify the process owning port `8000`;
1. determine whether it is the intended editor, another editor, or an unrelated
   process;
1. preserve the canonical loopback-only boundary;
1. avoid choosing a random port before the owning process is understood;
1. update repository configuration only for a durable port change;
1. rerun project configuration tests after a tracked change;
1. restart the intended editor only through the operator-controlled path;
1. rerun `doctor` against the final endpoint.

Do not kill an unknown process as port cleanup.

## Missing toolset

When a known toolset is absent:

1. verify the editor opened the canonical project;
1. verify required plugin entries in the project descriptor;
1. inspect editor plugin-load diagnostics;
1. compare discovered and enabled plugin inventories when available;
1. determine whether the toolset is installed, enabled, and registered;
1. refresh only after a known registration change;
1. restart only after a plugin-load or project configuration change;
1. regenerate the skill catalog if the final live surface changed.

Do not invoke a similarly named toolset as a substitute.

## Stale schema or digest

When a generated skill conflicts with live `describe` output:

- stop invocation;
- retain the live schema and interface digest;
- determine whether the installed plugin version changed;
- regenerate the complete skill catalog;
- review taxonomy ownership and protected-field preservation;
- leave mismatched guidance as review-required until reproduced.

Do not patch a generated capability page manually outside protected fields.

## Busy editor and compilation

A busy editor can make a healthy server appear unresponsive. Before retrying:

- check PIE state;
- check asset, shader, C++, Blueprint, or automation compilation;
- check active async operations;
- check level or asset loading;
- check whether the translator already issued cancellation after timeout.

Wait for a known operation to reach a terminal state. Do not issue overlapping
calls to probe progress unless the selected workflow defines a polling tool.

## Timeout and cancellation

The translator sends native cancellation for timed-out requests when supported.
A client timeout still means the command failed; it does not prove the native
operation stopped before changing state.

After timeout:

1. preserve request and cancellation evidence;
1. inspect active-operation status when available;
1. read the intended target state independently;
1. classify no change, completed change, partial change, or unknown state;
1. recover before any retry;
1. use a longer timeout only when bounded duration justifies it.

## PIE and editor context

Some tools require editor-only state or behave differently during PIE. When a
call fails unexpectedly:

1. read PIE status;
1. determine whether the selected capability supports the current mode;
1. stop PIE only through an explicit editor operation with known pre-state;
1. rerun readiness after the mode transition;
1. do not infer that every failure during PIE is caused by PIE.

Open asset editors, current selection, active world, and loaded packages can
also change local readiness without changing global server health.

## Diagnostics

Prefer structured translator and tool output. Use editor logs to supplement, not
replace, protocol evidence.

When logs are needed:

- bound the time range;
- filter to the native server, registry, plugin load, or request identity;
- avoid retaining complete unrelated logs;
- distinguish warning, recoverable error, and terminal failure;
- correlate messages with the exact editor process and request.

An old startup message from another editor instance is not current evidence.

## Operational evidence

Record:

- intended project and editor process;
- endpoint and protocol version;
- doctor result;
- toolset inventory before and after any refresh;
- plugin or registration change that justified the operation;
- active request or compilation state;
- timeout and cancellation evidence;
- final schema digest;
- final readiness result.

## Completion criteria

Server or registry recovery is complete only when:

- the intended editor owns the endpoint;
- `doctor` reports ready;
- all required meta-tools exist;
- the registry is nonempty and internally consistent;
- expected toolsets are present by exact identity;
- no ambiguous request remains unresolved;
- generated skills match the live interface when the catalog changed;
- operation-specific readiness can resume.

## Stop conditions

Stop when:

- the endpoint owner cannot be identified;
- a mutation may still be running;
- a restart would destroy unverified editor state;
- a refresh is being used without a known registration change;
- a required plugin is not installed;
- recovery would weaken loopback, session, or protocol validation;
- the live registry and installed plugin set cannot be reconciled;
- an unrelated interactive editor would need to be terminated.
