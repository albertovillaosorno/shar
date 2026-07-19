# Vehicles, handling, damage, and phone booths

- Status: Active
- Last reviewed: 2026-07-18

## Native asset set

Every drivable vehicle has one `SharVehicle` definition and one or more complete
vehicle presentations. The default native implementation uses Chaos Vehicles and
a Skeletal Mesh for moving wheels, suspension, steering, doors, and authored
moving parts. Non-drivable decorative vehicles use Static Mesh definitions.

## Canonical placement

<!-- markdownlint-disable MD013 -->
```text
/Game/SHAR/Data/Vehicles/<id>/DA_Vehicle_<id>
/Game/SHAR/Data/Vehicles/<id>/DA_VehiclePresentation_<id>_<variant>
/Game/SHAR/Art/Vehicles/<id>/Meshes/SK_Vehicle_<id>_<variant>
/Game/SHAR/Art/Vehicles/<id>/Physics/PHYS_Vehicle_<id>_<variant>
/Game/SHAR/Art/Vehicles/Shared/Animations/ABP_Vehicle_<vehicle_family>
/Game/SHAR/Art/Vehicles/<id>/Damage/<damage_asset>
/Game/SHAR/Art/Vehicles/<id>/Materials/MI_Vehicle_<id>_<surface_role>
/Game/SHAR/Art/Vehicles/<id>/Textures/T_Vehicle_<id>_<surface_role>_<texture_role>
```
<!-- markdownlint-enable MD013 -->

## Normalized source package

A drivable vehicle package contains one canonical binary FBX 7.7 skeletal scene,
external normalized textures, a vehicle-rig manifest, physics and tuning
records, damage and presentation records, optional animation packages, and one
import plan. A decorative vehicle package may use a static-mesh FBX and an
explicitly non-drivable definition.

## Coordinate and mesh contract

Final vehicle assets use centimeters, positive X forward, positive Z up. The
vehicle root is centered on the longitudinal centerline at ground level beneath
the center-of-mass reference. Import transforms are applied. Wheel radius,
wheelbase, track width, suspension travel, seat transforms, and clearance use
real centimeter values.

A drivable rig profile maps semantic roles for chassis, each wheel, steering,
required doors, lights, exhaust, camera anchors, seats, entry points, tow or
hardpoints, and damage sections. Runtime code does not search arbitrary bone or
node names.

## Geometry and LOD

The chassis and moving parts preserve watertight visible surfaces, stable
normals, material semantics, and collision separation. `vehicle_standard_v1`
requires:

<!-- markdownlint-disable MD013 -->
| LOD | Triangle target relative to LOD0 | Required preservation |
| :--- | :--- | :--- |
| 0 | 100 percent | Full body, cabin, wheels, lights, doors, and damage anchors |
| 1 | at most 65 percent | Silhouette, wheel shape, windows, and major trim |
| 2 | at most 35 percent | Silhouette, wheel motion, lights, and color identity |
| 3 | at most 15 percent | Traffic-distance silhouette and emissive identity |
<!-- markdownlint-enable MD013 -->

A vehicle presentation declares whether doors, hood, trunk, suspension,
steering, or special parts are animated. An absent capability is explicit and
not inferred from a missing source node.

## Physics definition

The vehicle definition references explicit native profiles for:

- mass and center of mass;
- wheel class, radius, width, friction, and load behavior;
- suspension travel, spring, damping, and anti-roll policy;
- engine or motor torque curve;
- transmission and differential;
- steering response and speed sensitivity;
- braking, handbrake, and traction assistance;
- aerodynamic drag and downforce;
- collision, rollover, recovery, and reset;
- damage thresholds and disabled-state behavior;
- AI, traffic, pursuit, and network prediction policy.

Player-facing speed, acceleration, toughness, and handling ratings are derived
presentation metadata. They never replace real physics parameters.

## Materials and damage

Surface roles include body paint, glass, tire, wheel, metal, interior, light
lens, emissive light, license or decal, dirt, and damage overlays. Damage is
implemented through typed state and native presentation assets, not raw pointers
to alternate textures. The definition may select material parameters, geometry
swaps, breakable parts, Niagara effects, audio, and handling degradation by
damage state.

## Seats and character handoff

Each seat declares identity, occupancy role, compatible character profile, entry
and exit transforms, animation choreography, camera profile, input policy, and
failure recovery. Vehicle code never teleports a character into a seat based
only on a bone name.

## Vehicle catalog and phone booths

The menu shows the complete vehicle catalog with locked, purchasable, reward,
mission-only, unavailable, and mod-owned states. Persistent vehicle selection
and retrieval occur through phone-booth Smart Objects.

A phone-booth transaction:

1. obtains exclusive interaction ownership;
1. opens the catalog filtered by progression and context;
1. validates ownership, mission restrictions, world support, and spawn capacity;
1. loads the selected vehicle definition and bundles;
1. reserves a safe spawn and replacement route;
1. spawns or retrieves the vehicle;
1. transfers declared persistent state;
1. removes or stores the previous ordinary vehicle according to policy; and
1. commits the active vehicle identity.

Failure restores the previous state and never charges currency or consumes a
reward. Missions may force a temporary vehicle without granting persistent
phone- booth ownership.

## Mod extension

Mods may add vehicle definitions, presentations, physics profiles, damage
profiles, AI behavior, phone-booth categories, and self-hosted multiplayer
policies. Base vehicle identities remain stable when only art or handling is
replaced.

## Validation

Publication rejects invalid wheel semantics, impossible dimensions, missing
Physics Assets, mismatched Skeletons, non-positive mass, invalid torque or gear
curves, duplicate seats, missing entry transforms, undeclared moving parts,
material-role drift, collision penetrating the reference ground plane, or native
read-back that differs from the plan.
