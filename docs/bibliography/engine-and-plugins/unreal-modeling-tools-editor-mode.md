# Unreal Modeling Tools Editor Mode

This non-governing record documents an enabled Epic-provided editor plugin and
does not make its source or tools part of the SHAR MIT-licensed code.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Project enablement and official Unreal
  Engine 5.8 Modeling Mode documentation verified; exact installed module
  revision, experimental status, dependencies, and notices remain
  installation-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
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

The plugin version follows the selected Unreal Engine installation. Exact module
contents, experimental status, third-party dependencies, and notices must be
read from the installed Engine 5.8 distribution used for a build.

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

Treat the plugin as an external engine-owned editor capability. Do not copy its
source or documentation into SHAR, and verify the installed version and notices
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
- SHAR repository (2026) `src/uproject/shar.uproject`.

[epic-modeling]:
  https://dev.epicgames.com/documentation/en-us/unreal-engine/modeling-mode-in-unreal-engine
