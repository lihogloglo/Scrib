# Shadows

**Current design (shadow reset, 2026-10-21).** Stock Bevy shadows plus vendored Bevy PRs (rule "patch Bevy, don't
inject", DECISIONS 2026-10-21); the far static sun map and the other injected layers are deleted (Reset A2).
Built and awaiting owner checks (SH-14..SH-19): A1 (contact shadows sun/moon only 0.3 m, GTAO, sun/moon off interior
views), A2 (lamp maps captured during the door fade; far map and caster cull deleted), B1 (PRs #26149/#26161 in
`vendor/`; the lamp cache `sf_shadow_cache_bevy` stays, "Caching" below), B2 (shadow early-out, 2x2 PCF for lamps).
Open: ROADMAP 1.4 (lamp-shadow count for High, LM-1, RO-1, SH-11, SH-13) and 1.12 (SH-20); quality is hard problem H2
(`bevyrrowind/docs/hard-problems.md`). Sun details: [weather/distant-shadows.md](weather/distant-shadows.md);
diagnosis: [../research/shadows/README.md](../research/shadows/README.md). The v2 design, F6 slices and slice
write-ups (superseded or built) live in [archive/shadows-history.md](archive/shadows-history.md).

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: shadow reset (2026-10-21) built, awaiting owner checks SH-14..SH-19; ROADMAP 1.4 and 1.12 (SH-20) open; quality is hard problem H2. Built: A1 (contact shadows sun/moon, GTAO), A2 (lamp maps captured during door fade; far map and caster cull deleted), B1 (vendored Bevy PRs #26149/#26161), B2 (shadow early-out, 2x2 PCF for lamps), light selection, lamp cache.
> Crates / key types: `sf_sky_bevy` (`shadow_importance`), `sf_shadow_cache_bevy`; `ShadowMobility::Movable` marks non-static casters.
> Decisions: stock Bevy plus vendored PRs, no render-world injection (DECISIONS 2026-10-21); static caster depth cached per lamp ("Caching"); lamp slot choice by `shadow_importance` ("Light selection").
> Read next: the "Current design" header for state; "Reset B2 as built" for the lamp filter; "What Bevy 0.20 gives" for engine limits.
<!-- /agent-summary -->

## Shadows v2: one design (2026-10-20)

Superseded by the shadow reset (2026-10-21); moved verbatim to [archive/shadows-history.md](archive/shadows-history.md#shadows-v2-one-design-2026-10-20).

## Reset B2 as built (shadowB2, 2026-10-21)

- **Own `bevy_pbr` patch** (`vendor/README.md`, `patches/superfargoth-shadow-early-out-lamp-filter.diff`, 44+/9-):
  - *Shadow early-out:* stock Bevy fetched a point/spot shadow (and lamp contact) for every shadow-receiving fragment
    in the light's clusters, before evaluating the light (no range, N·L or cone test). The loop now evaluates the light
    first and fetches only where the contribution is non-zero. Exact (skips only products with zero).
  - *Lamp filter:* camera marker `PointSpotShadowsHardware2x2` → shader def: point cubes and spot maps use 1-tap 2x2
    hardware PCF, directional keeps the camera's `ShadowFilteringMethod` (Temporal on High). Game: on by default,
    `--off lampfilter` = stock. Forward/prepass only (not deferred lighting or meshlets); PCSS lights unchanged.
- **Cost** (perf.md "Shadow reset B2"): filter -0.09-0.10 ms main-pass GPU per shadowed lamp at 1440p (lab 16 lamps
  4.83 → 2.72, census opaque 2.94 → 2.08); early-out helps only where lamps light little (census -0.36). Frames are
  render-thread bound, so medians barely move. Look: SH-19 (owner).

## Reset B1 as built (shadowB1, 2026-10-21)

- **Vendored Bevy patch**: `bevy_core_pipeline`, `bevy_pbr`, `bevy_render` 0.20.0 + PRs #26149 (`Core3dShadows`, a
  slim schedule per point/spot shadow view) and #26161 (`SkipRootViewRendering`: skip a light's views while its
  visible casters, view and queued count are unchanged) in `vendor/` (README: rebase/drop). ~300 lines, clean apply.
- **The lamp cache stays** (`sf_shadow_cache_bevy`; DECISIONS 2026-10-21 rule failed): #26161 redraws a whole cube
  whenever any caster in it moves or is skinned, so walkers/NPCs defeat it (lab 8 lamps 11.4 vs 6.9 ms; census 15.0
  vs 9.0); skipped views still cost ~0.13 ms render thread per lamp. Patch + cache ships: census 9.0 → 6.5 ms.
- Door-fade gate unchanged (still the cache's `settled()`); `--off shadowcache` is now the patched Bevy path.
- Open (B1b): attribute (d)'s census gain; a static-casters-only skip upstream (movers drawn over a cached copy,
  i.e. the cache's design) is what would let the cache go. Directional maps are not covered by either PR.

## Reset A2 as built (shadowA2, 2026-10-21)

- **Lamp maps are captured during the door fade.** `FadeTransition::hold()` (sf_world_bevy) is a gate: once the
  arrival's cells and textures are ready the hold lasts 3 more frames, then while a gate asks, at most `gate_max_secs`
  (1.5 s). The game's gate (`render.rs::shadow_captures_at_black`) sets `ShadowCacheSettings::warmup` while black
  (unlimited captures, immediate recaptures, new casters settle in 0.2 s, not 1 s) and holds until the budget has
  shadowed `min(max, lamps within fade_end)` lamps and `ShadowCacheStats::settled()` (no shadowed lamp in view waiting
  for or in a capture, due a recapture or touched by a settling caster; moving lamps don't count). Census: 0.35 s of
  gate. First exterior visit (dev build): capture pipelines still compiling at 1.5 s, the cap fades in. No initial-load
  fade exists, so no gate there. Not done (to B): a lamp's shadow fade-in waiting for its capture.
- **Deleted:** the far static sun map (`sf_sky::far_shadow`, `sf_sky_bevy::far_shadow`, the terrain/proxy sampling),
  `sf_sky_bevy::caster_cull`, `--off shadowtune`/`--off shadowsplits`. `--off farshadows` stays (terrain far shadow).

## Reset A1 as built (shadowA1, 2026-10-21)

- **Contact shadows: sun and moon only**, as engines use them (one key light, short ray). Lamps and carried torches
  get none (`PointShadowBudget::contact_shadows` false; game A/B `BEVYRROWIND_LAMP_CONTACT=1`): it was 0.80 ms GPU per
  shadowed lamp at 1440p and drew SH-14's bands. Ray length 0.3 m (Bevy's default; was 0.5), Medium 8 / High 16 steps.
- **Sun and moon leave interior views.** Bevy filters directional lights per *view* by `RenderLayers`, never per mesh;
  on `[0, exterior]` they stayed in every interior view (camera `[0, interior]`) through layer 0. They did not light
  interiors (illuminance 0 without a `SkyCamera`) but were shaded per pixel with the contact march (0.8 ms). The game
  now gives them `[0, active layer]` under the sky and `RenderLayers::none()` indoors (one writer, `light_layers`).
- **GTAO**: Bevy `ScreenSpaceAmbientOcclusion` (depth + normal + motion-vector prepass, `Msaa::Off`), Low preset off,
  levels Low/Medium/High on Medium/High/Ultra; ~0.9 ms at 1440p native on High. Torch-bearers: verified in the budget
  (BRP, Balmora 22:00: a `HeldLightSlot` light holds one of High's 8 maps). Numbers: bevyrrowind perf.md "Shadow reset A1".

## V1 as built (shadows2, 2026-10-20)

The cache built in lights1 cached **nothing**: diagnostic counters (now a 5 s `debug` log in the crate) showed
48 of 48 census faces holding a "mover". Causes and fixes, in the order found:

1. **Casters that cast nothing were tracked**: NIF particles (unculled, in every face), billboard subtrees and
   translucents swapped to `FroxelFogMaterial` (material shadows off, entity not marked). Hosts now put
   `NotShadowCaster` on them (`mw_assets` particles + billboards, `sf_atmos_bevy::FogTranslucentsPlugin`).
2. **Change detection fires on writes**: animation and propagation rewrite unchanged transforms; ~650 still casters
   (candles) never settled. The cache compares affine, bounds and visibility (1 mm).
3. **Per-face schedule**: every face ran the whole `Core3d` (dozens of host systems). Face views now run
   `LampShadowView` (GPU preprocessing + Bevy's shadow passes + ours), Bevy PR #26149's split, done from outside.
4. **Restores**: a detached face is re-copied only when Bevy's layer went stale (`CleanLayers`: Bevy's
   `TextureCache` returns the same cube texture while the shadowed count is unchanged; nothing clears a detached
   face). 731 restores in 50 k detached face-frames.
5. **Capture frames dropped movers** (actor shadows blinked, ~26 captures/s on the walk): captures now draw into
   the cache through their own views (subviews 6-11 of the light, injected like `sf_sky_bevy::sun_view`); Bevy
   draws the capturing light whole. ≤ 2 captures per frame; off-screen lights (not extracted) don't take a turn.
6. **Streaming dirtied lamps continuously** (40 k settles + 19 k despawns on the walk: LOD swaps, size culling):
   a light recaptures at most every 2 s; casters settled after its capture are drawn as its movers meanwhile.
7. **Captures never confirmed while streaming** (a global "pipelines compiling" check): now only the capture's
   own bins' pipelines count.
8. Floating-origin shifts re-express every transform: hosts report them (`ShadowOriginShift`); characters and
   their attachments are `ShadowMobility::Movable` (inherited from an ancestor), so NPCs that stop never bake.

Presets (bevyrrowind `render.rs`): High 8 lamps at 512², Ultra 8 at 1024², fade 30-40 m (was 1 lamp, 15-25 m);
carried torches rank like lamps (priority 4 → 1: with one slot the shadow hopped between torch-bearers).
Numbers: bevyrrowind `perf.md` "Shadows v2". **Gate**: census 8 lamps 16.8 → ~6.3 ms (960p; 0 lamps 4.0), met
in spirit; the Balmora walk is not: walkers next to lamps keep most cached faces holding a mover (each a Bevy
view, ~0.1 ms). Next levers (not built): movers at 30 Hz with layer reuse (Clockwork), movers only within a
distance of the light, a cheaper lamp PCF (the main pass pays ~0.5 ms GPU per shadowed lamp at 1440p).

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

Note 2026-10-26: since reset B1 the cache runs on vendored Bevy PRs #26149/#26161 (see "Reset B1 as built"); the 'no fork' wording below predates that.

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

Superseded by the shadow reset (2026-10-21); moved verbatim to [archive/shadows-history.md](archive/shadows-history.md#lamp-slices-f6-before-v2).


## Industry comparison

| Technique | Industry | Us | Verdict |
|---|---|---|---|
| Shadowed-light budget + fade | HDRP max shadows + fade distance; UE per-light fade; Clockwork | `PointShadowBudget`, importance, hysteresis, crossfade | **match** |
| Selection by contribution incl. off-screen lights | Clockwork; HDRP (screen-size based) | `shadow_importance` on shown lights, in view or not | **match** |
| Static caster caching | Doom 2016, HDRP mixed cached, UE cached shadows, Flax | `sf_shadow_cache_bevy` (auto mobility); Bevy still runs its per-face views | **match** since reset B1: vendored PRs #26149/#26161 skip unchanged views, cache kept (see "Reset B1 as built") |
| Dynamic casters over cached copy | HDRP, UE, Clockwork (30 Hz) | `sf_shadow_cache_bevy` composites movers over the static copy; #26161 alone redraws whole cubes (see "Reset B1 as built") | **stale, see Reset A1-B2** |
| Atlas, per-light resolution | Doom, HDRP | one global cube size | **GAP (P2)**: slice 3 |
| Contact shadows for unshadowed lights | UE | sun and moon only, 0.3 m; lamps none since reset A1 (see "Reset A1 as built") | **differ**: engines use one key light, short ray |

## Co-op

Presentation only: each client picks its own lights from its own viewer; no simulation state, nothing replicated.
