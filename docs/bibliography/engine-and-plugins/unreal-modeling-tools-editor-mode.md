# Unreal Modeling Tools Editor Mode

This non-governing record documents an enabled Epic-provided editor plugin and
does not make its source or tools part of the SHAR MIT-licensed code.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Project enablement, installed plugin
  descriptor version 0.1, beta status, editor module identity, dependent
  modeling
  toolsets, selected Unreal Engine 5.8.0 build, and official Modeling Mode
  documentation were verified. Module source revision, bundled notices, and
  runtime behavior remain installation-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Proprietary Unreal Engine editor plugin.

## Covered Material

The `ModelingToolsEditorMode` plugin enabled by the current SHAR project
descriptor and the associated Modeling Mode functionality supplied with Unreal
Engine 5.8.

## Repository Use And Scope

The plugin provides editor-side modeling and geometry tools. It is an installed
engine component, not a repository-owned implementation and not a runtime
library that SHAR redistributes. The public repository contains only the project
setting that enables it.

## Provenance And Version History

The installed descriptor identifies `ModelingToolsEditorMode` version 0.1 as a
beta editor plugin. It depends on `MeshModelingToolset`,
`MeshModelingToolsetExp`, `MeshLODToolset`, `ToolPresets`, and `StylusInput` for
Editor targets. The selected engine build is Unreal Engine 5.8.0 changelist
55116800.

Those values are dated installation evidence, not minimum requirements or a
permanent compatibility range. Exact module source revision, dependent plugin
versions, third-party components, and notices must be read from the engine
installation used for a build.

## Authorship, Ownership, And Attribution

Epic Games and applicable contributors or licensors retain rights in the plugin,
its source, documentation, interfaces, and branding.

## License Or Terms Basis

The plugin is part of Epic's Licensed Technology and is governed by the Unreal
Engine agreement, together with any component-specific or third-party notices in
the installed engine distribution.

## Distribution, Modification, And Compatibility

No plugin source or binary is published by SHAR. Any product distribution uses
the plugin only through the Unreal Engine build and packaging rules applicable
to editor and engine components.

## Compliance Posture

Treat the plugin as an external engine-owned editor capability. Preserve version
0.1 and engine 5.8.0 only as dated installation evidence; do not convert either
into an unbounded `>=` requirement. Do not copy plugin source or documentation
into SHAR, and verify the installed module revisions, dependencies, and notices
before publication or distribution.

## Source References

- Epic Games (2026) *Modeling Mode in Unreal Engine 5.8*. Available at: [Epic
  Modeling Mode][epic-modeling] (Accessed: 12 July 2026).
- Epic Games (2026) *Unreal Engine End User License Agreement*. Available at:
  <https://www.unrealengine.com/eula/unreal> (Accessed: 12 July 2026).
- Epic Games (n.d.) *Accessing Unreal Engine source code on GitHub*. Available
  at: <https://www.unrealengine.com/en-US/ue-on-github> (Accessed: 12 July
  2026).
- Epic Games (n.d.) *UnrealEngine GitHub network*. Access-controlled repository;
  an unauthenticated request may return `404 Not Found` until the GitHub account
  is linked and authorized as Epic documents. Available at:
  <https://github.com/EpicGames/UnrealEngine> (Accessed: 12 July 2026).
- SHAR repository and selected engine installation (2026),
  `src/uproject/shar.uproject`, Unreal Engine 5.8.0 build evidence, and
  installed
  `ModelingToolsEditorMode.uplugin` version 0.1 with beta status, editor module,
  and dependent toolsets.

[epic-modeling]:
  https://dev.epicgames.com/documentation/en-us/unreal-engine/modeling-mode-in-unreal-engine
