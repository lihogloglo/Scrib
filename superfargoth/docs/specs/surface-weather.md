# Surface weather: wetness, puddles, snow and ash cover, trails

Status: W1 built (2026-11-05: `sf_surface`, driven by `sf_weather_bevy`); W2-W8 spec. Slices W1-W7 (+ WM measurement, W8 character wetness) below.
Crates: `sf_surface` (new, Bevy-free), `sf_surface_bevy` (new, thin plugin); touches `sf_terrain_bevy`
(hook, displacement, prepass), `sf_lod_bevy` (proxies call the hook), vendored `bevy_pbr` (own patch "SW"),
`sf_weather` (`SurfaceRates`), `sf_save` (section). Adapter: `../../../bevyrrowind/docs/specs/weather-adapter.md`
"Surface weather". Research: `../research/surface-weather-2026-11-05.md`; Clockwork read:
`../research/clockwork-code-2026-10-25.md` row 5. Supersedes the "Surface state" design in
[weather/precipitation-surface.md](weather/precipitation-surface.md) (GPU-simulated clipmap, `WorldMaterial`).

<!-- agent-summary -->
> **Agent summary.** Deterministic CPU surface state in `sf_surface` (wet + puddle per climate, snow + ash per
> 32 m cell, game-time steps, saved, co-op-reconstructible) → a 128² cell texture + a top-down shelter depth map
> → one WESL layer `sf_surface::layer` called by every lit material: Bevy's `StandardMaterial` through an own
> `bevy_pbr` patch (mesh-view bindings, like the GI patch), terrain and proxies directly. Normals compose in
> Mikkelsen's surface-gradient framework. Trails (W6) are the only geometry change: a camera ring of height
> deltas displaces the finest CDLOD levels, with POM and prepass motion vectors. Read "Data model",
> "Shader integration" and your slice in "Build plan".
<!-- /agent-summary -->

## Goal

Owner (2026-10-21/22, 2026-11-05): the world reacts to weather. After rain, ground and objects are wet and
puddles fill low spots, rippling while it rains; snow and ash settle on up-facing, exposed surfaces and stay
after the storm; actors leave trails in snow, ash and mud; feet kick up splashes and puffs; all of it survives
a save and agrees between co-op peers. Morrowind needs rain wetness everywhere, ash around Red Mountain, snow
on Solstheim.

**Non-goals (v1):** SSR/planar puddle reflections (sky/probe first, SSR after a depth prepass: owner
2026-11-05); persistent terrain edits (terrain.md layer 2); trails off terrain; gameplay effects of cover
(ice, deep snow: later, host-side via the CPU query); wet camera; water bodies (water.md); interiors.

## Design rules

1. **Industry standard, normals where it counts (owner, binding).** "Always research the industry-standard
   way, adding normals where it counts." Each layer names its canonical technique and shipped references
   below, and changes the shading normal, not only albedo and roughness:
   - *Wetness* (Lagarde 2013 wet model): porosity-weighted albedo darkening, roughness drop, and the normal
     blending toward the macro (geometric) normal as the water film fills micro-detail.
   - *Puddles*: flat water normal (the geometric normal) plus animated rain-ripple normals; edges blend
     by height and porosity, so the detail normal fades into the flat one across the shoreline.
   - *Snow and ash*: height-blend against the base material, their own detail normals, coverage from
     `N·up` of the **per-pixel** normal (normal-mapped), guarded by the geometric normal and gated by the
     shelter map.
   - *Trails*: normals reconstructed from the displacement field's gradient (vertex normals from the ring,
     fragment normals from its finite differences, POM for sub-vertex prints), so light and shadow follow
     the deformation.
   - All layers compose normals as **surface gradients** (Mikkelsen 2020, "Surface Gradient-Based Bump
     Mapping Framework"): each layer contributes a gradient, weights blend gradients, one resolve at the end.
     No per-layer renormalised lerps of normals.
2. **Gameplay state apart from presentation (net.md rule 3).** Cover amounts are deterministic simulation
   state (`sf_surface`, game-time steps, saved). Shelter map, ripples, trails and kick-up are local
   presentation and never feed gameplay.
3. **Weather-gated cost.** In dry weather with zero cover the layer is one uniform branch; each layer
   compiles out per preset (shader defs) and switches off at runtime (`--off`).
4. **Framework stays game-agnostic.** Channels are generic (`wet`, `puddle`, `snow`, `ash`); looks come from
   `SurfaceLayer` records; climates, regions and textures come from the adapter.
5. **Patch Bevy, don't inject** (CONTRIBUTING 4): the `StandardMaterial` hook is a marked own patch in the
   vendored `bevy_pbr`, the same shape as the existing GI patch, with a drop condition.
6. **Ported code cites its source.** Shapes ported from Clockwork (`files/shaders/lib/light/surface.glsl`:
   `surfaceExposure`, `surfaceNoise`, `surfaceCrowns`, `surfaceRainGrids`/`surfaceRings`, `surfaceCovers`;
   `apps/openmw/mwrender/clockwork/scene/surfaceweather.cpp` rates) cite
   `gitlab.com/OursCodeur/openmw` branch `clockwork-alpha` commit `c5e5a9a6362c35512823884753d2d5284af204fa`
   (GPL-3.0) in the module docs, and the first porting slice adds the `THIRD_PARTY_NOTICES.md` entry.
   Constants are re-calibrated in meters and from our own look, never copied blind.

## Canonical techniques and industry comparison

| Feature | Canonical technique | Shipped / published references | Ours | Verdict |
|---|---|---|---|---|
| Wet surfaces | Lagarde, "Water drop 2a/2b, 3a/3b" (2012-13): porosity, albedo darkening, roughness to water, film flattens normal | Remember Me (Dontnod, Lagarde), UE wet material functions, Watch Dogs **(verify)** | Lagarde model, porosity from material (heuristic) or record | match |
| Puddles | height/porosity-driven water level, flat normal, F0 0.02, rain ripple normals (Lagarde 2a ripple atlas or procedural rings) | Remember Me, Watch Dogs, Unity Production Ready Weather sample, Clockwork | layer height + terrain basin signal; procedural rings (Clockwork shape); sky/probe reflection | match (SSR later) |
| Puddle reflection | env/probe → planar for big pools → SSR | UE (SSR + reflection captures), RDR2 **(verify)** | env map + GI probes; SSR parked (no depth prepass on the exterior camera) | gap, owner-accepted |
| Exposure / shelter | top-down precipitation occlusion depth map | Tatarchuk "Rain" (ATI ToyShop, 2006), Clockwork `surfaceRoof`, AC/RDR2 **(verify)** | vertical depth map 1024² over 128 m, tent-filtered gather | match |
| Snow/ash cover | world-aligned blend: `N·up` × exposure × amount, height-blend, noise patches | UE world-aligned blend, Uncharted 4/RDR2 snow **(verify)**, Clockwork `surfaceCovers` | same, per-pixel N, surface-gradient normals | match |
| Cover state | per-region or per-cell scalars over game time | Clockwork (per region, not saved), RDR2 (persistent snow, **verify**) | climates (wet) + 32 m cells (snow, ash), saved | match+ |
| Trails | camera-centred toroidal height render target, stamped by feet/bodies, refilled; displacement + relief mapping, lip normals | Batman: Arkham Origins (Barre-Brisebois, GDC 2014), Rise of the Tomb Raider (GPU Pro 7, **verify**), God of War 2018, RDR2 | ring 1024² over 64 m, vertex displacement of finest CDLOD + POM; no tessellation (wgpu has none) | match (console path) |
| Kick-up | particle bursts on foot contact by surface | RDR2, GoW | instanced sprite bursts from the stamp stream | match |

Clockwork: footprints are a hashed decal list (mask, no displacement), nothing saved, actors skip covers (we
gate skinned meshes alike). Rafael's OpenMW shader pack: visual reference only (licence unknown).

## Data model

### CPU state (`sf_surface`, Bevy-free)

```rust
pub struct SurfaceRates { wet, puddle, snow, ash, drying: f32 }   // exists in sf_weather (per game hour)
pub struct SurfaceTuning {  // record-backed; start values from Clockwork, re-calibrated by the owner's look
    step_h: f64, dry_h: f32,  // 1/6 h (10 game min) fixed step; 2.0 h full wet -> dry (Clockwork 120 min)
    puddle_dry_mult: f32, snow_melt_mult: f32, ash_clear_mult: f32, // 0.5, 1/3 (snow 3x slower), 1/4 (dust 4x)
    puddle_from_wet: f32,     // 0.9: puddles rise only on saturated ground
}
pub struct ClimateSurface { wet: f32, puddle: f32 }               // per ClimateIx
pub struct CellSurface { snow: f32, ash: f32 }                    // per 32 m cell, sparse
pub struct SurfaceState {
    t_h: f64,                                         // game hours of the last step
    climates: Vec<ClimateSurface>,                    // indexed by ClimateIx
    cells: HashMap<(WorldspaceId, IVec2), CellSurface>, // only non-zero cells
}
pub fn advance(state: &mut SurfaceState, to_h: f64, rates: &dyn RateSource, tuning: &SurfaceTuning);
pub fn sample(state: &SurfaceState, raster: &ClimateRaster, ws: &WorldspaceId, p: DVec2) -> SurfaceSample; // CPU query
```

- **Step** (pure, fixed `step_h` on a grid from hour 0, sub-stepped for rest/wait; catch-up capped at 72 h, a new
  state warms up over the same 72 h; values under half a save quantum snap to 0): per climate,
  `wet += (rates.wet − d)·Δ` with `d = rates.drying` (blended presets; `1/dry_h` where no preset sets it,
  0 while `rates.wet > 0`), clamped to [0, 1]; `puddle` rises at `rates.puddle` only while
  `wet ≥ puddle_from_wet`, dries at `puddle_dry_mult·d`. Per cell, `snow`/`ash` coverage add
  `rates.snow/ash × altitude factor` and decay at `snow_melt_mult·d` / `ash_clear_mult·d` while nothing falls. Cell rates come from `ClimateRaster::site_at(cell centre)` weights × `ClimateRow`s, the same
  border blend the WE4 weather map uses (map.rs), so cover agrees with the far weather.
- **RateSource** is `WeatherState::blend` per climate at the step's mid time: weather is a pure function of
  (seed, overrides, clock), so any peer computes the same rates.
- **Altitude factor** (snow only): `smoothstep(snow_line − 50 m, snow_line + 50 m, cell height)`, `snow_line`
  per climate (default −∞: snow everywhere it snows); as built a `CellGrid::set_altitude` hook (snow line, cell height), no
  record field until a game needs one (Morrowind: off).
- **Cell membership:** a cell becomes active when its rate is positive; it is dropped when both channels
  reach 0. Vvardenfell at 32 m is ~30k cells; only the snowy/ashy part is ever stored.
- **Character wetness** (W8): `CharacterWetness { wet, dried_at }` per actor, presentation, exposure from an
  upward physics ray (under roof or not), drying `dry_h`; immersion from the water query later.

### Records

- `SurfaceLayer` (`sf_records` kind): `{ channel: snow|ash, albedo_height: VfsPath, normal: VfsPath,
  arm: Option<VfsPath>, tiling_m, max_depth_m (visual depth for W6, default snow 0.15, ash 0.03),
  roughness, sparkle, contrast }`. One active layer per channel per worldspace (adapter picks).
- `TerrainLayer.surface` → surface type (terrain.md): gains `porosity: f32`, `puddles: bool`,
  `cover_adherence: f32`, `trail: none|snow|ash|mud|sand` (deformable material, its depth scale).
- Objects: porosity from the material, Lagarde's fallback: `porosity = (1 − metallic)·saturate((rough −
  0.3)/0.5)`; mod override later through the material extras key `surface.porosity` (open question 3).

### GPU resources (`sf_surface_bevy`, per exterior camera)

| Resource | Format, size | Content | Updated |
|---|---|---|---|
| `SurfaceUniform` | uniform | cell window origin, 1/cell, shelter `clip_from_world`, rain intensity at camera, time, ripple params, layer tiling/contrast, flags (layers on) | per frame |
| Cell texture | RGBA8 unorm 128² (4 km, 32 m texels), toroidal | R wet, G puddle (camera site's climate blend per cell), B snow, A ash | strips on recenter; all on a step |
| Shelter map | Depth32 1024² over 128 m (0.125 m/texel), vertical | highest caster depth, top-down | redraw on 16 m recenter / 8 m vertical / streamed casters (every 30 frames max) |
| Noise atlas | RGBA8 256², repeat | R/G tiling value noise (patches, edges), B/A wall streak pattern | static, generated at startup |
| Layer array | RGBA8 2D array 512², 4 layers, repeat, mips | snow albedo+height, snow normal xy+rough+ao, ash ×2 | on record change |
| Trail ring (W6) | RG16F 1024² over 64 m (6.25 cm/texel), toroidal, ×2 (current, previous) | R depression (m, ≤ 0), G lip (m, ≥ 0) | stamps + decay every frame |

The shelter map is a **second, vertical** subview of the WE-5 mechanism (`sf_sky_bevy::sun_view`, as
`sf_precip_bevy/src/occlusion.rs` does): the precipitation map follows the wind slant and turns with it, which
would move wet patches; cover needs a stable top-down view. Outside the 64 m radius, exposure fades to
"open sky" for up-facing surfaces (far roofs are exposed; far interiors are not visible).

## Shader integration

One WESL module, **`sf_surface::layer`** (`sf_surface_bevy/src/shaders/surface_layer.wesl`, registered with
`load_shader_library!`), replaces the no-op `sf_terrain_bevy/src/shaders/surface_state.wesl`:

```wgsl
struct SurfaceInput {
    world_pos: vec3<f32>,   // absolute, WorldOrigin phase (world-coordinates.md)
    height: f32,            // 0..1 height signal: terrain layer height; objects: noise atlas
    basin: f32,             // terrain only: concavity from the height clipmap (puddle sites), else 0
    porosity: f32,
    class: u32,             // STATIC, TERRAIN, DYNAMIC (skinned: wetness only), FAR (wetness only)
    instance_wet: f32,      // DYNAMIC: per-character wetness (W8), else 0
}
fn apply_surface_layer(pbr: ptr<function, PbrInput>, s: SurfaceInput)
```

Order inside: (1) early-out when `flags == 0` or the cell is all zero; (2) exposure (shelter gather, ported
`surfaceExposure` tent, slope rise term); (3) snow/ash cover (W5); (4) wetness + streaks (W3);
(5) puddles + ripples (W4); (6) resolve the summed surface gradient into `pbr.N`. Trail depth (W6) feeds
(3)-(5) as extra height and gradient. It writes only `PbrInput` fields: `material.base_color`,
`perceptual_roughness`, `reflectance` (water F0 0.02 ≈ reflectance 0.35), `N`, `diffuse_occlusion`.

**Hook sites.**
- **`StandardMaterial` (NIF statics, characters, held items, fades):** own patch "SW" in
  `superfargoth/vendor/bevy_pbr`, the GI patch's shape: a `SurfaceWeatherView` render-world component on the
  view sets `MeshPipelineViewLayoutKey::SURFACE_WEATHER` (next free bit) and `MeshPipelineKey::SURFACE_WEATHER`
  (shader def `SURFACE_WEATHER`), adds mesh-view bindings after the GI ones (uniform, cell texture, shelter
  depth, noise atlas, layer array, two samplers; trail ring added in W6), and in `render/pbr.wesl`
  `fragment()` calls `apply_surface_layer` between `pbr_input_from_standard_material` and
  `apply_pbr_lighting`, forward main pass only (not the prepass). Skipped for alpha-blend materials
  (glass, fog translucents) and `unlit`. Class DYNAMIC under `SKINNED`; `instance_wet` from `MeshTag` bits
  0-7 on skinned meshes (W8; 0 = dry). Every hunk carries `superfargoth patch (own, SW)`; README row with
  the drop condition "upstream Bevy gets a view-level material hook / global shader bindings".
- **Terrain:** `terrain.wesl` already calls the hook at the right place (line ~108, before AO and
  specular occlusion so the new roughness feeds specular occlusion). It switches to `apply_surface_layer`;
  `TerrainSurface` (`surface.wesl`) gains `height` (the blended layer height already used for height
  blending) and the hook gets `basin` from the height clipmap (`h_avg(±2 m) − h`, clamped to 0.1 m).
- **Proxies/impostors (`sf_lod_bevy` `proxy.wesl`, `impostor.wesl`):** class FAR, wetness darkening only
  (puddles are sub-pixel at that range).
- **Excluded:** water (`sf_water_bevy`), particles, precipitation, sky.

**Why a patch, not `WorldMaterial = ExtendedMaterial<…>`** (the earlier design): the data is per view, not
per material; one patch reaches every `StandardMaterial` user (~30 spawn sites, characters included) with no
type churn in `MaterialMapper`, `CookedMaterials`, fades or skinning; it repeats the GI patch. Risk: mesh-view
binding count; W3 checks limits first and, if tight, packs the noise atlas into the layer array.

**Prepass and SSAO.** Prepass normals stay unmodified (GTAO sees the base normal): acceptable, the surface
normal changes are small-scale; revisit if the owner sees AO mismatch on deep snow.

## Trails (W6) and why DLSS needs motion vectors

**Ring.** Toroidal, camera-centred (like the height clipmap), re-centred in whole texels; rows entering the
ring clear to 0. **Stamps:** each actor within 40 m with a grounded foot (animation foot-down events, the
same ones footsteps use, plus the planted foot bone pose while planted) and bodies lying down (ragdoll,
knockdown capsules) emit oriented ellipse/capsule stamps: one instanced draw into the ring with
`BlendOperation::Min` on R (depression to the ground: `−cover_depth`) and `Max` on G (a lip ring around the
stamp, profile from ROTTR). **Decay pass** (compute, 1024²): R relaxes toward 0 at `refill × snowfall rate`,
lips flatten with it; mud has no refill. Visual surface height on deformable terrain = ground +
`cover × max_depth_m` + R + G: the snow layer is raised geometry and feet, which the collider keeps on the
ground, appear sunk in it; the stamp carves exactly to the ground (RDR2-style).

**Geometry.** Vertex displacement only in the two finest CDLOD levels (0.25 / 0.125 m spacing, ≤ 64 m);
morph regions fade it out. CDLOD node bounds grow by `max_depth_m` (culling and the occlusion-culling
prepass stay conservative). Vertex normals from central differences of the ring + cover height.
**Fragment:** POM / relief mapping on the composite height (4-8 linear steps + 2 binary refinements, fade
by 15 m, UVs derived from world XZ so DLSS jitter doesn't swim), and the trail's surface gradient from
4-tap finite differences enters the layer's normal sum. **Shadows:** shadow views use the same vertex
function in cascade 0 (prints shadow each other); POM self-shadow only on Ultra.

**Motion vectors (owner asked why).** DLSS and TAA don't render each frame from scratch: they rebuild it
from previous frames, moving each pixel's history by its **motion vector** (where that surface point was
last frame). Static geometry gets this from the camera motion alone. Geometry that moves on its own
(displaced terrain while a print is stamped, snow refilling, the snow layer rising) must write its own
motion in the prepass, or the upscaler fetches the wrong history: smearing and ghost trails behind the
feet. So W6 applies the **same displacement function** in `terrain_prepass.wesl` for the current position
and, with the **previous ring texture** and `previous_view_uniforms`, for the previous position
(`motion = clip_now − clip_prev`). Depth must match the main pass or contact shadows and SSAO break. Only W6
moves geometry: wetness, puddles, ripples and cover (W3-W5) change shading only and need no motion vectors
(ripples are normal-only, never vertex waves).

## Persistence and co-op

- **Save:** section `sf/weather` (v2; v1 migrates with `surface: None`, which warms up from the weather) gains
  `surface: Option<SurfaceSave { t_h, climates: Vec<(RecordId, u16 wet, u16 puddle)>,
  cells: Vec<(WorldspaceId, Vec<(i32, i32, u16 snow, u16 ash)>)> }>`, quantized to 1/65535, only non-zero entries.
  Unknown climates (mod removed) drop silently. Trails, shelter and kick-up are not saved.
- **Load:** the state restores, then `advance` catches up to the clock; the cell texture is rebuilt.
- **Co-op (net.md):** host owns `SurfaceState`. Clients get the `SurfaceSave` bytes on join, then step it
  locally (pure function of clock, seed, overrides, raster); the host resends it every 6 game hours and after
  a script override to bound drift. Gameplay readers (footing later, blight exposure) query on the host.
  Shelter, trails (remote actors stamp from their replicated pose), ripples and kick-up are local
  presentation.

## Budget (RTX 4060 laptop, 1440p + DLSS Quality, GPU ms; estimates, WM measures)

Exterior medians are ~10.5 ms against the 11 ms target (perf.md, 2026-10-28): the headroom is thin, so cost
lands only in weather (dry clear days run the early-out) and every part cuts per preset. This table replaces
weather.md's "Surface state" budget row.

| Part | Low | Medium | High | Ultra |
|---|---|---|---|---|
| Layer ALU in materials (exposure 1 tap Low / 9-gather tent, wet, streaks) | 0.05 (wet only, no shelter) | 0.12 | 0.18 | 0.2 |
| Puddles + ripples | off | 0.03 | 0.05 | 0.05 |
| Snow/ash cover (layer array, height blend) | 0.03 (albedo only) | 0.05 | 0.08 | 0.08 |
| Shelter redraw (amortized; spike ≤ 0.4 ms on a redraw frame) | off | 0.02 | 0.03 | 0.03 |
| Trails: stamps + decay | off | off | 0.04 | 0.05 |
| Trails: displacement (main + prepass + cascade 0) + POM | off | off | 0.25 | 0.4 (POM self-shadow) |
| **Total, worst weather** | **0.08** | **0.22** | **0.63** | **0.81** |

CPU: `advance` per step ≤ 0.05 ms (active cells), cell strip upload ≤ 0.02 ms, stamp gathering ≤ 0.05 ms.
Counters: `surface/advance_cpu_ms`, `surface/active_cells`, `surface/shelter_redraws`,
`surface/trail/gpu_ms`, `surface/stamps`.

## A/B switches and console

- `--off surface` (whole layer, hook compiled in but flags 0), `--off wet`, `--off puddles`, `--off cover`
  (snow + ash), `--off shelter` (exposure 1: diagnostic), `--off trails`, `--off trailpom`,
  `--off trailshadows`, `--off kickup`. Game side: `mw_game_core/src/perf.rs` `Part` table.
- Console: `surface set <wet|puddle|snow|ash> <0..1> [here|<climate>]` (host; forces state),
  `surface clear`, `surface dump` (camera site, cell values, active cells), `surface debug
  <off|exposure|cells|porosity|trails>` (debug tint, like terrain's LOD tint).

## Test plan

Logic, exact assertions (`sf_surface`, headless `sf_surface_bevy`): `advance` hits closed forms under
constant rates (wet reaches 1 at 1/r h, dries in `dry_h`); one 24 h `advance` equals 144 steps bit-for-bit;
puddles stay 0 below `puddle_from_wet`; snow decays 3x and ash 4x slower; cell rates equal the
`ClimateRaster::site_at` blend at cell centres; no-snow climates allocate no cells, empty cells drop; equal
inputs give equal states; save → load → advance equals advance → save → load within one quantum; unknown
climates drop on load; cell window recenter writes exactly the entering strips; shelter matrix and redraw
triggers; trail stamp/clear/decay CPU twins; shader compile per preset's defs (log line, no picture).
Look and feel: no automated visual checks. Each slice registers a scene in
`bevyrrowind/docs/visual-checks/weather-sky.md` (`SW-n`) + index line, "awaiting owner", with camera, time,
weather override, flags and what to look for.

## Build plan (one deliverable + test + owner check each)

| Slice | Deliverable | Test | Owner check |
|---|---|---|---|
| **W1** ✅ | `sf_surface` crate: state, `advance`, `sample`, `SurfaceSave` codec + `sf/weather` section field; `sf_weather_bevy` drives it on game time; console `surface set/dump/clear` | exact logic list above (rates, sub-steps, determinism, save round-trip) | none (logic); `surface dump` line in the handoff. As built 2026-11-05: rates = presets' `surface` × PRECIP blend weight (`WeatherRates`); cells over the map's raster (`WorldWeatherMap::with_worldspace`) |
| **W2** | `sf_surface_bevy`: cell texture window, vertical shelter subview, noise atlas, `SurfaceUniform`, `SurfaceWeatherView`; debug tint `exposure` / `cells` through the terrain hook only | window/recenter tests, shelter matrix and redraw triggers, headless plugin smoke (no NaN, resources exist) | SW-2: debug tint shows roofs, trees and arches as sheltered in Balmora |
| **W3** | Wetness layer: vendored `bevy_pbr` patch "SW", `sf_surface::layer` (exposure port, Lagarde wet, film normal flatten, wall streaks), terrain + proxy calls, `--off wet/surface`, NOTICES entry | shader-def compile log check per preset; porosity heuristic unit test (WGSL mirrored in Rust) | SW-3: Seyda Neen after rain, wet vs dry A/B, under-roof dry, walls streaked |
| **W4** | Puddles: terrain height + basin fill, object flat-up fill, edge blend, procedural ripples (Clockwork rings) in rain, F0/roughness, sky/probe reflection | level/edge math unit tests (Rust twin); compile check | SW-4: Balmora streets in rain and after; ripples stop when rain stops |
| **W5** | Snow/ash cover: layer array from `SurfaceLayer`, per-pixel `N·up` × shelter × cell, height blend, surface-gradient detail normals; ash on statics | height-blend and coverage math twins; record parsing | SW-5: Ghostgate after an ash storm, Solstheim snow; no cover under roofs or on characters |
| **WM** | Measurement: counters + `perf suite` run lines in perf.md for wet/ash/snow bookmarks (`--weather` overrides); owner runs it (agents never launch) | `#[ignore = "measurement: ..."]` CPU timings | owner runs the suite; table updated |
| **W6** | Trails: ring, stamps from foot events/bodies, decay/refill, CDLOD displacement (finest 2 levels), prepass motion vectors with the previous ring, cascade-0 shadows, POM | ring/stamp/decay CPU twins; prepass and main use one displacement fn (compile check) | SW-6: walk through Solstheim snow and Ashlands ash, DLSS on: prints, lips, no smear |
| **W7** | Kick-up: footstep splashes in puddles, snow/ash puffs from the stamp stream, sprite-sheet bursts (PR2's splash pipeline; `bevy_hanabi` once the crate batch lands) | burst count/lifetime logic | SW-7: running in puddles and snow |
| **W8** | Character wetness: `CharacterWetness`, roof ray, `MeshTag` channel on skinned meshes | drying/exposure logic | SW-8: walk from rain into a doorway, clothes dry over minutes |

Order: W1 → W2 → W3 → W4/W5 (either) → WM → W6 → W7 → W8. W6 waits for WM headroom.

## Open questions

1. Ash and snow together: the layer applies snow then ash on top; is blight its own tint (second ash layer)
   or the same ash? (default: same.)
2. Wind-slanted exposure during storms (lee of walls stays clean): blend in the precipitation map with a
   slow temporal filter, or accept vertical only? (default: vertical; WE-5 follow-up already asks to cap
   the slanted shelter.)
3. Per-object porosity override: material extras key in the cook (needs a per-material channel; `MeshTag`
   bits on statics) or heuristic only? (default: heuristic until a mod needs it.)
4. Canopy: trees shelter fully in a depth map; Clockwork holds back 40% under leaves (`LEAF_HOLD`). Port
   the leaf term, or add a foliage channel to the shelter view?
5. Trail radius 40 m and 6.25 cm texels: confirm after WM; displaced shadows beyond cascade 0?
6. Mesh-view binding count headroom under the vendored `bevy_pbr` (W3 measures first).
