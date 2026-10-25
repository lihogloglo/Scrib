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
- **One viewer, not a second app** (`apps/bevyrrowind/src/level.rs` CLI entry; viewer in `crates/mw_exterior_bevy/src/level.rs`): the
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

## Actors, player, props (built, ts1b, 2026-10-18)

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
- **Player:** the game's own (`player.rs`, ROADMAP 2.1): F11 drops it under the camera
  (third person, jump, camera collision). Level scenes with a camera start on the free
  camera. The `level/walk.rs` U walk mode (a `player` level actor at `start`) is retired
  (2026-10-25: redundant, and U is the dialogue dev key).
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
| `anim-lab` | animation lab, six areas: ledges 0.5/1/1.5/2 m + 3 m wall (player start); props (crate row an NPC brushes, stack, light/heavy boxes, table); walk loop (12 m walker, 24 m runner); 10/20/30° ramps + 0.15/0.25 m stairs to a 2 m plateau with walkers; 3 m / 6 m drops over a 15° slope (walker steps off, `@impact` knockdown dummy); idles (three races fidgeting, hit and knockdown dummies, guar, mudcrab) | `level-anim-{overview,loop,stairs,ledges,props,drop,idles}` (AL-1..7) |

Leftovers: compound props (a table with legs),
a start flag for player mode in level scenes (F11 only; BRP `send_keys` F11).

## Light lab (built, lightlab, 2026-10-21)

Controlled cost model for lights and shadows (owner 2026-10-21: one variable at a time, fixed camera; real scenes stay
pass/fail gates). `levels/light-lab.ron`, scene `level-light-lab`, suite scenario `light_lab`.

- **Room:** closed 20 x 20 x 5 m (walls, ceiling), a 4 x 4 grid of pillars (r 0.3, 2.5 m) and boxes (1 x 1.2 x 1 m),
  two NPCs walking an 8 m loop (movers; `--off actors` drops them). Cameras `wide` (eye height, whole room), `close`, `top`.
- **Interior:** the level's `interior` registers a `CellEnvironment` (no sky) for its worldspace, so
  `doors::worldspace_look` treats it exactly as a Morrowind interior cell (cell ambient, sun and moon off its layer).
- **Lights:** the level's `lights` rig (`sf_level::LightRig`) spawns through `mw_assets::lights::spawn_light`, the
  lamp path: `LightCalibration` range (radius x `--range`) and power, `BudgetedShadow`, the worldspace's layer, so the
  point-shadow budget, contact shadows and the shadow cache treat them as lamps. Default 8 lamps, grid 4 m, 3.5 m up,
  radius 3 m (range 12 m). Differences from game lamps: steady (no flicker), no lamp mesh, no `AttachLight` parent.
- **Flags:** `--lab-lights N`, `--lab-ring D` (ring of radius D about the room center), `--lab-radius R`,
  `--lab-orbit DEG_S` (all lights orbit the center: carried torches; moving lights defeat the shadow cache). With the
  existing `--point-shadows N`, `--range`, `--off contactshadows|shadowcache|actors`, `BEVYRROWIND_SHADOW_FILTER`.
- **Sweeps:** `perf sweep` (docs/perf.md "Light lab cost model").
