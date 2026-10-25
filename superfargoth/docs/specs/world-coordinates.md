# World coordinates

Status: agreed (2026-10-11)
Crates: `sf_coords` (new, Bevy-free), `sf_world_bevy` (big_space glue)

## Goal

Every position in every system (rendering, physics, streaming, records,
saves, bake products) is exact to well under a millimeter anywhere in a
world hundreds of kilometers across, and is stored in one representation.
This settles WISHLIST early decision 1: grid coordinates from the first
exterior commit.

Non-goals: planetary curvature; nested moving grids (ships, vehicles with
their own frame) are possible later with big_space's nested grids, not
designed now; changing Bevy's f32 `Transform`.

## Canonical technique

**Integer grid cell + f32 offset, rendered relative to a floating origin**
(Thorne, *Using a floating origin to improve fidelity and performance of
large, distributed virtual worlds*, 2005). UE5's Large World Coordinates
reach the same rendering result with f64 positions plus per-tile render
offsets; we get it without f64 in hot paths. f64 is used only for
conversion and long-range arithmetic.

Implementation: [`big_space`](https://github.com/aevyrie/big_space)
(MIT/Apache). **0.14.0-rc.0, published 2026-10-08, targets Bevy 0.20**
(0.13 is Bevy 0.19). Types we use: `BigSpace`, `Grid`, `CellCoord`,
`FloatingOrigin`, `Stationary` (0.13+: skips recentering and propagation for
entities that don't move), and the `i32` precision feature.

Deliberate deviations:
- We don't use big_space's spatial hashing or partitions for streaming. The
  streaming index exists before any entity does (streaming.md). Hashing
  plugins stay off unless a gameplay system needs neighbor queries.
- Stream-cell entities, which parent every static placement, are
  `Stationary`, so their high-precision propagation costs nothing per frame
  until the origin changes cell (see "Implementation notes").

## Conventions

| Item | Value | Why |
|---|---|---|
| Engine space | right-handed, Y up, −Z forward, meters (Bevy) | Adapters convert source units once, at import. |
| Grid cell edge | `GRID_CELL = 256.0` m, one framework constant | Power of two. Same value in every worldspace, so a saved position means the same thing everywhere. |
| Cell index | `i32` × 3 (big_space feature `i32`) | ±5×10¹¹ m. `i16` would cap at ±4,000 km, too tight to rule out. |
| Local offset | f32, canonical range `[-128, 128)` per axis | Worst-case ulp ≈ 15 µm. |
| Switching threshold | 32 m | Hysteresis: objects straddling a border don't flip cells every frame. |
| Derived partitions | power-of-two multiples or divisors of 256 m, aligned to the same origin | Streaming cells (128 m), terrain tiles (256 m · 2^k), HLOD clusters: containment is integer shifts. |

Cells are 3D (big_space's model). Worlds are mostly flat, so `cell.y` is
usually 0 or ±1. 2D columns would save nothing and would rule out tall and
flying content.

## Data model (`sf_coords`, Bevy-free)

Depends only on `glam` (the version `bevy_math` uses) and `serde`. It
changes rarely, so many crates may depend on it (ARCHITECTURE §8b rule 3).

```rust
pub const GRID_CELL: f32 = 256.0;

/// A position in one worldspace. Canonical: each `local` axis in [-128, 128).
pub struct WorldPos { pub cell: IVec3, pub local: Vec3 }

impl WorldPos {
    pub fn from_f64(p: DVec3) -> Self;            // import, tools
    pub fn to_f64(self) -> DVec3;
    pub fn offset(self, d: Vec3) -> Self;        // renormalizes
    pub fn delta(self, to: Self) -> DVec3;       // exact: i64 cell diff, then f64
    pub fn partition(self, shift: i32) -> IVec3; // cell of the 256·2^shift grid (shift may be negative)
}

pub struct WorldTransform { pub pos: WorldPos, pub rotation: Quat, pub scale: Vec3 }

// In sf_world (streaming.md), because they name a record kind sf_world owns
// and sf_coords stays free of sf_records:
pub struct WorldspaceId(pub RecordId);           // a `Worldspace` record
pub struct Location { pub worldspace: WorldspaceId, pub pos: WorldPos }
```

`WorldPos`'s serde form deserializes either text shape (below), so it
needs a self-describing format. Binary products (postcard) store
`to_bytes`-style `(i32 × 3, f32 × 3)` reprs instead.

**Records and saves** store `WorldPos` (with the worldspace implied by the
containing record, or as a `Location`), never a raw f32 world position.
Binary form: `i32×3, f32×3`, little-endian. Save files record `GRID_CELL`
so a future change can be migrated.

**Text formats** (RON/TOML mod records) accept either
`{ cell = [x, y, z], local = [x, y, z] }` or absolute meters `[x, y, z]`
(parsed as f64, converted on load). Mod authors never do cell math.

## Interfaces

**Bevy side (`sf_world_bevy`).**
- Each loaded worldspace is one `BigSpace` root with one `Grid(256, 32)`
  (streaming.md: one root per worldspace).
- High-precision entities (stream cells, dynamic objects, actors, cameras)
  carry `CellCoord` + `Transform`. Their children (a placement's meshes,
  bones) use plain `Transform`.
- `to_grid(WorldPos) -> (CellCoord, Transform)`, and a `WorldPosOf` system
  parameter that reads an entity's `WorldPos`.
- **Rule:** `GlobalTransform` is origin-relative. Use it for rendering and
  same-frame local math only. Never persist it, send it to another worldspace,
  or compare it across frames as if it were absolute.
- The active camera carries `FloatingOrigin`. Portal cameras (M8) are the
  floating origin of their own worldspace's `BigSpace`.

**Shaders.** Tiling, noise, wind, wave phase, and scatter seeds need stable
world coordinates, but they only get origin-relative positions. Each frame,
`sf_world_bevy` writes a `WorldOrigin` uniform: the origin cell, plus for
each period `P` in {1024, 16384} m the phase `(cell · 256) mod P`
(computed in integers on the CPU). A shader that needs a pattern with
period dividing `P` uses `rel_pos + phase_P`. The result is continuous
across origin moves and precise near the camera. Periods that divide 256 m
need no phase, since `rel_pos` already repeats with them. A pattern with a
longer period gets its own entry.

**Physics contract (M4: physics.md "Stepping and the floating origin" implements it on rapier3d).** The
simulation runs in the floating-origin frame and only inside the active ring
(R0, streaming.md), so f32 is ample. When the origin changes cell, every
body shifts by the same exact multiple of 256 m in the same frame. Velocities
are untouched, and nothing is teleported in the solver's eyes. Bodies sync
from and to `CellCoord + Transform`, never from absolute f32.

**Third-party crates that assume absolute world XZ** (for example,
bevy-aqua's FFT wave phase and any noise-based shader) must be fed the
origin phase or forked. Check this before adopting.

**Long-range math** (streaming priority, AI over kilometers, map UI) uses
`WorldPos::delta`, then narrows to f32.

## Verification

- `sf_coords` unit tests: `from_f64`/`to_f64` round-trip within 1 µm up to
  10⁸ m; canonical normalization at every boundary (±128, −0.0); `delta`
  exact across i32 extremes; golden-byte serialization.
- Glue test (headless Bevy): two entities at x = 10⁶ m and 10⁶ m + 1 mm are
  1 mm apart in origin-relative `GlobalTransform`; a shader-phase test checks
  continuity across an origin move.
- `sf_diag` precision scene: a camera at (500 km, 0, 500 km) orbits a mesh
  with 1 mm detail. No visible jitter; screenshot checked.
- Perf counter: transform propagation cost with 200k `Stationary` statics
  (target ≈ 0 ms/frame).

## Decided (2026-10-11)

- big_space 0.14 is a release candidate. Pin `=0.14.0-rc.0` and move to
  0.14.0 when it ships. If upstream stalls, the surface we use is small
  enough to own (it sits behind `sf_world_bevy`).

## Implementation notes (`sf_world_bevy`, 2026-10-11)

- **`Stationary` (answered by reading big_space 0.14.0-rc.0's source).** A
  `Stationary` entity is processed for one frame, then skips recentering,
  spatial hashing, and propagation, but only while its grid's local
  floating origin is unchanged and no non-stationary entity in the subtree
  changed. When the floating origin changes cell, every entity of the grid,
  stationary or not, gets a new `GlobalTransform` that frame
  (`Grid::propagate_high_precision_channeled` sends all children when the
  origin moved; upstream test `stationary_entity_gt_updates_when_fo_moves`),
  and their low-precision children follow because the low-precision pass
  recomputes below any changed parent. So stream cells stay `Stationary`,
  as planned. Two costs remain, both measured by the fly-through
  (`bigspace/*` counters): an origin cell change rewrites every resident
  `GlobalTransform` once (at most once per 256 m of travel, 32 m
  hysteresis), and big_space's low-precision pass walks every
  low-precision descendant each frame to look for changes (it has no
  change-tick pruning like Bevy's static-scene optimization), so it scales
  with resident entity count.
- `Stationary` needs `BigSpaceStationaryPlugin`, which isn't in
  `BigSpaceMinimalPlugins`. `sf_world_bevy` adds minimal + stationary +
  timing plugins, not `BigSpaceDefaultPlugins`: that group adds
  `BigSpaceValidationPlugin` whenever `debug_assertions` are on (dev
  builds), which walks the whole hierarchy every frame.
- big_space requires exactly one `FloatingOrigin` per `BigSpace` and logs
  an error every frame otherwise. Each worldspace root therefore has an
  `OriginProxy` child that holds the origin whenever no camera of that
  worldspace does (prefetched interiors, M8 portal cameras replace it).
- big_space's `CellCoord` + `Transform` is measured from the cell
  *center*, exactly like `WorldPos`, so `to_grid`/`from_grid` copy fields.
  Moving entities may overshoot their cell by the 32 m switching
  threshold before big_space recenters them; `from_grid` renormalizes.
- The `WorldOrigin` uniform is a render-world `UniformBuffer`
  (`WorldOriginBuffer`) plus the WESL module `sf_world_bevy::world_origin`
  (struct and `world_mod_1k` / `world_mod_16k` helpers). Consumers bind
  the buffer in their own pipelines; Bevy has no extensible global view
  binding.
- Glue tests (headless): two entities 1 mm apart at 1,000 km are 1 mm
  apart in `GlobalTransform` (to 2 µm), and the phase test checks that
  `rel + phase` equals world position modulo `P` from three origin cells.

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`).
"Unverified" marks claims from memory not re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Representation | UE5 Large World Coordinates: f64 positions on the game thread (beta; default `WORLD_MAX` still 21 km, larger worlds opt-in), new LWC HLSL types; Godot 4 optional `precision=double` builds, rendering emulates double with f32 pairs; Star Citizen: 64-bit positions throughout (unverified detail); Unity: f32 only (community floating-origin scripts); Bethesda Creation, RAGE, Decima: f32 world positions, worlds sized to fit (tens of km at most; Starfield splits planets into separate small spaces, unverified); OpenMW: f32 positions, OSG's double-precision view matrices on the CPU (unverified) | integer cell + f32 offset (`WorldPos`), f64 only for conversion | **match** in precision, **differ** in representation: no f64 in hot paths, exact integer cell deltas, one format for saves and records |
| Rendering far from the origin | camera-relative rendering everywhere: UE LWC (view-relative translation, tile offsets in shaders, unverified detail), Unity HDRP "camera-relative rendering", Godot double builds; UE4 world origin rebasing (shifts the whole world, full update) | big_space floating origin: the origin moves by whole 256 m cells, every resident `GlobalTransform` is rewritten that frame | **partial**: same result on screen, but our shift re-collects every instance (3–4 ms; audit row 29, P2). UE5's design keeps GPU instance data stable and moves only the view; that's the upstream-sized fix |
| Shader phase for world-space patterns | UE LWC: `LWCFrac`/tile-relative world position helpers for noise and tiling (unverified names); Godot/Unity: users wrap their own | `WorldOrigin` phase uniform per period | **match** |
| Physics far from the origin | Star Citizen, KSP (floating origin + velocity frame "Krakensbane"), UE Chaos in LWC doubles (unverified), avian/Rapier f64 builds | simulate in the floating-origin frame, R0 only, exact cell shifts | **match** (KSP-style; f64 physics unnecessary while R0 is a few hundred m) |
| Depth precision at long range | reverse-Z with an infinite or very far plane (UE, Frostbite, most modern engines) | Bevy's `perspective_infinite_reverse` (0.20), far 60 km for culling only | **match** |
| Nested frames (ships, vehicles) | UE: moving platforms via based movement, sub-levels; Star Citizen: physics grids per ship (unverified) | deferred (open question 1) | **differ** (deferred until a feature needs it) |

No new gap: the one real difference (re-uploading instances on an origin
shift) is audit row 29.

**Sources** (checked 2026-10-12):
- UE, Large World Coordinates in UE5: https://dev.epicgames.com/documentation/en-us/unreal-engine/large-world-coordinates-in-unreal-engine-5
- UE, LWC project conversion guidelines: https://dev.epicgames.com/documentation/unreal-engine/large-world-coordinates-project-conversion-guidelines-in-unreal-engine-5
- Godot, Large world coordinates: https://docs.godotengine.org/en/4.4/tutorials/physics/large_world_coordinates.html
- Unity HDRP camera-relative rendering: https://docs.unity3d.com/Packages/com.unity.render-pipelines.high-definition@17.0/manual/Camera-Relative-Rendering.html (not re-read; unverified URL version)
- Bevy 0.20 `bevy_camera/src/projection.rs` (`perspective_infinite_reverse`), local registry.
- Thorne 2005 (cited above).

## Open questions (implementation checks)

1. Nested grids for moving platforms (boats in M6, airships): deferred until
   a feature needs one.
