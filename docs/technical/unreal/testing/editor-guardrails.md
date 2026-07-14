# Editor guardrails

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Native Unreal MCP terminal bridge](../../../adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md)

## Purpose

This specification explains how reproduced editor and MCP failures become
durable repository checks.

## Repository model

A reproduced failure is assigned to its owning boundary and converted into the
narrowest deterministic precondition, schema rule, serialization rule, timeout,
state check, or regression test. Guardrails remain observable and do not hide
upstream defects.

## Invariants

- Every guardrail names the failure condition it prevents.
- Read-only verification follows supported mutations when available.
- Guardrails do not depend on guessed tool names or undocumented editor state.

## Failure behavior

- Unknown editor state, stale schemas, overlapping mutation, missing approval,
  timeout, and structured tool errors stop the operation.
- A workaround cannot claim the upstream defect is fixed.

## Verification

- Synthetic protocol tests reproduce transport and schema failures.
- Native automation reproduces editor-state failures where required.
- Regression tests fail when a removed guard allows the original defect.
