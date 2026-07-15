# Common UI front end and progress projection

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Boot, main menu, save selection, scrapbook, options, and progress UI

## Context

The front end must expose new game, resume, load, scrapbook, options, credits,
calendar themes, and idle presentation across desktop and Android. It must also
project campaign and level completion without becoming a second save or
progression authority.

Unreal Common UI provides cross-platform widgets, style data, input routing,
focus management, controller glyphs, and cardinal gamepad navigation. Unreal
subsystems provide managed lifetimes for global and local-player services. The
project needs one fixed composition of those native facilities.

## Decision

The front end uses Common UI. `UCommonGameViewportClient` is the viewport input
routing base, and the navigation stack contains four fixed layers: boot, primary
screen, modal, and notification. Every screen derives from the project's C++
Common Activatable Widget base and receives immutable view data. Blueprint and
widget assets own layout, animation, styling, and presentation only.

`USharUiNavigationSubsystem`, a `UGameInstanceSubsystem`, owns the registered
screen catalog, typed navigation transactions, bounded restoration history,
layer reservations, asset leases, and the accepted screen revision. Frontend
and in-game routers use separate screen catalogs while sharing this kernel; they
do not exchange global integer messages or live widget pointers.

`USharFrontendSubsystem`, also a `UGameInstanceSubsystem`, owns boot state,
main-menu commands, accepted save-slot summaries, new-game requests, resume
selection, load requests, scrapbook entry, options entry, credits entry,
calendar-theme selection, and transitions into or out of gameplay.

`USharFrontendInputSubsystem`, a `ULocalPlayerSubsystem`, maps keyboard and
mouse, gamepad, and touch input into one semantic Common UI action set. Common
UI action data and controller data assets own user-interface action and glyph
projection. Widgets do not inspect platform-specific key codes, infer ownership
from controller indexes, or choose gameplay behavior.

Screen definitions and heavy presentation families are Asset Manager primary
assets with named bundles. Navigation retains streamable handles for the
accepted screen lease and rejects completion from a cancelled or superseded
request. Required bundles, view data, actions, and focus must validate before a
destination is committed.

Messages and prompts are typed modal transactions with semantic response
identities and safe defaults. Save, storage, settings, media, and application
operations remain owned by their application services; a modal response or
animation completion cannot claim that an operation succeeded.

The progression and campaign services calculate every completion value. The
front end and pause-menu progress screen receive read-only projections
containing category counts, exact progress, one-decimal display values, rewards,
and availability. A widget cannot infer completion from visible rows or mutate a
save slot.

Calendar and idle-menu scenes are presentation policies. They may replace menu
meshes, materials, lighting, audio, ambient characters, and animation but cannot
change command availability, save state, catalog identity, or gameplay.

## Consequences

- New game, resume, load, scrapbook, options, and credits have one command
  contract across all supported input adapters.
- Resume is available only when a complete accepted slot has a valid resume or
  campaign destination.
- Load enumerates validated logical slots rather than platform filenames or
  memory-card concepts.
- Scrapbook reads the same progression state as gameplay and cannot diverge from
  the level-progress screen.
- Options persist device-local configuration separately from portable gameplay
  progression.
- Common UI owns focus, action routing, controller glyphs, and navigation
  presentation; C++ services own commands and state.
- Calendar themes and idle gags are deterministic presentation and always have a
  standard fallback.
- Android uses the same screen and command model with touch-safe layout and
  input
  adapters.
- Front-end presentation may be replaced without migrating save or campaign
  identities.

## Rejected alternatives

- Separate desktop, gamepad, and Android menu logic.
- Widget-owned save discovery or progression calculation.
- Platform filenames, memory-card slots, or controller buttons as domain
  identity.
- Level progress calculated from the number of visible UI rows.
- Calendar themes that alter gameplay or command availability.
- A monolithic menu Blueprint that performs loading, saving, options, and
  campaign transitions directly.
