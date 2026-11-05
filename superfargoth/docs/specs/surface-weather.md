# Surface weather: wetness, puddles, snow and ash cover, trails

Status: W1-W4 + W2b built (2026-11-05: `sf_surface` state + rain wash, `sf_surface_bevy` GPU resources, light-independent shelter + canopy maps, debug tints, wetness on every lit material through `bevy_pbr` patch "SW", puddles with rain rings; [as built](surface-weather-as-built.md)); W5-W8 spec. Slices W1-W7 (+ WM measurement, W8 character wetness) below.
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
| Puddles | height/porosity-driven water level, flat normal, F0 0.02, rain ripple normals (Lagarde 2a ripple atlas or procedural rings) | Remember Me, Watch Dogs, Unity Production Ready Weather sample, Clockwork | Lagarde water level against a height (terrain layer height, object albedo luminance) over a pool score (noise + terrain basin), soft shoreline; procedural rings (OpenMW/Clockwork ring profile, no texture, no mip problem) on 4 drop grids turned on per quarter of rain (Lagarde's ripple layers); sky/probe reflection | match (SSR later) |
| Puddle reflection | env/probe → planar for big pools → SSR | UE (SSR + reflection captures), RDR2 **(verify)** | env map + GI probes; SSR parked (no depth prepass on the exterior camera) | gap, owner-accepted |
| Exposure / shelter | top-down precipitation occlusion depth map | Tatarchuk "Rain" (ATI ToyShop, 2006), Remember Me (top-down rain depth map, fxguide 2013), S.T.A.L.K.E.R. Clear Sky wet mask (forum recollection), Clockwork `surfaceRoof` | vertical depth map 1024² over 128 m, tent-filtered gather; leaves in a second (canopy) map, partial shelter (Clockwork `roofLeaves`) | match 
| Snow/ash cover | world-aligned blend: `N·up` × exposure × amount, height-blend, noise patches | UE world-aligned blend, Uncharted 4/RDR2 snow **(verify)**, Clockwork `surfaceCovers` | same, per-pixel N, surface-gradient normals | match |
| Cover state | per-region or per-cell scalars over game time | Clockwork (per region, not saved), RDR2 (persistent snow, **verify**) | climates (wet) + 32 m cells (snow, ash), saved | match+ |
| Trails | camera-centred toroidal height render target, stamped by feet/bodies, refilled; displacement + relief mapping, lip normals | Batman: Arkham Origins (Barre-Brisebois, GDC 2014), Rise of the Tomb Raider (GPU Pro 7, **verify**), God of War 2018, RDR2 | ring 1024² over 64 m, vertex displacement of finest CDLOD + POM; no tessellation (wgpu has none) | match (console path) |
| Kick-up | particle bursts on foot contact by surface | RDR2, GoW | instanced sprite bursts from the stamp stream | match |

Clockwork: footprints are a hashed decal list (mask, no displacement), nothing saved, actors skip covers (we
gate skinned meshes alike). Rafael's OpenMW shader pack: visual reference only (licence unknown).

## Data model

### CPU state (`sf_surface`, Bevy-free)

Built in W1 ([as built](surface-weather-as-built-archive.md) W1, [W2b](surface-weather-as-built.md)): `SurfaceState { t_h, climates: Vec<ClimateSurface {
wet, puddle }>, cells: per worldspace HashMap<IVec2, CellSurface { snow, ash }> }`, record-backed `SurfaceTuning`
(step 10 game min, dry 2 h, decay multipliers, `puddle_from_wet` 0.7, rain wash of snow and ash, `storm_wet_rate`
for rain rings), pure fixed-step `advance` from the weather blend per climate (cells weighted by
`ClimateRaster::site_at`), CPU `sample`. Only non-zero cells are stored (Vvardenfell at 32 m is ~30k cells).

- **Character wetness** (W8): `CharacterWetness { wet, dried_at }` per actor, presentation, exposure from an
  upward physics ray (under roof or not), drying `dry_h`; immersion from the water query later.

### Records

- `SurfaceLayer` (kind `surface.layer`): `{ channel: snow|ash, albedo_height: VfsPath, normal: Option<VfsPath>,
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
| Shelter map | Depth32 1024² over 128 m (0.125 m/texel), vertical | highest caster depth, top-down | redraw on 16 m recenter / 8 m vertical / streamed or newly foliage-marked casters (every 30 frames max) / origin shift; only while a layer or debug view is on; day and night (own anchor, W2b) |
| Canopy map (W2b) | same as the shelter map | highest leaf caster (`ShelterFoliage`) | with the shelter map, same view |
| Noise atlas | RGBA8 256², repeat | R/G tiling value noise (patches, edges), B/A wall streak pattern | static, generated at startup |
| Layer array | RGBA8 2D array 512², 4 layers, repeat, mips | snow albedo+height, snow normal xy+rough+ao, ash ×2 | on record change |
| Trail ring (W6) | RG16F 1024² over 64 m (6.25 cm/texel), toroidal, ×2 (current, previous) | R depression (m, ≤ 0), G lip (m, ≥ 0) | stamps + decay every frame |

The shelter and canopy maps are two vertical views of Bevy's own shadow pass (`sf_sky_bevy::sun_view`, like the
WE-5 precipitation map, which follows the wind slant and would move wet patches), owned by a light-independent
`DepthViewAnchor` (W2b: drawn at night too). Leaves the game marks `ShelterFoliage` draw into the canopy map only;
under a full canopy `SHELTER_LEAF_PASS` 0.4 of the rain gets through (why and how: [as built](surface-weather-as-built.md)
W2b). Outside the 64 m radius, exposure fades to "open sky" (far roofs exposed; far interiors not visible).

## Shader integration

One WESL module, **`sf_surface_bevy::layer`** (binding-agnostic: callers pass the uniform and textures), plus
`sf_surface_bevy::view` (the mesh-view bindings and the entry points materials call; CPU twin of the wet math:
`sf_surface::layer`):

```wgsl
struct SurfaceInput {
    world_pos: vec3<f32>,   // render space (uniform window corner and shelter matrix follow the origin)
    height: f32,            // 0..1 height signal, lows fill first: terrain layer height; objects albedo luminance
    basin: f32,             // terrain only: concavity from the height clipmap, meters (puddle sites), else 0
    porosity: f32,
    surface_class: u32,     // (`class` is reserved in WGSL) STATIC, TERRAIN, DYNAMIC (skinned: wetness only), FAR (wetness only)
    instance_wet: f32,      // DYNAMIC: per-character wetness (W8), else 0
    footprint: f32,         // meters per pixel (screen derivatives; rain rings fade below a pixel), 0 unknown
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
  `superfargoth/vendor/bevy_pbr`, the GI patch's shape: a `SurfaceWeatherView` render-world component on the view
  adds mesh-view bindings 43-51 (uniform, cells, shelter, noise, two samplers, canopy, W5a layer array + sampler; W6
  the trail ring) and the `SURFACE_WEATHER` def, and `render/pbr.wesl` calls `sf_surface_bevy::view` before
  lighting, forward main pass only (detail: [as built](surface-weather-as-built.md) W3). Skipped for alpha-blend and
  `unlit`; class DYNAMIC under `SKINNED` (`instance_wet` from `MeshTag` bits 0-7, W8). Drop condition: "upstream
  Bevy gets a view-level material hook / global shader bindings".
- **Terrain:** `terrain.wesl` calls the hook on the same view bindings (before AO and specular occlusion so
  the new roughness feeds specular occlusion), with `TerrainSurface.height` (the blended layer height) and, while
  puddles are on, `basin` from the height clipmap (`h_avg(±2 m) − h`, ≥ 0; the layer saturates it at 0.1 m).
- **Proxies/impostors (`sf_lod_bevy` `proxy.wesl`, `impostor.wesl`):** class FAR, wetness darkening only
  (puddles are sub-pixel at that range).
- **Excluded:** water (`sf_water_bevy`), particles, precipitation, sky.

**Why a patch, not `WorldMaterial = ExtendedMaterial<…>`**, and the binding count: [as built](surface-weather-as-built.md) W3.

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

CPU: `advance` per step ≤ 0.05 ms (active cells), cell strip upload ≤ 0.02 ms (a full window rewrite on a step: measured 0.73 ms worst case, every cell covered), stamp gathering ≤ 0.05 ms.
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
puddles stay 0 below `puddle_from_wet`; snow decays 3x and ash 4x slower; rain washes ash faster than snow, both faster than dry decay (closed forms); cell rates equal the
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
| **W1** ✅ | `sf_surface` state, `advance`, `sample`, save field, console (as built) | exact logic list above | none (logic) |
| **W2** ✅ | `sf_surface_bevy` cell window, shelter subview, noise atlas, uniform, debug tints (as built) | window, shelter, plugin smoke | SW-2 (awaiting owner) |
| **W2b** ✅ | Rain wash; shelter views on an own anchor (night); canopy map for `ShelterFoliage` (as built) | wash closed forms; anchor + canopy requests; shader compile | SW-2 at night, under trees |
| **W3** ✅ | Wetness layer: vendored `bevy_pbr` patch "SW", `sf_surface::layer` (exposure port, Lagarde wet, film normal flatten, wall streaks), terrain + proxy calls, `--off wet/surface`, NOTICES entry | shader-def compile log check per preset; porosity heuristic unit test (WGSL mirrored in Rust) | SW-3: Seyda Neen after rain, wet vs dry A/B, under-roof dry, walls streaked |
| **W4** ✅ | Puddles: terrain height + basin fill, object flat-up fill, edge blend, procedural rain rings, F0/roughness, sky/probe reflection, `--off puddles` (as built) | level/edge/ring math twins; compile check | SW-4: Balmora streets in rain and after; rings stop when rain stops |
| **W5** ✅ (W5a records + array, W5b shader; as built) | Snow/ash cover: layer array from `SurfaceLayer`, per-pixel `N·up` × shelter × cell, height blend, surface-gradient detail normals; ash on statics | height-blend and coverage math twins; record parsing | SW-5: Ghostgate after an ash storm, Solstheim snow; no cover under roofs or on characters |
| **WM** | Measurement: counters + `perf suite` run lines in perf.md for wet/ash/snow bookmarks (`--weather` overrides); owner runs it (agents never launch) | `#[ignore = "measurement: ..."]` CPU timings | owner runs the suite; table updated |
| **W6** 🔶 (W6a ring, stamps, refill, shading; as built) | Trails: ring, stamps from foot events/bodies, decay/refill, CDLOD displacement (finest 2 levels), prepass motion vectors with the previous ring, cascade-0 shadows, POM | ring/stamp/decay CPU twins; prepass and main use one displacement fn (compile check) | SW-6: walk through Solstheim snow and Ashlands ash, DLSS on: prints, lips, no smear |
| **W7** | Kick-up: footstep splashes in puddles, snow/ash puffs from the stamp stream, sprite-sheet bursts (PR2's splash pipeline; `bevy_hanabi` once the crate batch lands) | burst count/lifetime logic | SW-7: running in puddles and snow |
| **W8** | Character wetness: `CharacterWetness`, roof ray, `MeshTag` channel on skinned meshes | drying/exposure logic | SW-8: walk from rain into a doorway, clothes dry over minutes |

Order: W1 → W2 → W3 → W4/W5 (either) → WM → W6 → W7 → W8. W6 waits for WM headroom.

## Open questions

1. Ash and snow together: the layer applies snow then ash on top; is blight its own tint (second ash layer)
   or the same ash? (default: same.)
2. Wind-slanted exposure during storms (lee of walls stays clean): blend in the precipitation map with a
   slow temporal filter, or accept vertical only? (default: vertical; WE-5 follow-up already asks to cap
   the slanted shelter. Rain-slant 2026-11-06 capped and smoothed the drop lean itself, `weather/wind.md`; the lee-distance
   cap is still open.)
3. Per-object porosity override: material extras key in the cook (needs a per-material channel; `MeshTag`
   bits on statics) or heuristic only? (default: heuristic until a mod needs it.)
4. Leaf pass: the owner said "~40% passes through (Clockwork's value)", but Clockwork's `LEAF_HOLD` 0.4 lets
   60% through. Built with the owner's 40% (`SHELTER_LEAF_PASS`); confirm at SW-2. (Canopy map: resolved W2b.)
5. Trail radius 40 m and 6.25 cm texels: confirm after WM; displaced shadows beyond cascade 0?
6. Leaf marks follow the game's leaf-material rule (diffuse transmission): with `--off foliagetransmission` leaves
   shelter fully. A per-material foliage flag in the cook would decouple them (with question 3). (Night: resolved W2b.)
