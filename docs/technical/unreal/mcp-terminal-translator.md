# Unreal MCP terminal translator

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Native Unreal MCP terminal bridge](../../adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md)

## Purpose

This specification explains the repository-owned client that lets an AI agent
operate the native Unreal MCP server from a terminal.

## Repository model

The translator validates a loopback endpoint, initializes a bounded session,
negotiates protocol state, discovers live tools, validates arguments, invokes
one operation, interprets structured results, and closes the session. It exposes
no new server and treats the live editor catalog as authority.

## Invariants

Tool names and schemas are preserved losslessly. Calls are serialized.
Destructive or project-wide mutation requires explicit approval. A mutating call
is verified through read-only evidence when the native catalog offers it.

## Failure behavior

Non-loopback endpoints, invalid sessions, mismatched responses, malformed
payloads, repeated cursors, timeouts, cancellation failures, catalog drift, and
tool error states produce non-successful command results.

## Verification

Synthetic loopback tests prove lifecycle, negotiation, discovery, pagination,
transport handling, argument validation, serialized mutation, structured errors,
and cleanup.
