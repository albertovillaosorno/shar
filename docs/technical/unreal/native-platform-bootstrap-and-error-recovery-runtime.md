# Native platform bootstrap and error-recovery runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
- [Shared runtime tagging, modding, and platform compatibility](../../adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)
- [Device configuration and save-slot runtime](device-configuration-and-save-slot-runtime.md)
- [Semantic input, device, and haptics runtime](semantic-input-device-and-haptics-runtime.md)
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-enable MD013 -->

## Purpose

This specification defines process entry, Unreal engine startup, game-instance
bootstrap, platform capabilities, native frame ownership, legal presentation,
localized platform errors, recovery, restart, exit, and diagnostic capture.

It replaces platform-specific application entrypoints, manual subsystem
construction lists, custom frame loops, raw operating-system message dispatch,
compiled error-string arrays, direct device-reset procedures, embedded legal
byte arrays, and platform-specific gameplay branches.

Unreal owns the process launcher, engine loop, platform application services,
windowing, rendering context, and ordinary device events. Repository code owns
only typed product policy and application coordination.

## Ownership

<!-- markdownlint-disable MD013 -->
| Authority | Responsibility |
| :--- | :--- |
| Unreal launcher and build target | Process entry, engine creation, platform runtime, and package startup. |
| Engine loop | Frame scheduling, task servicing, rendering, input polling, audio integration, and shutdown callbacks. |
| Game instance | Application-mode coordinator and product service readiness. |
| Platform adapter | Capabilities, suspension, display, storage, system UI, restart, and terminal exit. |
| Error-recovery subsystem | Typed platform observations, pause policy, recovery requests, and terminal results. |
| Device configuration | User-requested display, window, audio, input, and accessibility settings. |
| Presentation catalog | Legal, startup, error, and recovery text and media. |
| Diagnostic subsystem | Crash, trace, screenshot, platform, and recovery evidence. |
<!-- markdownlint-enable MD013 -->

The platform adapter does not own missions, progression, world state, camera,
vehicle behavior, content selection, or gameplay timing.

## Runtime topology

The platform module owns these native types:

- `FSharPlatformTargetId`, an exact platform and architecture identity;
- `FSharProcessBootId`, one process-start attempt;
- `FSharPlatformSessionId`, one active platform service session;
- `FSharPlatformCapabilitySnapshot`, immutable capability evidence;
- `FSharPlatformErrorId`, a stable namespaced error identity;
- `FSharPlatformErrorObservation`, a typed normalized platform callback;
- `FSharPlatformRecoveryRequest`, a validated recovery transaction;
- `FSharPlatformRecoveryResult`, a closed terminal recovery result;
- `FSharPlatformPresentationLease`, scoped error or legal presentation;
- `FSharPlatformSuspendToken`, pointer-free suspension evidence;
- `USharPlatformLifecycleSubsystem`, game-instance coordination; and
- platform adapters behind narrow application ports.

A target-specific module may translate native callbacks. It cannot create a
second game loop or replace shared domain behavior.

## Process entry

Each package uses Unreal's generated native entrypoint for its target. Product
startup begins only after the engine creates a valid game instance.

Repository startup performs these ordered steps:

1. identify the exact packaged target and build profile;
1. parse the immutable launch configuration;
1. verify required package and platform metadata;
1. create the game instance and platform lifecycle subsystem;
1. obtain a revisioned capability snapshot;
1. initialize device-local configuration and localization;
1. register root game features and application services;
1. submit the typed `entry` application-mode request; and
1. record one terminal boot result.

No product entrypoint manually initializes memory, files, timers, rendering,
audio, input, debug communication, or platform SDK services before Unreal.
Those services follow engine and platform-module ownership.

A failed prerequisite returns a typed process-start failure. It does not
continue with a partially initialized singleton graph.

## Build targets

Every target definition declares:

- exact target and architecture identity;
- Unreal target and module set;
- supported build profiles;
- required platform SDK and packaging features;
- legal and startup presentation profile;
- platform capability expectations;
- crash and diagnostic policy;
- storage, input, display, audio, and system-UI adapters; and
- package acceptance tests.

Historical console-specific launchers do not create modern product targets.
Only the repository's declared native target matrix may be built and advertised.

## Capability snapshot

The platform adapter publishes one immutable snapshot containing:

- target and architecture identity;
- build and package profile;
- operating-system and platform-runtime revision when safely available;
- display, window, refresh, HDR, and safe-area capabilities;
- input, pointer, touch, haptic, and controller capabilities;
- storage, save, quota, and package-mount capabilities;
- suspension, resume, restart, and terminal-exit support;
- media, audio, locale, and text-input capabilities;
- crash-reporting and diagnostic-capture support; and
- snapshot revision and observation time.

Gameplay code queries semantic capabilities through ports. It does not test
compiler platform macros to choose different mission or simulation behavior.

A changed display, input, storage, or resume capability produces a new snapshot
revision. Callers must not mutate the prior snapshot.

## Service dependency graph

Product services are Unreal subsystems, game features, world services,
components, or explicit native objects with declared ownership. Startup and
shutdown order follow dependency edges.

The service graph records:

- stable service identity;
- owner and scope;
- required predecessors;
- readiness and teardown contract;
- package and feature dependencies;
- failure and degraded policy;
- availability profile; and
- verification evidence.

A manually ordered function that creates every manager is forbidden.
Constructor order, global static order, source-file order, and destructor order
cannot define runtime dependencies.

Service readiness may execute concurrently when declared resources do not
conflict. One terminal barrier proves the boot set is ready.

## Native frame ownership

The Unreal engine loop owns frame advancement. Repository code participates
through native subsystem ticks, actor and component ticks, task graph work,
Slate, audio, render delegates, and declared engine callbacks.

The product does not:

- spin a custom `while` loop;
- poll operating-system messages itself;
- manually service file, debug, audio, render, and input libraries each frame;
- call every gameplay manager from one timer callback;
- switch asynchronous loader tasks by hand; or
- pause the complete engine loop because one product service reports an error.

Simulation delta, pause policy, fixed-step systems, frame pacing, and rendering
follow their owning engine contracts. Application modes coordinate leases and
readiness but do not become a second scheduler.

## Deterministic session seed

Every gameplay or automated session receives an explicit seed from its session
request, save, replay, test fixture, or validated generation policy.

Wall-clock date and time may seed a non-replayable presentation session only
when its result is recorded. Wall-clock values cannot silently select mission,
gag, traffic, progression, physics, AI, or test behavior.

The accepted seed and generator revision enter deterministic evidence. Platform
integer width or compiler type aliases do not change the sequence.

## Platform error catalog

Platform errors use stable semantic identities rather than array positions.
Initial families include:

- package or required content unavailable;
- installed data corrupt or unreadable;
- storage provider unavailable;
- quota or free-space failure;
- save media removed or changed;
- input device disconnected or reassigned;
- display, surface, or rendering device lost;
- audio device unavailable;
- suspension, resume, focus, or user-session change;
- network or platform service unavailable when applicable;
- out-of-memory or resource-budget exhaustion;
- platform restart or terminal-exit request; and
- unrecoverable process or engine failure.

Each definition declares severity, scope, retry policy, user action,
presentation, audio and haptic policy, timeout, telemetry, recovery target, and
shipping availability.

A native numeric code is adapter evidence only. The adapter maps it to one
canonical error identity and preserves the redacted native code for diagnostics.

## Localized error presentation

Error definitions refer to localization keys and presentation assets. They do
not compile translated strings into platform headers.

Presentation declares:

- title, body, action, and accessibility text keys;
- required glyph and active-input policy;
- modal or non-modal behavior;
- safe-area and screen-reader policy;
- permitted retry, continue, settings, storage-management, restart, and exit
  actions;
- audio ducking or pause policy;
- haptic cancellation policy; and
- legal or platform-required wording profile.

Locale selection follows the ordinary localization service. Missing required
text blocks the package or fails safely to an approved fallback language.
Mojibake, invalid encoding, or array-offset language selection is forbidden.

## Recovery transaction

A recoverable platform observation creates one transaction:

1. normalize and validate the native observation;
1. correlate platform, user, device, world, mode, and request revisions;
1. acquire the required application, input, audio, and presentation leases;
1. pause or limit only the affected product scope;
1. cancel or retain asynchronous work according to policy;
1. present typed recovery actions;
1. execute the accepted action through the platform adapter;
1. revalidate every affected capability and owner; and
1. publish one terminal recovery result.

A recovery callback from an older observation cannot dismiss a newer error,
resume a replacement world, reactivate a removed controller, or complete a newer
storage request.

Recovery is idempotent by transaction identity. Repeated native callbacks may
refresh diagnostics but do not create duplicate modal screens or actions.

## Error pause policy

A platform error never mutates one global paused flag. The definition selects a
policy such as:

- presentation-only warning;
- local-player input suspension;
- gameplay simulation pause;
- media or audio pause;
- save and storage transaction block;
- application-transition block;
- world recovery; or
- terminal process failure.

The engine, UI, input, audio, and platform services continue the minimum work
required to display and resolve the error. Unaffected local players and services
follow the declared product policy.

## Input-device recovery

Controller removal follows the semantic input and device contract. The platform
adapter reports connection and user-association observations; it does not own
player identity.

The recovery surface may request reconnect, reassignment, pause, continue under
a permitted fallback, or return to the front end. Haptics stop before the device
session is released.

Input recovery never resumes hidden held actions or reconstructs a mapping from
physical port order.

## Storage and package recovery

Storage and package errors correlate provider, package or slot, operation,
expected revision, and safe retry policy.

A retry creates a new operation identity. Continue-without-saving, choose
another slot, open platform storage management, redownload required content, or
return to the front end are explicit product actions when the target supports
them.

A required package failure cannot be dismissed into gameplay. A
presentation-only optional package may degrade according to its load-result
policy.

## Display and window lifecycle

Display mode is device-local configuration projected through Unreal's window and
rendering APIs. The platform adapter reports supported monitor, mode, refresh,
window, fullscreen, HDR, and safe-area capabilities.

A configuration transaction validates the requested mode, applies it, reads the
resolved engine state, and commits or rolls back. A fixed source-era resolution
enum is not product authority.

Focus, minimize, display removal, surface recreation, and rendering-device
recovery produce typed observations. They do not call gameplay objects from an
operating-system window procedure.

Legacy progressive-scan prompts become ordinary display capability and
configuration behavior. Startup button chords do not choose hidden display
modes.

## Suspension and resume

Operating-system suspension follows the application lifecycle contract. The
platform adapter produces a suspend token with platform user, capability,
configuration, world, session, and operation revisions.

Resume validates every affected service before application-mode restoration.
A stale world, removed user, invalid storage provider, or lost package follows
the declared recovery target rather than resuming stale pointers.

## Restart, system UI, and exit

Restart, platform system UI, storage management, and terminal exit are typed
platform requests. Every request declares caller, permission, reason, user,
timeout, save policy, and fallback.

The application coordinator reaches a safe terminal boundary before process exit
or restart when the platform permits it. A dashboard or system-UI request does
not stand in for product cleanup.

Unsupported system UI returns `unavailable`. Repository code never guesses a
platform executable, shell command, or private URI.

## Legal and startup presentation

Legal and ownership presentation uses cooked UI, texture, font, localization,
and media assets selected by a package profile.

Generated source headers containing image bytes and scripts that convert images
into code are conversion provenance only. They are not runtime assets and cannot
be the only retained legal evidence.

The package validates required notice identity, asset hash, locale coverage,
minimum display policy, accessibility, and completion evidence. Product legal
presentation does not imply rights in third-party content.

## Diagnostic screenshot capture

Screenshot capture is an editor, automation, development, or explicitly
permitted user feature implemented through Unreal's screenshot and render-target
facilities.

A capture request declares:

- capture identity and caller permission;
- world, viewport, player, camera, and frame identity;
- resolution, color-space, alpha, UI, and debug-overlay policy;
- output format and approved destination;
- redaction and privacy policy;
- timeout and cancellation; and
- expected rendering and package revisions.

The result includes image dimensions, format, hash, capture metadata, and typed
failure. It does not expose raw frame-buffer pointers, network transfer buffers,
or platform-specific pixel-conversion code to gameplay.

Automated golden captures use deterministic camera, presentation, locale,
quality, and frame barriers. A screenshot alone does not prove gameplay parity.

## Crash and fatal failure

Recoverable product errors use typed results. Unrecoverable engine, memory,
rendering, package-integrity, or platform failures enter the fatal policy.

The fatal path:

1. prevents new gameplay and save mutations;
1. records bounded crash and correlation evidence;
1. flushes approved diagnostic sinks when safe;
1. presents only platform-approved fatal UI when available;
1. avoids allocating unbounded recovery state; and
1. returns control to Unreal and the platform crash policy.

A fatal handler cannot continue gameplay after an invariant or memory state is
known to be unsafe.

## Platform-specific modules

Target adapters may implement:

- capability normalization;
- suspension and resume callbacks;
- native storage, system UI, and user services;
- display and window integration;
- platform error-code mapping;
- input and haptic capability reporting;
- crash metadata; and
- package-mount integration.

They may not define alternate content IDs, missions, physics, save semantics,
progression, AI, or quality meaning.

## Mods and game features

A validated game feature may register platform-dependent capabilities or
presentation only when its manifest declares target support, owner, teardown,
package policy, permissions, and tests.

A feature cannot replace process entry, fatal recovery, legal presentation,
crash policy, storage security, or terminal exit without an accepted first-party
override.

## Diagnostics

Development diagnostics expose:

- process boot and platform session identities;
- target, architecture, package, and capability revisions;
- service dependency and readiness state;
- application and engine-loop observations;
- active platform error and recovery transactions;
- suspension and resume tokens;
- display and window observations;
- restart, system-UI, and exit requests;
- legal and startup presentation results; and
- screenshot and crash-capture results.

Diagnostics use stable identities and redaction. Native handles, private paths,
user secrets, and raw crash memory are not ordinary log output.

## Failure behavior

The runtime fails closed on:

- unknown or mismatched platform target;
- invalid build, package, or capability revision;
- service dependency cycle or missing required service;
- duplicate process or platform session identity;
- malformed native error mapping;
- missing required localized error or legal presentation;
- stale recovery callback;
- invalid display or window configuration;
- unsupported restart, system UI, or exit action;
- screenshot destination or permission violation;
- recovery after fatal state; and
- platform adapter behavior that changes shared gameplay semantics.

A platform failure cannot grant progression, skip required migration, or create
another gameplay definition.

## Validation

Build and catalog validation prove:

- every target has one Unreal launcher and package profile;
- every platform service has one owner and dependency contract;
- required capabilities have adapters and acceptance tests;
- native error codes map uniquely to canonical identities;
- required error and legal text exists for supported locales;
- display and window profiles contain only supported modes;
- screenshot policies use approved destinations and permissions;
- development-only capture and diagnostics are excluded as required;
- shutdown dependencies are acyclic; and
- platform adapters contain no alternate gameplay catalog.

## Tests

Required tests include:

- process entry and boot success;
- failure before and after game-instance creation;
- service dependency ordering and independent concurrency;
- native engine-loop ownership and absence of a custom loop;
- deterministic session seed and replay;
- target and capability snapshot validation;
- localized error mapping and encoding;
- repeated error observation deduplication;
- input disconnect and reassignment recovery;
- storage, package, and quota recovery;
- display mode apply, read-back, rollback, and device loss;
- suspension, resume, stale token, and recovery;
- restart, system UI, and terminal exit;
- required legal presentation and optional startup fallback;
- screenshot permission, determinism, and teardown;
- fatal error path with bounded allocation; and
- platform gameplay-parity comparisons.

## Invariants

- Unreal owns process entry and the engine loop.
- Product startup is a typed dependency plan, not singleton construction order.
- Platform targets differ only through declared adapters and capabilities.
- Error identity is semantic and localized presentation is data-driven.
- Recovery is revisioned, idempotent, scoped, and terminal.
- A display or device callback cannot mutate gameplay directly.
- Wall-clock time is not an implicit gameplay seed.
- Legal presentation uses cooked assets, not embedded source byte arrays.
- Diagnostic screenshots are bounded and non-authoritative.
- Platform failure cannot create alternate gameplay or progression behavior.
