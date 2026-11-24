# Grass density at the player's feet (research, 2026-11-23)

Paths are under `superfargoth/vendor/bevy_grass/src/`.

## Facts
- Candidates: one blue-noise tile of `TILE_POINTS = 1024` points (`shader_model.rs:19`). A chunk (16 m, `fields.rs:44`)
  is covered by `tiles_per_side^2` copies of it, each with a per-cell random offset (`generate.wesl:313-325`).
  `ChunkLayout::for_density` (`shader_model.rs:101`) picks tiles_per_side = ceil(16 * sqrt(density/1024)), i.e.
  the coarsest tile giving >= style density. Ours 150/m^2 -> 7 tiles, candidate density 196/m^2. The layout is
  **per field** (`fields.rs:1709`), uniform for every chunk of it.
- cull (`generate.wesl:150-197`): per visible chunk, `keep = shape.w * density_scale * falloff / candidate_density`;
  `needed = ceil(keep*1024) * cells`, split into slices (`:185-196`); `generate` runs 64 lanes per slice (`:446`).
  Candidates are a rank prefix, so fewer needed = less work. `VisibleChunk` has a spare 4th u32 (`:193`, set to 0).
- Falloff (`common.wesl:162-183`): 1.0 to `lod1_start`, then mid-range x0.5 from `mid_range_start` (10 m,
  `settings.rs:250-257`). Within 10 m we already keep every candidate (keep = 150/196); more density at the feet
  is impossible without more candidates. Nothing in the crate raises density near the viewer.
- Chunks are culled by frustum and `view.radius` only (`:150-170`). Overlapping fields are allowed (`edit.rs:213`
  "overlapping fields"); each field owns its chunk table. A field's density map is an image (`api.rs:65,208`).
- Blades carry a `clump` id 0-63, used for tint only (`common.wesl:249`, `look.wesl:30`). A candidate yields one
  blade; `tufts` (style.rs:104) only moves blades toward tuft centres (`generate.wesl:326-335`). No multi-blade
  candidates.

## Options
**(b) Near field, own code, no crate patch.** In `sf_groundcover_bevy/blades.rs` (fields built at `:709-714`) add one
extra `GrassField` per blade species: 32x32 m (2x2 chunks, snapped to the 16 m grid as blocks already are), style
density = extra (e.g. 2x = +300/m^2), density map = radial mask 1 inside r (5 m), smooth to 0 by 7 m, rewritten when the
viewer crosses a cell (128x128 R8 image, trivial). It draws the same blade types, so it must reuse the ground mask of
the main field (grass only where grass grows). Cost: extra blades only; chunks of the near field outside r still
spend candidate slices (needed ignores the mask), 4 chunks x ~300 x 256 cells... measure; plus one more field
in the chunk table. Look: blue-noise of the second field is independent, so overlay is random, no regular pattern;
density at feet = 3x. Risk: field follows viewer = image update + chunk-table rebuild on cell change (hitch?
verify `fields.rs:784` update path), and must hide from shadows/trample twins.
**(a) Per-chunk tile size, vendor patch.** In cull, derive `tiles_n` from distance (e.g. 2x tiles per side within
r = 4x candidates), store it in the spare `VisibleChunk` word, use it in `evaluate` instead of `field.placement.y` /
`extent.w` (`:313-325`, `:364`) and in `needed`; Rust twin `mirror.rs` + tests (`mirror/tests`). ~40 lines WESL + Rust.
Plus a near multiplier in `density_falloff` (`common.wesl:162`, can exceed 1 for d < r). Cleanest, no second field,
exact density, no overlay randomness; cost = same blades. Needs a patch note in `vendor/README.md` and a gpu_test.
**(c) Clumps, vendor patch.** N blades per candidate near the camera: needs generate.wesl to emit N instances per
keep with jittered roots (bucket capacity/atomics change, `:446-480`). Same blade count as (a), more shader work
per candidate, less control; cheaper only in candidate evaluation (small). Not worth it over (a).

## Cost model
Our numbers: 6.5k near blades at 60/m^2 -> near zone ~108 m^2. 0-5 m = 78.5 m^2 * 60 = 4.7k blades; 3x adds ~9.4k
(+19% on 49k) -> ~58k. Blades near the camera are LOD0 (most vertices), so GPU cost is somewhat above +19% of grass
cost, probably ~0.1-0.3 ms (guess, unmeasured; judge by an interleaved A/B per CONTRIBUTING). Offset by thinning
mid/far slightly. Cap r at 3-5 m: area grows with r^2 (8 m = 2.6x the blades of 5 m).

## Industry
- Ghost of Tsushima (Wohllaib, GDC 2021 "Procedural Grass in Ghost of Tsushima"): blade count per tile by distance
  with the densest ring near the player, blades are 7-vertex strips, root colour sampled from terrain, clumping and
  a distance-scaled blade width keep ground coverage. Horizon Zero Dawn / Forbidden West: dense LOD0 near, billboards
  and ground-colour texture beyond. UE5 landscape grass: per-type density x per-LOD `StartCullDistance`, near
  instances spawned in a small high-density ring. BotW and Genshin: dense, short, wide blades near the camera plus a
  ground texture tinted to grass colour so gaps are never bare. (From memory; verify before citing in a spec.)
- Shared trick: ground albedo under grass = blade root colour (dark-green near, tuft-tinted), so thin coverage looks
  like shadowed grass, not bare soil. Check our terrain material under groundcover first; likely the cheapest win.

## Recommendation
1. Check/match ground colour to blade root colour near the camera (own code, ~0 ms), may remove most of the
   "bald" read by itself.
2. Then a near multiplier. Preferred (a) (vendor patch, exact, no second field) since the repo already carries
   vendor patches (`vendor/README.md`); if a patch is unwanted for now, (b) as an interim is not recommended
   ("right from the start"): go (a). Start r = 4 m, 2.5x, taper to 1x at 7 m, tunables in `tunables.md`;
   A/B for ms, owner judges the look.
