# Test levels in Bevyrrowind (TS1)

Synthetic greybox levels opened like any test scene. Format, primitives,
research and the reuse decision (RON now, Skein/glTF later, no BSN files in
Bevy 0.20): `../superfargoth/docs/specs/test-levels.md` (`sf_level`). This
file covers the game side. Owner go 2026-10-18 (ROADMAP "Next up", TS1).

## How it runs (built, ts1a)

- **Files:** `levels/<name>.ron`. Lookup: `levels/` in the current folder,
  then next to the exe, then the copies built into the exe; a `.ron` path
  works too. No Rust per level.
- **Entry points:** `bevyrrowind view level:<name> [view flags]`, or a
  `scenes.ron` entry with `level: "<name>"` instead of `cell` (`scene
  --list` marks them `[level <name>]`). `--camera <name>` picks one of the
  level's cameras; raw `ex,ey,ez,tx,ty,tz` still works.
- **One viewer, not a second app** (`apps/bevyrrowind/src/level.rs`): the
  level is its own worldspace `level/<name>` in the streamed viewer
  (`exterior.rs`). Its streaming index is an empty `sf_world::MemoryIndex`
  layered over the bake's (`LevelIndex`), so nothing of Vvardenfell streams
  and streaming settles at once. Everything else of the viewer comes along
  unchanged: sky at the scene's fixed hour (a worldspace without an interior
  environment shows the sky), clouds, fog, wind, physics, BRP
  (`bevyrrowind/status` reports `ready`), the F1 A/B keys, `--screenshot`,
  the free camera, `V` (throw a ball), `G` (collider wireframes). No
  terrain, sea or far field: those draw in the exterior worldspace only.
- **Greybox:** one mesh per primitive under the worldspace root (big_space
  grid via `to_grid`, the worldspace's render layer), a `StandardMaterial`
  per colour over a procedural 1 m prototype-grid texture (128², full mip
  chain, anisotropic) with meter UVs. Colliders: each `collide` object's
  same triangles as a `Fixed` `MeshShape` on `Layer::STATIC`, inserted into
  the level worldspace's physics world, exactly the body world statics get
  from their `collision.cook`, so the controller's ground and context probes
  and thrown bodies see it the same way.
- **Content still loads** (records, VFS, bake manifest): actors, walkers,
  items and scripts work in a level, which part 2 needs. Measured: content
  0.34 s (warm cache), see "Load time" below.

## Levels

| level | what | scene |
|---|---|---|
| `flat-slopes` | 80 m floor; 10/20/30/40° ramps (2 m rise) onto a 2 m plateau; stairs 0.15/0.30 (12), 0.18/0.28 (10), 0.25/0.22 (8) with landings; a 2 m post | `level-flat-slopes` |
| `ledges` | boxes 0.3/0.5/0.9/1.2/1.6/2.0/2.5 m (1 × 1.5 m); thin 0.3 m walls at 0.9/1.2 m vs a deep 1.2 m block (vault vs mantle); 1 m platforms with 1/2/3/4 m gaps; a 0.3 m beam (top 1 m, 8 m long); a 3.5 m wall; a pole | `level-ledges` |

Heights follow `locomotion.md` "Context probes as built": thresholds scale
with the capsule height h (vault ≤ 0.7 h and ≤ 0.33 h deep, mantle ≤ 1.5 h,
climb ≤ 2.5 h); colours band the heights (green, yellow, orange, red), the
probe decides.

## Load time

Dev build, warm file cache (2026-10-18): content (records, VFS, bake manifest) 0.32-0.37 s; greybox spawn 1-2 ms
(13-22 objects, 344-494 triangles, one collider each); **ready 2.1-2.8 s after process start** (window, render
and pipeline setup dominate; logged as `level <name> ready <s> after start`).

## Actors, walk mode, props (built, ts1b, 2026-10-18)

- **Actors** (`level/actors.rs`): each `actors` entry is resolved against
  the records (`NPC_` or `CREA`; unknown ids are skipped with a warning) and
  becomes an actor `sf_world::Placement` at its spawn in the level's
  `MemoryIndex` page (`build_pages` in `LevelIndex::new`), so the streamed
  world spawns it like any cell's NPC. A level spawner
  (`LevelActorSpawner`, replacing `CharacterSpawner` for the level) dresses
  it with `CharacterState::worn` = the outfit and tags it `LevelActor`;
  `walkers.rs` gives it its `Mover`; the level's `script` system then drops
  the `AI_W` scaffold and avoidance and adds the script: an
  `sf_controller::PathFollower` (loop, back-and-forth, once; walk or run)
  writing `MoveIntent`, and the periodic action (graph floats + a trigger,
  e.g. `fidget` with `idle_variant`).
- **Coverage markers:** cells with no content count as unloaded, and a mover
  there parks (the processing range). The level index adds invisible
  `Marker` placements every 64 m over `Level::bounds()`, so every streaming
  cell under the level is in the directory and goes active.
- **Walk mode** (`level/walk.rs`): the player is a level actor too
  (Morrowind's `player` record at the `start` spawn, else the first spawn).
  `U` toggles fly/walk: WASD in the camera's heading frame, Shift runs; the
  free camera keeps the mouse look and sits 3.5 m behind a 1.6 m pivot.
  Scaffold: no camera collision, no jump (the player controller is M7).
  Verified (ts1c, 2026-10-19, BRP): the player spawns dressed at `start`,
  W walks it 2.6 m in 2 s (1.6 m/s), Shift+W asks 4.5 m/s, the camera follows;
  the log prints `walk: player starts/stops at ..., wants ..., measured ...`.
- **Props:** `dynamic: Some(mass)` boxes are `BodyKind::Dynamic` bodies on
  `Layer::PROP` (box shape about the center, the render mesh shifted to
  match) with `PhysicsBody` for the pose write-back; `LevelProp` keeps the
  `interact` hook for P11.
  Verified (ts1c): `V` balls (3 kg, 12 m/s) shove the 15 kg crates off their
  stack; with `G` the colliders sit on the meshes. They shove rather than
  topple: a heavier throw or lighter crates if the owner wants a knock-over.
- **`--camera`:** `level::pick_camera` takes a level camera by name (tested);
  `view` lets a repeated flag override the earlier one (`args_override_self`),
  so `scene <name> --camera ...` also replaces a scene's own camera (before:
  a clap error). ts1b's "course camera not applied" did not reproduce: BRP
  status reports (14, 2.5, 3) -> (4, 1, 2) for `scene level-robe-walk --camera course`.
- **No land:** a level worldspace never gets `TerrainCamera` (exterior
  setup, `doors::worldspace_look`): before, the terrain streamed
  Vvardenfell's patches around the camera and each new patch drew for a
  frame before `terrain_layers` moved it to the exterior's layer (black
  flashes, TS-1 failed 2026-10-18).
- **Headless runner:** framework side (`sf_level/tests/course.rs`, see the
  framework spec); `level.rs` tests check its `ledges.ron` copy matches ours.

| level | what | scene |
|---|---|---|
| `robe-walk` | the Balmora temple priest (own robe) walks a flat loop and fidgets at its corner; Fargoth (common robe) walks and Elone (extravagant robe) runs back and forth over a 15° ramp, a 1 m plateau and stairs (5 × 0.20/0.32); a table (30 kg) and three stacked crates (15 kg) | `level-robe-walk` (cloth PH-3) |

Leftovers: compound props (a table with legs), camera collision in walk mode,
a `--walk` start flag (walk mode is a key only; BRP `send_keys` U).
