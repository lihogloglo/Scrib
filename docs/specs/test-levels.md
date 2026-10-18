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

## Not built yet (TS1 part 2, ts1b)

- **Actors by id** (`actors: [(id: "...", spawn: "start", outfit: [...], path: [...], action: ...)]`):
  put them as actor placements into the level's `MemoryIndex` page
  (`sf_world::build_pages` with the placements instead of `std::iter::empty()`
  in `LevelIndex::new`), so the existing actor spawner dresses them; scripted
  path/action loops drive their `MoveIntent`.
- **Props and interaction points:** dynamic bodies (table, chairs, stacks)
  via the item path or direct `BodyDesc::new(Dynamic, ...)`; doors/ladders later.
- **Headless scripted-input runner:** load a level into `sf_physics`
  (`Shape::mesh()` → `MeshShape`, no Bevy), drive a `KinematicMover` with an
  input script, assert positions (traversal courses, research 01).
- **Player walk mode:** the viewer has only the free (fly) camera; walking
  needs a player `Mover` + camera follow (not in the app yet).
