# Mass Entity ambient population

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Ambient pedestrians and named non-story placements

## Context

Every level needs bounded sidewalk population, level-specific character groups,
reactive behavior, conversations, collision avoidance, streaming, and
presentation scaling. Some named characters are persistent, talkable, mission,
driver, race-host, cinematic, or gag placements and therefore cannot be treated
as disposable crowd instances.

A hand-spawned actor pool would duplicate Unreal's crowd, representation, State
Tree, Smart Object, and World Partition facilities. Treating every named
appearance as a unique character would duplicate identity and dialogue state.

## Decision

Ambient pedestrians use Mass Entity and Mass Gameplay. Repository-owned Mass
traits define canonical archetypes, level population groups, path movement,
avoidance, look-at behavior, threat response, representation LOD, StateTree,
signals, and Smart Object participation.

`USharAmbientPopulationSubsystem`, a world subsystem, consumes deterministic
population-zone definitions after the owning World Partition cell and Runtime
Data Layers are active. It selects archetypes from explicit weighted groups
and a stable session seed. Budgets are platform and quality presentation
policies;
they cannot remove required named or gameplay-relevant placements.

Named characters retain one canonical character definition. A disposable ambient
appearance may use a Mass entity. A talkable, mission, race-host, driver,
cinematic, gag, or save-relevant placement is promoted to or authored as a
repository-owned actor representation with stable placement identity.

## Consequences

- Generic population density scales through Mass representation LOD rather than
  separate gameplay rules.
- Spawn, despawn, and representation changes never create character identity.
- Named placements survive streaming and representation changes through stable
  placement records.
- Pedestrians look toward nearby players, avoid traffic, react to horns and
  violence, evade imminent vehicles, fall from accepted impacts, recover when
  safe, and may enter bounded ambient conversations.
- Mission and interaction reservations override ambient behavior.
- Off-screen removal cannot delete a mission target, active conversation, Smart
  Object reservation, or accepted save state.
- Low quality may reduce disposable population and representation complexity but
  cannot remove required mission, interaction, or narrative characters.
- Population generation is deterministic for equivalent level, layer, seed,
  policy, and streaming state.

## Rejected alternatives

- Spawning arbitrary character actors near the camera every frame.
- Using filenames or skeletal meshes as character identity.
- Keeping every ambient pedestrian as a full actor at every distance.
- Allowing LOD or device profile changes to remove gameplay-relevant characters.
- Duplicating one named character for each ambient, mission, driver, or gag
  role.
- Despawning entities while they own active interaction or mission state.
