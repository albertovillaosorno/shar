# Unreal MCP terminal translator

`src/mcp` is a Python command-line translator for the native Unreal Engine 5.8
MCP server. It is **not an MCP server**, it does not replace Unreal's native
plugins, and it does not contain or copy Epic Games plugin source.

The package turns terminal commands into MCP lifecycle, discovery, and tool-call
messages so terminal-capable AI agents can operate Unreal Editor through a
reviewable and testable command surface.

## Architecture

The package follows the same hexagonal direction used by `src/fbx`:

```text
src/mcp/
├── src/
│   ├── domain/             Protocol-independent values and invariants
│   ├── application/        Discovery and invocation use cases
│   ├── ports/              Transport and execution contracts
│   └── adapters/
│       ├── driving/        Terminal CLI
│       └── driven/         Streamable HTTP and serial execution
├── tests/                  Synthetic black-box and contract tests
└── pyproject.toml          Distribution and console-entry metadata
```

There is no repeated `shar_unreal_mcp/` source directory. The repository path
itself provides the internal `mcp.src` Python namespace required by static
analysis and wheel packaging. That namespace is an implementation detail; the
public operator interface is the `shar-unreal-mcp` command.

Dependencies point inward. Domain code does not import HTTP, command-line,
filesystem, Unreal, or process APIs. The application layer invokes ports. The
outer adapters own transport and operator-facing behavior.

## Unreal plugin boundary

The Unreal project enables:

- `ModelContextProtocol` for the native inbound MCP server; and
- `AllToolsets` for the complete native editor tool catalog.

`MCPClientToolset` is explicitly disabled because it is an outbound adapter for
connecting Unreal to other MCP servers. This package needs the opposite
direction: terminal to Unreal. It remains an available future fallback if a
required workflow cannot be expressed through the official inbound server and
an external repository-owned MCP service becomes the safest interoperable
boundary. Enabling that fallback requires a reviewed ADR, tests, and an explicit
project configuration change; it is not part of the default architecture.

Installed project plugins remain local under `src/uproject/Plugins/` and
are ignored by Git.

The tracked Unreal project defaults enable automatic server startup, preserve
native tool-search mode, and use `http://127.0.0.1:8000/mcp`. Unreal Editor must
be restarted after plugin or MCP configuration changes because an existing
process cannot load newly enabled plugins retroactively.

## Commands

After installing the package in the repository Python environment:

```text
shar-unreal-mcp doctor
shar-unreal-mcp toolsets
shar-unreal-mcp describe EditorToolset.EditorAppToolset
shar-unreal-mcp call \
  EditorToolset.EditorAppToolset \
  EditorToolset.EditorAppToolset.GetCameraTransform --arguments '{}'
shar-unreal-mcp catalog --format markdown
shar-unreal-mcp skills
```

The default endpoint is `http://127.0.0.1:8000/mcp`. Only loopback HTTP
endpoints are accepted. Every command creates an MCP session, performs bounded
work, and closes the session.

The `call` command accepts either the native leaf name or the fully qualified
tool identity shown by `describe` and the generated skills. Qualified names are
validated against the selected toolset and converted to the leaf name required
by Unreal's `call_tool` meta-tool.

The `skills` command discovers every live toolset and schema, verifies explicit
taxonomy ownership, and safely replaces only `skills/unreal/index.md` and
`skills/unreal/capabilities/**`. Manual workflow skills are preserved. Each
per-tool file contains five project-evidence fields plus one protected reviewed-
revision token. Regeneration updates the generated shell while preserving exact
text between valid field markers. It derives review status from the installed
Unreal MCP plugin `VersionName` and live interface digest; `1.0` is normalized
to public SemVer `1.0.0`. The Python package CalVer remains separate and is not
part of skill review identity. Mismatched or legacy guidance is marked **Review
required** without data loss. The central index records the Unreal MCP version,
revision, and exact status counts. Malformed or unknown markers fail before
cleanup or writes.

The generated tree contains one mandatory central index and exactly one skill
per native tool. Paths are derived from native names, such as
`automation/test/toolset/discover-tests.md`. New tools start with `[TODO]` and
`[FILL_ME]` fields; removed tools delete their obsolete generated files.

Use `--output RELATIVE_PATH` only for repository-relative test or review output.
Unsafe absolute or parent-traversing paths fail before any MCP session opens.

The wire adapter sends the canonical loopback `Origin` header and fails closed
unless initialization returns JSON-RPC 2.0, protocol version `2025-11-25`, a
Tools capability, text-typed server metadata, and a visible-ASCII session
identifier. UE 5.8 currently returns empty server name, title, and version
strings, so Unreal readiness is established by the required tool-search
meta-tools and a non-empty Toolset Registry rather than those informational
fields. The `doctor` result includes `toolsetCount` and reports `ready: false`
when the registry is empty. Subsequent requests carry the negotiated protocol
and session headers.

The driven execution gate serializes tool calls. This is intentionally stricter
than the native server, which can track multiple asynchronous requests at once.
Serial execution prevents overlapping editor mutations from producing
nondeterministic state.

Native tool outcomes preserve the complete raw result, concatenated text
fallback, and first-class `structuredContent` JSON value. Programmatic callers
can consume structured results without reparsing the text fallback, while the
CLI continues to print the complete raw MCP result.

HTTP JSON bodies, SSE streams, and session-delete responses are bounded to
64 MiB by default. Declared and streamed overflows fail before the response can
grow without limit. Programmatic transports may select a smaller positive
ceiling for constrained automation or deterministic tests.

When a native tool call exceeds the configured timeout, the translator sends a
`notifications/cancelled` notification with the original JSON-RPC request ID.
UE 5.8 removes the matching active request and invokes the tool's asynchronous
cancellation hook. The command still exits as a timeout; cancellation never
converts incomplete work into success.

Always start with the mandatory
[Unreal MCP capability index](../../skills/unreal/index.md).
