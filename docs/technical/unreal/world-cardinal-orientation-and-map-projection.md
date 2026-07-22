# World cardinal orientation and map projection

- Status: Active
- Last reviewed: 2026-07-22
- Runtime owner: `SharWorld`

## Canonical contract

The connected SHAR world uses one orientation contract in Unreal world space:

| Meaning | Unreal world value |
| --- | --- |
| North | `+X` |
| East | `+Y` |
| South | `-X` |
| West | `-Y` |
| Up | `+Z` |
| Map center | `(0, 0, 0)` centimeters |
| Sea level | `Z = 0` centimeters |

The world, Landscape, imported structural guide, missions, minimap, compass,
sun path, route guidance, and spatial content must use this contract. They must
not introduce a second yaw offset or per-feature interpretation of north.

The canonical harbor and port region belongs north of the map center. Content
validation may use a stricter positive northing threshold, but a harbor at the
origin or in negative world X is invalid.

## Runtime authority

`FSharWorldOrientationDefinition` is embedded in `USharWorldDefinition` and is
validated with every connected-world definition. The accepted definition uses
exactly the axes, origin, sea level, and positive harbor threshold listed above.
A mirrored or rotated orientation is rejected.

`USharWorldOrientationLibrary` exposes the same contract to C++ and Blueprint.
Its pure functions provide:

- canonical north, east, south, west, and up vectors;
- normalized bearings in the range `[0, 360)`;
- bearings between two world locations;
- headings derived from Unreal yaw;
- signed shortest heading deltas;
- eight-way cardinal classification and localized labels;
- northing and easting relative to the world origin;
- north-up logical-map and screen-map coordinates;
- sea-level classification; and
- northern-harbor placement validation.

Consumers use this API rather than duplicating `atan2`, yaw wrapping, axis
swaps, or screen-Y inversion.

## Bearings and headings

Bearings increase clockwise from north:

| Bearing | Direction | World vector |
| --- | --- | --- |
| `0` degrees | North | `(1, 0, 0)` |
| `45` degrees | Northeast | normalized `(1, 1, 0)` |
| `90` degrees | East | `(0, 1, 0)` |
| `135` degrees | Southeast | normalized `(-1, 1, 0)` |
| `180` degrees | South | `(-1, 0, 0)` |
| `225` degrees | Southwest | normalized `(-1, -1, 0)` |
| `270` degrees | West | `(0, -1, 0)` |
| `315` degrees | Northwest | normalized `(1, -1, 0)` |

For locations `From` and `To`, the bearing is derived from world northing and
easting:

```text
northing delta = To.X - From.X
easting delta  = To.Y - From.Y
bearing        = degrees(atan2(easting delta, northing delta))
```

Coincident horizontal locations have no bearing and are rejected. Vertical
displacement does not change a compass bearing.

Unreal yaw already follows the same horizontal convention for the canonical
world: yaw `0` faces `+X`, yaw `90` faces `+Y`, and normalized yaw is the
compass heading. Signed heading deltas remain in the shortest-turn interval
from `-180` through `180` degrees.

## North-up map projection

`FSharNorthUpMapCoordinate` preserves geographic semantics:

```text
easting  = World.Y - MapCenter.Y
northing = World.X - MapCenter.X
```

`FSharNorthUpScreenCoordinate` converts that logical map point to conventional
screen coordinates:

```text
horizontal = easting
vertical   = -northing
```

Therefore, north appears at the top of a minimap without rotating the world or
Landscape. UI may scale, translate, clip, or normalize these coordinates, but
must not swap the axes again.

## Sea level and harbor placement

Sea level is a world-space plane at `Z = 0`. A point with non-negative Z is at
or above sea level; a point with negative Z is submerged. Terrain, water,
mission markers, and imported source evidence use this same datum.

The canonical harbor check requires positive world northing. The overload with
an explicit minimum northing lets authored content require a larger northern
margin while preserving the same axis contract.

The harbor rule does not require a particular easting. Ports may extend east or
west along the northern coast as long as their authoritative placement remains
north of the configured threshold.

## Canonical FBX import

The structural-guide exporter must bake orientation into geometry. The imported
actor must use:

```text
Location = (0, 0, 0)
Rotation = (0, 0, 0)
Scale    = (1, 1, 1)
```

The northern harbor in the source must appear toward Unreal `+X`. The exporter
must not leave a negative scale, mirrored parent transform, or runtime yaw
correction. Sea-level geometry must align with world `Z = 0`.

The Landscape itself is not rotated or mirrored to compensate for source-data
errors. A corrected guide is imported at identity, and authored terrain is
sculpted against that canonical reference.

## Consumer rules

- `SharWorld` owns orientation and map-projection math.
- `SharUI` consumes north-up coordinates for minimap and compass presentation.
- `SharMissions` consumes bearings and northing/easting for objectives and route
  guidance.
- day-and-night presentation interprets sunrise as east and sunset as west.
- import and editor validation must call the canonical orientation and
  northern-harbor validators before accepting authored evidence.
- save data stores world positions or stable identities, not a private compass
  offset.

## Verification

`SHAR.World.Orientation` automation covers:

- all four cardinal and four intercardinal bearings;
- abbreviations and unit vectors;
- bearing normalization and heading wraparound;
- north-up logical and screen projections;
- sea-level classification;
- northern-harbor acceptance and rejection; and
- rejection of a mirrored `USharWorldDefinition` orientation.
