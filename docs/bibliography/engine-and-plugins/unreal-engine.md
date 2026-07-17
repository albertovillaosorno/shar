# Unreal Engine

This non-governing record documents a proprietary engine dependency and does not
apply the SHAR MIT License to Unreal Engine code, content, tools, plugins,
examples, or documentation.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Project descriptor, selected engine
  association, installed Unreal Engine 5.8.0 promoted build changelist 55116800,
  release branch identity, current Epic EULA, its generative-AI input
  restriction, public source-access rules, Common UI input routing and action
  data, managed subsystem lifetimes, C++ UMG MVVM viewmodels and field
  notifications, Asset Manager primary-asset and bundle boundaries, soft
  references, asynchronous loading and streamable-handle behavior, Sequencer
  timeline and track authoring, localization and culture fallback, Control Rig
  integration, visibility and occlusion culling, post-process lens facilities,
  Niagara systems and emitters, native actor tick groups and prerequisites,
  spline-component authoring, navigation-system boundaries, Unreal Audio Mixer,
  Sound Attenuation, MetaSounds, Sound Concurrency and voice-management policy,
  Sound Classes and Sound Mixes, Audio Modulation, submix graphs and effect
  chains, Audio Volumes, native audio stream caching, audio project settings and
  quality controls, Enhanced Input actions, mapping contexts, modifiers,
  triggers,
  and per-local-player subsystems, `ACharacter` and
  `UCharacterMovementComponent`, native movement modes and network movement
  support, StateTree states, tasks, and transitions, Chaos physics, collision
  and
  traces, Chaos Vehicles, `UChaosVehicleMovementComponent`,
  `UChaosWheeledVehicleMovementComponent`, native wheel, suspension, engine,
  transmission, differential, asynchronous physics, vehicle animation, and
  vehicle setup boundaries, the experimental Chaos Modular Vehicles distinction,
  FBX static-mesh, skeletal-mesh, animation, material, and LOD import, import
  options, Animation Sequence frame-range and Skeleton binding, Animation
  Blueprints, Animation Montage sections and slots, Sync Groups and Sync
  Markers,
  root motion, Motion Warping, native Data Validation, native physics
  components,
  ISM and HISM behavior, Chaos Visual Debugger, and Unreal Engine 5.8 Media
  Framework and Electra boundaries were verified.
  Accepted regional agreement, bundled
  notices, decoder availability, and external-service retention remain
  environment- or service-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-17.
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

The project descriptor records `EngineAssociation` 5.8. The selected local
installation reports Unreal Engine 5.8.0, changelist 55116800, promoted-build
status, and branch `++UE5+Release-5.8`. Those values are dated build evidence,
not a minimum requirement or permanent compatibility range.

The live EULA was re-reviewed on 14 July 2026, but the retrieved page does not
expose a revision date or version identifier. A reproducible build or
distribution must still capture the exact engine build, installation channel,
accepted agreement and date, plugin revisions, third-party components, platform
toolchain, and packaging command.

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

The current EULA prohibits using Licensed Technology as training input to a
Generative AI Program and prohibits prompt-based input where that program trains
on the input. This is a bounded input-use restriction, not a factual basis for a
blanket claim that all AI-assisted project work is prohibited. A workflow that
transmits Engine Code or other Licensed Technology must verify the exact
service,
account, retention, and training behavior before transmission.

## Distribution, Modification, And Compatibility

A SHAR product incorporating Engine Code must satisfy the applicable
object-code, product-distribution, notice, royalty, seat, and end-user-license
conditions. Engine source or tools may be distributed only through the channels
and to the licensees permitted by Epic's agreement.

## Compliance Posture

Keep engine and plugin source outside the public repository. Treat Unreal Engine
5.8.0 changelist 55116800 as observed build evidence only; do not convert it
into
an unbounded `>=` requirement. Do not submit Licensed Technology to an external
generative-AI service unless exact account and service evidence proves that the
input will not be used for training within the EULA's restricted scope. Before
packaging, record the accepted agreement, exact engine build, third-party
notices, required credits, compiler and SDK versions, distribution type, and any
royalty or seat analysis.

## Source References

- Epic Games (n.d.) *Unreal Engine End User License Agreement*. The live page
  did not expose a revision date or version identifier in the reviewed text.
  Available at: <https://www.unrealengine.com/eula/unreal> (Accessed: 14 July
  2026).
- Epic Games (n.d.) *Unreal Engine documentation*. Available at:
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/> (Accessed: 14
  July 2026).
- Epic Games (2026) *Common UI Quickstart Guide for Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/common-ui-quickstart-guide-for-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Programming Subsystems in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/programming-subsystems-in-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Asset Management in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/asset-management-in-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Data Validation in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/unreal-engine/data-validation-in-unreal-engine?lang=en-US>
  (Accessed: 17 July 2026).
- Epic Games (2026) *Asynchronous Asset Loading in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/asynchronous-asset-loading-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *UMG Viewmodel for Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/umg-viewmodel-for-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Sequencer Cinematic Editor for Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/sequencer-cinematic-editor-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Localization Overview for Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/localization-overview-for-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Control Rig in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/control-rig-in-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Visibility and Occlusion Culling in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/visibility-and-occlusion-culling-in-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Post Process Effects in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/post-process-effects-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Niagara Overview in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/overview-of-niagara-effects-for-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Actor Ticking in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/actor-ticking-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Blueprint Spline Components Overview in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/blueprint-spline-components-overview-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Navigation System in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/navigation-system-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Audio Engine Overview in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/audio-engine-overview-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Sound Attenuation in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/sound-attenuation-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *MetaSounds in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/metasounds-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Sound Concurrency Reference Guide for Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/sound-concurrency-reference-guide>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Audio Mixer Overview in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/audio-mixer-overview-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Sound Classes in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/sound-classes-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Overview of Submixes in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/overview-of-submixes-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Audio Volumes in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/audio-volumes-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Audio Settings in Unreal Engine Project Settings 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/audio-settings-in-the-unreal-engine-project-settings>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Audio Modulation Overview in Unreal Engine 5.8*. Available
  at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/audio-modulation-overview-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Audio Stream Caching Overview in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/an-overview-of-audio-stream-caching-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Enhanced Input in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/enhanced-input-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Networked Movement in the Character Movement Component for
  Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/understanding-networked-movement-in-the-character-movement-component-for-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *UCharacterMovementComponent in Unreal Engine 5.8*.
  Available
  at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/Engine/UCharacterMovementComponent>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Overview of StateTree in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/overview-of-state-tree-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Importing Static Meshes Using FBX in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/importing-static-meshes-using-fbx-in-unreal-engine>
  (Accessed: 17 July 2026).
- Epic Games (2026) *Importing Skeletal Meshes Using FBX in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/importing-skeletal-meshes-using-fbx-in-unreal-engine>
  (Accessed: 17 July 2026).
- Epic Games (2026) *Importing Static Mesh LODs Using FBX in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/importing-static-mesh-lods-using-fbx-in-unreal-engine>
  (Accessed: 17 July 2026).
- Epic Games (2026) *Importing Skeletal Mesh LODs Using FBX in Unreal Engine
  5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/importing-skeletal-mesh-lods-using-fbx-in-unreal-engine>
  (Accessed: 17 July 2026).
- Epic Games (2026) *Importing Materials Reference in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/importing-materials-reference-in-unreal-engine>
  (Accessed: 17 July 2026).
- Epic Games (2026) *Importing Animations Using FBX in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/importing-animations-using-fbx-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *FBX Import Options Reference in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/fbx-import-options-reference-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Animation Sequences in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/animation-sequences-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Animation Montage in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/animation-montage-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Animation Blueprints in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/animation-blueprints-in-unreal-engine>
  (Accessed: 17 July 2026).
- Epic Games (2026) *Animation Sync Groups in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/animation-sync-groups-in-unreal-engine>
  (Accessed: 17 July 2026).
- Epic Games (2026) *Root Motion in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/root-motion-in-unreal-engine>
  (Accessed: 17 July 2026).
- Epic Games (2026) *Motion Warping in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/motion-warping-in-unreal-engine>
  (Accessed: 17 July 2026).
- Epic Games (2026) *Chaos Vehicles in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/chaos-vehicles>
  (Accessed: 16 July 2026).
- Epic Games (2026) *How to Set Up Vehicles in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/how-to-set-up-vehicles-in-unreal-engine>
  (Accessed: 16 July 2026).
- Epic Games (2026) *UChaosWheeledVehicleMovementComponent in Unreal Engine
  5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Plugins/ChaosVehicles/UChaosWheeledVehicleMovementComp->
  (Accessed: 16 July 2026).
- Epic Games (2026) *Chaos Modular Vehicles Overview in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/chaos-modular-vehicles-overview>
  (Accessed: 16 July 2026).
- Epic Games (2026) *Physics in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/physics-in-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Collision in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/collision-in-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Traces with Raycasts in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/traces-with-raycasts-in-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Physics Components in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/physics-components-in-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Instanced Static Mesh Component in Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/instanced-static-mesh-component-in-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Chaos Visual Debugger in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/chaos-visual-debugger-in-unreal-engine>
  (Accessed: 15 July 2026).
- Epic Games (2026) *Media Framework Technical Reference for Unreal Engine 5.8*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/media-framework-technical-reference-for-unreal-engine>
  (Accessed: 14 July 2026).
- Epic Games (2026) *Electra Media Player in Unreal Engine 5.8*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/electra-media-player-in-unreal-engine>
  (Accessed: 14 July 2026).
- Epic Games (n.d.) *Accessing Unreal Engine source code on GitHub*. Available
  at: <https://www.unrealengine.com/en-US/ue-on-github> (Accessed: 14 July
  2026).
- Epic Games (n.d.) *UnrealEngine GitHub network*. Access-controlled repository;
  an unauthenticated request may return `404 Not Found` until the GitHub account
  is linked and authorized as Epic documents. Available at:
  <https://github.com/EpicGames/UnrealEngine> (Accessed: 14 July 2026).
- SHAR repository and selected engine installation (2026),
  `src/uproject/shar.uproject`, `README.md`, and Unreal Engine `Build.version`
  identifying version 5.8.0, changelist 55116800, promoted-build status, and
  branch `++UE5+Release-5.8`.
