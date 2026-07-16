# Application lifecycle and mode runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions

- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity test boundary](../../adr/unreal/runtime/runtime-parity-test-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native render-frame, view, and layer runtime](native-render-frame-view-and-layer-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native gameplay audio, dialogue, and listener boundary](../../adr/unreal/runtime/native-gameplay-audio-dialogue-and-listener-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Local split-screen minigame session UI runtime](local-split-screen-minigame-session-ui-runtime.md)

## Purpose

This specification defines native application startup, mode transitions,
loading,
front-end entry, gameplay entry, pause, demonstration sessions, suspension,
resume, and exit. It replaces singleton context objects, manually ordered
manager
initialization, raw previous-and-next enums, platform heap choreography, and
untyped global event callbacks with explicit asynchronous transition plans.

Application mode coordinates subsystem readiness and presentation. It does not
own mission, progression, save, input-device, vehicle, character, audio, or
world
state.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Game-instance mode coordinator | Active application mode, transition transaction, cancellation, and recovery. |
| Mode catalog | Stable mode identities, allowed transitions, entry and exit plans, and verification. |
| World and feature services | World creation, streaming, gameplay feature activation, and teardown. |
| Profile and save services | Device-local settings, profile selection, migration, and durable state. |
| Front-end and presentation services | Startup media, menus, loading presentation, subtitles, and prompts. |
| Input service | Per-local-player mapping contexts and mode-specific input leases. |
| Audio service | Boot, front-end, loading, gameplay, pause, and demo audio states. |

<!-- markdownlint-enable MD013 -->

No mode directly calls another subsystem's internal manager methods. It submits
a typed request and waits for structured completion evidence.

## Runtime topology

The application module owns these C++ types:

<!-- markdownlint-disable MD013 -->

| Type | Responsibility |
| :--- | :--- |
| `USharApplicationModeDefinition` | Immutable identity, transition rules, required services, entry and exit plan, and verification. |
| `USharApplicationModeCatalogSubsystem` | Definition lookup, graph validation, and revision activation. |
| `USharApplicationModeCoordinator` | One game-instance authority for active mode and transition transactions. |
| `FSharApplicationModeRequest` | Requested target, reason, caller, parameters, and expected revisions. |
| `FSharApplicationModeTransition` | Move-only handle for preparation, commit, rollback, cancellation, and result. |
| `FSharApplicationModeObservation` | Immutable current mode, world, profile, local players, readiness, and presentation state. |
| `FSharApplicationModeResult` | Closed success or failure result with verified postconditions. |

<!-- markdownlint-enable MD013 -->

World-specific gameplay state remains in world and local-player subsystems. The
coordinator stores stable identities and handles, not raw manager pointers.

## Canonical modes

The initial catalog includes these mode identities:

<!-- markdownlint-disable MD013 -->

| Mode | Contract |
| :--- | :--- |
| `entry` | Minimal process and game-instance readiness before startup work. |
| `boot` | Configuration, profile, catalog, save, service, and startup-presentation readiness. |
| `front_end` | Main menu and profile-safe presentation with no active gameplay world. |
| `loading_gameplay` | Transactional preparation of the selected gameplay world and session. |
| `gameplay` | Active campaign or free-roam session. |
| `pause` | Gameplay session retained while pause policy owns input and presentation. |
| `super_sprint_front_end` | Contextual local multiplayer or bonus-race selection. |
| `loading_super_sprint` | Transactional preparation of the selected super-sprint world and participants. |
| `super_sprint` | Active super-sprint session. |
| `loading_demo` | Isolated preparation of a bounded demonstration session. |
| `demo` | Active non-progressing demonstration session. |
| `exit` | Final cancellation, save flush policy, service shutdown, and process return. |

<!-- markdownlint-enable MD013 -->

Mode display names are not identity. Additional modes require a namespaced
catalog definition and transition-graph validation.

## Transition graph

Every definition declares allowed predecessor and successor modes. The catalog
validator rejects:

- missing entry or exit reachability;
- a required mode with no valid predecessor;
- transition cycles that cannot be cancelled or completed;
- a pause transition without a resumable owner;
- a loading mode without one terminal active or recovery mode;
- a mode that retains a world while declaring no world ownership; and
- transitions that bypass required profile, save, or catalog readiness.

The graph is data-driven, but execution steps use a closed native task library.
Arbitrary callbacks or script fragments are forbidden.

## Transition transaction

A transition follows these phases:

1. validate source mode, target mode, caller, parameters, and revisions;
1. freeze or reject conflicting transition requests;
1. prepare required services, worlds, features, assets, and presentation;
1. verify readiness without changing the active mode;
1. acquire input, audio, world, and presentation leases;
1. commit the target mode atomically;
1. release source-mode leases and tear down obsolete state;
1. verify the target postcondition; and
1. publish one terminal result.

Failure before commit leaves the source mode active. Failure after commit
follows
the definition's rollback or safe-recovery mode. A loading-screen animation or
transport callback is not readiness evidence by itself.

## Mode requests and frame execution

Callers submit typed mode requests; they do not push, pop, or replace an ordinal
context stack directly. Every request records source mode, target mode, reason,
priority, caller, expected revisions, parameters, and stable request identity.

Pause and other reversible overlays receive an explicit return token naming the
retained session and permitted predecessor. Returning validates that token
rather than reading the previous value from a mutable global stack. Root
transitions such as front end, gameplay load, recovery, and exit replace the
active mode only through the ordinary transition transaction.

The coordinator evaluates requests at one declared game-instance safe point. A
request issued while another transition is preparing is rejected, queued,
cancelled, or superseded by the concurrency policy. Two requests arriving in one
frame use priority and stable request identity; callback arrival and container
order cannot select the winner.

Mode commit occurs before the target receives active input or simulation leases.
The source remains authoritative until commit. Native subsystem and tick-group
ordering performs per-frame work after commit; the coordinator does not manually
update every manager from one custom timer callback.

Engine delta time and pause policy govern simulation. Development breakpoint or
single-step handling may clamp diagnostic presentation time, but cannot rewrite
accepted gameplay time or silently substitute a fixed frame duration.

Audio, input, save, world, and presentation services tick through their native
owners. A mode request coordinates leases and readiness; it does not become a
second game loop.

Frame execution, native tick groups, local-player views, render scopes, loading
barriers, presentation freezes, display policy, and renderer ownership follow
<!-- markdownlint-disable-next-line MD013 -->
[Native render-frame, view, and layer runtime](native-render-frame-view-and-layer-runtime.md).
A completed timer callback, visible frame, or successful buffer presentation
cannot commit an application-mode transition.

## Result model

Every request reaches one status:

<!-- markdownlint-disable MD013 -->

| Status | Meaning |
| :--- | :--- |
| `success` | Target mode and all required postconditions are active. |
| `rejected` | Graph, caller, parameter, readiness, or concurrency validation failed before mutation. |
| `failed` | Preparation or commit began but could not reach a valid target or rollback state. |
| `timed_out` | A declared transition bound elapsed and cleanup completed. |
| `cancelled` | An allowed cancellation restored the source or recovery mode. |
| `superseded` | A higher-priority lifecycle request replaced the pending request. |

<!-- markdownlint-enable MD013 -->

Every non-success result includes a typed reason and the verified resulting
mode.

## Entry and exit modes

`entry` is the minimum process-to-game-instance handoff. Process launch,
platform
capabilities, service dependencies, native frame ownership, and platform
recovery
follow the
<!-- markdownlint-disable-next-line MD013 -->
[native platform bootstrap and error-recovery runtime](native-platform-bootstrap-and-error-recovery-runtime.md).
Entry may verify package, launch-configuration, crash-recovery, and
game-instance
prerequisites, but it cannot construct gameplay managers or select a player
profile by side effect. Its only successful successor is the validated boot
request.

`exit` is a terminal transition plan. It:

1. rejects new non-exit mode requests;
1. closes modal front-end and pause interactions;
1. resolves required save and configuration policy;
1. cancels or joins bounded asynchronous handles;
1. releases local-player input and presentation leases;
1. tears down active worlds and gameplay features;
1. stops media and audio through typed service requests;
1. flushes permitted diagnostics and platform metadata; and
1. returns control to the platform only after terminal verification.

Exit does not jump through gameplay, pause, or loading modes to trigger cleanup.
A required save that has not reached a terminal result follows explicit block,
cancel, or prior-revision policy. Shutdown order is declared by dependencies and
<!-- markdownlint-disable-next-line MD013 -->
cannot rely on singleton destruction order.

## Boot mode

Boot mode prepares the minimum services needed for a valid front end. Its plan
is
explicitly ordered by dependency, not by singleton construction side effects.

The plan includes:

- platform and device-local configuration;
- localization and accessibility defaults;
- input-device discovery without assigning gameplay ownership;
- root content catalog activation;
- save schema and migration service readiness;
- profile discovery and selection policy;
- front-end UI and presentation feature activation;
- audio service readiness;
- shared native asset bundles required before the front end;
- progression and collectible catalog validation;
- development command and cheat availability policy; and
- startup media sequence or declared fallback.

Services that can prepare independently may run concurrently when their declared
resources do not conflict. The plan still publishes a deterministic readiness
<!-- markdownlint-disable-next-line MD013 -->
result.

## Configuration

Configuration load returns one of `loaded` , `migrated` , `defaulted` , or
`failed` .
A missing optional device-local configuration may create validated defaults. A
malformed or incompatible configuration is quarantined before defaults are
written.

Platform settings do not select different gameplay definitions. Graphics, input,
audio, display, and accessibility settings are projected through the same native
schemas across supported platforms.

## Startup media

Startup logos and other boot presentation are ordered cinematic identities in a
data asset. Each step declares:

- media identity;
- required or optional status;
- audio and localization policy;
- minimum and maximum display policy;
- skippability and skip prerequisite;
- render layer and aspect policy;
<!-- markdownlint-disable-next-line MD013 -->
- failure fallback; and
- completion evidence.

Platform file paths are resolved during native packaging, not constructed by the
runtime. Missing optional media advances to the next step. Missing required
media
returns a typed failure and follows the boot recovery policy.

A development startup override may skip media or request a validated destination
mode. It is excluded from ordinary player builds and cannot grant progression by
changing the startup route.

## Front-end entry

Front-end entry requires:

- active root catalogs;
- a valid device-local configuration;
- profile-selection state;
- save service availability or explicit offline fallback;
- front-end UI and audio feature activation;
<!-- markdownlint-disable-next-line MD013 -->
- no retained gameplay-world authority; and
- input leases for the active local front-end users.

Returning from gameplay, demo, or super sprint must cancel world-bound handles
before front-end commit. Front-end presentation cannot infer completion or
unlock
state from the previous mode.
<!-- markdownlint-disable-next-line MD013 -->

The committed front end acquires one UI-navigation lease, one front-end audio
state, and local-player menu input leases. It may discover slot summaries,
rewards, and cheat availability through read-only ports. It cannot retain world
physics, gameplay cameras, trigger volumes, vehicle simulation, mission state,
or
world-bound render observers.

A deferred front-end asset or audio callback must match the current mode and
feature revision. Completion from an earlier front-end visit cannot reactivate a
released screen or dismiss a newer loading state.

## Loading-mode contract

Every loading mode is a preparation transaction, not an active gameplay mode.
The common loading plan owns:

- destination request and transition identity;
- staged world and game-feature handles;
- Asset Manager bundle requests;
- loading-screen and progress observations;
- local-player and controller preparation;
- audio-bank and mix preparation;
- save or transient-session snapshot validation;
<!-- markdownlint-disable-next-line MD013 -->
- timeout, cancellation, and recovery mode; and
- one final readiness barrier.

Loading UI may continue updating while preparation is in progress, but gameplay
input, mission time, rewards, collisions, and world interactions remain
disabled.
<!-- markdownlint-disable-next-line MD013 -->
A loading callback is accepted only when transition, destination, world, bundle,
and service revisions still match.

The loading plan follows the
<!-- markdownlint-disable-next-line MD013 -->
[native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md).
It never depends on platform heap replacement for correctness. Native world,
object, asset, and gameplay-feature lifetimes own memory release.
<!-- markdownlint-disable-next-line MD013 -->

### Loading progress authority

Loading presentation consumes the correlated load-plan snapshot defined by the
<!-- markdownlint-disable-next-line MD013 -->
[frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md).
Progress comes only from validated operation producers and required readiness
barriers.

Process memory use, free memory, expected file counts, callback counts, elapsed
animation time, and frame count are not progress authority. Operations without a
meaningful measure remain indeterminate. A loading screen may reach its visual
end state only after the lifecycle service accepts the terminal transition
result.

## Gameplay loading

`loading_gameplay` consumes one validated session request containing campaign,
level, mission, player, character, vehicle, world composition, save revision,
and
mod-set identities as applicable.

Preparation includes:

- session-definition validation;
- destination world and game-feature activation;
- World Partition and Runtime Data Layer readiness;
- required native asset bundles;
- mission, population, vehicle, camera, audio, and UI subsystem readiness;
- local-player creation and controller assignment;
<!-- markdownlint-disable-next-line MD013 -->
- save snapshot or new-session state;
- spawn and recovery locations; and
- final cross-subsystem read-back.

The active mode changes to gameplay only after every required authority agrees
on
the same session and world revision.

## Active gameplay mode

The committed gameplay mode activates one verified session composition. Its
lifecycle lease set includes:

- world and Runtime Data Layer composition;
- mission and objective execution;
- character, vehicle, traffic, and population simulation;
- physics, collision, trigger, and interaction services;
- camera managers for each local player;
- HUD, radar, navigation, and presentation;
- gameplay audio, music, ambience, and dialogue;
- persistent-world state projection; and
- save-boundary and checkpoint observations.

Subsystem updates follow native engine tick groups and explicit dependencies.
Application mode does not manually call every gameplay manager in one arbitrary
order. Presentation may continue during selected partial-pause states, but it
<!-- markdownlint-disable-next-line MD013 -->
cannot advance domain time or durable transactions.
<!-- markdownlint-disable-next-line MD013 -->

Gameplay exit first freezes new mission and progression transactions, captures
any permitted checkpoint candidate, releases local-player world input, and then
tears down world-bound services. A level-complete or mission-complete result
must
already be committed by its owning domain before the mode transition consumes
it.

## Pause and resume

Pause is a reversible application-mode overlay over one active gameplay or
super-sprint session. It declares:

- simulation pause or partial-pause policy;
- input mapping and focus ownership;
- audio mix;
- UI presentation;
- online or platform restrictions; and
- save and settings permissions.

Resume verifies that the retained world, local players, controllers, and session
remain valid. If they do not, the coordinator follows the pause recovery policy
rather than restoring stale pointers.

## Platform suspension

Operating-system suspension is separate from the player pause menu. Suspension
requests:

- stop accepting new application transitions;
- checkpoint permitted device-local and portable state;
- pause or release media, audio, network, input, and platform resources;
- preserve the active mode and world revision when supported;
- cancel unsafe asynchronous work; and
- record a resume token with no raw object pointers.

Resume revalidates profile, storage, controller, display, audio, world, and
transition state. A stale or unrecoverable world returns to a declared recovery
<!-- markdownlint-disable-next-line MD013 -->
mode, normally the front end.

## Demonstration loading

`loading_demo` creates an isolated non-progressing session. Its request
declares:

- demo definition identity;
- world and route;
- character and vehicle presentation;
- camera policy;
- duration and exit conditions;
<!-- markdownlint-disable-next-line MD013 -->
- input policy;
- audio and UI presentation; and
- return mode.

The loading transaction uses the same world, asset, audio, and verification
ports
as gameplay loading. It cannot set a global demo flag and then rely on unrelated
systems to infer altered behavior.

## Demonstration session

Demo mode is a bounded session with no durable progression authority. It may run
AI or recorded input under an explicit definition. The session:

- uses a dedicated transient session identity;
- rejects save, purchase, reward, achievement, and completion transactions;
- uses typed camera and UI policies;
- records a deterministic timeout or terminal observation;
- supports permitted skip or user takeover behavior; and
- returns to the declared front-end mode after cleanup.

A demonstration transition triggered from a cheat remains an immediate typed
command. It does not enable arbitrary mode changes.

## Super-sprint modes

`super_sprint_front_end` owns participant selection, controller assignment,
vehicle selection, race selection, and return intent. It may load front-end-only
presentation bundles but cannot construct the race world or start race time.

`loading_super_sprint` validates the complete participant and race request, then
uses the common loading contract for world, local players, vehicles, characters,
audio, cameras, UI, and route readiness. Every local player must have one stable
identity, controller assignment, viewport policy, and accepted vehicle before
commit.

The committed `super_sprint` mode owns the transient race session. It activates:

- race route, lap, checkpoint, and finish policy;
- split-screen viewport and camera assignments;
- local-player input and vehicle command leases;
- race HUD, countdown, result, and return presentation;
- gameplay physics, collision, effects, and audio; and
- explicit non-campaign reward or progression policy.

Race start occurs only after mode commit and verified participant readiness.
Returning to the super-sprint front end cancels race-bound handles and destroys
the race world before selection UI becomes interactive.

Historical platform-specific context variants do not create different gameplay
rules. Platform presentation and input adapters remain separate.

The complete built-in lobby, join, controller, character, vehicle, readiness,
countdown, per-player HUD, pause, summary, replay, and teardown behavior follows
<!-- markdownlint-disable-next-line MD013 -->
[Local split-screen minigame session UI runtime](local-split-screen-minigame-session-ui-runtime.md).
That same-device transient session remains separate from mod-owned network
multiplayer and cannot become campaign progression authority implicitly.

## Input

<!-- markdownlint-disable-next-line MD013 -->
Every committed mode owns a set of semantic input leases. Input registration is
per local player and explicit about front-end, gameplay, pause, demo, or
super-sprint actions. Device discovery, assignment, mappings, pointer behavior,
and haptics follow the
<!-- markdownlint-disable-next-line MD013 -->
[semantic input, device, and haptics runtime](semantic-input-device-and-haptics-runtime.md).

Transition preparation may stage mappings, but only commit activates them.
Cancellation, controller removal, local-player removal, world teardown, and mode
exit release all affected leases. A mode cannot leave a controller assigned to a
destroyed player or retain a hidden input handler.
<!-- markdownlint-disable-next-line MD013 -->

## Audio

Mode definitions request audio states rather than calling internal audio-manager
methods. Boot, front-end, loading, gameplay, pause, demo, and super-sprint
states
declare required banks, mixes, ambience, music, and transition policy.

A loading completion callback is accepted only for the current transition and
audio revision. Late callbacks cannot activate audio in a replacement mode.

Vehicle audio, dialogue queues, local-player listeners, positional sources,
ducking, subtitles, and mouth presentation are owned by their dedicated runtime
contracts. A mode transition freezes new affected requests, validates target
audio
and listener readiness, commits the new leases atomically, and then releases
obsolete sources and callbacks.

An audio component becoming audible or completing playback cannot commit a mode,
mission, interaction, vehicle, or save transition.

## World and feature lifecycle

World creation and destruction occur through native engine travel, world,
streaming, and game-feature services. The application coordinator owns only the
transition handle.

A source mode's world remains authoritative until target preparation succeeds.
After commit, obsolete worlds and features are torn down in declared order.
Memory ownership, budgets, pressure response, pools, and diagnostics follow the
<!-- markdownlint-disable-next-line MD013 -->
[memory ownership, budget, and diagnostics runtime](memory-ownership-budget-and-diagnostics-runtime.md).
No platform heap reset is required for correctness; ownership follows native
object, world, subsystem, feature, streaming, and asset-manager lifetimes.
<!-- markdownlint-disable-next-line MD013 -->

## Events

Subsystems publish typed readiness and lifecycle observations. The coordinator
correlates them by transition, world, service, and revision identity. Channel,
payload, scope, delivery, and subscription rules follow the
<!-- markdownlint-disable-next-line MD013 -->
[typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md).

Untyped global events, listener order, and payload pointers cannot complete a
transition. A late event for an earlier transition is ignored and recorded.

## Concurrency

Only one committing application transition may exist at a time. Read-only
observations and independent preparation tasks may run concurrently.

Request priority is:

1. fatal platform or process exit;
1. unrecoverable world or save recovery;
1. operating-system suspension;
1. explicit user mode transition;
1. gameplay-driven transition;
1. development or automation request.

Equal-priority requests use stable request identity. Frame arrival order cannot
select the winner.

## Failure and recovery

Every mode declares a safe recovery target. Typical policies are:

- boot failure to a recoverable error presentation or exit;
- gameplay-load failure back to front end;
- demo-load failure back to its invoking front end;
- pause-resume invalidation back to front end;
- super-sprint-load failure back to super-sprint front end; and
- shutdown failure to best-effort process exit after recording diagnostics.
<!-- markdownlint-disable-next-line MD013 -->

Recovery cannot grant progression, duplicate rewards, retain stale worlds, or
<!-- markdownlint-disable-next-line MD013 -->
skip required save migration.

## Save boundary

Application mode is device and session state, not portable progression. Saves
may
record the last safe front-end or gameplay resume intent, but they do not
serialize raw context state, transition tasks, worlds, input handlers, or
manager
pointers.

A save transaction begun before a transition either completes under its original
session identity or is cancelled according to save policy. Mode change alone
does
not imply a successful save.

## Mods

A validated feature overlay may add a namespaced application mode only when it
declares transition graph edges, required services, assets, worlds, permissions,
recovery, teardown, and tests.

A mod cannot replace boot, front-end, save recovery, suspension, or exit policy
without an accepted first-party override contract.

## Validation

Catalog validation rejects:

- unreachable modes;
- invalid transition cycles;
- loading modes without success and recovery targets;
- entry or exit tasks without typed native implementations;
- missing timeouts or cancellation behavior;
- modes that own undeclared worlds, input, audio, or UI leases;
- startup media with platform paths;
- demo definitions with durable progression permissions;
- suspend or resume plans without storage and controller reconciliation; and
- development overrides included in unauthorized player packages.

## Tests

Required tests include:

- graph reachability and invalid-edge rejection;
- boot success, optional-media fallback, and required-media failure;
- configuration load, migration, quarantine, and defaulting;
- front-end readiness and gameplay-world absence;
- gameplay loading success, failure, timeout, cancellation, and rollback;
- pause and resume lease restoration;
- operating-system suspend and resume reconciliation;
- demo loading and non-progressing session enforcement;
- super-sprint local-player isolation;
- late loading and audio callback rejection;
- world teardown and feature deactivation;
- conflicting transition priority and stable tie-breaking;
- save transaction interaction; and
- mod mode registration and recovery.

## Invariants

- Application mode is a stable identity, not an array ordinal.
- One transition transaction owns each mode change.
- Preparation completion is verified before commit.
- Input, audio, world, and presentation ownership uses move-only leases.
- Demo sessions cannot mutate durable progression.
- Platform suspension is not the same as player pause.
- Late callbacks cannot complete a replacement transition.
- Every failed transition reaches a declared valid mode or typed fatal result.
