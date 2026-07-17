# Unreal MCP workflow map

Read the [central Unreal MCP index](../index.md) before entering a workflow.

This directory contains the manually maintained operating runbooks for the
SHAR Unreal MCP surface. Generated per-tool skills remain under
`skills/unreal/capabilities/**`; workflow files are not generated from native
metadata and may be edited as complete Markdown documents.

## Taxonomy

The folders follow the lifecycle of a trustworthy editor operation.

### Connection

Use connection workflows to establish the intended project, editor process,
native server, protocol session, and Toolset Registry surface.

- [Project connection setup](connection/project-connection-setup.md)
- [Editor readiness](connection/editor-readiness.md)
- [Server and registry operations](connection/server-and-registry-operations.md)

### Planning

Use planning workflows to translate an operator outcome into one native
capability and one live-schema-valid argument object.

- [Capability selection](planning/capability-selection.md)
- [Schema and arguments](planning/schema-and-arguments.md)

### Execution

Use execution workflows according to the actual side-effect posture and scale of
the operation.

- [Read-only operations](execution/read-only-operations.md)
- [Safe mutations](execution/safe-mutations.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Long-running and batch operations](execution/long-running-and-batch-operations.md)
- [Programmatic tool scripts](execution/programmatic-tool-scripts.md)

### Assurance

Use assurance workflows to prove postconditions, classify ambiguous outcomes,
and restore a known state without compounding uncertainty.

- [Verification and recovery](assurance/verification-and-recovery.md)

### Maintenance

Use maintenance workflows when the live native interface or SHAR-specific
manual evidence changes.

- [Manual guidance maintenance](maintenance/manual-guidance-maintenance.md)
- [Regeneration and taxonomy](maintenance/regeneration-and-taxonomy.md)

### Extension

Use extension workflows only when the task changes the callable surface or
introduces reusable editor guidance rather than merely invoking an existing
capability.

- [Toolset design and extension](extension/toolset-design-and-extension.md)
- [Agent guidance authoring](extension/agent-guidance-authoring.md)

## Default operating route

A normal native tool operation follows this sequence:

1. Prove editor readiness.
1. Select the narrowest capability.
1. Refresh the live toolset schema.
1. Construct exact arguments.
1. Choose the read, mutation, batch, or programmatic execution workflow.
1. Verify the outcome independently.
1. Record reusable evidence only when it is reproduced and current.

A first-time or drifted connection starts with project connection setup. A
registry, port, protocol, or tool-discovery failure routes through server and
registry operations before returning to editor readiness.

A change to native tool implementation begins with toolset design and extension.
A change to reusable guidance begins with agent guidance authoring. Neither
workflow substitutes for ordinary capability invocation.

## Ownership boundaries

The workflow tree owns reusable procedure and stop conditions. It does not own:

- live native tool identities or schemas;
- generated capability paths;
- project architecture decisions outside the MCP operating surface;
- installed engine or plugin source;
- transient editor output, logs, caches, or test artifacts.

The central index and generated capability pages remain projections of the live
Toolset Registry. The workflow taxonomy remains manually authored and stable by
responsibility.

## Navigation rules

- Keep this file as the only workflow map.
- Do not create nested `index.md` files.
- Place one runbook in the folder matching its lifecycle responsibility.
- Link every runbook from the generated central index.
- Link related workflows directly only when the transition is deterministic.
- Do not duplicate native schemas or full capability catalogs in workflows.
- Do not add YAML frontmatter or harness-specific metadata.

## Change rules

When adding or moving a workflow:

1. Define the single responsibility and lifecycle folder.
1. Confirm no existing workflow already owns the procedure.
1. Update the central index renderer.
1. Update workflow regression tests.
1. Update all relative links.
1. Regenerate the central index.
1. Run canonical scoped validation and the complete MCP test suite.
1. Verify no old workflow path remains.

A workflow move is complete only when generated navigation, manual navigation,
local-link validation, and all tests resolve the new path.
