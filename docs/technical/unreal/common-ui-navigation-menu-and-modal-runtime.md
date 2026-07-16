# Common UI navigation, menu, and modal runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
- [UI parity boundary](../../adr/unreal/ui/ui-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Minimal hexagonal native runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md)
- [Frontend shell and menu runtime](frontend-shell-and-menu-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [In-game HUD, pause, and transition runtime](in-game-hud-pause-and-transition-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [HUD feedback cue and presentation-primitives runtime](hud-feedback-cue-and-presentation-primitives-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Local split-screen minigame session UI runtime](local-split-screen-minigame-session-ui-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission briefing, result, and replay UI runtime](mission-briefing-result-and-replay-ui-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Reward browser, preview, and purchase UI runtime](reward-browser-preview-and-purchase-ui-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Semantic input, device, and haptics runtime](semantic-input-device-and-haptics-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)

## Purpose

This specification defines the native Unreal user-interface kernel shared by
boot, frontend, gameplay, pause, mission-result, loading, gallery, prompt, and
notification screens.

It replaces global integer messages, mutable screen singletons, fixed-size
history arrays, widget-owned domain state, compile-time platform screen forks,
manual controller polling, storage-device-specific prompts, and callbacks that
can mutate a replacement screen after their owner has exited.

The kernel preserves observable screen lifecycle, navigation, focus, menu,
prompt, input, transition, localization, and recovery behavior while expressing
those behaviors through Common UI, managed subsystems, Asset Manager leases, and
typed application ports.

## Native Unreal foundation

The runtime uses Unreal-native facilities as follows:

- `UCommonGameViewportClient` is the viewport input-routing base;
- Common UI activatable widgets own activation, deactivation, focus, and action
  presentation;
- Common UI action data owns user-interface click, back, navigation, and
  screen-specific actions;
- controller data assets own platform and device glyph projection;
- `UGameInstanceSubsystem` owns application-wide navigation and screen state;
- `ULocalPlayerSubsystem` owns per-player focus, device assignment, and input
  context;
- Asset Manager primary assets and bundles own screen-definition discovery and
  asynchronous presentation dependencies; and
- streamable handles keep a screen bundle resident only for the accepted
  activation lease.

Common UI actions are user-interface actions. Gameplay Enhanced Input mappings
remain a separate runtime surface and cannot become a second menu-command
registry.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Lifetime | Authority |
| :--- | :--- | :--- |
| `USharUiNavigationSubsystem` | Game instance | Screen registry, navigation transactions, history, layer ownership, and accepted screen revision. |
| `USharUiViewModelSubsystem` | Game instance | Immutable view-model construction from accepted domain snapshots. |
| `USharUiModalSubsystem` | Game instance | Modal queue, modal ownership, prompt results, and notification policy. |
| `USharFrontendInputSubsystem` | Local player | Semantic UI actions, current input method, focus restoration, and glyph context. |
| `USharApplicationLifecycleSubsystem` | Game instance | Boot, frontend, loading, gameplay, demo, pause, and shutdown transitions. |
| Save application port | Application | Logical slots, storage capability, transactions, migration, recovery, and durable results. |
| Localization port | Application | Locale, text identity, formatted arguments, fallback, and text revision. |
| Presentation ports | Driven adapters | Widget, animation, audio, media, and transition execution only. |

<!-- markdownlint-enable MD013 -->

A widget can request a command and render a snapshot. It cannot select an
application mode, mutate progression, write a save, format storage, change a
controller assignment, commit settings, or infer availability from visible
children.

## Runtime identities

Every accepted operation carries explicit identity:

- `FSharUiFlowId` identifies one boot, frontend, or in-game flow;
- `FSharUiFlowRevision` changes whenever the accepted flow is replaced;
- `FSharUiScreenId` identifies a registered screen definition;
- `FSharUiScreenRevision` identifies one activation of that screen;
- `FSharUiLayerId` identifies boot, primary, modal, notification, or a
  registered feature layer;
- `FSharUiNavigationRequestId` correlates one navigation attempt;
- `FSharUiModalRequestId` correlates one message or prompt;
- `FSharUiFocusLeaseId` correlates focus ownership for one local player;
- `FSharUiAssetLeaseId` correlates loaded presentation dependencies; and
- domain revision tokens identify the save, progression, settings, media, or
  mission snapshots used to build the view model.

An observation without the expected owner and revision is stale. Stale input,
asset completion, animation completion, media completion, storage completion,
or controller observations are recorded and ignored.

## Layer stack

The game viewport owns one registered layer stack:

1. boot;
1. primary;
1. modal;
1. notification; and
1. optional validated feature layers declared by data.

Only one boot or primary activatable widget may own focus for a local player at
a time. A blocking modal suspends primary actions without destroying the primary
screen. Notifications do not steal focus unless their definition explicitly
requires acknowledgement.

Layer registration is validated at startup. Missing roots, duplicate layer
identities, invalid z-order, or a focusable notification layer fail startup
validation rather than degrading into input races.

## Screen definition

`FSharUiScreenDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ScreenId` | Stable canonical identity. |
| `LayerId` | Registered destination layer. |
| `WidgetClass` | Cooked Common UI activatable widget class. |
| `ViewModelSchemaId` | Exact immutable snapshot schema. |
| `ActionSetId` | Common UI actions accepted while active. |
| `EntryPredicateId` | Application, feature, player, and domain requirements. |
| `ExitPolicyId` | Back, complete, replace, modal-result, or blocked policy. |
| `FocusPolicyId` | Initial focus, restoration, and no-focus behavior. |
| `HistoryPolicyId` | Push, replace, clear, preserve, or no-history behavior. |
| `TransitionPolicyId` | Enter, exit, interruption, reduced-motion, and timeout policy. |
| `RequiredBundles` | Asset Manager bundles required before activation. |
| `FailurePolicyId` | Retry, prior-screen restoration, fallback, or blocked behavior. |
| `AccessibilityProfileId` | Narration, contrast, timing, text, and motion requirements. |
| `FeatureOwnerId` | Base game or validated feature package. |

<!-- markdownlint-enable MD013 -->

Definitions fail validation for duplicate identities, missing classes, unknown
layers, unresolved actions, impossible focus, undeclared destinations, recursive
fallback, or an asset bundle that is unavailable in the target cook.

## Screen lifecycle

One activation has the following states:

- `reserved`;
- `loading`;
- `constructing`;
- `entering`;
- `active`;
- `suspended`;
- `exiting`;
- `completed`;
- `cancelled`; or
- `failed`.

A screen becomes `active` only after its definition, domain snapshots, asset
bundles, widget instance, semantic actions, and focus policy validate. An enter
animation is presentation within the transaction; it is not authority to commit
the destination by itself.

Exit first revokes input and command leases, then runs bounded presentation,
then releases view-model and asset leases. Timeout, removal, suspension, or
feature deactivation must still reach one terminal state.

## Navigation request

`FSharUiNavigationRequest` contains:

- request, flow, source-screen, destination-screen, owner, and local-player
  identities;
- expected flow, source-screen, and relevant domain revisions;
- destination parameters validated against the destination schema;
- history and transition policy;
- required asset bundles and timeout;
- optional modal or loading presentation;
- cancellation reason policy; and
- typed fallback behavior.

Unknown parameter keys, values outside the destination schema, cross-player
ownership, undeclared destinations, or stale revisions are rejected before any
visible state changes.

## Navigation transaction

Navigation follows this sequence:

1. validate source ownership, revisions, destination, and feature availability;
1. reserve the destination layer and history operation;
1. acquire required domain snapshots and Asset Manager bundles;
1. construct and validate the immutable destination view model;
1. instantiate the destination widget and bind only the declared actions;
1. revoke or suspend source focus and command leases;
1. activate the destination and verify its initial focus target;
1. commit destination, history, and screen revision atomically; and
1. release source-only presentation and asset leases.

Failure before commit leaves the source active. Failure after source suspension
restores the source widget, focus, actions, and history from the transaction
journal. The runtime cannot leave an empty primary layer or two committed
primary screens.

## History and back navigation

History is a bounded typed sequence of screen restoration records. Each record
contains the screen identity, validated parameters, focus restoration target,
view-model reconstruction policy, feature owner, and accepted revisions.

Back is a semantic command resolved in this order:

1. the active modal handles or rejects back according to its policy;
1. the active screen handles a declared local back command;
1. navigation restores the newest valid history record;
1. the application lifecycle handles a declared frontend exit or gameplay
   resume command; or
1. the command is rejected with a reason.

Invalid, removed, or feature-owned history records are skipped with diagnostics.
History never stores widget pointers, physical controller buttons, or mutable
domain objects.

## Focus ownership

Each local player has at most one focus lease per blocking layer. A focus policy
identifies the preferred widget, fallback search order, restoration target, and
whether pointer-only focus is valid.

Activation verifies that the target is visible, enabled, focusable, and owned by
the current widget tree. If it is not, deterministic fallback runs. Failure to
find required focus blocks activation and restores the prior screen.

Focus restoration uses semantic item identity, not child index. Dynamic list
changes, disabled items, localization, feature removal, or viewport rebuilds
therefore cannot restore focus to an unrelated item.

## Menu definition

`FSharUiMenuDefinition` contains stable item definitions rather than widget
children. Each item contains:

- item identity and localized label identity;
- optional value-domain identity;
- enabled, visible, and selectable predicates;
- command identity;
- cardinal navigation neighbors or deterministic ordering policy;
- wrap policy;
- hold, repeat, and pointer policy;
- visual-state and accessibility metadata; and
- feature owner.

A menu snapshot contains the evaluated visible order, selection identity,
accepted value per item, disabled reasons, and snapshot revision. Widgets cannot
infer command identity from label text or array position.

## Selection and command transaction

A movement command resolves the next enabled and selectable item from the
accepted snapshot. Disabled items may remain visible and narrated but cannot
receive command focus unless a screen explicitly supports explanatory focus.

A command selection follows this sequence:

1. validate screen, menu, item, player, and snapshot revisions;
1. reject duplicate activation while a prior command is pending;
1. evaluate the item predicate again against accepted domain state;
1. publish one typed application command;
1. render pending state without changing domain authority;
1. accept one typed command result; and
1. navigate, update the snapshot, show a modal, or restore interactivity.

Repeated select, mouse, touch, and controller input for the same pending command
is idempotent.

## Value selectors and sliders

A value-bearing item declares a finite typed domain, step, wrap policy, preview
policy, formatter, and persistence owner. Left and right actions request a draft
value change. The widget does not mutate audio, display, input, or save state
directly.

Continuous sliders distinguish active adjustment from adjustment completion.
Preview-capable services may apply a scoped preview while the value changes.
The terminal adjustment observation triggers expensive reconciliation, audio
sample playback, or validation only once. Cancel restores the accepted value;
commit persists the complete validated settings session atomically.

## Input routing

Common UI routes user-interface actions through the viewport and active widget
stack. The local-player input subsystem owns:

- current input method;
- controller and local-player assignment;
- UI action context;
- glyph data identity;
- focus lease;
- repeat state;
- pointer capture; and
- controller-loss state.

Widgets receive semantic actions such as navigate, confirm, back, page-left,
page-right, tab-left, tab-right, and screen-specific commands. They never branch
on platform key codes or controller indices.

## Repeat and hold behavior

Repeatable navigation uses monotonic time and a policy containing initial delay,
repeat interval, acceleration, diagonal arbitration, and release reset. One
physical hold produces deterministic semantic repeats independent of frame rate.

Press, hold, repeat, and release are distinct observations. A released input
cannot remain latched after focus loss, modal activation, application
suspension, controller reassignment, or screen replacement.

## Pointer and touch behavior

Pointer hover may change focus only when the active screen policy permits it.
Pointer press captures one item identity and revision. Release activates only if
that same item remains eligible and owns the capture.

Touch uses the same command and item identities with touch-safe hit targets.
Manual coordinate hotspots, art-name parsing, and separate touch commands are
not domain authority.

## Controller connection and assignment

A disconnect observation contains physical device, local player, screen, flow,
and observation revisions. The local-player subsystem first revokes the device
lease and releases held actions.

A blocking controller-loss modal appears only when the disconnected device was
required by the current local player or session. Reconnection validates the
physical device, local-player assignment, and active session before restoring
actions and focus.

A different device may satisfy the modal according to session policy. Device
index alone never proves ownership.

## Transition presentation

Enter, exit, fade, scale, slide, iris, letterbox, and reduced-motion variants
are registered presentation policies. A transition declares duration, interrupt
behavior, input barrier, required assets, accessibility fallback, timeout, and
completion observation.

The navigation transaction owns transition completion. An animation callback
without the expected request and screen revision cannot commit navigation.
Reduced-motion mode substitutes a validated low-motion policy without changing
screen state or command timing requirements.

## Asset readiness and leases

Screen definitions and heavy presentation families are Primary Assets. Named
bundles separate minimal layout, localized text, thumbnails, three-dimensional
previews, media, and optional feature content.

The navigation subsystem requests bundles asynchronously and retains their
streamable handles for the screen asset lease. A screen may expose a bounded
loading projection while optional content loads, but required content must be
ready before activation.

Cancellation releases handles not shared by another accepted lease. Completion
from a cancelled request cannot attach assets to the replacement screen.

## Messages and prompts

Messages and prompts are modal transactions, not special global screens with
static mutable payloads.

`FSharUiModalRequest` contains:

- request, owner, flow, screen, and local-player identities;
- localized title and body identities with typed arguments;
- severity and icon policy;
- ordered response definitions;
- default and cancel response identities;
- timeout and application-suspension policy;
- optional remediation command identities; and
- expected domain revisions.

A message may have no response beyond acknowledgement. A prompt has one or more
explicit responses. Response identity is semantic, such as continue, retry,
confirm, reject, load, save, delete, repair, manage storage, or continue without
saving. Display labels and physical buttons are projections.

## Modal transaction

A modal request follows this sequence:

1. validate owner, revisions, response set, localization, and remediation;
1. reserve the modal layer;
1. suspend primary-screen actions and capture restoration focus;
1. activate the modal and verify safe initial focus;
1. accept one semantic response or terminal cancellation;
1. publish one correlated result to the owning application operation;
1. close the modal and restore the prior focus lease; and
1. release modal assets.

A response is accepted at most once. Back and start actions map only when the
modal definition declares them. Destructive prompts default to the safe response
and never activate merely because a controller reconnects or focus changes.

## Storage and save recovery

Storage UI consumes typed save-service findings. It never exposes platform
filenames, block counts, memory-card ports, or raw platform error codes as
domain identity.

Recovery findings may offer policy-approved actions such as retry, choose a
logical provider, repair or migrate, delete a confirmed corrupt slot, open a
platform storage-management surface, continue without saving, or cancel.

Formatting, deletion, migration, load, and save are separate application
transactions with request identities and durable results. A modal response only
requests the operation. It cannot report success before the save service
verifies the result.

Free-space and capacity projections use human-readable quantities supplied by
the platform adapter. Unsupported or indeterminate capacity is explicit and
cannot be represented as a fabricated number.

## Localization and text formatting

All visible text uses stable text identities and typed formatting arguments.
The localization service owns locale selection, fallback, plural rules, number
formatting, line breaking inputs, and text revision.

A missing required string is a validation error. Runtime fallback may use a
registered safe string and diagnostic identity, but widgets cannot search a
mutable global table by display text.

Tutorial, prompt, credit, and notification text is laid out by Slate and the
project typography policy. Hard-coded line counts, byte lengths, or platform
suffixes do not define wrapping.

## Frontend and in-game routers

Frontend and in-game flows use separate registered screen catalogs but share the
same navigation, menu, modal, input, focus, transition, and asset kernel.

The frontend router owns boot completion, main-menu commands, galleries,
options, save browsing, media, and transitions into gameplay. The in-game router
owns HUD, pause, mission loading, mission result, level result, map, tutorial,
phone-booth, reward purchase, save, and return-to-frontend projections.

Neither router owns mission, save, progression, vehicle, audio, or application
state. Each submits typed requests to the owning service and projects accepted
results.

## Pause and mission transitions

Pause requires an accepted application-lifecycle pause result. Opening a pause
widget cannot pause simulation by itself. Resume first closes blocking modals,
restores the accepted gameplay input context, and then releases the pause lease.

Mission-load, mission-complete, mission-failed, restart, abort, skip, level-end,
and return-to-frontend flows carry mission-session and stage revisions. A stale
mission observation cannot replace the current HUD or reopen a prior result
screen.

## HUD, overlays, and iris barriers

HUD visibility, tutorial text, letterbox, map, credits-after-media, and mission
overlays are registered projections with owner revisions. Visibility requests
are idempotent and cannot outlive their owner.

An iris or full-screen transition may serve as a visual readiness barrier, but
the lifecycle service owns the actual world or mode transition. Closing the iris
does not prove that a world, save, or mission load completed.

## Concurrency

Navigation, modal, save, media, asset, and application operations may complete
asynchronously. Every completion is accepted on the game thread through its
request identity and expected revisions.

The runtime serializes conflicting operations per layer and owner. Independent
notification or optional-asset operations may proceed concurrently when their
leases do not conflict.

Timeout has a terminal typed result. It does not imply native cancellation
succeeded. Late completion is observed, classified as stale, and prevented from
changing accepted UI state.

## Feature and mod overlays

A validated feature package may register screens, menu items, actions, modals,
and asset bundles through namespaced identities. Registration validates
ownership, destination references, layer permissions, localization, cook
availability, and removal fallback.

Feature removal cancels owned operations, removes history records, closes owned
modals, releases assets, and restores the nearest valid base-game screen. A mod
cannot replace the core navigation kernel or intercept unrelated actions.

## Diagnostics

Structured diagnostics include:

- flow, screen, navigation, modal, focus, asset, player, and feature identities;
- expected and observed revisions;
- source and destination screen identities;
- layer reservation and transaction state;
- command and response identity;
- asset bundle and load result;
- focus target and fallback result;
- stale-observation reason;
- save or platform finding identity; and
- terminal result and recovery action.

Logs never include private local routes, raw save payloads, proprietary asset
content, credentials, or user-entered text not required for a bounded finding.

## Failure behavior

The kernel fails closed for unknown screens, actions, items, responses, layers,
features, schemas, or revisions. It restores the latest accepted screen and
focus whenever safe.

A required screen that cannot be constructed enters a registered recovery or
blocked state. It cannot silently route to an unrelated neighboring screen.

A malformed modal cannot appear. A failed storage or settings operation keeps
the prior accepted state. A failed optional presentation bundle may use a
validated fallback without changing command availability.

## Validation

Validation proves:

- the viewport uses the Common UI routing base;
- every screen, layer, action set, destination, fallback, and feature owner
  resolves;
- every required bundle is discoverable and present in the target cook;
- every menu item has stable identity and a valid command or value domain;
- every modal has unique semantic responses and a safe default;
- every localization identity resolves for required locales;
- focus is reachable for keyboard, mouse, gamepad, touch, and accessibility
  traversal;
- no screen or widget owns save, progression, mission, settings, or application
  authority; and
- every asynchronous callback checks request and revision identity.

## Tests

Automated tests cover:

- first-screen activation and initial focus;
- push, replace, clear, preserve, and back history behavior;
- invalid and removed history records;
- duplicate navigation and duplicate response rejection;
- transition completion before and after cancellation;
- stale asset, media, save, animation, and controller completions;
- disabled, hidden, removed, and dynamically reordered menu items;
- value wrapping, bounded values, continuous adjustment, and commit rollback;
- deterministic input repeat across frame rates;
- pointer capture, touch activation, and focus-method switching;
- controller disconnect, reassignment, reconnection, and modal restoration;
- prompt safe defaults and destructive-action confirmation;
- storage retry, remediation, continue-without-save, and cancellation;
- frontend-to-gameplay and pause-to-gameplay transitions;
- feature registration and removal while history or a modal references it;
- reduced-motion transitions and screen-reader traversal; and
- empty, single-item, normal, and maximum supported screen catalogs.

## Invariants

- One flow revision has at most one committed primary screen.
- One local player has at most one blocking focus lease per layer.
- One navigation or modal request reaches exactly one terminal result.
- History contains restoration data, never live widget or domain pointers.
- A widget renders immutable snapshots and publishes typed commands only.
- Physical inputs, labels, file names, and child indexes are not domain
  identities.
- Required assets are ready before screen commit.
- Stale callbacks never mutate accepted state.
- Destructive prompts default to a safe response.
- Storage UI never claims success before the save service verifies it.
- Frontend and in-game routers share one kernel without sharing domain
  authority.
- Feature removal cannot leave an orphan screen, focus lease, modal, history
  record, or asset lease.
