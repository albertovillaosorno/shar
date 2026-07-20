# Frontend shell and menu runtime

- Status: Active
- Last reviewed: 2026-07-20

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)
- [UI parity boundary](../../adr/unreal/ui/ui-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI navigation, menu, and modal runtime](common-ui-navigation-menu-and-modal-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend media, gallery, and audio runtime](frontend-media-gallery-and-audio-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Device configuration and save-slot runtime](device-configuration-and-save-slot-runtime.md)

## Purpose

This specification defines boot, main-menu, save-slot, resume, new-game,
load-game, scrapbook, Unused Content, options, credits, calendar-theme,
idle-presentation, input-routing, failure, and verification behavior for the
native front end.

## Module and service ownership

The front end is implemented by the runtime user-interface module. It depends on
read-only application ports for campaign, progression, save, options, credits,
calendar, and content loading.

The owning services are:

<!-- markdownlint-disable MD013 -->

| Service | Lifetime | Authority |
| :--- | :--- | :--- |
| `USharFrontendCatalogSubsystem` | Game instance | Immutable screen definitions, root-catalog activation, and cross-catalog identity validation. |
| `USharFrontendFlowSubsystem` | Game instance | Revisioned frontend observations, deterministic navigation, history, modal state, readiness evidence, rollback, and terminal results. |
| `USharFrontendInputSubsystem` | Local player | Semantic menu actions, active input method, focus restoration, and glyph context. |
| `USharFrontendViewModelSubsystem` | Game instance | Immutable view-model construction from accepted domain snapshots. |

<!-- markdownlint-enable MD013 -->

The frontend subsystem never serializes saves, calculates progression, applies
options directly to engine objects, grants rewards, or opens a world without an
accepted campaign transition.

The current native slice is intentionally renderer-independent. Common UI
activation, view-model construction, bundle loading, save and settings
operations, and application transitions are external services that return
correlated evidence; they do not mutate the accepted flow observation directly.

## Common UI layer stack

The viewport owns four ordered Common UI layers:

1. `boot` for startup, profile discovery, and blocking initialization;
1. `primary` for main menu, load, scrapbook, options, and credits;
1. `modal` for confirmations, typed failures, and destructive-choice review; and
1. `notification` for non-blocking status and input feedback.

Each layer is a fixed project-owned Common UI stack. A screen may push only to
its declared layer. A modal blocks semantic actions below it. A notification
never steals focus or becomes a command authority.

Every primary or modal screen derives from
`USharCommonActivatableWidget`. The C++ base owns activation, deactivation,
semantic action bindings, focus restoration, and immutable view-model receipt.
Widget Blueprints own layout, animation, sound, style, and accessibility
presentation.

## Frontend state machine

The frontend state is one of:

- `booting`;
- `main_menu`;
- `slot_browser`;
- `gallery`;
- `scrapbook`;
- `unused_content`;
- `achievements`;
- `local_bonus_setup`;
- `options`;
- `settings_edit`;
- `credits`;
- `transitioning_to_gameplay`;
- `returning_from_gameplay`; or
- `blocked`.

`booting` remains active until required catalogs, local configuration, storage
readiness, and slot summaries have typed results. A recoverable slot failure
does
not block other valid slots or the new-game command. Missing required base
content enters `blocked` with no false resume option.

A transition request disables duplicate commands until it succeeds or fails. A
failed request returns to the prior stable state and restores the prior focused
semantic action.

## Boot transaction

Boot executes the validated task graph defined by the
<!-- markdownlint-disable-next-line MD013 -->
[frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md).
The required base tasks are:

1. initialize the root gameplay and user-interface catalogs;
1. resolve platform, legal, locale, and accessibility requirements;
1. load device-local configuration through the platform adapter;
1. query storage readiness and logical save-slot summaries;
1. validate each summary against save schema and catalog revision rules;
1. determine the most recent resumable accepted slot;
1. select the standard or calendar front-end presentation theme;
1. load only the required front-end presentation and audio bundles;
1. build the main-menu view model;
1. activate the primary layer; and
1. release the boot layer after focus and input routing are verified.

Boot cannot migrate or replace a save merely to display its summary. A slot that
requires migration is labeled as requiring load-time migration and remains
unchanged until the user selects it.

## Logical save-slot summary

`FSharSaveSlotSummary` contains:

| Field | Contract |
| :--- | :--- |
| `SlotId` | Stable logical slot identity, never a filename. |
| `RevisionToken` | Accepted save revision identity. |
| `SchemaVersion` | Recognized logical save schema. |
| `CatalogRevision` | Catalog revision required by the save. |
| `LastAcceptedTime` | Platform-normalized display timestamp. |
| `CampaignId` | Canonical campaign identity. |
| `CurrentChapterId` | Last accepted narrative chapter boundary. |
| `GameplayState` | Accepted `mission` or `non_mission` state. |
| `ResumeMissionId` | Optional valid checkpoint mission. |
| `ResumeCheckpointId` | Optional accepted mission checkpoint. |
| `ChapterProgress` | Exact progress projection for the current chapter. |
| `GameProgress` | Read-only exact overall progress projection. |
| `WorldClock` | Accepted world-clock observation and revision. |
| `MapDiscoveryRevision` | Accepted discovered-map state revision. |
| `PlayableCharacterId` | Character shown in the summary. |
| `MissingContentIds` | Required content unavailable to the current catalog. |
| `MigrationState` | None, supported, unsupported, or failed. |
| `IntegrityState` | Valid, recoverable prior revision, or invalid. |
| `CommandState` | Loadable, resumable, inspectable only, or unavailable. |

Display text, screenshots, native paths, storage-container names, and graphics
settings are not save-slot identity.

## Main-menu commands

The primary menu exposes these semantic commands:

<!-- markdownlint-disable MD013 -->

| Command | Availability |
| :--- | :--- |
| `new_game` | Base campaign and an empty or replaceable target slot are available. |
| `resume_game` | One accepted slot has a valid resume or campaign destination. |
| `load_game` | At least one logical slot summary is inspectable. |
| `scrapbook` | One accepted progression snapshot is selectable. |
| `unused_content` | The accepted Unused Content catalog and required generic fallback bundles resolve. |
| `achievements` | Shows `Achievements pending` until the required achievement service and catalog are implemented. |
| `options` | Device-local configuration service is available. |
| `credits` | Credits definition and presentation bundle resolve. |
| `quit` | Platform policy exposes an application-exit command. |

<!-- markdownlint-enable MD013 -->

A command is visible according to product policy and enabled according to typed
availability. Disabled commands expose a localized reason. A hidden command is
never used to conceal a failed dependency that should be diagnosed.

The base main menu does not expose network login, cloud synchronization,
matchmaking, multiplayer campaign, a marketplace, a hosted mod browser, or a
general-purpose launcher. A validated server mod may contribute its own
namespaced connection surface through a declared UI extension point governed by
the
<!-- markdownlint-disable-next-line MD013 -->
[multiplayer adapter and community-server extension](../modding/multiplayer-adapter-and-community-server-extension.md).

## Unused Content command

The `unused_content` command opens a shipping Common UI surface backed by the
canonical Unused Content catalog. It groups accepted characters, vehicles,
missions, world objects, costumes, animation, audio, effects, and presentation
records by stable semantic identity and content class.

Each entry exposes:

- verified identity and content class;
- available gallery, sandbox, selection, preview, or world-placement actions;
- campaign-integration and progression-isolation state;
- whether presentation uses an original generic fallback;
- active replacement-mod package and revision when one is applied; and
- a localized unavailable reason for any unsupported action.

Generic fallbacks are ordinary cooked base-game assets. The screen never prompts
for proprietary downloads or implies that a fallback reproduces unavailable
protected material. A validated replacement overlay updates the same view model
without changing menu routing, content identity, or save meaning.

Launching an Unused Content sandbox or preview creates a transient,
non-progressing session. Closing it returns to the prior frontend state and
releases all content, input, camera, audio, and mod-overlay leases.

## Achievements command

Achievements are required product scope but remain pending implementation. Until
the achievement catalog and service are available, the command opens a bounded
screen that displays `Achievements pending` and no fabricated progress.

The final screen projects base and mod-owned achievements, current progress,
no-missable reachability, platform mapping, mod compatibility, and replay
routes. Every base achievement remains obtainable through free roam, mission
replay, side-activity replay, or post-game play.

## New-game command

New game requires a target logical slot. When the target contains accepted
progress, the frontend presents a confirmation modal with the exact slot and
accepted revision that would be replaced.

After confirmation:

1. request a fresh base-campaign domain snapshot;
1. validate Chapter 1, Homer, terrain family 1, the initial clock, map fog,
   collectible activation, renewable-source eligibility, and required content;
1. commit the new accepted save revision through the save service;
1. request the connected world in `non_mission` state;
1. select a valid Homer ambient vignette; and
1. keep the main menu active until the world transaction is ready.

Failure before the new revision commits leaves the prior slot unchanged. Failure
after commit but before world activation retains a valid Chapter 1 non-mission
resume state and returns a typed retry option.

## Resume command

Resume selects the most recent accepted resumable slot according to normalized
accepted-save time. The selection rule is deterministic when timestamps are
equal: the lowest canonical slot identity wins.

The command validates the slot again before transition. It resumes an accepted
mission checkpoint when one exists; otherwise it activates the connected world
in the saved chapter boundary's `non_mission` start policy. A stale widget
summary cannot bypass this revalidation.

## Load-game screen

The load screen orders logical slots by canonical slot identity unless the user
selects a supported deterministic sort. Each row shows only validated summary
data and one typed status.

Selecting a loadable row requests migration when required, reads the resulting
accepted snapshot, and begins the campaign transition. Selecting an inspectable
but unavailable row opens a diagnostic modal without altering it.

Deletion is not part of this specification. Replacing a slot occurs only through
the reviewed new-game transaction.

## Scrapbook and progress

The scrapbook view model joins accepted progression identities to catalog
presentation. It has two canonical modes.

`game_stats` presents the aggregate campaign projection:

- seven level summaries in campaign order;
- the eight progress-category counts for each level;
- one-decimal level percentages;
- the overall percentage and movie-reward contribution;
- combined mission, bonus mission, race, vehicle, costume, wasp, gag, and card
  totals;
- unlocked rewards and bonus maps; and
- movie availability, including ordinary cinematics and the all-card reward
  movie.

`open_book` presents level-separated catalog galleries for:

- missions;
- character clothing;
- the six persistent vehicles assigned to each level; and
- the seven collector cards assigned to each level.

Locked vehicle and clothing rows remain visible through the declared transparent
or placeholder presentation. Visibility never grants ownership or changes
progress. Mission and card rows use canonical level and ordinal identities
rather
than screenshot order.

The pause-menu level-progress screen consumes the same level progress view
model.
It cannot use a second formula or count visible widgets. The scrapbook is a
main-menu extra and may be opened whenever one accepted progression snapshot is
available; it is not gated by bonus-game availability.

Missing presentation for an accepted identity produces a placeholder and typed
diagnostic. It does not remove accepted progression or change the percentage.

## Local bonus-mode setup

A local bonus-mode command opens a bounded setup session rather than campaign
multiplayer. The view model exposes the selected bonus-mode rules, supported
local-player count, joined player and device assignments, character or
presentation choices, map and round options, readiness, and disabled reasons.

Join, leave, selection, and readiness are typed local-player transactions. A
controller index never defines participant identity. Starting the session
requires every participant, device, option, and content dependency to validate.
Cancellation releases temporary assignments and preserves campaign progression.

The detailed setup, navigation, and transition contract follows the
<!-- markdownlint-disable-next-line MD013 -->
[frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md).

## Controller loss and recovery

The frontend consumes typed device and assignment observations. Losing the only
valid navigation device opens one correlated reassignment modal, pauses owned
media or transitions when policy requires it, and retains prior focus.

Reconnection succeeds only after the input subsystem accepts a compatible local
player assignment. Duplicate disconnects do not stack prompts, and a newly
enumerated device cannot inherit ownership from a physical index. Recovery,
media pause, modal lifetime, and focus restoration follow the
<!-- markdownlint-disable-next-line MD013 -->
[frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md).

## Options screen

Options are device-local and separated from portable gameplay state. The screen
edits a staged `FSharDeviceConfiguration` containing supported display,
graphics,
audio, input, accessibility, language, safe-area, and platform presentation
fields.

Apply follows this transaction:

1. validate every changed field and cross-field constraint;
1. preview changes that require runtime confirmation;
1. apply through the owning platform or engine adapter;
1. verify resolved state;
1. persist the accepted device-local revision; and
1. rebuild affected view models and input glyphs.

A failed apply restores the prior accepted configuration. Gameplay progression
and save revisions remain byte-equivalent.

## Semantic input actions

The frontend action set contains:

- navigate up, down, left, and right;
- accept;
- back;
- primary action;
- secondary action;
- previous and next tab;
- previous and next page;
- open contextual help; and
- skip or pause presentation when the active screen permits it.

Keyboard and mouse, gamepad, and touch adapters map physical input into this
set.
Device discovery, assignment, rebinding, pointer capture, and haptic feedback
follow the
<!-- markdownlint-disable-next-line MD013 -->
[semantic input, device, and haptics runtime](semantic-input-device-and-haptics-runtime.md).
The active-input method changes glyphs and hints only. It does not change
command
availability or navigation meaning.

Every activatable screen declares an initial focus target and a restoration
target. Focus loss, controller reconnect, viewport resize, safe-area change, and
modal dismissal restore a valid semantic target deterministically.

## Calendar themes

The front end consumes the existing calendar-theme table. Verified fixed-date
rules include Christmas on December 25 and Halloween on October 31. Theme
selection uses the normalized local date once per boot and when the application
resumes after a date boundary.

A theme may replace front-end world presentation, props, materials, lighting,
audio, ambient characters, and idle animations. It cannot alter commands, slot
ordering, campaign state, progression, collision, or save data.

When a selected theme fails to load or validate, the standard theme is activated
before the main menu appears.

## Idle presentation

The main-menu definition may contain an ordered set of idle events. Each event
has a stable presentation identity, eligibility predicate, minimum idle
duration,
cooldown, weight, required bundle, and cancellation policy.

Idle events may animate the living-room scene, show background characters,
trigger ambient dialogue, or play non-interactive gags. Any navigation input
cancels the active idle event at its declared safe point and restores menu
focus.

Idle presentation does not grant gag completion, currency, collectibles,
achievements, or campaign progress.

## Credits

The credits command requests the front-end credits sequence. Final-story credits
remain a separate post-ending sequence owned by progression and campaign flow.
Both use the credits subsystem and return to their declared destination.

Skipping or failing front-end credits returns to the main menu without changing
progression. Skipping post-ending credits cannot undo the already accepted final
mission.

## Failure behavior

The front end fails closed on:

- missing required catalogs or incompatible catalog revisions;
- no valid base-campaign definition;
- invalid or duplicate logical slot identities;
- unsupported save schema or failed integrity validation;
- unavailable required content for a requested transition;
- a widget command not registered in the semantic command catalog;
- a screen with no valid focus target;
- calendar or idle presentation that attempts a gameplay mutation;
- an options apply that cannot verify resolved state; or
- a world transition that reports presentation success before campaign commit.

A typed failure identifies the owning service, operation, slot or content
identity, recoverability, and next permitted command. Raw platform routes,
credentials, memory addresses, and private diagnostics are never displayed.

## Invariants

- One accepted domain snapshot produces one deterministic frontend view model.
- Widgets never discover saves, levels, rewards, or options by filesystem or
  object scan.
- Resume and load revalidate the selected slot before transition.
- Frontend and pause progress use the same exact domain projection.
- Calendar themes and idle events are presentation-only.
- Input adapters preserve one semantic command model.
- A failed transition leaves a valid accepted slot and stable frontend state.
- The primary menu remains usable when one non-selected slot is recoverably
  invalid.
- No menu command advances progression without the owning service transaction.

## Verification

Automated verification includes:

- boot with zero, one, multiple, corrupt, migratable, and missing-content slots;
- deterministic resume selection with tied timestamps;
- reviewed new-game replacement and failure injection at every transaction step;
- slot ordering and status projection across supported platforms;
- equal progress values in scrapbook and pause-menu projections;
- Common UI focus, back-stack, modal blocking, and input-method transitions;
- keyboard and mouse, gamepad, and touch semantic command parity;
- safe-area, display-cutout, density, aspect-ratio, and localization layouts;
- standard, Christmas, and Halloween theme selection and fallback;
- idle-event cancellation without progression effects;
- options rollback after failed engine or platform application;
- credits skip, interruption, and return-state behavior; and
- clean transition between frontend, the connected sandbox, every chapter
  boundary, and mission checkpoint resume.
