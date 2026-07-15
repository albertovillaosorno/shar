# Model Context Protocol

This non-governing record documents a protocol and related reference tooling
without treating every MCP implementation as one licensed product.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Operational SHAR client use, live
  protocol
  version `2025-11-25`, official dated specification, current-edition label,
  schema, documentation, and reference repository were verified.
  Implementation-specific capabilities, authentication, and transport behavior
  remain server- and version-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Open protocol, schema, documentation, and reference tooling.

## Covered Material

The Model Context Protocol specification, protocol schema, official
documentation, and MCP Inspector reference client.

## Repository Use And Scope

SHAR uses a repository-owned terminal MCP client that communicates with the
unchanged native Unreal MCP server. The live doctor command creates a bounded
session, verifies the meta-tool surface, reports protocol version `2025-11-25`,
and closes the session. The repository architecture also identifies MCP
Inspector as an independent reference client for testing and debugging. The
protocol does not make any particular server, client, toolset, model, or host
implementation part of the same license family.

## Provenance And Version History

The live Unreal server reports protocol version `2025-11-25`. The official
versioned specification page labels that edition `latest` on 14 July 2026, and
the mutable `/latest` endpoint redirects to the same dated page. The versioned
endpoint is the reproducible authority for this observation; the redirect is
currentness evidence only.

Protocol editions, extensions, transports, schemas, and Inspector versions must
be recorded separately in implementation, build, or distribution evidence. A
future `/latest` redirect does not retroactively change the protocol edition
used
by the reviewed live session.

## Authorship, Ownership, And Attribution

The official specification repository identifies David Soria Parra and Justin
Spahr-Summers as the protocol's creators. Individual SDKs, servers, clients,
extensions, and integrations have their own contributors and license evidence.

## License Or Terms Basis

The official MCP specification and documentation repository is licensed under
the MIT License. The official MCP Inspector repository is also licensed under
the MIT License. Those grants do not automatically apply to third-party MCP
implementations, hosted services, Unreal Engine plugins, or vendor integrations.

## Distribution, Modification, And Compatibility

Implementing the protocol does not require vendoring the specification
repository. Any copied schema, documentation, Inspector package, SDK, or example
must retain its applicable license and notices. Credentials, local process
execution, remote endpoints, and tool permissions remain independent security
boundaries.

## Compliance Posture

Pin and record the negotiated protocol edition, validate schemas, use least
privilege and local-only binding where intended, require explicit
authentication, and review each implementation's license. Use the dated
`2025-11-25` specification for the reviewed session rather than relying on the
mutable `/latest` redirect. Do not describe MCP conformance as certification or
endorsement.

## Source References

- Model Context Protocol contributors (2025) *Specification 2025-11-25*.
  Identified as the latest edition on 14 July 2026 and as the authoritative
  requirements based on the TypeScript schema. Available at:
  <https://modelcontextprotocol.io/specification/2025-11-25> (Accessed: 14 July
  2026).
- Model Context Protocol contributors (n.d.) *Latest specification endpoint*.
  Redirected to the 25 November 2025 edition during review. Available at:
  <https://modelcontextprotocol.io/specification/latest> (Accessed: 14 July
  2026).
- Model Context Protocol contributors (n.d.) *Official GitHub repository*.
  Available at: <https://github.com/modelcontextprotocol/modelcontextprotocol>
  (Accessed: 14 July 2026).
- Model Context Protocol contributors (n.d.) *MCP Inspector official GitHub
  repository*. Available at: <https://github.com/modelcontextprotocol/inspector>
  (Accessed: 14 July 2026).
- SHAR repository and live Unreal MCP session (2026), `README.md`,
  `docs/adr/unreal/mcp/`, and doctor output reporting protocol version
  `2025-11-25`, ready state, 52 toolsets, and no missing meta-tools.
