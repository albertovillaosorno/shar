# Unreal asset conversion

`src/unreal` is the deterministic asset-conversion planning crate used by
`pipeline`.

Its only accepted source families are:

- normalized JSON records;
- PCM WAV audio;
- HAP video packages and synchronized audio metadata; and
- canonical binary FBX 7.7 models and animation.

The crate validates conversion evidence and produces stable Unreal target
families, object paths, dependency plans, artifact records, and provenance. It
does not import assets by opening Unreal Editor itself.

## Boundary

This crate must never contain:

- an MCP client or server;
- HTTP, SSE, endpoint, or client-configuration code;
- editor command, query, recovery, scheduler, or support behavior;
- live actor, graph, widget, level, or world mutation;
- an Unreal MCP tool catalog;
- runtime gameplay implementation;
- arbitrary Unreal package inspection;
- project INI or build-configuration parsing; or
- generated editor scripts as the source of truth.

Phase 5 owns terminal access to the unchanged Unreal Engine 5.8 native MCP
server. Phase 6 uses this crate to create deterministic conversion plans, and a
separate terminal client applies those plans through discovered native tools.

## Dependency direction

```text
normalized pipeline evidence -> src/unreal conversion plan
src/unreal conversion plan    -> Phase 5 terminal MCP client
Phase 5 terminal MCP client   -> native Unreal Engine tools
```

`src/unreal` never opens a network connection, starts a process, or controls an
Unreal session. `pipeline` is the intended caller and orchestration owner.

Relevant decisions:

- [Asset-conversion boundary](../../docs/adr/unreal/architecture.md)
- [Native Unreal MCP terminal bridge](../../docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md)
- [Eleven-phase roadmap](../../docs/adr/pipeline/eleven-phase-remake-delivery-roadmap.md)
