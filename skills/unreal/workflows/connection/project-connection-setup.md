# Project connection setup

Read the [central Unreal MCP index](../../index.md) and the
[workflow map](../README.md) before using this workflow.

## Goal

Establish or repair the repository-owned connection between the canonical SHAR
Unreal project, the native inbound MCP server, and the terminal translator
without making client-specific configuration a repository authority.

## When this workflow applies

Use this workflow when:

- the canonical project has never exposed the native MCP server;
- tracked plugin or editor defaults drifted;
- a fresh workstation cannot complete `shar-unreal-mcp doctor`;
- the endpoint belongs to the wrong project or editor process;
- the registry starts with no toolsets;
- a repository update changed the expected project integration.

Do not use this workflow for an ordinary temporary connection failure. Route a
running but unhealthy server through
[server and registry operations](server-and-registry-operations.md).

## Repository authorities

Use these repository-owned sources in this order:

1. `src/uproject/shar.uproject` for project and plugin posture.
1. `src/uproject/Config/DefaultEditorPerProjectUserSettings.ini` for tracked
   server defaults.
1. `src/mcp/README.md` for translator architecture and operator commands.
1. `src/mcp/tests/test_project_configuration.py` for enforced integration
   invariants.
1. live `shar-unreal-mcp doctor` output for the running session.

Do not replace these authorities with a generic client guide, copied plugin
source, remembered console command, or workstation-specific path.

## Canonical project identity

The project entry point is:

```text
src/uproject/shar.uproject
```

Before changing configuration:

1. Confirm the interactive editor is intended to open that project.
1. Confirm no commandlet is being mistaken for the interactive editor.
1. Confirm the project descriptor is readable JSON.
1. Capture the current plugin entries.
1. Capture the tracked MCP settings file.
1. Record whether the editor predates the current configuration.

Do not edit an arbitrary `.uproject` selected from a recent-file list.

## Plugin posture

The tracked project descriptor must express:

- `ModelContextProtocol` enabled for the native inbound server;
- `AllToolsets` enabled for complete installed toolset aggregation;
- `MCPClientToolset` disabled as a direct project entry.

The outbound client adapter is not the normal SHAR connection route. The
terminal translator connects to Unreal's inbound server. A future architecture
change requires a separate repository decision and its own tests.

After a descriptor change, the interactive editor must be restarted by the
operator before plugin-load evidence can be trusted. Automation must not
terminate an editor process it did not launch.

## Tracked server defaults

The tracked editor defaults must retain:

```ini
[/Script/ModelContextProtocolEngine.ModelContextProtocolSettings]
ServerUrlPath=/mcp
ServerPortNumber=8000
bAutoStartServer=True
bEnableToolSearch=True
```

These defaults establish:

- loopback HTTP transport;
- the canonical `/mcp` route;
- automatic server startup with the project;
- native tool-search mode with three meta-tools.

Do not broaden the bind address, remove loopback validation, disable tool
search, or add secret material to tracked editor settings.

## Translator boundary

`src/mcp` is a terminal translator, not a replacement MCP server. Its normal
operator surface is:

```text
shar-unreal-mcp doctor
shar-unreal-mcp toolsets
shar-unreal-mcp describe TOOLSET_IDENTITY
shar-unreal-mcp call TOOLSET_IDENTITY TOOL_IDENTITY --arguments '{}'
```

Use the repository-managed Python environment and package entry point declared
by the repository. Do not make system Python or a client-generated connection
file the canonical integration layer.

Client applications can maintain local connection records, but those records
are outside the repository workflow authority. The repository proves its
connection through the translator and native protocol session.

## Bootstrap sequence

Perform this sequence once per setup or configuration repair:

1. Validate the canonical project descriptor.
1. Validate the tracked editor settings.
1. Confirm installed native plugins are available to the editor build.
1. Start the canonical project in the interactive editor.
1. Allow plugin loading and registry initialization to complete.
1. Run `shar-unreal-mcp doctor`.
1. Require `ready: true`, no missing meta-tools, and a nonzero toolset count.
1. Run `shar-unreal-mcp toolsets` and retain the exact inventory count.
1. Describe one known toolset to prove live schema access.
1. Run one narrow read-oriented capability with no persistent side effect.
1. Close the session cleanly.

The bootstrap is incomplete when only the TCP port responds.

## First live read

Choose a bounded read that proves the open editor context without changing
assets or project configuration. Before invoking it:

- read the exact generated per-tool skill;
- refresh the live schema;
- use the narrowest arguments;
- verify the returned identity belongs to the canonical project;
- confirm no save, compile, selection, or mutation occurred unexpectedly.

A successful first read proves dispatch, not complete readiness for every tool.
Continue with [editor readiness](editor-readiness.md) for the requested
operation.

## Client configuration boundary

The repository does not use a client-specific connection file as its source of
truth. Different clients can represent the same local endpoint differently,
and those files can be regenerated or maintained locally.

When a client cannot connect:

1. prove the native server with `doctor` first;
1. prove the current endpoint and protocol session;
1. inspect the client's local configuration separately;
1. avoid changing repository plugin or server settings to fit one client;
1. keep credentials, tokens, and workstation-only paths outside tracked files.

A client failure with a healthy translator session is not project setup drift.

## Drift detection

Treat the setup as drifted when any of these differ from repository authority:

- project plugin entries;
- server route or port;
- auto-start or tool-search setting;
- expected protocol version;
- required meta-tools;
- toolset count or identity inventory;
- generated interface digest;
- canonical project identity.

Do not repair drift by weakening validation. Identify whether the repository,
installed plugin set, editor process, or generated skills are stale.

## Configuration change procedure

When a tracked integration value must change:

1. identify the repository decision requiring the change;
1. capture the old descriptor and settings values;
1. update the narrowest tracked file;
1. update deterministic configuration tests;
1. validate JSON or INI syntax;
1. restart the editor through the operator-controlled path when required;
1. rerun `doctor`, inventory, schema, and first-read checks;
1. regenerate skills only when the live interface changed;
1. verify no client-only file entered the tracked change set.

Do not combine connection changes with unrelated editor, asset, or gameplay
configuration.

## Setup evidence

Retain enough evidence to reproduce the result:

- repository revision;
- canonical project path;
- relevant plugin entries;
- tracked server defaults;
- editor process and project identity;
- doctor JSON;
- toolset count;
- described toolset identity;
- first-read tool identity and bounded result;
- whether an editor restart occurred.

Do not retain workstation secrets, client tokens, or complete transient logs.

## Failure classification

Classify a failed setup before changing anything:

- **descriptor drift**: required plugin entries differ from tracked authority;
- **settings drift**: route, port, auto-start, or tool-search defaults differ;
- **installation gap**: the editor build lacks a required native plugin;
- **process mismatch**: another editor owns the endpoint;
- **transport failure**: loopback connection cannot be established;
- **protocol failure**: initialization or session checks fail;
- **registry failure**: meta-tools or toolsets are missing;
- **catalog drift**: generated skills disagree with live discovery;
- **client-only failure**: the translator is healthy but one client is not.

Each class has a different recovery path. Do not rewrite project configuration
for a client-only failure or regenerate skills for a transport-only failure.

## Completion criteria

Connection setup is complete only when:

- the canonical project owns the interactive editor session;
- required project plugins are available and loaded;
- tracked server defaults match repository tests;
- the translator negotiates the required protocol;
- all three registry meta-tools exist;
- the live registry is nonempty;
- one schema and one bounded read succeed;
- the generated skill catalog is not stale.

## Stop conditions

Stop when:

- the project descriptor is not the canonical SHAR project;
- required installed plugins are absent;
- the endpoint belongs to an unknown process;
- setup would require patching installed engine or plugin source;
- loopback or protocol validation would need to be weakened;
- the registry remains empty after the correct editor restart;
- a client-specific file would become the only connection record;
- completing setup would overwrite unrelated project configuration.
