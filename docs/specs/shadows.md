# Shadows: local lights at scale (F6)

Status: **slice 2 built, slice 2b next** (shf6 2026-10-19; shf6b slice 2). Crates: `sf_sky_bevy` (`shadow_budget.rs` today; the cache in a new
`sf_shadow_cache_bevy` from slice 2). Sun shadows (cascades, contact shadows, far tiers) stay in
[weather.md](weather.md) ("Distant shadows", "Shadow bias and contact shadows"); the diagnosis and the owner's order
(F1+F2, F3, **F6**, F5) are in [../research/shadows/README.md](../research/shadows/README.md). The current budget
is summarized in [sky-lighting.md](sky-lighting.md) "Local-light shadows".

Goal: interiors lit by 20-30 point lights where every light that matters on screen casts a shadow (8-16 on High,
more on Ultra), at well under 1 ms, with no visible popping when the shadowed set changes. Today High shadows
**one** light, faded out over 15-25 m, because every shadowed light re-renders six cube faces every frame.

## What Bevy 0.20 gives (read in `bevy_light-0.20.0` / `bevy_pbr-0.20.0`, 2026-10-19)

- **Per light:** `PointLight::shadow_maps_enabled`, `shadow_depth_bias`, `shadow_normal_bias`,
  `shadow_map_near_z`, `soft_shadows_enabled` (PCSS, `experimental_pbr_pcss`, uses `radius`),
  `contact_shadows_enabled`. Casters opt out with `NotShadowCaster`; `RenderLayers` on a light limit which meshes
  it sees. Spot lights: one 2D layer each in the directional array (`DirectionalLightShadowMap::size`).
- **Storage:** one `PointLightShadowMap { size }` for **all** lights (no per-light resolution); a cube-array depth
  texture of `count × 6` layers taken from the render world's `TextureCache` **every frame** (a new count means a
  new texture). Max cubes = `max_texture_array_layers / 6` (42 at wgpu's default 256). Shadowed lights sort first,
  then by `Entity`, so a light's layer index changes when the set changes.
- **Drawing:** every shadowed light is six views, each frustum-culled against the light's range, **cleared and
  redrawn every frame**. Retained render phases cache binning on the CPU, but there is no "skip this view", no
  static/dynamic split, no atlas, no caching anywhere. GPU occlusion culling (`OcclusionCulling` on a light) is
  wired for shadow cascades.
- Measured cost (2026-10-13, `hi-balmora-night`, 1440p, RTX 4060 laptop): +0.8 ms frame for 1 light, +3.9 ms
  for 2, +4.4 ms for 4, mostly render-thread CPU (six views of every caster in range). Census office numbers:
  "Measurements" below.

So Bevy gives the sampling (cube lookup, filtering, PCSS, clustering) and a correct but uncached renderer. The
budget on top of it is ours (`PointShadowBudget`, since 2026-10-13).

## Industry design

- **Shadow atlas + caching.** Doom 2016 (one 8k² atlas; a light's tile is redrawn only when something in its
  volume moved, tile size by distance/importance), Unity HDRP (punctual-light atlas, update modes EveryFrame /
  OnEnable / OnDemand, "mixed cached shadow maps": static casters cached, dynamic ones drawn each frame over a
  copy; a cached light keeps its atlas slot when it is re-enabled), UE (cached whole-scene shadows for movable
  point/spot lights: static primitives cached, movables composited each frame; VSM page caching with a separate
  static cache), Flax ("shadows caching for static geometry", invalidated on light move/splits change).
  Clockwork (OpenMW fork, behaviour only, research note §2): 8-61 lights (2 per GiB VRAM), 512² faces, static
  casters cached, moving objects at 30 Hz over the cached copy, 0.06-0.24 ms for 8 lights.
- **Selection:** a light's contribution at the viewer, **including lights out of view** (turning the camera must
  not change the set), its screen coverage, and a game priority; hysteresis so two similar lights don't swap;
  crossfade when a light gains or loses its shadow (HDRP max-shadows-on-screen + shadow fade distance, UE
  per-light shadow fade, Clockwork).
- **Resolution by importance:** atlas tiles sized by screen coverage (HDRP "dynamic rescale", Doom), with a
  per-preset and VRAM budget.
- **Flicker:** a torch's flicker animates intensity only; its cached map is untouched (position, range and casters
  unchanged). Light jitter, if any, is applied as a sub-texel offset or not at all.
- **Many-light fallback:** lights without a map keep contact shadows (UE per-light contact shadows) and AO.

## Light selection (built, slice 1)

`sf_sky_bevy::shadow_importance(range, d, priority) = priority · (range / max(d, 1))² · min(1, range / d)²`:

- *Strength at the viewer*: `range²/d²`. Strength enters through `range`, not `intensity`: games set the range
  where the light fades out (Bevy's cutoff), so `range²` tracks power, and it doesn't flicker (an animated
  intensity would reshuffle the set every frame). Clamped under 1 m.
- *Screen coverage*: 1 inside the light's range, `(range/d)²` outside (the lit volume's share of the view).
  Continuous at `d = range`.
- *Priority*: game-set multiplier (Bevyrrowind: a carried torch 4, i.e. "twice nearer").
- Candidates: shown lights (`Visibility`, not `ViewVisibility`: out-of-view lights count) within `fade_end`.
  Holders rank `1/hysteresis` (1.25×) better. A newcomer waits for a free slot; shadows crossfade by the existing
  light split (`fade_time` 0.3 s, distance band `fade_start..fade_end`).

## Caching (slice 2+, ours)

**What to cache:** per shadowed light, the depth of **static casters** (placed world geometry: everything without a
`ShadowMobility::Movable` marker; games mark actors, physics bodies, animated doors and carried items movable).
A light's static faces are redrawn only when (a) it gains a slot, (b) it moves or its range changes beyond a
threshold, or (c) a static caster whose bounds touch its range spawns, despawns or moves (a dirty set fed from
`Changed<GlobalTransform>`/spawn events, tested against the light spheres on the CPU). Redraws are spread over
frames (a budget of faces per frame; a new light first appears unshadowed and fades in once all six faces exist).
**Movable casters** are drawn every frame (Low/Medium: 30 Hz) over a copy of the static faces, only on faces whose
frustum holds one; faces with none sample the static copy directly.

**How it plugs into Bevy (no fork):** Bevy keeps sampling (filters, PCSS, clusters). For cached lights the budget
sets `shadow_maps_enabled` and a system after Bevy's `check_point_light_mesh_visibility` empties the light's
`CubemapVisibleEntities`, so Bevy's six views draw nothing (a clear only; a `RenderLayers` mask would also hide
casters but lights' layers gate which views they light, so it's not used). A
render-graph node after Bevy's shadow pass copies each light's composed cube from our persistent texture into
Bevy's cube-array layer for that light (index from Bevy's sorted light order: shadowed lights first, then by
`Entity`). Cost: 512² × 6 × 4 B = 6 MiB per light per frame, ≈ 0.1-0.2 ms for 8 lights on a laptop 4060
(bandwidth estimate, to be measured).

**Storage:** a persistent depth cube array owned by `sf_shadow_cache_bevy`: static cubes + one dynamic composite
per light; slots keep their layer while the light holds them (HDRP's rule), so re-entering a room costs nothing
until evicted (LRU). Count by preset with a VRAM cap (12 MiB per light at 512²: 16 lights = 192 MiB).

**Why each earlier step doesn't fit:** Bevy redraws and clears every shadow view every frame, has one global cube
size and per-frame textures (no persistence); a Bevy PR adding cached/persistent shadow views is the long-term
home (raise it upstream with this design, adopt if it lands). Crates: none found for Bevy 0.20 (web search
2026-10-19: Bevy tracker has no shadow-caching issue/PR; other engines' code is not reusable here). Clockwork is
GPL and a different renderer: behaviour only.

## Slice 2 as built (shf6b, 2026-10-19): cache works, Bevy's per-view cost remains

Crate `sf_shadow_cache_bevy` (`ShadowCachePlugin`, `ShadowCacheSettings`, `ShadowMobility`, `ShadowCacheStats`;
Bevyrrowind adds it in `render.rs`, A/B `--off shadowcache`). Built as specified above with these choices:

- **Mobility is automatic** (no game markers needed): skinned meshes and `ShadowMobility::Movable` always move; any
  other mesh is baked once it has been still for `settle_time` (1 s) after appearing; a baked caster that moves
  (transform, `InheritedVisibility` or `Aabb` change) unbakes the lights it touches and **stays a mover** (at most one
  bake + one unbake per caster; without this rule actors' rigid parts and live emitters kept the lights recapturing,
  p99 300 ms). A light that moves is left to Bevy until it stops; flicker (intensity) never recaptures.
- **Copies are draws:** Bevy's cube texture has no `COPY_*` usage, so a fullscreen depth-copy pipeline
  (`copy_depth.wesl`, compare Always, `frag_depth`) restores our layer into Bevy's face view before
  `shared_shadow_pass` (taking the attachment's first use, so Bevy loads instead of clearing) and captures Bevy's
  face after the late shadow pass. GPU: 0.027 ms restore, 0.024 ms capture (8 lights, census).
- Captures are confirmed (six faces, current generation) only while no pipeline is compiling; slots are LRU, the
  cache texture grows in steps of 4 cubes (24 max at 1024² = 576 MiB worst case; usually 8 → 192 MiB).
- Known gaps: movers are missing from a light's map for the 2-3 frames of a capture; a static mesh whose asset
  loads more than `settle_time` after its entity spawns is baked without it.

**Measured (census bench, 720p, `--off farshadows`, machine busy with other builds):** median frame / render
thread, cache on vs off: 1 light 6.3 / 6.5 ms; 4: 12.0 / 13.0; 8: 20.7 / 21.0 (render thread 20.3 / 20.7);
16: 28.2 / 29.8. Without actors (`--off actors`, 8 lights): 19.6 / 20.2 ms, render thread 19.3 / 20.5. **So the
cost is not the casters drawn: it is Bevy's fixed per-view work** (~0.35 ms render thread per face view: view
setup, specialize/queue/batch/preprocess per view, even with an empty `CubemapVisibleEntities`). Emptying the
lists, as planned, saves only ~1 ms at 8 lights. Presets stay 0/0/1/4.

**Next (slice 2b): skip Bevy's views for cached lights.** In the render world after `prepare_lights`, take the
six face-view entities of a cached light out of Bevy's per-view systems (remove `ExtractedView` and
`RootNonCameraView`, keep `ShadowView` for its texture layer) and run our restore from one system (e.g. the
camera's Core3d before `shared_shadow_pass`, looping over cached lights), with movers drawn by our own small
shadow phase or by letting Bevy keep views only on faces whose frustum holds a mover (spec "per-face"). Check
that `prepare_lights` doesn't re-insert the removed components each frame (it does only for new/changed lights;
a flickering light is "changed" every frame: then filter `changed_point_lights` or re-remove after it). Then
re-bench 1/4/8/16 and raise High to 8.

## Slices


1. **(done)** Spec; importance selection (presets unchanged: the cost table shows uncached maps cost ~1.9 ms each).
2. Cache, static only: `ShadowMobility` marker, persistent cube array, dirty tracking, copy node, cached lights
   with emptied Bevy views; movable casters not drawn yet (A/B `--off shadowcache`). Verify flicker leaves the
   cache alone (render count stays 0 while standing still).
3. Movable casters over the static copy (per-face, 30 Hz on Low/Medium), crossfade on slot moves; raise presets
   to 8-16 lights at 512²; per-light tile size by importance (needs our own sampler or Bevy per-light sizes:
   decide then).
4. Upstream to Bevy if it stays small; Clockwork-style extras later (fixture self-shadowing, per-actor shadows).

## Measurements

Census office bench (`benchmarks/census_office.json`, streamed view, release, 1280×720, High, 1024² faces, RTX 4060
laptop + Ryzen 7 7435HS; shf6 2026-10-19), uncached Bevy maps, `--point-shadows N` (156 lights shown):

| Shadowed lights | Median frame | Render thread | Shadow views GPU | Shadow views CPU (encode) |
|---|---|---|---|---|
| 0 | 4.8 ms | 4.3 ms | 0 | 0 |
| 1 | 6.6 ms | 6.1 ms | 0.54 ms | 0.85 ms |
| 4 | 12.2 ms | 11.9 ms | 2.35 ms | 2.69 ms |
| 8 | 21.7 ms | 21.8 ms | (timer dropped) | 6.37 ms |

≈ +1.9 ms frame per shadowed light, **render-thread bound** (six views queued, specialized and encoded per light);
the GPU part is ≈ 0.55 ms per light at 1024². So raising the count on Bevy's uncached maps is not affordable: the
presets stay (Low/Medium 0, High 1, Ultra 4) and slice 2's cache must remove the per-frame views, not only the GPU
draws (empty `CubemapVisibleEntities` → nothing to queue). Visually 8 lights already change the room a lot
(table, chair and chandelier shadows on floor and ceiling; SH-9).

## Industry comparison

| Technique | Industry | Us | Verdict |
|---|---|---|---|
| Shadowed-light budget + fade | HDRP max shadows + fade distance; UE per-light fade; Clockwork | `PointShadowBudget`, importance, hysteresis, crossfade | **match** |
| Selection by contribution incl. off-screen lights | Clockwork; HDRP (screen-size based) | `shadow_importance` on shown lights, in view or not | **match** |
| Static caster caching | Doom 2016, HDRP mixed cached, UE cached shadows, Flax | `sf_shadow_cache_bevy` (auto mobility); Bevy still runs its per-face views | **partial**: slice 2b skips the views |
| Dynamic casters over cached copy | HDRP, UE, Clockwork (30 Hz) | none | **GAP (P1)**: slice 3 |
| Atlas, per-light resolution | Doom, HDRP | one global cube size | **GAP (P2)**: slice 3 |
| Contact shadows for unshadowed lights | UE | budgeted lights only (cost × lights) | **differ**: cheap stopgap, revisit with measurements |

## Co-op

Presentation only: each client picks its own lights from its own viewer; no simulation state, nothing replicated.
