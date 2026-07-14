# Model Context Protocol

This non-governing record documents a protocol and related reference tooling
without treating every MCP implementation as one licensed product.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — SHAR client use, official protocol
  specification, schema, documentation, and reference repository verified;
  implementation-specific capabilities, authentication, and transport behavior
  remain server- and version-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
- Subject class: Open protocol, schema, documentation, and reference tooling.

## Covered Material

The Model Context Protocol specification, protocol schema, official
documentation, and MCP Inspector reference client.

## Repository Use And Scope

SHAR plans a repository-owned terminal MCP client that communicates with the
unchanged native Unreal MCP server. The repository architecture also identifies
MCP Inspector as an independent reference client for testing and debugging. The
protocol does not make any particular server, client, toolset, model, or host
implementation part of the same license family.

## Provenance And Version History

The repository uses the protocol behavior supported by the selected Unreal
Engine integration. The compatible MCP edition is established by the installed
engine, native plugin, and recorded interface evidence. The stable official
`latest` endpoint currently resolves to a dated edition; that resolution is
evidence for the access date, not a permanent assertion. Protocol editions,
extensions, transports, and Inspector versions must be recorded separately in
implementation and build or distribution evidence.

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

Use protocol-version pinning, schema validation, least privilege, local-only
binding where intended, explicit authentication, and implementation-specific
license review. Do not describe MCP conformance as certification or endorsement.

## Source References

- Model Context Protocol contributors (n.d.) *Latest specification endpoint*.
  Available at: <https://modelcontextprotocol.io/specification/latest>
  (Accessed: 12 July 2026; resolved to the 25 November 2025 edition on that
  date).
- Model Context Protocol contributors (n.d.) *Official GitHub repository*.
  Available at: <https://github.com/modelcontextprotocol/modelcontextprotocol>
  (Accessed: 12 July 2026).
- Model Context Protocol contributors (n.d.) *MCP Inspector official GitHub
  repository*. Available at: <https://github.com/modelcontextprotocol/inspector>
  (Accessed: 12 July 2026).
- SHAR repository (2026) `README.md` and `docs/adr/unreal/mcp/`.
