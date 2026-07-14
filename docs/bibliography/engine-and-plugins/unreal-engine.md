# Unreal Engine

This non-governing record documents a proprietary engine dependency and does not
apply the SHAR MIT License to Unreal Engine code, content, tools, plugins,
examples, or documentation.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Project descriptor, selected engine
  association, current Epic EULA, public source-access rules, and Unreal Engine
  5.8 Media Framework and Electra platform boundaries verified; the exact
  installed build, accepted regional agreement, bundled notices, and target
  decoder availability remain installation-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
- Subject class: Proprietary real-time engine, editor, toolchain, and SDK.

## Covered Material

Unreal Engine 5.8, as selected by `src/uproject/shar.uproject`, together with
Epic-provided engine modules and project-generation tooling required by the SHAR
Unreal target. Organization identity and general Epic source routing are
recorded separately in [Epic Games](../organizations/epic-games.md).

## Repository Use And Scope

Unreal Engine is the target runtime and editor environment for the reconstructed
project. The public repository tracks independently authored project source,
configuration, adapters, tests, and documentation. It does not distribute Engine
Code, Epic-provided plugin source, Starter Content, Examples, marketplace
content, or installed engine files.

## Provenance And Version History

The current project descriptor records `EngineAssociation` 5.8. Exact engine
build, installation channel, accepted agreement, plugin revisions, third-party
components, and platform toolchain must be captured for a reproducible build
or distribution.

## Authorship, Ownership, And Attribution

Epic Games and its licensors retain rights in the Licensed Technology, Epic
content, documentation, and marks. SHAR contributors retain rights in
independently authored project material to the extent supported by the
repository license and applicable agreements.

## License Or Terms Basis

Unreal Engine is governed by the Unreal Engine End User License Agreement and
any applicable additional terms. Epic's general Terms of Service expressly
excludes Unreal Engine and must not be substituted for the engine agreement.
Engine distributions also include separately licensed third-party software whose
notices are supplied in the installed engine license directories. The accepted
agreement and installed notices control.

## Distribution, Modification, And Compatibility

A SHAR product incorporating Engine Code must satisfy the applicable
object-code, product-distribution, notice, royalty, seat, and end-user-license
conditions. Engine source or tools may be distributed only through the channels
and to the licensees permitted by Epic's agreement.

## Compliance Posture

Keep engine and plugin source outside the public repository. Before packaging,
record the accepted agreement, exact engine build, third-party notices, required
credits, compiler and SDK versions, distribution type, and any royalty or seat
analysis.

## Source References

- Epic Games (2026) *Unreal Engine End User License Agreement*. Available at:
  <https://www.unrealengine.com/eula/unreal> (Accessed: 13 July 2026).
- Epic Games (n.d.) *Unreal Engine documentation*. Available at:
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/> (Accessed: 12
  July 2026).
- Epic Games (2026) *Media Framework Technical Reference for Unreal Engine 5.8*.
  Available at:
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/media-framework-technical-reference-for-unreal-engine>
  (Accessed: 13 July 2026).
- Epic Games (2026) *Electra Media Player in Unreal Engine 5.8*. Available at:
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/electra-media-player-in-unreal-engine>
  (Accessed: 13 July 2026).
- Epic Games (n.d.) *Accessing Unreal Engine source code on GitHub*. Available
  at: <https://www.unrealengine.com/en-US/ue-on-github> (Accessed: 12 July
  2026).
- Epic Games (n.d.) *UnrealEngine GitHub network*. Access-controlled repository;
  an unauthenticated request may return `404 Not Found` until the GitHub account
  is linked and authorized as Epic documents. Available at:
  <https://github.com/EpicGames/UnrealEngine> (Accessed: 12 July 2026).
- SHAR repository (2026) `src/uproject/shar.uproject` and `README.md`.
