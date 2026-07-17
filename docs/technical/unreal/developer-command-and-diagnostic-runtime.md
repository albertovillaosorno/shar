# Developer command and diagnostic runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions

- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity test boundary](../../adr/unreal/runtime/runtime-parity-test-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Validated game-feature mod overlays](../../adr/unreal/runtime/validated-game-feature-mod-overlays.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Gameplay audio source, residency, mix, and environment runtime](gameplay-audio-source-residency-mix-and-environment-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native audio device, resource, player, and tuning adapter runtime](native-audio-device-resource-player-and-tuning-adapter-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Authored state-prop animation and event runtime](authored-state-prop-animation-and-event-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Local supersprint race session runtime](local-supersprint-race-session-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md)

## Purpose

This specification defines the native development-command registry, diagnostic
console, structured logging, aliases, completion, asynchronous execution, and
input presentation. It replaces fixed function tables, raw callback pointers,
runtime text-script evaluation, process-wide mutable console state, and custom
case-insensitive string helpers with validated command definitions and typed
execution.

The surface exists for editor, automation, development, and explicitly
authorized diagnostic builds. It is not a second gameplay scripting language and
is not an ordinary player feature.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Command catalog | Stable command identities, schemas, permissions, availability, help, and aliases. |
| Command registry subsystem | Definition activation, lookup, completion, dispatch, and lifecycle. |
| Command handlers | Typed application-port calls and structured results. |
| Logging subsystem | Categories, severity, sinks, filtering, and redaction. |
| Development UI | Entry, history, completion, output projection, and accessibility. |
| Gameplay and domain services | Authoritative state transitions requested by permitted handlers. |

<!-- markdownlint-enable MD013 -->

The command runtime never owns progression, mission, vehicle, world, save,
economy, or content state. A command handler may request a typed domain
operation
only when its definition authorizes that operation.

## Runtime topology

The development module owns these C++ types:

<!-- markdownlint-disable MD013 -->

| Type | Responsibility |
| :--- | :--- |
| `USharDeveloperCommandDefinition` | Immutable identity, argument schema, availability, permission, execution, and help contract. |
| `USharDeveloperCommandAliasDefinition` | Validated alias and typed argument-template mapping. |
| `USharDeveloperCommandRegistrySubsystem` | Catalog activation, normalized lookup, completion, dispatch, and revocation. |
| `ISharDeveloperCommandHandler` | Native typed handler interface registered by canonical command identity. |
| `FSharDeveloperCommandRequest` | Caller, command identity, typed arguments, world, local player, and request revision. |
| `FSharDeveloperCommandResult` | Closed status, observations, diagnostics, and optional asynchronous handle. |
| `FSharDeveloperCommandHandle` | Cancellation-safe handle for long-running work. |
| `USharDeveloperConsoleViewModel` | Development-only entry, history, completion, and output projection. |

<!-- markdownlint-enable MD013 -->

The registry is world-aware. Definitions may be shared immutable assets, while
active handles and caller permissions belong to a specific editor, world, or
local-player scope.

## Command definition

Every command definition contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `CommandId` | Globally unique canonical identity. |
| `DisplayName` | Human-readable development label, never runtime identity. |
| `ArgumentSchema` | Ordered typed arguments with names, types, bounds, and defaults. |
| `Availability` | Editor, automation, development build, authorized diagnostic build, or excluded. |
| `Permission` | Read-only, presentation mutation, world mutation, domain transaction, or process control. |
| `ExecutionKind` | Registered native handler identity. |
| `WorldPolicy` | No world, editor world, preview world, game world, or explicit target world. |
| `ConcurrencyPolicy` | Reject, replace, queue, or parallel under declared resource rules. |
| `TimeoutPolicy` | Positive bound or explicit long-running permission. |
| `ResultSchema` | Closed success and failure observation types. |
| `Help` | Summary, argument descriptions, examples, and safety notes. |
| `DefinitionRevision` | Immutable revision used to reject stale invocations. |

<!-- markdownlint-enable MD013 -->

Definitions cannot contain source-language callbacks, memory addresses, raw
function pointers, arbitrary executable text, machine-specific paths, or
unbounded variadic argument rules.

## Launch configuration and startup overrides

Startup overrides use a separate immutable launch schema. They are parsed before
application-mode entry and cannot be mutated as process-wide bits after startup.
Every launch field declares:

- canonical identity and typed value;
- build and platform availability;
- default source and precedence;
- permission and shipping policy;
- affected subsystem and readiness requirement;
- redaction and diagnostic behavior; and
- whether the field is test, presentation, or process configuration.

Initial development families include:

- startup campaign, level, mission, or demonstration identity;
- front-end, startup-media, tutorial, traffic, pedestrian, and HUD test policy;
- audio mix, music, dialogue, effects, and haptics diagnostic policy;
- window, display, language, and platform presentation overrides;
- profiling, memory, loading, frame-rate, bounds, and zone diagnostics;
- deterministic input-injection and demonstration automation;
- save, media, package, and missing-content fault injection; and
- explicitly authorized developer or designer capabilities.

Campaign, level, and mission overrides resolve through canonical content
identities and ordinary availability checks. A trailing digit, enum position, or
unchecked integer cannot select content.

User-facing graphics, audio, input, language, and accessibility preferences
remain owned by device configuration. A launch override may temporarily replace
a value for editor or automation evidence, but it does not rewrite saved
preferences.

Shipping builds use an explicit allowlist limited to platform-required and
product-approved startup fields. Developer diagnostics, random input, content
skips, population removal, cheats, profiling, fault injection, and direct
mission selection are excluded unless an authorized diagnostic package declares
them.

Unknown, duplicate, malformed, unavailable, or conflicting launch fields fail
with typed diagnostics. Parsing never copies an unbounded token into a fixed
buffer or evaluates the token as console text.

Precedence is deterministic:

1. repository and platform defaults;
1. signed package or deployment profile;
1. device-local user configuration;
1. authorized launch profile; and
1. explicit automation invocation when the build permits it.

Later sources may override only fields whose schema allows that source. Launch
configuration is recorded in the application diagnostic snapshot and cannot
silently change after mode entry.

## Stable lookup

Command identity is normalized during catalog generation. Lookup uses the
canonical identity or one validated alias. Display casing is presentation only.

Normalization must:

- reject empty or whitespace-only names;
- reject duplicate canonical identities;
- reject two aliases that normalize to the same key;
- preserve an exact canonical spelling for diagnostics;
- avoid locale-dependent comparisons; and
- reject control characters, path separators, shell metacharacters, and hidden
  Unicode distinctions not permitted by repository policy.

Native `FName` or generated identity wrappers may implement lookup. Ad hoc
uppercase buffers and custom string-comparison functions are not runtime
authority.

## Arguments

The parser accepts only the types declared by the command schema, including:

- Boolean;
- bounded signed or unsigned integer;
- bounded finite scalar;
- string with length and character policy;
- gameplay tag;
- stable catalog identity;
- soft object path validated against an allowed class and root;
- world, local-player, actor, vehicle, or mission identity resolved through a
  typed selector; and
- closed enum generated from repository-owned data.

Quoted text and escaping follow one documented grammar. Comments, nested command
substitution, environment expansion, shell operators, pointer literals, and
implicit file reads are forbidden.

Parsing returns a typed result with token span and reason. It never truncates an
overlong token into a different valid command or argument.

## Registration

Handlers register by canonical command identity through module startup or an
explicit feature activation transaction. Catalog activation fails when:

- a definition has no handler;
- a handler declares a different argument or result schema;
- a command exceeds the availability or permission of its module;
- an alias references a missing command;
- an alias template supplies an invalid argument;
- a command depends on an unavailable world or feature; or
- two modules claim the same identity.

Registration order cannot change lookup, completion, permissions, or execution.
There is no fixed command, alias, argument, buffer, or callback capacity.

## Aliases

An alias maps one stable alias identity to one command identity plus a typed
argument template. Placeholders bind by declared argument name, not positional
text replacement.

Alias validation rejects:

- recursive or cyclic aliases;
- expansion into another unresolved alias;
- missing required arguments;
- type-invalid literals;
- privilege escalation;
- hidden world or caller changes; and
- expansion that exceeds the target command's argument bounds.

The result records both the invoked alias and canonical command identity.

## Execution

Execution follows this order:

1. resolve the caller, world, and command revision;
1. verify build availability and caller permission;
1. parse and validate all arguments without side effects;
1. resolve stable target identities;
1. acquire declared resources;
1. invoke the registered typed handler;
1. verify the declared postcondition;
1. release resources; and
1. publish one structured result.

A handler cannot call another command through formatted text. Shared behavior is
an application service or typed helper invoked directly.

## Result model

Every invocation returns one status:

<!-- markdownlint-disable MD013 -->

| Status | Meaning |
| :--- | :--- |
| `success` | The declared postcondition was observed. |
| `rejected` | Availability, permission, schema, target, or precondition failed before mutation. |
| `failed` | Execution began but did not reach the postcondition. |
| `timed_out` | The authored bound elapsed and cleanup completed. |
| `cancelled` | The caller or lifecycle cancelled the operation and cleanup completed. |
| `queued` | The concurrency policy accepted the request for later execution. |

<!-- markdownlint-enable MD013 -->

Free-form text may accompany a result for developers, but text cannot drive
subsequent gameplay or command transitions.

## Asynchronous commands

Long-running commands return a move-only handle with stable request identity,
progress observations, timeout, cancellation, and terminal result. Completion of
file loading, editor mutation, asset compilation, world travel, or automation is
verified through the owning native subsystem.

World teardown, module shutdown, caller revocation, or editor PIE termination
cancels all affected handles. A callback arriving after cancellation cannot
publish success or mutate a replacement world.

## Runtime text scripts

Shipping and diagnostic runtime builds do not evaluate arbitrary command-script
files. Repository-authored command batches used by tests or editor automation
are converted before execution into validated typed plans containing command
identities and typed arguments.

A batch plan records:

- plan identity and revision;
- ordered or explicitly parallel steps;
- caller and permission profile;
- allowed worlds;
- timeout and cancellation policy;
- expected results; and
- provenance for the repository-authored source.

Unknown commands, parse errors, invalid aliases, or stale revisions fail the
plan
before the first side effect unless the plan explicitly declares an isolated
best-effort diagnostic step.

## Logging

Runtime and editor diagnostics use native log categories and structured fields.
Each event declares category, severity, message identity, request identity,
world, local player when applicable, and redacted typed fields.

Supported sinks are selected by build policy:

- editor output log;
- automation result stream;
- development console projection;
- platform diagnostic output; and
- explicitly configured repository-safe log file.

The command surface cannot choose an arbitrary output path. Logs must not
include
credentials, private workstation paths, proprietary source paths, save payloads,
or unrestricted object serialization.

Category filtering changes presentation and storage only. It cannot suppress a
required command result, assertion, validation failure, or domain transaction
outcome.

## Interactive development console

The interactive console is available only when the build and caller policy allow
it. Its input layer uses semantic actions for:

- open and close;
- submit;
- cancel;
- history previous and next;
- cursor movement;
- completion;
- selection; and
- output scrolling.

Text input uses the platform's native text-entry path. Gamepad or touch
presentation may expose a virtual keyboard, but both produce the same typed
request.

Opening the console acquires an input-mode lease and optional pause request.
Closing, world travel, focus loss, controller removal, or UI destruction
releases
all leases. The console cannot leave gameplay input disabled.

## History and completion

History stores bounded command identities and redacted typed arguments.
Sensitive
or non-repeatable arguments are omitted. History is device-local development
state and never enters player saves.

Completion is generated from the caller's currently available definitions and
aliases. It does not reveal excluded, higher-privilege, or unavailable commands.
Completion ordering is deterministic by canonical identity.

## Diagnostic channels

Named diagnostic channels are stable category identities, not integer positions.
A caller may adjust a permitted channel's presentation threshold for the current
development session. Blocking a channel cannot disable safety, validation,
transaction, or crash diagnostics.

## Diagnostic overlay

Development visualization uses a world-aware diagnostic-draw service rather than
one process-wide stack of mutable sections. Each draw request declares:

- stable channel and section identity;
- world and optional local-player identity;
- primitive kind;
- finite positions, dimensions, and color values;
- world-space or screen-space projection;
- lifetime or one-frame policy;
- depth and visibility policy; and
- caller availability and permission.

Supported primitive families include lines, arrows, circles, boxes, points,
world text, screen text, and bounded graphs. Native engine debug-draw helpers
may
implement presentation, but they cannot become gameplay state.

Sections are selected by stable identity. Push and pop order, fixed section
capacity, pointer ownership, or toggling through an allocation array cannot
select
which diagnostic data exists. One-frame sections clear automatically after the
owning world frame; persistent sections require an explicit handle and teardown.

Overlay requests are bounded by per-channel count, text length, lifetime, and
memory budgets. Overflow drops lower-priority diagnostic presentation and
records
one structured finding. It never corrupts gameplay memory or blocks simulation.

## Audio diagnostics

Audio diagnostics consume immutable snapshots from
<!-- markdownlint-disable-next-line MD013 -->
[Gameplay audio source, residency, mix, and environment runtime](gameplay-audio-source-residency-mix-and-environment-runtime.md)
and
<!-- markdownlint-disable-next-line MD013 -->
[Native audio device, resource, player, and tuning adapter runtime](native-audio-device-resource-player-and-tuning-adapter-runtime.md).
Registered views may show:

- output-device and audio-engine revision;
- active, queued, virtualized, paused, stopping, and failed sources;
- semantic source, role, owner, world, local-player, and feature identities;
- listener and nearby-source positions;
- Sound Class, Sound Mix, submix, bus, send, modulation, and effect state;
- residency scopes, bundles, retained handles, memory, and stream-cache demand;
- concurrency, significance, voice, decoder, underrun, and starvation results;
- Audio Volume, room, environment, reverb, and fade state; and
- stale callback, fallback, cancellation, and teardown findings.

Page identity and selection are stable definitions, not a fixed five-page array.
Visible line count, screen position, text color, nearby-object radius, and dump
format are development presentation settings.

Audio diagnostics cannot force a source, alter a player, retain an asset, change
a mix, move a listener, select an environment, clear a queue, or mutate
gameplay.
Shipping builds exclude or permanently disable mutable audio-debug controls.

## State-prop, character, vehicle, and supersprint diagnostics

Registered read-only views may consume immutable snapshots from the authored
state-prop, playable-character, native-vehicle, and local supersprint contracts.
They may show:

- state-prop definitions, instances, active state, transitions, markers,
  callbacks, listeners, component projection, persistence, and streaming;
- avatar, local-player, controller, character, input-context, movement,
  collision, support, vehicle-handoff, camera-target, prop, interaction, and
  footprint state;
- vehicle definition, instance, native-physics readiness, asynchronous step,
  transform, velocity, acceleration, engine, gear, and control state;
- per-wheel contact, load, suspension, slip, steering, rotation, brake, drive,
  and physical-surface observations;
- vehicle artificial-intelligence intent, road and traffic projection, parking,
  pursuit, collisions, damage, destruction, husks, resets, recovery, input,
  haptics, presentation, capacity, streaming, and teardown;
- supersprint session, participants, controller assignments, selections,
  loading,
  vehicles, artificial intelligence, route progress, checkpoints, laps, clocks,
  positions, finish windows, results, high scores, and cleanup; and
- coin and sparkle presentation capacity, ownership, and stale-callback
  findings.

These views cannot force state transitions, inject shipping input, possess or
teleport characters, commit vehicle handoffs, spawn, possess, move, accelerate,
brake, damage, repair, reset, destroy, park, pursue, retrieve, or grant a
vehicle,
advance checkpoints, consume turbo, finish races, write high scores, grant
currency, create persistence, or retain presentation resources.

## Screenshot and frame capture

Screenshot and frame capture follow the
<!-- markdownlint-disable-next-line MD013 -->
[native platform bootstrap and error-recovery runtime](native-platform-bootstrap-and-error-recovery-runtime.md).
Development commands may request a bounded capture by stable world, viewport,
player, camera, frame, quality, locale, and presentation identity.

Capture uses native screenshot or render-target facilities, an approved output
destination, typed cancellation, redaction, and a closed result. Raw
frame-buffer
pointers, custom platform transfer protocols, and gameplay-owned pixel
conversion
are prohibited.

Golden captures require deterministic readiness barriers. Image similarity is
presentation evidence only and cannot replace gameplay, input, state, or timing
verification.

## Runtime profiling
<!-- markdownlint-disable-next-line MD013 -->

Performance profiling uses native Unreal tracing, CSV profiling, Insights,
platform counters, and repository-owned measurement labels. Memory ownership,
budgets, pressure, traces, leak verification, pools, and instance accounting
follow the
<!-- markdownlint-disable-next-line MD013 -->
[memory ownership, budget, and diagnostics runtime](memory-ownership-budget-and-diagnostics-runtime.md).
A profile scope contains:

<!-- markdownlint-disable-next-line MD013 -->
- stable sample identity;
- parent scope identity;
- thread and task context;
- world and frame identity when applicable;
<!-- markdownlint-disable-next-line MD013 -->
- monotonic start and end observations;
<!-- markdownlint-disable-next-line MD013 -->
- count and optional byte metrics;
- build and platform capability metadata; and
- capture-session identity.

Begin and end calls must be balanced in the same declared execution context.
Missing end, recursive mismatch, cross-thread closure, non-monotonic time, or
sample overflow records a profiling error rather than reassigning another
sample.

Frame aggregation may calculate inclusive time, exclusive time, call count,
minimum, maximum, average, percentile, and bounded history. Presentation paging
is a view concern and cannot cap the number of instrumented identities in the
trace format.

Hardware performance counters are optional platform-adapter observations. A
platform-specific counter implementation cannot change gameplay behavior,
timing,
or quality policy. Unsupported counters return unavailable evidence and never
fall back to guessed values.

Profiling is disabled or compiled out according to package policy.
Instrumentation
must not allocate unbounded memory, retain destroyed worlds, expose private
paths,
or write arbitrary files. Capture export uses an approved diagnostic destination
and redaction policy.

## Assertions and failures

Assertions remain native engine or repository validation mechanisms. A command
cannot convert a failed invariant into success. Recoverable handler failures
return structured results; unrecoverable development invariants use the
repository's native assertion policy.

Formatting uses bounded native strings. Invalid format data, overlong output,
non-finite values, or encoding failure produces a safe diagnostic rather than a
buffer overrun or truncated executable request.
<!-- markdownlint-disable-next-line MD013 -->

## Mods and extensions

A validated development feature or mod overlay may add commands only when its
manifest declares:

- a namespaced identity;
- availability and permission no broader than the host profile;
- typed schema and native handler;
- dependencies and conflicts;
- teardown behavior; and
- tests.

An extension cannot replace a first-party command unless an accepted override
policy explicitly permits that identity and revision.

## Determinism

Given the same catalog revision, caller profile, world snapshot, typed request,
and application-service observations, lookup and dispatch select the same
handler
and result semantics.

Wall-clock time, registration order, pointer address, hash-table iteration,
localized display text, platform key label, and log-listener order cannot select
behavior.

## Validation

Catalog validation rejects:

- duplicate or ambiguous command and alias identities;
- missing handlers;
- schema mismatch;
- unbounded strings or numeric values;
- forbidden availability or permission combinations;
- runtime text evaluation;
- arbitrary file or process access;
- alias cycles or privilege escalation;
- unresolved target selectors;
- missing timeout or cleanup policy; and
- development commands included in an unauthorized shipping package.

Cook validation proves excluded modules, UI, command assets, and symbols are not
reachable from ordinary player packages except where an explicitly accepted
diagnostic profile requires them.

## Tests

Required tests include:

- deterministic normalized lookup and completion;
- duplicate identity and alias rejection;
- typed argument parsing and bounds;
- alias placeholder binding and cycle rejection;
- permission and availability enforcement;
- local-player and world isolation;
- synchronous success and failure;
- asynchronous completion, timeout, cancellation, and late callback rejection;
- world teardown and PIE restart cleanup;
- structured log redaction and category filtering;
- diagnostic section identity, lifetime, bounds, and world teardown;
- overlay overflow and invalid primitive rejection;
- balanced profile scopes and cross-thread mismatch rejection;
- frame aggregation, bounded history, and unsupported hardware counters;
- input lease restoration;
- history redaction;
- unauthorized shipping exclusion; and
- extension registration and teardown.

## Invariants

- Command text is never gameplay identity.
- Arbitrary runtime script evaluation is forbidden.
- Every mutation crosses a typed application port.
- Every long-running command has timeout, cancellation, and terminal evidence.
- Development availability never follows from a platform macro alone.
- Registration and listener order never select behavior.
- Diagnostic output cannot expose private or secret material.
- Player saves never contain development console state.
