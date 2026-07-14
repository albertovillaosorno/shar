# Unreal Native MCP Plugins

This non-governing record documents enabled Epic-provided plugin identities
without copying plugin source or treating them as MIT-licensed SHAR components.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Enabled plugin identities, repository
  client architecture, and Epic licensing boundary verified; exact installed
  plugin revisions, protocol surface, dependencies, and notices remain
  installation-specific and partly access-controlled.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
- Subject class: Proprietary Unreal Engine editor and protocol-integration
  plugins.

## Covered Material

The current project descriptor enables this related engine-plugin family:

| Plugin identity        | Repository role                                             |
| :--------------------- | :---------------------------------------------------------- |
| `ModelContextProtocol` | Native Unreal MCP server and protocol integration           |
| `MCPClientToolset`     | Epic-provided MCP toolset exposed through the native server |

## Repository Use And Scope

SHAR communicates with the unchanged native Unreal MCP server through an
independently authored terminal client. The public repository does not contain
or patch the Epic plugin source. Project configuration records only that the
installed plugins are enabled.

## Provenance And Version History

The plugin versions follow the selected Unreal Engine 5.8 installation. Their
exact modules, protocol support, experimental status, notices, and third-party
dependencies must be verified from that installation before publication or
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
installation, and outside Git. Record the observed engine and plugin identities
for each run. Validate tool schemas, authentication, local binding, permissions,
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
- SHAR repository (2026) `src/uproject/shar.uproject`, `README.md`, and
  `docs/adr/unreal/mcp/`.
