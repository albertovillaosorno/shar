# Native flying-hazard actors and StateTree execution

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Wasp-camera, flying-hazard, projectile, and UFO runtime architecture

## Context

Flying hazards combine low-volume but high-fidelity behavior: three-dimensional
movement, authored altitude, target perception, attack and evasion decisions,
projectile collision, shields, tractor beams, animation, rewards, persistence,
and camera presentation. They are not ambient crowd members. Each active hazard
has player-visible state and may own a counted progression identity.

A global manager with hand-managed arrays, behavior subclasses, and direct
scene-graph access would make streaming, collision, cancellation, and save state
implicit. Treating these hazards as Mass entities would also remove the native
Actor, component, collision, and animation lifecycle required by their bespoke
behavior.

## Decision

Each flying hazard is an Unreal `APawn` with a dedicated movement component, an
AI controller, AI Perception, and a `UStateTreeComponent`. StateTree owns
hierarchical behavior selection and task lifetime. C++ domain and application
services remain authoritative for identity, progression, rewards, persistence,
and damage results.

`USharFlyingHazardDefinition` primary data assets provide immutable archetype,
movement, perception, attack, defense, projectile, presentation, reward, and
spawn policies. `USharFlyingHazardSubsystem` validates definitions, resolves
stable identities, coordinates streaming and pooling, and publishes typed
results. It does not tick every hazard or duplicate Pawn lifecycle.

EQS supplies bounded candidate locations for attack, observation, and evasion.
A custom `USharFlyingHazardMovementComponent` performs three-dimensional swept
movement, ground-clearance enforcement, orientation, and deterministic fallback
when a query result becomes blocked. AI Perception supplies sight, damage,
hearing, touch, and custom gameplay stimuli. Raw global-event subscriptions are
not an awareness mechanism.

Projectiles are pooled Unreal actors with explicit owner identity, projectile
definition, sweep collision, lifetime, and one authoritative impact result.
Shield and tractor-beam behavior are separate components with explicit state;
they are not encoded through model animation state or collision side effects.

Mass Entity remains the ambient-population solution. It is not used for wasp
cameras, UFO encounters, boss hazards, or their projectiles.

## Consequences

- Every counted hazard retains a stable Actor and save identity while loaded.
- StateTree replaces behavior-subclass arbitration without becoming the owner of
  progression, rewards, or persistence.
- AI Perception and EQS replace broad global event listening and ad hoc waypoint
  searches with native, bounded queries.
- Swept movement and projectile collision prevent frame-rate-dependent
  tunneling.
- Pooling is an implementation detail; recycled instances cannot retain the
  previous owner's identity, target, reward, or reservation state.
- Wasp-camera destruction, shield loss, projectile penalties, and UFO damage are
  typed domain transactions and are committed exactly once.
- Camera presentation is requested through the camera subsystem. A hazard cannot
  replace or restore the active camera directly.
- Streaming may unload presentation actors without losing persistent destruction
  or progression state.

## Rejected alternatives

- Recreating one singleton actor manager with fixed arrays and manual banks.
- Representing bespoke flying hazards as Mass entities.
- Selecting behavior through dynamic-cast order or first-registered listeners.
- Using animation state as combat, shield, or persistence authority.
- Performing world-wide physics searches every frame for every hazard.
- Letting projectiles, collision callbacks, or visual effects grant rewards
  directly.
- Letting hazards switch the player camera without a camera-policy decision.
