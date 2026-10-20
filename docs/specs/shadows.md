# Shadows

Status: **v2 design (lights1, 2026-10-20, owner: "we need a smarter way to do shadows")**: one design for sun and
lamps, below; supersedes the F-slices. Lamp slices 1-2 built (`sf_sky_bevy::shadow_budget`, `sf_shadow_cache_bevy`,
the cache parked since 2026-10-19 and unparked by v2). Sun details (cascades, bias, far tiers) live in
[weather.md](weather.md) ("Distant shadows", "Shadow bias and contact shadows"); diagnosis and Clockwork notes in
[../research/shadows/README.md](../research/shadows/README.md); as-built history in
[shadows-history.md](shadows-history.md).

## Shadows v2: one design (2026-10-20)

**Why.** Bevy 0.20 redraws every shadow view from scratch every frame, and each lamp cube face is a root view that
runs the whole `Core3d` schedule (~0.35 ms render thread per face, below). Our answers so far were layers on top of
that (budget of 1 lamp, far map off, cache without the view skip), and the look still fails (shadows "only when
super close", SH-11/13/14). Industry engines and Clockwork cache what doesn't move and redraw only what changed.

**Budget (High, 1440p, the walk):** all shadows ≤ 2 ms render thread and ≤ 1.5 ms GPU; 8 shadowed lamps; sun
shadows visible to the far-field handoff (~500 m).

| Tier | What | Redrawn | Bevy pieces | Ours |
|---|---|---|---|---|
| Sun near | CSM, **3 cascades** to ~150 m (Clockwork: 3 to 230 m), practical split | every frame | cascades, PCF/PCSS | splits (`CascadeSplits`), caster size cull |
| Sun far | **one cached static map** over the view distance (Clockwork, UE cached shadows), statics only; terrain horizon march past it | on sun move > 0.25°, camera move > 1/16 reach, new cells; spread over frames | an extra shadow view on redraw frames only | `sf_sky::far_shadow` (built F3b-d, off: verify, fix the interior panic) |
| Lamps | **cached static cube faces** (Doom 2016, HDRP mixed cached, Clockwork 8-61 lamps); movers drawn over the copy, per face | statics on change; movers per frame (30 Hz Low/Medium), only faces holding one | cube array + sampling | `sf_shadow_cache_bevy` + **2b: no Bevy views for cached faces** |
| Contact | screen-space, sun + shadowed lamps, **short** (SH-14) | every frame | `ContactShadows` (0.19) | lengths per preset |
| Grounding | GTAO | every frame | SSAO (XeGTAO) | ROADMAP 1.5 |

**What goes away:** High's one-uncached-lamp budget (replaced by 8 cached), the 504 m four-cascade CSM (3 short
cascades + the far map), cascade-texel PCSS sizes (world-size penumbra, V3). Kept: importance selection, crossfade,
caster size cull, terrain far shadow, rain occlusion (vertical, already redrawn only on demand: Nsight 0.04/frame).

**Slices (each measured with the perf suite + Nsight; owner checks per slice):**
- **V0** Measure today's shadow costs: `--off shadows`, 3 vs 4 cascades, Nsight per cascade, census 8 lamps.
- **V1 Lamps 2b** (biggest CPU win): cached lights keep Bevy's cube-array layer but their six face views lose
  `RootNonCameraView`/`ExtractedView` after `prepare_lights` (Bevy runs nothing for them); one node per frame
  restores cached faces into Bevy's layers (slice 2's depth copy); faces whose frustum holds a mover keep a Bevy
  view that loads (not clears) the restored depth. Raise High to 8 lamps at 512², fade 40 m. Gate: 8 lamps ≤ 1 ms
  render thread (census, walk); no flicker when the set changes.
- **V2 Sun**: 3 cascades on High/Ultra to ~150 m + far static map on (verified on screen, interior panic fixed).
  Gate: render -1 ms, shadows present 0-500 m (owner check).
- **V3 Soft shadows by world size**: Bevy's directional PCSS size is in cascade texels (penumbra jumps between
  cascades); scale it per cascade so the penumbra is the sun's angular size × blocker distance (Clockwork). Needs a
  small `bevy_pbr` shader patch (vendored like `big_space`, upstream PR). Plus SH-14 contact length.
- **V4 Cascade update scheduling** (only if V2 leaves cascades > 0.5 ms): far cascade at half rate with its matrix
  held between redraws (CryEngine `r_ShadowsCache`, Unity HDRP cached cascades).
- **V5 Correctness**: LM-1 leaf shadows, RO-1 translucent casters, SH-11 distant flashes, SH-13 lamp grain.

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

### Why a face view costs ~0.35 ms, and the stale cube-face views (lights1, 2026-10-20)

- **Each cube face is a root view running the whole `Core3d` schedule.** `prepare_lights` gives every face a
  `RootNonCameraView(Core3d)`; `bevy_core_pipeline::camera_driver` runs the full camera schedule once per root
  view (prepass, opaque, transparent, post-process systems, each checking and mostly skipping its work). Sun
  cascades are not root views (they render inside the camera's `shared_shadow_pass`). This is the fixed per-face
  cost slice 2 measured; slice 2b (skip the views) stays the right fix.
- **Bevy 0.20 leak, worked around in `sf_sky_bevy::shadow_budget` (`drop_stale_light_views`):** `prepare_lights`
  frees a point light's six views only while the light sorts among the first `min(point lights, max cubes)` (42),
  shadowed first, then by entity. A lamp that loses its shadow sorts back among hundreds of unshadowed lights and
  a hidden light isn't visited at all, so their views live on and keep running `Core3d`. Walking Balmora with a
  budget of 1, the trace showed 15.7 `Core3d` runs per frame (1 camera + ~2.4 lamps' views). Our render-world
  system after `prepare_lights` despawns the views of any light without a shadow map this frame: walk 14.7 →
  12.7 ms (1440p). Report upstream; remove when Bevy frees them itself.
- **What one lamp costs now:** ~1.5 ms frame, 1.4 ms render thread on the walk (1024², bevyrrowind perf.md
  "Lights"). High's lamp count is the owner's call (ROADMAP 1.4): 0, or 1 at this cost, or slice 2b.
- **GPU-driven paths:** shadow views use Bevy's GPU preprocessing (instance culling, indirect draws); two-phase
  occlusion culling exists only for directional cascades and only when the light entity has `OcclusionCulling`
  (we put it on the camera only). Point/spot shadow views have no GPU occlusion culling ("TODO" in
  `prepare_lights`). Neither touches the render-thread cost per view.

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

## Lamp slices (F6, before v2)

1. **(done)** Spec; importance selection. 2. **(done, parked 2026-10-19, unparked by v2)** cache, static only
(shadows-history.md). 3. Movers over the static copy, 8-16 lamps: now v2's V1. 4. Upstream to Bevy if small;
Clockwork extras (fixture self-shadowing, per-actor shadows) later.

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
