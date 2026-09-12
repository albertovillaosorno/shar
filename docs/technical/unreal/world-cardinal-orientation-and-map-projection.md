# World cardinal orientation and map projection

- Status: Active
- Last reviewed: 2026-09-12
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

World construction, missions, minimap, compass, route guidance, and spatial
content use
this contract. Consumers must not introduce a second yaw offset or per-feature
interpretation of north.

## Native orientation boundary

Source world geometry remains in its declared source basis in normalized JSON.
The native world construction plan applies the source-to-Unreal basis exactly
once to geometry, placement, collision, locators, and related spatial evidence.

Actor transforms remain identity unless source evidence explicitly owns a
placement transform. Importer front-axis switches, export-root reflections, yaw
corrections, height offsets, UV mirrors, and map translations are not coordinate
authority.

## Runtime authority

`FSharWorldOrientationDefinition` is embedded in `USharWorldDefinition` and is
validated with every connected-world definition. `USharWorldOrientationLibrary`
provides canonical cardinal vectors, bearings, heading deltas, northing,
easting, map projection, and labels.

Consumers call this API instead of duplicating axis swaps, yaw wrapping,
`atan2`, or screen-Y inversion.

## Bearings and headings

Bearings increase clockwise from north:

| Bearing | Direction | World vector |
| --- | --- | --- |
| `0` degrees | North | `(1, 0, 0)` |
| `90` degrees | East | `(0, 1, 0)` |
| `180` degrees | South | `(-1, 0, 0)` |
| `270` degrees | West | `(0, -1, 0)` |

For locations `From` and `To`:

```text
northing delta = To.X - From.X
easting delta  = To.Y - From.Y
bearing        = degrees(atan2(easting delta, northing delta))
```

Vertical displacement does not change a compass bearing. Coincident horizontal
locations have no bearing.

## North-up projection

Logical map coordinates preserve geography:

```text
easting  = World.Y - MapCenter.Y
northing = World.X - MapCenter.X
```

Screen coordinates use:

```text
horizontal = easting
vertical   = -northing
```

UI may scale, translate, clip, or normalize map coordinates. It must not rotate
or reposition world geometry to make the projection work.

## Verification

Automation covers cardinal and intercardinal bearings, normalization, heading
wraparound, north-up projection, and rejection of mirrored runtime orientation.
Native construction validation additionally confirms identity actor transforms
and the single declared source-to-Unreal basis conversion.
