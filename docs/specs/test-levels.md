# Test levels (`sf_level`)

Synthetic greybox levels for tests and visual checks: small authored spaces
(ground, boxes, ramps, stairs, poles) that load in a second, with named cameras
and spawn points. Uses: locomotion and traversal courses (context probes,
`docs/research/animation/01-faith-runner-movement.md` "greybox courses as
scripted-input fixtures"), physics checks (stacks, ragdolls on stairs), cloth
on slopes. The game's integration (scenes, viewer, Morrowind actors) is
`../bevyrrowind/docs/specs/test-levels.md`.

## Research and reuse (rules 7-8; 2026-10-18)

| option | status (Bevy 0.20, released 2026-10-08) | verdict |
|---|---|---|
| Bevy `DynamicScene` (`.scn.ron`) | works, but it is a reflected ECS dump: `Transform`s, asset handles by path, component type paths. No primitives ("stairs, 10 steps of 0.18 m"), verbose by hand, breaks on component renames, and it knows nothing of our floating-origin grid or physics bodies | no: wrong level of abstraction for authoring |
| BSN (`bsn!`) | 0.19 introduced the macro, 0.20 polished its syntax; `.bsn` **files** are "what's next" in the 0.20 notes, no loader yet | no (macro = Rust per level); revisit when `.bsn` assets land: a level could become a BSN asset of our primitives |
| Skein (glTF extras → reflected components; Blender add-on) | `bevy_skein` 0.6 supports Bevy 0.19; no 0.20 release yet (maintained, last push 2026-04) | later: the Blender path for art-heavy levels once it supports 0.20 |
| Blenvy | alpha, Blender add-on and crate must match, stalled on older Bevy | no |
| Engine practice | UE/Unity/Godot greybox with CSG/ProBuilder-style primitives and a "prototype grid" material; data-driven test maps (UE functional-test maps, GASP's traversal test level) | adopted: primitives + meter grid + named cameras |

**Decision:** a small serde/RON format of greybox primitives, Bevy-free
(`sf_level`: parse, validate, mesh), hand-authored and diffable, no Rust per
level. glTF/Skein later as a `Gltf { path }` primitive (render mesh and
trimesh collider from the file), not a second format.

## Format

```ron
(
    name: "flat-slopes",                 // kebab-case, the file stem
    description: "one line",
    cameras: [(name: "overview", eye: (x, y, z), target: (x, y, z))],   // first = default
    spawns: [(name: "start", at: (x, y, z), yaw: 0.0)],                  // feet + facing (deg, 0 = -z)
    objects: [
        (name: "slope-20", shape: Ramp(width: 4.0, rise: 2.0, angle: 20.0),
         at: (x, y, z), yaw: 0.0, material: Yellow, collide: true),
    ],
)
```

Meters, y up, x east, z south (Bevy's frame). `at` is the center of the
footprint at its bottom; `yaw` turns about +y in degrees. Shapes:

| shape | parameters | geometry |
|---|---|---|
| `Ground(size: (x, z))` | | 1 m thick slab, top at `at.y` |
| `Box(size: (x, y, z))` | | |
| `Ramp(width, rise, angle)` | angle in (0.5°, 89.5°) | wedge climbing toward local -z, run = rise / tan(angle), vertical back face |
| `Stairs(width, steps, rise, run)` | 1..=500 steps | solid stairs climbing toward local -z, only visible faces |
| `Cylinder(radius, height)` | | 24 sides |

Materials: `Floor` (default), `Grey`, `Dark`, `White`, `Orange`, `Yellow`,
`Green`, `Blue`, `Red`, `Purple`: flat colours the host multiplies over a
1 m prototype grid. Unknown fields are errors; validation lists every
problem at once (sizes > 0 and finite, ramp angle, step count, unique
camera/spawn/object names, cameras with eye ≠ target, at least one camera
and one object).

### Actors and props (TS1 part 2, 2026-10-18)

```ron
    objects: [
        (name: "crate", shape: Box(size: (0.6, 0.6, 0.6)), at: (3.0, 0.0, 0.0), dynamic: Some(15.0), interact: Some("open")),
    ],
    actors: [
        (id: "<host record id>", spawn: "a", outfit: Some(["<item id>", ...]),
         path: Some((points: [(x, y, z), ...], mode: PingPong, speed: 1.4, run: false, wait: 1.5)),
         action: Some((trigger: "<graph trigger>", every: 6.0, params: [("<graph float>", 3.0)]))),
    ],
```

- **Props:** `dynamic: Some(mass_kg)` makes a box a dynamic body (boxes only;
  compound props such as a legged table wait for a `Gltf` or compound
  primitive). `interact: Some(name)` is the interaction-point hook (P11),
  parsed and kept, unused.
- **Actors:** ids, outfit items and action names are the host's (opaque
  strings here); `spawn` must name a spawn. `path` modes: `Loop` (default),
  `PingPong` (ends are stops), `Once`; `wait` is the time at stops (a loop's
  first point, both ends, a one-way walk's end); `speed` m/s (default 1.4),
  `run` picks the run gait. The host maps this onto
  `sf_controller::PathFollower` (same modes). `action` fires `trigger` after
  setting `params`, every `every` seconds while the actor stands.
- `Level::bounds()` gives the objects' horizontal extent (hosts cover it with
  streaming content, so movers anywhere on the level are in range).

## Headless runner (course tests)

`crates/sf_level/tests/course.rs`, Bevy-free, the faith-runner model
(`docs/research/animation/01-faith-runner-movement.md` §9): a level's collide
objects become a `StaticWorld` (the same triangles as the render mesh), a
`KinematicMover` steps at 60 Hz, and **input is a function of controller
state** (`Sim::run(secs, |sim| Option<LocomotionIntent>)`; `None` ends the
script), not a timeline, so scripts survive tuning. Assertions are outcomes.
Built: `ledges` obstacles (row A heights, row B thin vs deep, the 3.5 m wall)
classified by `ContextProbes` against the table in locomotion.md "Context
probes as built", heights within 3 cm; ramp onto the first platform, feet on
top within 2 cm, `DropAhead` sees the 1 m gap at the platform edge.
`tests/levels/ledges.ron` is a copy of the game's level (the game's tests
fail if they drift). Promote `Course`/`Sim` to a module (feature-gated) when
a second crate needs them.

## Meshes

`Shape::mesh()` → `MeshData { positions, normals, uvs, indices }`:
flat-shaded, counter-clockwise outward, UVs in meters along each face (a 1 m
texture tile is a 1 m square, so the grid reads scale). The **same
triangles** are the static collider: what you see is what you collide with.
Tests: normals agree with the winding, sizes and slope angles match the
parameters, UVs are meters, the validation and unknown-field errors.

## Host contract

The host (an app or a headless test) owns spawning: one render mesh per
object, one `Fixed` mesh collider on the static layer per `collide` object
(the same layer world statics use, so the controller's ground and context
probes see it unchanged), the sky and lighting, and what spawns do.

## Open

- `Gltf { path }` primitive (Skein once on Bevy 0.20).
- Interaction points (doors, ladders, seats; P11): `interact` is the hook.
- Compound props (a table with legs) and cylinders as dynamic bodies.
- More course tests (vault/mantle execution once traversal actions exist,
  M4/M7; rotated copies of a course, research 01 `mesh_tests`).

## Lights and interiors (2026-10-21, light-cost lab)

```ron
    lights: Some((count: 8, layout: Grid(spacing: 4.0), at: (0.0, 3.5, 0.0), radius: 3.0, color: (1.0, 0.85, 0.6), orbit: 0.0)),
    interior: Some((ambient: (0.12, 0.11, 0.10))),
```

- `lights` (`LightRig`): `count` point lights generated from parameters, not hand-placed, so a perf sweep can
  step one variable from the command line. Layouts: `Grid(spacing)` (rows of `ceil(sqrt(count))`, centered on `at`)
  or `Ring(radius)` (a horizontal circle about `at`); `at.y` is the height. `radius` is the host's light size
  (meters; the host maps it to range and power as for its own lights); `orbit` (deg/s) turns every light about `at`
  (`LightRig::positions(t)`). Validation: count <= 1024, sizes > 0, all finite.
- `interior`: an enclosed level, no sky, sun or moon; the host shows it with this ambient (sRGB 0..=1).
