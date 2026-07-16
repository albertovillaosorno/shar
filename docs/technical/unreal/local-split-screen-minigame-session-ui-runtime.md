# Local split-screen minigame session UI runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Mod-owned multiplayer adapters and community servers](../../adr/modding/mod-owned-multiplayer-adapters-and-community-servers.md)
<!-- markdownlint-disable-next-line MD013 -->
- [HUD, radar, camera, and navigation parity](../../adr/unreal/ui/hud-radar-camera-and-navigation.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
- [UI parity boundary](../../adr/unreal/ui/ui-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI navigation, menu, and modal runtime](common-ui-navigation-menu-and-modal-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [In-game HUD, pause, and transition runtime](in-game-hud-pause-and-transition-runtime.md)
- [Race route and opponent runtime](race-route-and-opponent-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Semantic input, device, and haptics runtime](semantic-input-device-and-haptics-runtime.md)
- [Vehicle access and roster runtime](vehicle-access-and-roster-runtime.md)

## Purpose

This specification defines the native Unreal user-interface and application flow
for the built-in local split-screen bonus minigame. It covers track and race
options, local-player joining, controller assignment, character and vehicle
selection, readiness, countdown, loading, per-player HUD, pause, controller-loss
recovery, race summary, replay, and return to the frontend.

This local mode is part of the base product and remains separate from mod-owned
network multiplayer. It does not add an official cooperative campaign, online
matchmaking, hosted servers, accounts, network replication, or shared campaign
progression.

It replaces process-global minigame managers, fixed player arrays as authority,
platform-specific controller branches, filename-derived character loading,
widget-owned readiness, countdowns that directly change application mode, and
result screens that compute durable progression.

## Product boundary

The base campaign remains single-player. The local minigame is a transient
same-device competitive session with independent local players and split-screen
viewports.

A future networked minigame mod may reuse stable definitions and adapter ports,
but it must declare its own authority, protocol, package set, persistence, and
teardown policy. The built-in local session does not imply that its application
services are replicated or server-authoritative.

## Native Unreal composition

The runtime uses:

- `USharLocalMinigameSubsystem`, a `UGameInstanceSubsystem`, for shared session,
  lobby, loading, race, pause, summary, and return flow;
- `USharLocalMinigamePlayerSubsystem`, a `ULocalPlayerSubsystem`, for each
  joined
  participant's controller assignment, selection, readiness, focus, and HUD;
- Common Activatable Widgets for the shared lobby, pause, prompt, and summary;
- per-player Common UI action contexts for join, back, confirm, character,
  vehicle, and ready commands;
- Enhanced Input only for gameplay vehicle commands after session commit;
- C++ UMG viewmodels derived from `UMVVMViewModelBase` for immutable lobby,
  player, options, loading, HUD, and summary projections;
- Asset Manager primary assets and bundles for tracks, characters, vehicles,
  icons, cameras, audio, and UI presentation; and
- retained streamable handles correlated to the accepted lobby or race request.

Widgets publish typed commands. They do not create local players, assign
devices,
start application loading, spawn vehicles, award points, rank players, or commit
progression directly.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Authority |
| :--- | :--- |
| `USharLocalMinigameSubsystem` | Shared session identity, lobby state, race request, loading, pause, summary, and return intent. |
| `USharLocalMinigamePlayerSubsystem` | One participant's join state, controller assignment, selections, readiness, and local HUD. |
| Local-player manager | Creation and removal of `ULocalPlayer` instances and viewport assignment. |
| Input service | Device claims, semantic UI actions, gameplay mapping leases, disconnect, and reconnect. |
| Race application service | Route, lap, direction, grid, countdown readiness, race state, finish, and results. |
| Catalog and progression services | Track, character, and vehicle visibility and accepted unlock state. |
| Vehicle service | Vehicle eligibility, tuning identity, presentation, and race spawn request. |
| Application lifecycle service | `super_sprint_front_end`, loading, active race, summary, and frontend transitions. |
| Common UI kernel | Shared and per-player activation, focus, actions, prompts, and restoration. |

<!-- markdownlint-enable MD013 -->

No lobby widget, character portrait, vehicle carousel, timer, pause screen, or
summary row may become race or progression authority.

## Runtime identities

The runtime uses:

- `FSharLocalMinigameSessionId` for one complete local session;
- `FSharLocalMinigameLobbyRevision` for the accepted lobby projection;
- `FSharLocalParticipantId` for one joined player;
- `FSharControllerAssignmentId` for one claimed input device;
- `FSharTrackDefinitionId` for canonical track identity;
- `FSharRaceOptionsRevision` for laps and direction;
- `FSharParticipantSelectionRevision` for character, vehicle, and readiness;
- `FSharLocalRaceRequestId` for one loading and race transaction;
- `FSharLocalRaceResultId` for one accepted terminal result; and
- exact catalog, progression, input, local-player, feature, and application
  revisions.

Every asynchronous asset, input, loading, viewport, and result callback must
match the accepted session, participant, request, and revisions.

## Session definition

`FSharLocalMinigameDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ModeId` | Stable local-minigame mode identity. |
| `MinimumPlayers` | Validated minimum joined participants. |
| `MaximumPlayers` | Validated local-player and platform limit. |
| `TrackSetId` | Ordered eligible track definitions. |
| `CharacterSetId` | Eligible character-presentation definitions. |
| `VehicleSetPolicyId` | Ownership, unlock, fallback, and duplicate-selection policy. |
| `LapPolicyId` | Minimum, maximum, default, and allowed steps. |
| `DirectionPolicyId` | Normal, reverse, or other registered route variants. |
| `ReadinessPolicyId` | Required selections, countdown, cancellation, and timeout behavior. |
| `ViewportPolicyId` | Split-screen layout and per-player safe areas. |
| `HudProfileId` | Per-player and shared HUD definitions. |
| `PausePolicyId` | Pause owner, vote, input, and resume behavior. |
| `ResultPolicyId` | Ranking, statistics, replay, and return behavior. |
| `ProgressionPolicyId` | Explicit transient or declared non-campaign effects. |
| `RequiredBundles` | Lobby, player, track, vehicle, race, HUD, audio, and result bundles. |
| `FeatureOwnerId` | Base game or validated feature package. |

<!-- markdownlint-enable MD013 -->

The base definition may expose seven canonical tracks and a bounded local-player
count, but neither value is a widget compile-time constant. Definitions and
platform policies establish the valid set.

## Session states

One session is in exactly one state:

- `created`;
- `loading_lobby`;
- `track_selection`;
- `participant_join`;
- `participant_selection`;
- `waiting_for_readiness`;
- `counting_down`;
- `loading_race`;
- `active_race`;
- `paused`;
- `showing_summary`;
- `returning_to_lobby`;
- `returning_to_frontend`;
- `cancelled`;
- `failed`; or
- `terminated`.

Transitions are application transactions. A screen animation or countdown
completion can satisfy one presentation node but cannot set the application
mode.

## Lobby activation

Opening the local minigame performs:

1. create a session identity;
1. validate the base or feature-owned mode definition;
1. query track, character, vehicle, progression, and input revisions;
1. load the required lobby and selection bundles;
1. create the shared lobby viewmodel;
1. activate the shared Common UI lobby layer;
1. establish the primary user's navigation and quit authority; and
1. enter track selection with no gameplay mappings active.

Failure before activation preserves the frontend and releases the session's
partial assets and device claims.

## Track selection

The track projection contains stable identity, localized name, thumbnail,
unlock state, route variant support, required bundles, and typed unavailable
reason.

Ordering comes from the definition, not from widget names or numeric filenames.
Locked tracks may be shown according to product policy but cannot be committed.
At least one selectable track is required before the lobby becomes interactive.

Selecting a track snapshots its exact definition and catalog revision. A later
unlock or feature change rebuilds the projection through a replacement lobby
revision rather than mutating a selected row in place.

## Lap and direction options

Lap count is a bounded integer declared by `FSharLapPolicy`. The UI exposes
semantic increase and decrease commands and disables movement at the accepted
minimum or maximum.

Direction is a typed route variant, such as normal or reverse. It is accepted
only when the selected track declares compatible route, checkpoint, grid, and
camera data.

The widget never derives race options from arrow opacity, text index, or current
animation state.

## Participant joining

A join request contains device identity, requesting platform user, session,
expected lobby revision, and requested participant slot.

The input service validates that:

- the device is connected and not already claimed;
- the platform user is permitted to join;
- capacity remains available;
- the session accepts joins in its current state;
- one local-player instance can be created; and
- the viewport policy supports the resulting participant count.

Commit creates or accepts one `ULocalPlayer`, assigns one stable participant
identity, establishes per-player focus and Common UI actions, and publishes a
new
lobby revision.

Joining cannot duplicate a device, reuse a stale local player, or overwrite
another participant's selection.

## Leaving and slot release

A participant may leave before race commit according to policy. Leaving:

1. cancels that participant's pending selection and asset requests;
1. releases readiness and UI focus;
1. releases the controller assignment;
1. removes or deactivates the corresponding local player;
1. rebuilds viewport layout and shared lobby projection; and
1. publishes one new lobby revision.

If the primary authority leaves, the session either transfers authority through
an explicit deterministic rule or opens a confirmed session-cancel prompt.

## Controller assignment

Each participant has exactly one accepted device assignment during lobby and
race. Controller identity is separate from participant identity, allowing a
reconnect or approved replacement without rewriting results or selections.

Common UI routes lobby actions to the owning participant. One player's input
cannot move another player's character or vehicle carousel. Shared track and
options commands require the declared lobby owner or shared policy.

## Controller disconnect and reconnect

A disconnect produces a typed observation containing device, participant,
session, and input revision. The response depends on session state:

- before race commit, the participant becomes disconnected and not ready;
- during loading, commit pauses or aborts according to readiness policy;
- during the race, gameplay pauses or applies the declared safe control policy;
- during summary, the row remains valid while actions wait for an eligible
  controller.

The reconnect prompt identifies the affected participant without exposing raw
controller indexes. A reconnect validates the platform user and accepted
assignment before resuming input.

A late reconnect for a removed participant or terminated session is ignored.

## Character selection

The character projection contains canonical presentation identity, localized
name, portrait or preview assets, availability, duplicate-selection policy, and
feature ownership.

The base selection set may include the five established local-race characters,
but membership is catalog data. Character model filenames, load order, and fixed
array indexes do not define identity.

Character selection affects local presentation and race-intro binding only. It
does not change campaign character state, costumes, progression, or save data.

## Vehicle selection

Each participant receives an immutable vehicle projection derived from:

- mode and track compatibility;
- accepted ownership and unlock state;
- completion or feature policy;
- platform and split-screen budgets;
- duplicate-selection policy; and
- catalog and progression revisions.

At least one selectable vehicle is required for every joined participant. The
mode may include a declared fallback vehicle, but it cannot invent ownership or
select an arbitrary traffic vehicle.

Vehicle ratings are read-only catalog projections. Presentation bars or stars do
not change tuning. Rapid carousel movement cancels stale preview loads and
retains only the accepted preview lease.

## Selection state

Each participant moves through:

- `joined`;
- `choosing_character`;
- `character_selected`;
- `choosing_vehicle`;
- `vehicle_selected`;
- `ready`;
- `disconnected`; or
- `left`.

Back reverses only the owning participant's valid selection step. It cannot
silently unselect another participant or return to track selection after race
commit.

A selection command carries the exact participant and lobby revisions. Stale
commands are rejected with typed feedback.

## Readiness

A participant becomes ready only when all definition-required fields are valid:

- connected controller assignment;
- accepted local player and viewport;
- selected character when required;
- selected eligible vehicle;
- selected track and race options; and
- all participant-specific required presentation bundles ready.

The shared lobby derives `ReadyCount`, `JoinedCount`, and typed waiting reasons.
A widget cannot increment a ready counter directly.

Changing a required selection, losing a controller, losing content, or replacing
an asset request clears readiness through one accepted lobby revision.

## Start countdown

When the mode's minimum participant count is met and every joined participant is
ready, the session may begin a data-driven lobby countdown.

The countdown duration is product policy, not a platform compile-time branch. It
shows the remaining time and a localized waiting message through the shared HUD
feedback scheduler.

Joining, leaving, disconnecting, changing a required option, losing asset
readiness, or cancelling the session stops the countdown and publishes one new
lobby revision.

Countdown expiry publishes a start request. It cannot directly leave the lobby
or start race time.

## Race request

`FSharLocalRaceRequest` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `RaceRequestId` | Unique idempotency identity. |
| `SessionId` | Owning local session. |
| `TrackId` | Accepted track and route identity. |
| `RaceOptionsRevision` | Laps, direction, and other mode options. |
| `Participants` | Ordered stable participant snapshots. |
| `ControllerAssignments` | Exact accepted device claims. |
| `ViewportPolicyId` | Split-screen composition. |
| `RequiredBundles` | World, vehicles, characters, cameras, HUD, audio, and results. |
| `CatalogRevision` | Exact content projection. |
| `InputRevision` | Exact device and mapping state. |
| `FeatureRevision` | Exact package set. |

<!-- markdownlint-enable MD013 -->

Participant ordering is deterministic by accepted local-player slot and stable
participant identity, never by callback timing.

## Loading transaction

The application lifecycle service validates the race request, enters
`loading_super_sprint`, and waits for:

- track world and route readiness;
- local-player and viewport creation;
- character and vehicle presentation;
- vehicle spawn and starting-grid reservation;
- camera, HUD, audio, and input mapping readiness;
- race service initialization; and
- feature-package revision stability.

Only after every required node verifies does the transaction commit active race
mode. Failure returns to the lobby with selections preserved when safe.

A late load from a replaced request cannot start the race or release the
accepted
request's assets.

## Race start

The race service owns the gameplay countdown and starting grid. The UI presents
the countdown and receives one release observation, while the race service
controls timing and vehicle command activation.

Race time begins only after active mode commit, participant readiness, and the
race service's accepted start transition. Lobby countdown completion is not race
start authority.

## Per-player HUD

Each local player owns one HUD viewmodel and safe area. The minigame HUD may
project:

- speed and vehicle state;
- race position and lap count;
- route, checkpoints, and finish markers;
- local reset availability;
- shared countdown and terminal state; and
- controller or participant warnings.

Shared cues use a dedicated shared layer. One participant's HUD never reads or
mutates another participant's vehicle or input state.

## Pause

Pause is a typed session command. The mode definition declares whether one
participant, the primary participant, or a shared vote owns pause authority.

The pause screen provides resume and confirmed quit actions. Resume restores the
accepted race state and each participant's mappings. Quit tears down the race
transaction before returning to the lobby or frontend.

A participant pressing back or start cannot independently change application
mode from a widget.

## Manual reset and auxiliary actions

Reset, camera, view, or other auxiliary HUD actions are semantic gameplay
commands scoped to one participant. Eligibility comes from race and vehicle
state.

The HUD cannot teleport a vehicle, reset a checkpoint, or change a camera by
direct actor access. The owning gameplay service returns the accepted result.

## Race completion

Race completion produces one immutable result for every participant, including:

- finish position or did-not-finish state;
- race time;
- best lap;
- awarded session points;
- cumulative session points;
- cumulative wins; and
- race and participant revisions.

The summary screen does not calculate or modify these values. It consumes the
accepted race result and declared result policy.

## Summary ranking

The base ranking orders participants by:

1. higher cumulative session points;
1. higher cumulative wins;
1. stable participant identity as the deterministic tie breaker.

This preserves the observed points-then-wins meaning while removing insertion
order as hidden authority.

Rank labels, portraits, character presentation, points, wins, total time, and
best lap are immutable summary projections. A non-finisher uses a localized
`did not finish` state rather than a magic numeric time.

Optional category highlights, such as best total time or best lap, are enabled
only when the result definition and complete statistics support them.

## Replay and continuation

The summary may offer:

- replay the same accepted track and options;
- return to the local lobby with participants and eligible selections retained;
- choose another track;
- leave the local mode for the frontend; or
- confirm complete session termination.

Replay creates a new race request identity. It never reuses terminal loading,
countdown, vehicle, or result handles.

## Progression boundary

The base local minigame is transient and cannot silently mutate campaign mission
progression, current campaign character, active campaign vehicle, portable save,
or campaign achievements.

Any declared base unlock or reward effect must be an explicit application
transaction with its own progression and save policy. A feature-owned local mode
must declare the same boundary in its manifest.

## Network-mod boundary

Local participants, controller assignments, and split-screen viewports are not
network player or server identities. The built-in session remains locally
authoritative.

A multiplayer mod may adapt serializable mode, participant, command, and result
schemas, but it must provide separate network session, authority, protocol,
package, trust, persistence, and teardown behavior. It cannot expose ordinary
campaign saves as server authority.

## Asset readiness and leases

Lobby, character, vehicle, track, world, HUD, audio, and summary assets use soft
references in registered primary assets.

The lobby retains shared and participant preview handles until selection changes
or the race request takes ownership. Loading retains race bundles until race
teardown. Summary presentation retains only the assets required by the accepted
result.

Selection changes cancel stale preview requests. Feature removal cancels only
feature-owned assets and rebuilds the lobby or terminates safely if no valid
base selection remains.

## Localization and accessibility

All text uses localized identities and typed arguments. The lobby supports text
expansion, bidirectional layout where enabled, locale-aware numbers and times,
and per-player safe areas.

Each flow declares:

- focus and narration ownership;
- join and ready announcements;
- color-independent selection and ranking state;
- reduced-motion lobby and summary presentation;
- minimum reading and confirmation durations;
- touch or keyboard accessibility where the target platform permits local play;
- controller-disconnect messaging; and
- split-screen readability at every supported player count.

A participant's readiness and rank are never communicated by color or animation
alone.

## Feature and mod overlays

A validated feature package may add namespaced local tracks, characters,
vehicles, modes, options, result fields, and presentation profiles.

Definitions must remain within platform local-player, viewport, memory, and
performance budgets. Feature removal during the lobby cancels owned selections
and rebuilds the projection. Removal during loading or race follows the feature
teardown contract and cannot leave stale local players or device claims.

## Concurrency

Join, leave, selection, readiness, disconnect, and feature mutations are
serialized through the session subsystem. Per-participant commands carry exact
lobby revisions.

Asset loading, device callbacks, application transitions, and race completion
may
finish asynchronously, but only the owning request may accept them. A replaced
request's result cannot start a race, open a summary, or release another
request's assets.

## Diagnostics

The runtime records bounded structured diagnostics for:

- session, lobby, participant, controller, and race-request identities;
- accepted catalog, progression, input, feature, and application revisions;
- joined and ready participant projections;
- track, option, character, and vehicle selection results;
- controller disconnect, reconnect, and reassignment;
- viewport and local-player composition;
- countdown and loading-node state;
- race, pause, summary, replay, and return transitions;
- asset-lease ownership; and
- stale or rejected callbacks.

Diagnostics use canonical identities and typed reasons, never raw local asset
paths or machine-specific locations.

## Failure behavior

- No selectable track blocks lobby activation.
- No eligible vehicle for a joined participant blocks readiness.
- Duplicate or stale device claims reject the join or reassignment.
- Unsupported participant count rejects the newest join without disturbing
  accepted participants.
- Invalid track, lap, direction, character, or vehicle selection is rejected.
- Controller loss clears readiness or pauses according to mode policy.
- Countdown cancellation returns to waiting without starting loading.
- Loading failure restores the lobby when safe and preserves accepted
  selections.
- Invalid result data blocks summary publication and records a race defect.
- Feature removal cannot leave an unresolved selection, viewport, or asset
  lease.
- Stale callbacks cannot start, pause, summarize, replay, or terminate a
  replacement session.

## Validation

Validation proves:

- unique mode, track, character, vehicle, and presentation identities;
- valid minimum and maximum participant counts;
- complete viewport and safe-area layouts for every supported count;
- valid lap ranges and route-direction combinations;
- every joined participant can obtain an eligible vehicle under base policy;
- required lobby, race, HUD, and summary bundles exist;
- controller-disconnect and primary-authority policies are total;
- countdown and loading transitions have terminal cancellation behavior;
- ranking and did-not-finish rules are deterministic; and
- feature overlays remain within local-player and package budgets.

## Tests

Automated tests cover:

- track projection, locking, ordering, and option bounds;
- participant join, duplicate-device rejection, leave, and authority transfer;
- per-player focus and command isolation;
- character and vehicle selection, backtracking, and stale-command rejection;
- readiness derivation and countdown cancellation;
- controller disconnect and reconnect in lobby, loading, race, and summary;
- successful and failed race loading transactions;
- split-screen viewport and HUD composition;
- pause, resume, manual reset, and confirmed quit;
- points-then-wins ranking with stable tie breaking;
- did-not-finish, total-time, and best-lap formatting;
- replay and return cleanup;
- transient campaign-progression isolation; and
- feature registration and teardown.

## Invariants

- The base campaign remains single-player.
- The built-in minigame is local split-screen, not network multiplayer.
- Every joined participant has one stable local identity and at most one device.
- Readiness is derived from accepted selections and assets.
- A lobby countdown cannot directly start race time.
- Race loading commits atomically or returns to a valid lobby.
- Each participant owns isolated focus, input, HUD, and selection state.
- Results are immutable and ranking is deterministic.
- Campaign saves and progression are not implicit local-minigame authority.
- Asset, input, viewport, and player leases are released exactly once.
- Stale callbacks cannot mutate a replacement session.
