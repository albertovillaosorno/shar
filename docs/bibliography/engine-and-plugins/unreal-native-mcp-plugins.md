# Unreal Native MCP Plugins

This non-governing record documents enabled Epic-provided plugin identities
without copying plugin source or treating them as MIT-licensed SHAR components.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Project plugin references, installed
  descriptor versions and roles, experimental and beta flags, editor-only and
  no-redistribution boundaries, selected Unreal Engine 5.8.0 build, repository
  client architecture, and Epic licensing boundary were verified. The resolved
  runtime load state, live tool catalog, module revisions, and notices remain
  installation- and execution-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Proprietary Unreal Engine editor and protocol-integration
  plugins.

## Covered Material

The current project descriptor and selected engine installation identify this
related family:

| Plugin identity | Project reference | Installed role and status |
| :-------------- | :---------------- | :------------------------ |
| `ModelContextProtocol` | Enabled | Version 1.0 experimental native Unreal MCP server; `NoRedist` |
| `AllToolsets` | Enabled | Version 1.0 experimental editor-only aggregator for all toolset plugins; `NoRedist` |
| `MCPClientToolset` | Explicitly disabled | Version 1.0 beta and experimental editor-only adapter for connecting toolset-registry customers to local or private MCP servers |

`AllToolsets` also declares `MCPClientToolset` as an enabled dependency. The
project-level disable and aggregate dependency therefore create a resolution
question that must be answered from the actual plugin manager or live runtime,
not from either descriptor in isolation.

## Repository Use And Scope

SHAR communicates with the native Unreal MCP server supplied by
`ModelContextProtocol` through an independently authored terminal client. The
project also enables `AllToolsets` to aggregate the installed native toolset
family. The public repository does not contain or patch Epic plugin source.

`MCPClientToolset` is not the native server or a server-exposed toolset. Its
installed descriptor describes an editor adapter for connecting toolset-registry
customers to local or private MCP servers. The project explicitly disables that
plugin, although the enabled aggregate declares it as a dependency. This record
does not claim the final load state without runtime evidence.

## Provenance And Version History

The selected installation is Unreal Engine 5.8.0 changelist 55116800. Installed
plugin descriptors identify `ModelContextProtocol`, `AllToolsets`, and
`MCPClientToolset` as version 1.0. The server and aggregator are experimental and
marked `NoRedist`; the aggregator is editor-only. The client adapter is
editor-only, beta, experimental, and loads after engine initialization when
resolved.

Those values are dated installation evidence, not minimum requirements or a
permanent compatibility range. Exact module revisions, resolved plugin state,
protocol support, tool catalog, notices, and third-party dependencies must be
verified from the selected installation and live runtime before publication or
distribution.

## Authorship, Ownership, And Attribution

Epic Games and applicable contributors or licensors retain rights in the
plugins, toolsets, interfaces, documentation, and branding. The open MCP
specification is a separate subject with a separate MIT-licensed repository.

## License Or Terms Basis

The installed plugins are governed as Unreal Engine Licensed Technology under
the applicable Epic agreement, together with component-specific and third-party
notices. The MCP specification's MIT License does not relicense Epic's
implementation.

## Distribution, Modification, And Compatibility

No plugin source or standalone plugin binary is intended for SHAR publication. A
packaged product must follow Unreal Engine's distribution rules and must not
expose credentials, private endpoints, unrestricted process execution, or
engine-editor tooling outside the intended local boundary.

## Compliance Posture

Keep the native plugins unchanged, tied to the selected compatible engine
installation, and outside Git. Treat engine 5.8.0 and plugin version 1.0 values
as dated evidence only, not unbounded `>=` requirements. Preserve the explicit
project disable for `MCPClientToolset` unless a reviewed architecture decision
changes it, and verify whether `AllToolsets` changes the resolved load state.

Record observed engine, plugin, and live catalog identities for each run. Validate
`NoRedist` boundaries, tool schemas, authentication, local binding, permissions,
and installed notices before enabling automated control.

## Source References

- Epic Games (2026) *Unreal Engine End User License Agreement*. Available at:
  <https://www.unrealengine.com/eula/unreal> (Accessed: 12 July 2026).
- Epic Games (n.d.) *Accessing Unreal Engine source code on GitHub*. Available
  at: <https://www.unrealengine.com/en-US/ue-on-github> (Accessed: 12 July
  2026).
- Epic Games (n.d.) *UnrealEngine GitHub network*. Access-controlled repository;
  an unauthenticated request may return `404 Not Found` until the GitHub account
  is linked and authorized as Epic documents. Available at:
  <https://github.com/EpicGames/UnrealEngine> (Accessed: 12 July 2026).
- Model Context Protocol contributors (n.d.) *Official GitHub repository*.
  Available at: <https://github.com/modelcontextprotocol/modelcontextprotocol>
  (Accessed: 12 July 2026).
- SHAR repository and selected engine installation (2026),
  `src/uproject/shar.uproject`, `README.md`, `docs/adr/unreal/mcp/`, Unreal Engine
  5.8.0 build evidence, and installed `ModelContextProtocol.uplugin`,
  `AllToolsets.uplugin`, and `MCPClientToolset.uplugin` descriptors.
