# Unreal asset conversion

`uasset` is the deterministic asset-conversion planning crate used by
`pipeline`.

Its only accepted source families are:

- normalized JSON records;
- explicit decoded image interchange;
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

Phase 5 owns terminal access to the unchanged Unreal Engine 5.8.1 native MCP
server. Phase 6 uses this crate to create deterministic conversion plans, and a
separate terminal client applies those plans through discovered native tools.

## Dependency direction

```text
normalized pipeline evidence -> uasset conversion plan
uasset conversion plan    -> Phase 5 terminal MCP client
Phase 5 terminal MCP client   -> native Unreal Engine tools
```

`uasset` never opens a network connection, starts a process, or controls an
Unreal session. `pipeline` is the intended caller and orchestration owner.

## Plan invariants

A published operation is accepted only when all of these conditions hold:

- the source format maps to its supported native family and readiness state;
  FBX permits `requires-conversion` until absent output is generated and `ready`
  only after complete catalog verification;
- the destination is a canonical object path under `/Game/Generated/SHAR/`;
- the expected target class participates in the stable operation identity;
- operation dependencies use canonical operation identities, are acyclic, and
  never point to a later plan family; and
- validation failures do not echo rejected physical paths or path-shaped
  identities.

These checks describe planning evidence only. A final Unreal asset still needs
independent editor readback, save, restart, dependency, cook, and packaged-load
validation before it is accepted.

Relevant decisions and contracts:

- [Asset-conversion boundary](../../../docs/adr/unreal/architecture.md)
- [Native Unreal MCP terminal
  bridge](../../../docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md)
- [Eleven-phase
  roadmap](../../../docs/adr/pipeline/eleven-phase-remake-delivery-roadmap.md)
- [Generated Unreal plan
  bundle](../../../docs/technical/pipeline/unreal/generated-plan-bundle.md)
- [Native asset
  planning](../../../docs/technical/unreal/native-asset-planning.md)
