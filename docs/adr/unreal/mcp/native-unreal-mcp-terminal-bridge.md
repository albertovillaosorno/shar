# Native Unreal MCP terminal bridge

- Status: Accepted
- Decision date: 2026-07-12
- Scope: AI-to-Unreal control

## Context

AI-to-Unreal control crosses a live protocol and a mutable editor boundary.
Without current discovery, serialized mutation, bounded calls, and independent
read-back, a successful response can conceal stale schemas or incorrect state.

## Decision

AI agents connect to Unreal through the official native inbound MCP server. The
repository-owned terminal translator is an MCP client, not a server; it uses
loopback-only live discovery, fail-closed validation of arguments against the
current native input schema, serialized state changes, bounded calls, and
independent postcondition verification. Unsupported schema assertions stop the
call before the native mutation meta-tool runs.

## Consequences

- AI control remains an outbound repository client connected to the official
  native inbound server on loopback.
- Live discovery, structural argument validation, and bounded serialized calls
  prevent stale catalog assumptions, malformed native requests, and concurrent
  editor mutation races.
- Transport success is insufficient; each consequential call requires an
  independent postcondition check.

## Rejected alternatives

- Implementing a competing repository-owned Unreal MCP server.
- Calling remembered native tools without current discovery and schema evidence.
- Treating a successful protocol response as proof that editor state is correct.
