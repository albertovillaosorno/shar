# Frontend screen flow and settings runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
- [UI parity boundary](../../adr/unreal/ui/ui-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend shell and menu runtime](frontend-shell-and-menu-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI navigation, menu, and modal runtime](common-ui-navigation-menu-and-modal-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend media, gallery, and audio runtime](frontend-media-gallery-and-audio-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Device configuration and save-slot runtime](device-configuration-and-save-slot-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Semantic input, device, and haptics runtime](semantic-input-device-and-haptics-runtime.md)
- [Presentation playback runtime](presentation-playback-runtime.md)

## Purpose

This specification defines the native Unreal screen-flow runtime for boot,
language and legal presentation, loading, frontend navigation, save selection,
galleries, local bonus-mode setup, controller and display settings, media input,
and typed recovery.

It replaces screen classes that own domain state, integer message routing,
compile-time platform variants, file-count and memory-use progress heuristics,
direct platform-setting mutation, device-index ownership, and storage-specific
error prompts.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Authority |
| :--- | :--- |
| Frontend subsystem | Owns stable frontend state, screen-flow requests, history, and command results. |
| Common UI layer service | Owns widget activation, focus, action routing, and modal stacking. |
| Application lifecycle service | Owns boot, loading, frontend, gameplay, demo, and bonus-mode transitions. |
| Asset-load service | Owns correlated load plans, progress observations, cancellation, and readiness. |
| Device-configuration service | Owns editable settings, previews, validation, commit, and rollback. |
| Input subsystem | Owns devices, local-player assignment, semantic actions, and rebinding. |
| Save service | Owns slot discovery, summaries, load, migration, recovery, and durable results. |
| Progression service | Owns gallery availability, collected content, mission replay eligibility, and unlocks. |
| Presentation playback service | Owns cinematic playback, skip, pause, fallback, and teardown. |

<!-- markdownlint-enable MD013 -->

A Common UI widget, animation, loading indicator, media player, platform dialog,
or storage adapter is a projection. It cannot select application mode, mutate
progression, own a save slot, assign a controller, or commit configuration.

## Runtime topology

The runtime uses:

- `FSharFrontendFlowId`, one stable flow identity;
- `FSharFrontendFlowRevision`, one accepted flow revision;
- `FSharFrontendScreenId`, a canonical screen identity;
- `FSharFrontendScreenDefinition`, immutable screen and layer policy;
- `FSharFrontendNavigationRequest`, one typed navigation request;
- `FSharFrontendScreenSnapshot`, one immutable view-model snapshot;
- `FSharFrontendModalResult`, one typed modal result;
- `FSharLoadingPresentationId`, one correlated loading projection;
- `FSharSettingsEditSessionId`, one editable configuration session;
- `USharFrontendFlowSubsystem`, the game-instance flow authority; and
- Common UI, Asset Manager, platform, save, input, media, and progression
  adapters.

Every event carries frontend, screen, owner, local-player, and relevant domain
revisions. A stale widget, load callback, media callback, controller
observation,
or storage result cannot navigate or mutate a replacement screen.

## Screen definition

`FSharFrontendScreenDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ScreenId` | Canonical screen identity. |
| `LayerId` | Boot, primary, modal, notification, or another registered Common UI layer. |
| `WidgetClass` | Cooked Common UI widget class. |
| `ViewModelSchemaId` | Immutable snapshot schema accepted by the widget. |
| `RequiredBundles` | Asset bundles required before activation. |
| `SemanticActionSetId` | Accepted Common UI actions. |
| `EntryPredicateId` | Application, feature, local-player, and domain requirements. |
| `ExitPolicyId` | Back, complete, replace, modal result, or transition behavior. |
| `FocusPolicyId` | Initial focus and restoration behavior. |
| `LoadingPolicyId` | Optional loading presentation and readiness barrier. |
| `FailurePolicyId` | Typed fallback, retry, prior-screen restoration, or blocked state. |
| `AccessibilityProfileId` | Narration, text, contrast, timing, and reduced-motion policy. |
| `FeatureOwnerId` | Base game or validated feature package. |

<!-- markdownlint-enable MD013 -->

Definitions reject duplicate identities, missing widgets, unresolved action
sets,
invalid layer ownership, inaccessible focus, and transitions to undeclared
screens.

## Flow states

One frontend flow revision has one state:

- `uninitialized`;
- `booting`;
- `waiting_for_input`;
- `loading`;
- `screen_active`;
- `modal_active`;
- `transitioning`;
- `recovering`;
- `blocked`;
- `shutting_down`; or
- `terminated`.

`screen_active` owns one primary screen and optional notifications.
`modal_active` owns one blocking modal above the prior stable screen.
`transitioning` disables duplicate commands until the destination or typed
failure is accepted.

A screen animation completing does not complete a flow transition. The flow
accepts a destination only after definition, assets, view model, focus, and
semantic action routing are ready.

## Navigation transaction

A navigation request contains:

- source and destination screen identities;
- frontend and source-screen revisions;
- local-player and initiating action identities;
- history policy;
- required domain snapshot revisions;
- required asset-load plan;
- optional modal or loading policy; and
- cancellation and fallback behavior.

Navigation follows this sequence:

1. validate the source, destination, feature owner, and revisions;
1. reserve the destination layer and history operation;
1. request required domain snapshots and asset bundles;
1. construct the immutable destination view model;
1. create and validate the destination widget;
1. deactivate or suspend the source according to policy;
1. activate the destination and verify focus and actions;
1. commit history and the new screen revision; and
1. release source-only resources.

Failure restores the prior stable screen and focus. It cannot leave a blank
screen, duplicate active widgets, or partially committed history.

## Boot task graph

Boot is a validated task graph rather than a mutable queue of screen
identifiers.
The base graph may contain:

- platform and legal readiness;
- hardware-language observation;
- locale selection or confirmation;
- device-configuration load;
- storage-provider readiness;
- logical save-slot summary discovery;
- optional display-mode recovery or preview;
- required startup media;
- frontend asset-bundle readiness; and
- main-menu activation.

Every task declares prerequisites, optionality, retry behavior, terminal result,
and presentation. Tasks with no dependency relationship may run concurrently,
but the accepted result order remains deterministic.

A completed boot screen cannot skip an unfinished required task. An optional
task
may degrade presentation or expose a typed notice without blocking the main
menu.
A required task failure enters recovery or `blocked` according to policy.

## Language selection

The platform adapter reports a normalized hardware or account locale
observation.
The localization service resolves it against the cooked supported-locale
catalog.
The result is one of:

- `supported_and_selected`;
- `supported_but_confirmation_required`;
- `unsupported_requires_selection`;
- `configuration_override_selected`;
- `fallback_selected`; or
- `failed`.

A language screen receives an immutable list of supported locale identities,
localized self-names, accessibility metadata, current selection, and
confirmation
policy. It commits only through the device-configuration edit transaction.

Unsupported platform values never index a fixed language array or silently
select an arbitrary entry. A safe fallback may be proposed, but the accepted
configuration and localization revisions must agree before boot continues.

Changing locale invalidates only locale-dependent frontend assets and text. It
does not recreate save, player, controller, or progression identity.

## Legal and license presentation

Required legal and license surfaces are definition-driven. Each entry declares:

- legal-surface identity;
- platform, region, locale, and distribution predicates;
- cooked presentation asset or localizable document;
- minimum display and accessibility timing;
- whether acknowledgement is required;
- accepted input actions;
- offline behavior; and
- evidence and revision metadata.

The boot graph selects the exact required ordered set. Compile-time platform
image
names and screen-duration constants do not define legal coverage.

A legal surface that cannot resolve fails according to release policy. The
runtime cannot substitute a different platform notice, omit required text, or
fabricate acknowledgement.

## Boot loading presentation

A boot loading screen projects one correlated boot task graph. Its snapshot
contains task counts, required and optional completion, current semantic phase,
known progress, indeterminate state, localized status, and accessibility timing.

A minimum presentation duration may prevent a flash, but it cannot delay a
critical error or imply work remains after readiness. Animation completion does
not prove task completion.

The boot layer releases only after the final required task commits and the next
screen verifies focus and semantic input routing.

## Loading transaction

Application transitions request one `FSharLoadPlan` from the asset and lifecycle
services. The frontend loading projection consumes:

- load-plan and application-transition identities;
- required and optional operation identities;
- completed, active, waiting, failed, and cancelled operations;
- weighted progress where the producer can prove it;
- indeterminate phases;
- current semantic operation label;
- world, feature, and asset readiness barriers;
- minimum and maximum presentation policy; and
- terminal transition result.

Progress is produced by the load plan. The widget cannot derive it from process
memory, free memory, an expected file count, callback count, frame count, or
elapsed animation time.

Operations without meaningful measurable progress remain indeterminate. Weighted
progress is monotonic for one load-plan revision and cannot reach complete until
all required readiness barriers are accepted.

## Loading presentation selection

The loading definition may select presentation by:

- source and destination application modes;
- campaign, chapter, mission, or bonus-mode identity;
- platform and quality profile;
- locale and accessibility profile;
- active calendar or feature theme; and
- validated mod overlay.

Presentation may include artwork, text, animation, audio, hints, and progress
style. It cannot inspect local filenames or infer mode from a global context
enum.

Gameplay, frontend, demonstration, and local bonus-mode loading share one domain
contract. They may use different presentation definitions without different
completion semantics.

## Backend transition handoff

The frontend flow and application lifecycle coordinate through one transition
request. The screen flow may request a loading projection, but the lifecycle
service owns the destination mode and terminal result.

The sequence is:

1. accept the application transition request;
1. suspend duplicate frontend commands;
1. activate the correlated loading presentation;
1. execute and observe the load plan;
1. prepare the destination mode;
1. verify required world, player, feature, and input state;
1. commit the application transition;
1. publish the terminal loading result; and
1. release the frontend or loading screen according to destination policy.

A loading widget cannot set application context, launch gameplay, quit a demo,
or
choose the destination screen directly.

## Cinematic playback and input

A frontend or boot flow may request a cinematic presentation through the
presentation playback subsystem. The request declares media identity, owner and
flow revisions, skip policy, minimum time before skipping is allowed,
pause policy, audio focus, controller-loss behavior, fallback, and
accepted terminal results.
controller-loss behavior, fallback, and accepted terminal results.

Semantic `skip`, `confirm`, `back`, and platform-required actions are routed
through the local-player input session. The media adapter never registers raw
button maps or decides controller ownership.

A skip before eligibility returns `skip_not_allowed`. An accepted skip publishes
one typed result and executes the same restoration and teardown contract as
normal completion. Fade, frame readiness, decoder state, drive flush, or render
completion cannot become owner-visible completion.

Controller loss may pause the presentation and open one correlated recovery
modal. Reconnection revalidates local-player assignment and resumes or remains
paused according to policy. A newly enumerated device is never assumed to own
the
prior player merely because it uses the same index.

## Controller-loss recovery

A controller-loss observation identifies device, local player, assignment,
frontend flow, and screen revisions. The frontend response is policy-driven:

- show a non-blocking notice when another assigned device remains valid;
- open one blocking reassignment modal when required navigation ownership is
  lost;
- pause an owned cinematic or transition when continuing would be unsafe;
- retain the prior focused semantic action; and
- release stale held actions, haptics, pointer capture, and device callbacks.

The modal accepts only compatible reconnect or reassignment results from the
input
subsystem. Closing the modal restores focus and actions for the accepted
assignment revision. Duplicate disconnects do not stack duplicate prompts.

## Main-menu routing

The main menu consumes one immutable command snapshot. Each command contains:

- command identity and localized presentation;
- visibility and enabled state;
- disabled reason;
- required save, progression, catalog, feature, and application revisions;
- destination screen or application transition identity;
- confirmation policy; and
- optional preview presentation.

Selection submits a command request. The widget cannot load a mission, begin a
new game, resume, exit the application, start a local bonus mode, or play media
directly.

Idle character, vehicle, television, calendar, and environmental animations are
presentation definitions. Their random or cyclical selection uses a stable
session seed and cannot change command availability or progression.

## Save browser and automatic resume

The save browser consumes logical slot summaries from the save service. It never
enumerates storage filenames or interprets platform media codes.

Each slot row exposes:

- logical slot identity and accepted revision;
- empty, valid, recoverable, migrating, incompatible, or corrupt state;
- campaign and progression summary;
- timestamp and platform-safe metadata;
- missing feature or catalog requirements;
- load, inspect, repair, replace, or unavailable commands; and
- localized typed findings.

Selecting a loadable slot creates one asynchronous load request with the
expected
slot and summary revisions. The screen remains active or displays a correlated
modal until one terminal result is accepted.

Storage unavailable, permission denied, quota, corruption, unsupported schema,
missing content, migration failure, cancellation, and platform removal are
distinct results. Platform adapters may offer remediation, but the screen does
not format media or mutate storage itself.

Automatic resume chooses the most recent eligible accepted slot through the save
service. It uses the same load transaction and failure policy as manual loading.
A failed automatic attempt returns to a stable frontend state and never loops or
silently chooses another slot without policy.

## Card gallery

The card gallery consumes one progression snapshot containing:

- active chapter or deck identity;
- ordered canonical card identities;
- collected and uncollected state;
- title, episode, description, quote, and image presentation references;
- current selection and focus;
- full-card-view eligibility; and
- accessibility alternatives.

Browsing and full-card viewing are presentation states. The gallery cannot mark
a
card collected, infer collection from image availability, or change deck order.

High-resolution assets load through one correlated view request. A missing image
uses the declared fallback while preserving card identity and collection state.
Closing the view releases only view-specific assets and restores the prior
thumbnail focus.

Quote playback is a presentation request. It cannot advance gallery selection or
progression, and repeated view callbacks cannot replay a quote outside policy.

## Mission gallery and replay

The mission gallery consumes a progression and replay-eligibility snapshot. Each
row contains mission, chapter, display order, completion state, replay policy,
required content, image and title presentation, checkpoint policy, and disabled
reason.

Mission images are primary assets addressed by mission identity. The gallery
never constructs filenames from chapter and ordinal values at runtime.

Selecting an eligible mission creates a replay transition request containing:

- mission and replay-definition identities;
- source save and progression revisions;
- selected participant and allowed equipment policy;
- checkpoint or stage entry policy;
- isolated replay-progression policy;
- required world and feature load plan; and
- frontend restoration behavior.

The lifecycle service accepts or rejects the replay. A menu animation, image
load, or widget exit cannot start the mission. Ineligible or incomplete missions
remain visible according to product policy and expose the exact reason.

## Local bonus-mode setup

Local bonus-mode setup is not campaign multiplayer. It is a bounded local
session
configuration for supported bonus modes.

A setup session contains:

- bonus-mode and rule-set identities;
- supported local-player count;
- joined local-player and device assignments;
- selected character or presentation identities;
- map, vehicle, lap, round, and option policies;
- uniqueness and compatibility constraints;
- readiness state per participant; and
- transition and cancellation policy.

Joining, leaving, character selection, and readiness are local-player
transactions. Controller index does not define participant identity. Duplicate
character restrictions, if any, are rule-set data rather than widget logic.

The session starts only after every required participant, device, option, and
content dependency validates. Cancellation releases temporary assignments and
returns to the prior screen without changing campaign progression.

## Options hub

The options hub presents registered configuration categories such as:

- input and controller;
- audio;
- display and graphics;
- language and accessibility;
- media and credits viewing; and
- development-only diagnostics when explicitly authorized.

Categories are catalog entries with platform and feature predicates. A platform
without an editable display mode omits that category through definition policy,
not compile-time screen code.

Opening a category creates one `FSharSettingsEditSession` from the accepted
device-configuration revision. The category widget edits a draft and cannot
persist individual changes directly.

## Input-binding editor

The input-binding editor consumes semantic action and profile definitions. It
supports keyboard, mouse, gamepad, wheel, touch, and future adapters through one
schema.

A capture request contains:

- local profile and edit-session identities;
- action and binding-slot identities;
- eligible device classes and value types;
- reserved-input policy;
- timeout and cancellation policy;
- conflict policy; and
- accessibility requirements.

The input subsystem captures one candidate and returns its engine-key identity,
device class, axis or direction metadata, modifiers, and capability snapshot.
The binding transaction validates compatibility and conflict resolution before
updating the draft.

Displayed button names and glyphs are projections. Platform-specific string
formatting, virtual-key switches, manual hotspot detection, and separate old and
new screen implementations do not define the binding contract.

Back, device loss, feature removal, or failed conflict resolution cancels the
capture and restores the last accepted draft. Commit persists the complete
validated profile atomically.

## Controller and gameplay settings

Controller settings include vibration or haptic preference, mouse-look policy,
mouse and wheel sensitivity, axis inversion, and other registered profile
values.
They are draft configuration fields, not direct mutations of input devices.

Every setting declares type, finite range, step, default, compatibility, preview
policy, and persistence scope. Device capability may disable or hide a value
with
a typed reason. A wheel-only setting cannot appear for an incompatible profile.

Input values that support preview update a scoped preview profile. Cancel
restores the accepted profile. Commit validates the complete draft and applies
one new
configuration revision.

## Display settings edit session

Display settings are edited through a correlated session containing:

- display-adapter and monitor identity;
- current accepted mode;
- supported resolution, refresh, window, color, HDR, and quality values;
- gamma or brightness policy;
- accessibility constraints;
- expected configuration revision;
- preview timeout and confirmation policy; and
- known safe fallback mode.

The platform adapter supplies supported values. The widget cannot use a fixed
resolution enum or assume color depth and fullscreen choices are portable.

Gamma or brightness preview uses a scoped rendering override. Leaving without
commit restores the accepted value. The preview cannot save merely because the
screen deactivated.

## Risky display preview

A mode change that can make the display unusable follows this transaction:

1. validate the proposed mode and safe fallback;
1. capture the accepted configuration and platform mode revisions;
1. apply a temporary platform preview;
1. show a modal using a presentation path valid in both modes;
1. start a monotonic confirmation deadline;
1. accept semantic confirm or reject;
1. commit configuration only after confirmation; and
1. restore the known safe mode on reject, timeout, focus loss, suspension,
   adapter loss, crash recovery, or failed verification.

Progressive, interlaced, fullscreen, resolution, refresh, HDR, and similar risky
changes use this one policy where applicable. A screen-specific timer or direct
platform call cannot own confirmation.

Startup detects an unfinished preview journal and restores the safe mode before
normal frontend presentation.

## Settings commit

`FSharSettingsEditSession` contains a base configuration revision, a typed
draft,
preview handles, validation findings, and a terminal result.

Commit follows this sequence:

1. validate field schemas and platform capabilities;
1. validate cross-field constraints;
1. prepare input, audio, display, localization, and accessibility adapters;
1. verify any required risky preview was confirmed;
1. write the device-local configuration atomically;
1. activate the new accepted revision;
1. release previews and old adapter state; and
1. publish the result to all dependent view models.

Failure leaves the accepted configuration unchanged and restores every preview.
Individual category screens never write the configuration file directly.

## Cheat and diagnostic overlay

Cheat recognition and state remain owned by the cheat runtime. A shipping
options
screen does not expose an undocumented master overlay through ordinary input.

An explicitly authorized development overlay may project registered cheat or
diagnostic state. It uses semantic development actions, reports unavailable
entries honestly, and cannot intercept unrelated frontend input outside its
active lease.

The overlay never becomes the source of cheat identity, activation, persistence,
or progression behavior.

## Platform adapters

Platform adapters may provide:

- hardware locale and legal predicates;
- application-exit capability;
- storage and remediation surfaces;
- display modes and risky-preview application;
- physical-device and glyph capabilities;
- decoder and media surfaces;
- native accessibility services; and
- suspension, focus, and device-loss observations.

Adapters return typed capabilities and results. Compile-time screen subclasses,
platform-specific menu arrays, storage-device names, and raw error codes do not
enter the domain contract.

Unsupported capability returns `unavailable` with a reason. It does not select a
neighboring enum value or silently omit a required release surface.

## Presentation lifecycle

Widget animations, loading art, main-menu idle scenes, gallery transitions,
button glow, prompts, and modal effects are presentation requests. They may
delay
focus or input until a declared safe point but cannot mutate domain state.

Screen deactivation cancels or transfers owned presentation requests by
revision.
A callback from a prior screen cannot hide the current screen, restore old
focus,
play a stale quote, or navigate history.

Reduced-motion policy may replace spatial or timed transitions with accessible
alternatives while preserving the same screen and command results.

## Feature and mod overlays

A validated feature package may add namespaced screens, commands, settings
categories, galleries, local bonus-mode rules, loading themes, and presentation.
It must declare dependencies, layer ownership, navigation edges, platform
support, persistence, and teardown.

An overlay cannot replace a base screen in place while it is active, mutate
another package's navigation history, claim another package's configuration
fields, weaken display recovery, or gain save and progression authority.

Feature removal rejects new requests, cancels or migrates active screens through
the declared policy, restores previews, releases input and presentation leases,
and verifies zero owned widgets or load handles.

## Diagnostics

Development diagnostics expose immutable snapshots of:

- frontend, screen, flow, local-player, and feature revisions;
- current layer stack and history;
- active navigation and modal requests;
- boot task graph and terminal results;
- load-plan progress and readiness barriers;
- active controller-loss recovery;
- save-browser request and findings;
- gallery and replay snapshot revisions;
- local bonus-mode setup state;
- settings draft, previews, and validation findings;
- active asset, input, media, and presentation handles; and
- last rejection, recovery, or teardown result.

Diagnostics may request a validated retry or cancellation in a test world. They
cannot force navigation, confirm a risky display mode, load a save, unlock a
gallery entry, or commit configuration.

## Failure behavior

The runtime fails closed on:

- missing or duplicate screen identity;
- invalid layer ownership or navigation edge;
- stale flow, screen, local-player, domain, or feature revision;
- destination activation without assets, view model, focus, or actions;
- boot completion with an unfinished required task;
- loading progress derived from memory or guessed operation counts;
- widget-driven application transition;
- uncorrelated media skip or controller recovery;
- save load against a changed summary revision;
- gallery state inferred from presentation assets;
- mission replay started by widget exit or animation completion;
- local-player setup based on device enumeration order;
- invalid or conflicting input binding;
- display preview without a safe rollback journal;
- partial configuration commit;
- feature removal with owned widgets, previews, leases, or handles; or
- presentation callback mutating authoritative state.

Failure returns typed evidence and restores the prior stable screen,
configuration, focus, input assignment, display mode, and application state
where
possible. It never leaves an unresponsive blank frontend.

## Validation

Definition validation proves:

- every screen, layer, action set, view model, and navigation edge resolves;
- boot task dependencies are acyclic and every required task has recovery;
- every legal surface has platform, region, locale, and release evidence;
- loading progress comes from validated load-plan producers;
- every command has availability and failure behavior;
- save, gallery, and replay screens use canonical identities;
- local bonus-mode setup declares participant and device constraints;
- every input action has compatible binding capture and conflict policy;
- every risky display edit has a safe fallback and recovery journal;
- every settings field has schema, range, default, and persistence scope;
- platform adapters preserve one domain result contract; and
- overlays cannot weaken base recovery or authority boundaries.

## Tests

Required automated tests include:

- boot task ordering, concurrency, optional degradation, and required failure;
- supported, unsupported, overridden, and fallback locale selection;
- legal-surface selection across supported platform and locale matrices;
- loading progress monotonicity and indeterminate phases;
- load completion only after every required readiness barrier;
- frontend-to-gameplay, demo, and local bonus-mode transitions;
- cinematic completion, skip denial, accepted skip, pause, and controller loss;
- duplicate disconnect and reconnect prompt suppression;
- navigation rollback after destination load or focus failure;
- manual load, automatic resume, corruption, migration, and storage removal;
- card browsing, full view, quote playback, and missing-image fallback;
- mission gallery eligibility and replay transition rejection;
- local-player join, leave, assignment, character selection, and cancellation;
- binding capture, conflict, swap, clear, reject, cancel, and device loss;
- input preference preview, cancel, and commit;
- display preview accept, reject, timeout, suspension, and startup recovery;
- atomic settings commit and rollback;
- feature removal during screen, loading, media, capture, and preview activity;
- accessibility focus, narration, reduced motion, and timing; and
- identical authoritative results across supported platforms and quality
  presets.

## Invariants

- One frontend flow revision owns one stable primary screen.
- Widgets consume immutable snapshots and submit typed requests.
- Screen animation and presentation completion never imply domain completion.
- Boot ends only after every required task has a terminal accepted result.
- Loading progress comes from the correlated load plan.
- Controller identity and local-player ownership never derive from array index.
- Save, card, mission, and settings use canonical identities and explicit
  revisions.
- One settings edit session commits one complete validated revision or none.
- Every risky display preview can restore a known safe mode.
- Every navigation, modal, load, media, capture, and preview callback is
  revision-correlated.
- Feature removal leaves no owned widget, load, input, media, or preview handle.
