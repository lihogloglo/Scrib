# Water: ocean, water bodies at any height, rivers, underwater, buoyancy (M6)

Status: draft (2026-10-13, m6-spec). W1 `sf_water` core built (2026-10-13, m6-w1): see "As built (W1)".
Crates (new): `sf_water` (Bevy-free: records, CPU water query, wave spectrum + CPU twin, buoyancy
math, bake steps `water.shore`, `water.hydro`, `water.flow`, `water.caustics`, `water.pontoons`),
`sf_water_bevy` (thin Rust, heavy WESL: FFT compute, surface mesh, water pass, water-level clipmap,
reflections, underwater, dynamic ripples, floaters).
Touches: `sf_physics` (a per-step force API), `sf_surface_bevy` (the surface hook learns "below water":
caustics, wet band), `sf_atmos_bevy` (froxels gain a water medium; the water pass calls the fog
function), `sf_precip_bevy` (ripples and splashes on water), `sf_controller` (`Swim` gets its surface
from the query), `sf_world` (`Worldspace::sea_level` becomes the default ocean body).
Adapter spec (first content pack): `../../../bevyrrowind/docs/specs/water-adapter.md`.
Related: `weather.md` (wind, precipitation, surface state, fog, budgets), `terrain.md` ("Water"),
`physics.md`, `locomotion.md` ("Swim"), ARCHITECTURE §7.2, WISHLIST §4.

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: draft; W1 `sf_water` core built, `sf_water_bevy` has FFT ocean surface, material, refraction snapshot and motion vectors (ROADMAP water polish item is marked [p]). Designed, not built: W4 reflections, W5 underwater, W6 buoyancy, W7 rivers, water bodies at any height.
> Crates: `sf_water` (Bevy-free: records, CPU `WaterQuery`, spectrum, bake steps), `sf_water_bevy`; adapter `water-adapter.md`; `mw_water`. Decisions: render order opaque, water, other translucents, precipitation last ("Render order"); water casts no shadow (Open questions 2); buoyancy runs on CPU via `sf_physics` ("Buoyancy"). Hard problem H3 covers draw order.
> Read next: "The water query" and "Ocean surface" for code; "Water pass, refraction, reflections" for rendering; "Render order" for pass placement.
<!-- /agent-summary -->

## Goal

ARCHITECTURE §9 M6 and WISHLIST §4: "ocean (evaluate `bevy-aqua`), water bodies at any height, baked
rivers to the sea." M6 is done when, among others, "the Bitter Coast in a rainstorm: puddles, wet
wading, a boat that floats" runs within the weather.md budgets.

1. **One water model.** Water bodies with kinds (ocean, lake, river, pool), each with a surface, a
   level, optics and waves. One CPU query (height, normal, velocity, depth, body) that rendering,
   buoyancy, swimming, wetness, sound and AI all share (§10 early decision 4).
2. **An ocean** with real waves (spectral, cascaded), foam, a shoreline that damps waves and draws foam
   lines, depth-dependent colour, refraction, reflections and caustics, to the horizon.
3. **Water at any height**: lakes, ponds and interior pools with their own levels and optics.
4. **Rivers that flow to the sea**, derived at bake time when the content has no river data.
5. **Underwater**: absorption and scattering, light shafts, a clean camera transition through the
   surface (including a camera split by the waterline).
6. **Buoyancy** on the CPU through `sf_physics`: a boat floats and rocks; wooden items float, iron sinks.
7. **Hooks** for wading, character wetness, footsteps and splashes, all from the query.
8. **Large worlds**: exact under the floating origin; nothing jumps on an origin shift.

Non-goals: fluid simulation (SPH/FLIP, flooding, water that flows over terrain at runtime), breaking
waves and barrels, ship sailing physics (sails, keel lift: a vehicle spec later), waterfalls as
simulated volumes (authored meshes keep rendering; they can get the water material), ice, tides,
interactive water level changes at runtime beyond a script setting a body's level.

Moved to [history](archive/water-history.md#research-summary-2026-10-13).

## Reuse decision

| Need | Bevy | Crate | Ours, and why |
|---|---|---|---|
| Wave spectrum, FFT, cascades, foam | none | `bevy-aqua` (0.19, no origin, GPU queries) | **Ours, porting aqua's code** (MIT/Apache) into `sf_water_bevy` shaders; GodotOceanWaves as reference |
| CPU wave twin | n/a | `rustfft` | **`rustfft`** for the lowest cascade(s) |
| Water shading (absorption, scattering, refraction) | transmission (colour only, no depth, no MVs) | aqua's optics | **Ours**: a water material drawn after a colour + depth snapshot (UE Single Layer Water structure) using Bevy's PBR lighting functions |
| Sky reflection | `AtmosphereEnvironmentMapLight` (used) | n/a | n/a |
| Local reflections | SSR (needs deferred + depth prepass) | aqua planar cameras | **Ours: SSPR** for the water plane, environment map and light probes as fallback |
| Water bodies, query | none | aqua (one sea level, GPU) | **Ours** (§10 decision 4) |
| Rivers without data | none | none (WhiteboxTools is an MIT application, not a library) | **Ours**: priority-flood + D8 in `water.hydro` (~300 lines) |
| Buoyancy | none | none on rapier | **Ours**: pontoons over `body.cook` shapes, forces through `sf_physics` |
| Underwater fog | `DistanceFog` (no medium) | aqua (homogeneous, no waterline closure) | **Ours**: froxel water medium + an analytic post for Low |

## Canonical technique (overview)

UE/Crest/HDRP structure, sized to Bevy:

```
Bake:  terrain heights ─► water.shore (per tile: shore distance, openness)   water.hydro ─► water.flow
       body.cook ─► water.pontoons          adapter WaterSource ─► WaterBody records
CPU:   WeatherFrame.wind ─► Spectrum (h0) ─► GPU upload + CPU twin (cascade 0, rustfft) at 60 Hz
       WaterQuery (bodies BVH + twin + flow) ─► buoyancy forces ─► sf_physics step
                                             ─► swim/wade, character wetness, cues, camera medium
GPU:   compute: FFT cascades (amortized) · foam accumulate · water-level clipmap (on move)
                · dynamic ripples (iWave, 32 m around the camera)
       opaque (terrain/WorldMaterial hook: caustics + wet band below/at the waterline)
       Bevy render_sky · sf_atmos composite (clouds, froxel fog)
       water snapshot (copy colour + depth) · SSPR (compute, 1 pass)
       water pass: clipmap surface, refraction, absorption/scattering, reflections, foam, fog
       water velocity (motion vectors) · underwater post (when the camera is below or split)
       transparents (precipitation, particles) · post (bloom, tonemap, DLSS/TAA)
```

## Data model

Records (`sf_records` kinds; adapters generate them, mods patch them field by field):

```rust
pub struct WaterBody {
    pub id: RecordId,
    pub worldspace: WorldspaceId,
    pub kind: WaterKind,              // Ocean | Lake | River | Pool
    pub shape: WaterShape,            // Infinite (ocean: everything below `level` not cut by land)
                                      // | Polygon { outline: Vec<DVec2> (world XZ), holes }
                                      // | Ribbon { spline: Vec<RiverPoint> }   (river: per-point
                                      //   centre, width, surface height, flow speed)
                                      // | Bounds (interiors: the worldspace's bounds)
    pub level: f64,                   // world Y of the rest surface (rivers: per point)
    pub priority: i32,                // overlap rule: higher wins (a lake inside the ocean's area)
    pub optics: RecordRef<WaterOptics>,
    pub waves: WaveTerms,             // cascades used (ocean: all; lake: the short ones, fetch-scaled;
                                      // river: flow normals only; pool: ripples only), amplitude scale
    pub flow: Option<FlowSource>,     // baked flow map product id (rivers, sea-level channels)
}
pub struct WaterOptics {              // per metre, linear RGB
    pub absorption: [f32; 3], pub scattering: [f32; 3], pub phase_g: f32,
    pub roughness: f32, pub foam: FoamTerms, pub refraction_strength: f32,
    pub caustics: f32,                // strength multiplier
    pub surface_tint: Option<[f32; 3]>,   // floating film (swamp scum, ash); none by default
}
pub struct WaterSample {              // the CPU query result
    pub body: Option<RecordId>, pub kind: WaterKind,
    pub height: f64,                  // displaced surface at this XZ (world Y)
    pub normal: Vec3, pub velocity: Vec3,  // flow + wave orbital velocity
    pub depth: f32,                   // surface − bed (terrain heights), metres
}
```

`Worldspace::sea_level: Option<f32>` (streaming.md) stays as the shorthand: `sf_water` turns it into an
`Ocean` body (exteriors) or a `Bounds` pool (interiors) unless the content's `WaterSource` lists bodies
itself. That keeps every existing worldspace record valid.

**Optics blending.** An adapter hook `WaterOpticsSource::optics_at(ws, WorldPos) -> RecordRef` lets one
body change optics by area (a swamp inside the ocean). A bake step rasterizes it per worldspace like
`weather.climate` (border band 128 m). The water pass blends the two nearest optics.

## The water query (CPU)

```rust
pub trait WaterQuery {
    fn sample(&self, ws: &WorldspaceId, at: WorldPos) -> Option<WaterSample>;   // state of the last step(t)
    fn sample_many(&self, ws: &WorldspaceId, at: &[WorldPos], out: &mut [Option<WaterSample>]);
    fn immersion(&self, ws: &WorldspaceId, capsule: Capsule) -> Immersion; // depth, fraction, sample
}
```

- **Bodies**: a 2D BVH per worldspace over the bodies' XZ bounds; priority resolves overlaps. Ocean =
  "below `level` and not inside a higher body". Depth = `height − TerrainHeights::height_at(xz)`.
- **Waves**: the ocean's CPU twin evaluates cascade 0 (the longest waves; cascade 1 too on High when
  the sea state is rough) on the same 60 Hz fixed step as physics: the same `h0` spectrum the GPU uses
  (generated on the CPU from the seed and uploaded), `rustfft` 2D inverse FFT of 64², displacement
  grids sampled with a Catmull-Rom cubic (W1: bilinear was 4-8 cm off). Displaced height at an XZ needs inverting the horizontal displacement:
  3 fixed-point iterations (HDRP and Crest do the same). Short cascades (chop, ripples) are left out:
  they don't move a boat (HDRP's "evaluate ripples" off). Twin vs GPU agree within 1 cm (test).
- **Flow**: river/channel flow maps (bake) sampled bilinearly; orbital velocity from the twin.
- **Determinism**: pure function of `(seed, spectrum inputs, time)`. Saves store nothing for water.
- **Cost**: ≤ 2 µs per sample (bodies + 3 bilinear fetches + iterations), twin FFT ≤ 0.05 ms per step.

## Ocean surface

**Spectrum.** JONSWAP with fetch + TMA depth correction + Donelan-Banner spreading + optional swell
(Horvath 2015), from `WindState` (weather/wind.md) speed and direction and a per-body fetch. Cascades are
square tiles with **power-of-two sizes in metres** (default 4: 256, 64, 16, 4 m; 256² High, 128² Low),
each band-limited so they don't double-count energy (boundary at the wavelength `size(i+1)/2`, W1). **Weather changes**: the FFT is linear, so a
weather transition blends `h0 = mix(h0_a, h0_b, w)` (two spectra, re-uploaded when `w` moves 2 %); the
CPU twin does the same. No pop when the wind rises.

**Floating origin.** Waves are a function of absolute XZ. Each cascade samples at
`rel_xz + phase_k`, `phase_k = (origin_cell · 256 m) mod size_k`, computed from integer cells (exact),
like the clouds' `CloudsOrigin` and the wind's phase. Tiles divide 256 m (or 1 024 m if a game wants
longer swells), so an origin shift moves no wave.

**Simulation (GPU, compute).** Per cascade: spectrum → time evolution → inverse FFT (Stockham
radix-2/4 in WGSL; aqua's passes) → displacement (xyz) + derivatives → normal/Jacobian textures, mip
chain. Amortized: the longest cascade updates every 2nd frame on Medium (sub-pixel at that scale).
**Foam**: whitecap coverage from the Jacobian, accumulated in a per-cascade texture with decay
(Dupuy and Bruneton 2012; aqua's persistent foam).

**Mesh.** A camera-centred **geometry clipmap**: 7 rings of 128×128 quads (High), base spacing 0.25 m,
each ring double the previous, snapped to twice its spacing, vertices morphed at ring edges (CDLOD
style, as our terrain), plus a horizon skirt to 32 km (Bevy's aerial-view LUT range). One instanced
draw per ring. Vertex shader: displacement from the cascades whose size the ring resolves (cascade
fade by ring), shore attenuation (below). Blocks whose terrain is everywhere above `level + max wave`
(from `water.shore`'s per-block mask) are not drawn, so the sea under an island costs nothing.
Not chosen: projected grid (swims and aliases at the horizon, hard to cull), quadtree of tiles (more
draws, same result).

**Far shading.** Past the finest cascade's Nyquist distance, normals come from the mips and the
lost slope variance goes into roughness (Bruneton 2010): the sun glitter widens instead of aliasing.
Far horizon: aerial perspective (Bevy) applies in the water pass (below).

## Shoreline and depth colour

**`water.shore` bake** (per terrain tile, per body that reaches it; keyed by terrain tile + body):
- signed shore distance (R16, 2 m texels), from the land/water mask at rest level by an exact EDT
  (Felzenszwalb-Huttenlocher), ± 256 m range;
- **openness** (R8): geodesic distance through water to open water (the body's "far" region),
  normalised. Enclosed water (swamp pools, canals, harbours) gets low openness: wave amplitude
  × `openness` (a fetch proxy), chop kept for wind ripples. Computed per connected water component at
  rest level (worldspace pass, 2 m grid): a component that doesn't reach the open region (an inland
  lake that is part of a single-level ocean body) uses its own extent as fetch instead;
- a per-block "land only" bit for mesh culling;
- `TerrainChanged` (persistent edits, M7) re-runs the tile.

**Runtime depth** is not baked: the **water-level clipmap** (below) gives the surface, the terrain's
height clipmap gives the bed.

**Shore effects** in the water pass and vertex shader:
- wave attenuation: displacement × `smoothstep(0, d_shore, depth)` × openness; TMA already shortens
  waves in shallows;
- shore foam: a foam band at shore distance 0-6 m modulated by a wave-phase signal travelling shoreward
  (aqua's shoreline foam), plus **contact foam** where the scene depth behind the water is within a few
  cm of the surface (rocks, piers, boats, legs);
- **wet band** on land: the surface hook darkens and smooths terrain within the swash height above the
  level (`sf_surface`'s wetness, so it also dries).

**Depth colour** (the water pass, per pixel): view path length `L` through water from the snapshot
depth (refracted ray, clamped), Beer-Lambert `T = exp(−σt·L)` per channel; single-scattering
in-scatter `σs/σt·(1 − T)·(sun·phase(g)·shadow·exp(−σt·depth_below_surface) + sky ambient)`. Sun
shadow = CSM + far shadow map + cloud shadow (`DirectionalLightTexture`), so shadowed water is darker
inside, not only on its specular. A crest scattering boost (thin wave tops lit from behind, from the
displacement height) gives the green-blue backlit look.

## Water pass, refraction, reflections

**Pass structure (UE Single Layer Water, Bevy terms).** After `render_sky` and the `sf_atmos` composite:
1. **Snapshot**: copy scene colour (HDR) and depth into two textures (1440p: ≈ 0.1 ms). The colour copy
   is also what Bevy's transmission would take; we take it after the sky and fog so the bed behind the
   water is already fogged.
2. **SSPR** (compute, Cichocki 2017) for the dominant plane: the camera's nearest body's level (the sea
   level covers most exterior views; interior pools their own). Each opaque pixel above the plane is
   projected to its mirror pixel with an atomic-min depth hash; a resolve pass fills holes. One pass,
   resolution ½ on Medium. Waves perturb the lookup by the normal; misses fall back.
3. **Water draw**: the surface meshes as a Bevy material (`ExtendedMaterial` or a full `Material` in
   our own phase, decided at W2 by what keeps the view bindings: lights, shadows, environment map),
   depth-tested against the scene and **depth-writing**, so precipitation and particles sort against
   it. Shading: GGX sun and moon specular (Bevy's functions, slope-variance roughness), clustered point
   lights, reflection = SSPR hit → local light probe (where content has one) → environment map, with
   Fresnel; refraction = snapshot colour at `uv + normal.xz·strength·saturate(depth_behind)`, rejected
   (falls back to the unrefracted uv) when the sampled depth is in front of the water; depth colour;
   foam; rain ripple normals (weather/precipitation-surface.md's ripple atlas × rain intensity); dynamic ripple normals;
   flow-advected normals (rivers). Then fog: aerial perspective from Bevy's aerial-view LUT and the
   froxel fog through `sf_atmos::fog` (weather/fog.md "Fog", step 4), so far water fades like terrain.
4. **Velocity**: the water mesh is drawn again into the motion-vector texture (displacement now vs the
   previous frame's, kept one frame), depth-tested, colour off. Without it DLSS/TAA would smear waves
   with the bed's motion. Foam and sparkle also write the reactive/bias mask once weather.md open
   question 9's upstream patch exists.
5. **Underwater post** (only when the camera is below or split, next section).

Not chosen: Bevy `Transmissive3d` (no depth, no MVs, unordered vs the sky); planar reflection cameras
(aqua's default: a second render of the streamed world, ≈ the cost of the main opaque pass; kept as a
possible Ultra option for small interior pools only, not planned); Bevy SSR (needs deferred + depth
prepass on the exterior camera, weather.md open question 4: if that resolves, water can use it for
non-planar hits).

**Off-screen reflections.** SSPR only knows what is on screen; the environment map knows only the sky.
For the far shore and mountains that leave the screen (looking down at the sea), a **camera-centred
dynamic reflection probe** (GTA V / RDR2-style real-time cubemap): 256² faces of sky + terrain + far-field
proxies only (no L0 objects, no shadows), one face re-rendered every 4th frame (a full refresh ≈ 0.4 s),
filtered by Bevy's environment-map filtering, used as the water's (and wet surfaces') fallback instead of
the sky-only map on Medium+. If filtering a runtime cubemap proves too costly, refresh it only when the
camera moves 64 m or the sun turns 1°.

**Solari path.** Water is not in the TLAS (Solari BLASes are opaque-only). The water pass runs after
Solari's output with the same snapshot design. Reflections: SSPR + probes/environment, and later
**RT reflections from the water pass** by ray queries against Solari's TLAS, shading hits from
Solari's world cache (open question 3). Underwater beds lit by Solari get no water absorption of
sunlight (they're lit as if dry); the water pass's transmittance covers the view path only. Accepted
for M6.

## Render order (transparent side; rainwater, 2026-10-19)

Whole-frame picture (every pass, fog/shadow/prepass per material), the rules and the guard test: [render-order.md](render-order.md) (renderorder, 2026-10-19).

**Canonical order** (UE's Single Layer Water pass before translucency; Unity's render queues
Transparent < Overlay; Frostbite's "refractive first, then sorted translucency, then particles"):
the opaque scene, then the **water** (it refracts the opaque scene, so nothing translucent may be under
it in the snapshot), then the **other translucents** back to front, then **particles and
precipitation** last.

**As built (Bevy 0.20 Core3d):**

| Step | Node / phase | What | Sort |
|---|---|---|---|
| 1 | `main_opaque_pass_3d` (Opaque3d, AlphaMask3d) | terrain, statics, impostor/proxy meshes | binned |
| 2 | sky, `sf_atmos` fog composite | froxel fog on opaque pixels | (fullscreen) |
| 3 | `water_snapshot` (after everything before the transparent pass) | colour + depth copy for refraction | (fullscreen) |
| 4 | `main_transparent_pass_3d` (Transparent3d), band **Refractive** | ocean + water surfaces (Blend, depth-write) | `AlwaysOnTop` = drawn first |
| 5 | same phase, band **Sorted** (default) | blended NIF meshes and leaves swapped to `FroxelFogMaterial` (`transparent_fog.rs`), additive NIF particles / flames (`AlphaMode::Add`), `sf_lod_bevy` impostor light sprites (Add) | Bevy: view depth of mesh AABB centre + `depth_bias`, back to front |
| 6 | same phase, band **Overlay** | `sf_precip_bevy` boxes (64 m, 24 m, 8 m) | `OVERLAY_BASE` + order: after every sorted draw, far box first |
| 7 | `water_velocity` (after the transparent pass) | water motion vectors for DLSS/TAA | (redraw) |

**One policy, one place:** `sf_render_order_bevy` (`DrawBand` component, a render system in
`RenderSystems::PhaseSort` before Bevy's `sort_phase_system::<Transparent3d>` rewriting the sorting info
of tagged draws). Never order draws with a per-material `depth_bias` sort hack. A new shader-placed or
camera-local translucent (its mesh AABB centre isn't where it draws) must take a band.

**Why (the bug):** Bevy sorts by the AABB centre. The water clipmap (grid-space positions, entity at the
origin) and the precipitation boxes (seed-space positions, at the origin) are both placed in the shader,
so their keys differed only by `forward · (0.5, -3.5, 0.5)`: looking down at the sea drew the rain first
and the water (near-opaque refracted colour, depth-write) erased it. Every world-placed translucent vs the
water was arbitrary the same way (the water key was the distance to the world origin). Test:
`sf_render_order_bevy` `bevy_keys_alone_flip_water_and_rain_with_pitch` + the band-order test.

**Known gaps:** translucents *under* the water are not in the snapshot (they draw after it and fail the
water's depth): accepted (UE has the same limit). World-placed particles stay in the Sorted band (smoke
must sort against windows and leaves); only camera-local volumes are Overlay.

## Water bodies at any height (lakes, pools, interiors)

- Every body draws with the same material. Kinds differ in mesh and waves: Ocean = clipmap + all
  cascades; Lake = the clipmap rings clipped to its polygon (vertex shader discards outside via the
  water-level clipmap's body id) + short cascades × fetch from its size; Pool = a flat polygon mesh
  (tessellated at bake) + ripples + dynamic ripples; River = the ribbon mesh + flow normals.
- **Water-level clipmap** (UE's water info texture): camera-centred, toroidal like the terrain's, 2
  levels (512² at 0.5 m = 256 m; 512² at 4 m = 2 km). Per texel: rest surface height relative to the
  clipmap's origin cell (R16F), body index (R16U), flow (RG8 snorm), shore distance (R8). Rasterized
  by a small GPU pass from the bodies' outlines and the `water.shore`/`water.flow` products when the
  camera moves a step or a body changes (rare). Consumers: the water draw (body clipping, flow, shore),
  the surface hook (below water / wet band / caustics), precipitation (splash on water vs ground,
  ripple placement), froxel fog (water medium), grass scatter (no grass under water).
- **Interiors**: an interior worldspace with `sea_level` is one `Bounds` pool: flat, no FFT, ripples,
  dynamic ripples, SSPR at its level, optics from the adapter (murky by default). Interiors have
  their own clipmap instance when active (one per active worldspace view; portals later, M9).
- Overlaps resolve by priority in the clipmap raster and the BVH; equal priority is an authoring error
  in the bake report.

## Rivers: baked flow to the sea

Content packs either author rivers (`WaterBody::Ribbon` records) or let the bake derive them.
`WaterSource::river_policy()`:

- `Authored`: use the records; `water.flow` still bakes their flow maps.
- `FlowOnly`: no new water. Bake flow on the water the content already has (bodies at their level).
  For content whose rivers are channels at sea level (the first content pack), this is what turns a
  still sea inlet into a flowing river.
- `Generate { min_catchment_km2, carve: bool }`: create river bodies above sea level from drainage
  (for generated or sparse worlds); `carve` lowers terrain along the channel through the terrain edit
  layer (terrain.md stack layer 2), otherwise channels whose bed isn't below the bank are skipped.

**`water.hydro`** (worldspace bake, CPU, from the terrain heights resampled at 16 m; 40 × 40 km
worlds ≈ 6 M cells, ≤ 2 s single-threaded): priority-flood depression filling with an epsilon slope
(Barnes 2014; depressions under a body level are lakes, not filled), D8 directions, flow accumulation
(catchment area per cell), watershed labels. Product: catchment raster + D8 directions (compressed),
fingerprinted on terrain + bodies.

**`water.flow`** (per water body that can flow):
1. Inflows: cells where the drainage network (accumulation ≥ threshold) enters the body's water mask
   carry their catchment as discharge `Q` (relative units; a content scale factor maps it to m³/s).
2. Direction: the geodesic distance through the body's water to its outlet (the open sea, or the
   lowest boundary for lakes) by a fast-marching pass on a 2 m grid; flow follows its negative
   gradient, smoothed.
3. Discharge routing: each inflow's `Q` is routed down that gradient and accumulated; speed =
   `Q / (width · depth)` (local width from the shore distance field, depth from the bed), clamped
   (0-2.5 m/s default). Water with no inflow (canals, harbours) stays still.
4. Product: flow map tiles (RG8 velocity, 2 m texels) per body, read by the water-level clipmap and the
   CPU query.

**Generated rivers** (`Generate`): trace from each cell where accumulation crosses the threshold down
D8 to a body; smooth to a spline (Catmull-Rom, simplified to 4 m error); width by hydraulic geometry
(`w = a·Q^0.5`); surface height per point monotone non-increasing (min of bank − depth along the
channel); ribbon mesh + flow map; join at confluences; end in the receiving body with a mixing zone.
Waterfalls where the surface drops more than 2 m over 4 m: a marker for an effect mesh (content
supplies it), not simulated.

**Shading** (Vlachos 2010): two-phase flow-advected normal maps (and foam where speed or shore
contact is high), wave cascades × openness (rivers get little), ripples. Buoyant bodies drift with
`velocity`.

## Underwater

- **Camera medium** per frame: the query at the near plane's four corners and centre. All below →
  underwater; mixed → split; all above → above. Hysteresis 1 cm.
- **Medium**: the body's optics. Below the surface, in-scatter and transmittance follow the same
  formula as depth colour, with light attenuated by depth below the surface. **Froxel water medium**
  (High+): froxels below the water-level clipmap's surface get the water's extinction and albedo,
  lit through the surface (sun × transmittance to the surface × cascade-normal shafts), so light shafts
  under the waves come from the froxels like god rays above. **Low/Medium**: an analytic full-screen
  pass (homogeneous medium, distance from the depth buffer).
- **Surface from below**: the water material's back face: total internal reflection outside Snell's
  window (reflect the underwater colour/fog), refraction of the sky and scene inside it.
- **Waterline split**: in the underwater post, a per-pixel test of the near-plane point against the
  displaced surface (cascades sampled at that XZ) picks above/below, with a thin meniscus band (Crest's
  approach). No hard horizontal cut when waves cross the lens.
- **Caustics**: a tiling animated caustics atlas made by the `water.caustics` bake step (light refracted
  through a tiling wave normal field onto a plane, photon splatting; Guardado 2004), 32 frames, 256².
  The surface hook (terrain, `WorldMaterial`, characters) adds it on receivers below the water-level
  clipmap's surface, projected along the refracted sun, faded by depth (0.3-12 m), × sun visibility
  (CSM + cloud shadow). Because it runs in the opaque pass, the refracted view from above and the view
  from below both show it.
- **Exposure and cues**: no special exposure; the cue list gets `underwater` (the audio system applies a
  low-pass), `surface_break` on crossings.

## Buoyancy (CPU, `sf_physics`)

**Model** (UE Buoyancy component, Crest floating objects): each floating body carries **pontoons**,
spheres with a local centre and radius whose volumes sum to the body's volume. Per physics step
(60 Hz, the same fixed step):
- per pontoon: water sample at its centre (query, batched with `sample_many`), submerged fraction of the
  sphere from the closed-form spherical cap, buoyant force `ρ_w·g·V_sub` along +Y (optionally tilted to
  the surface normal for surfing on waves), drag `−c_lin·v_rel − c_quad·|v_rel|·v_rel` with
  `v_rel = v_point − water velocity` (so flow carries things), added torque damping when submerged;
- forces applied at the pontoon centres; the body's mass and centre of mass come from `body.cook`.
- **Density** = `body.cook` mass / volume (adapter mass rules, physics-adapter.md). Below water
  density bodies float, above sink with water drag and rest on the bed. Wood floats, iron sinks.
- Accurate option for hero bodies (boats): Kerner's per-triangle hull clipping over `body.cook`'s hull
  (hydrostatic pressure per submerged triangle + pressure drag), 0.02-0.05 ms per boat. Pontoons are
  the default; the hull method is a per-body flag.

**Pontoons bake** (`water.pontoons` over `body.cook`, feature `bake` in `sf_water`): voxelize the
body's shape parts at `extent/12`, k-means into 1-8 clusters by size (1 for small items, 4-8 for
boats), each sphere's radius from its cluster's volume. Stored as a separate product next to
`body.cook` so physics doesn't depend on water.

**`sf_physics` change**: `PhysicsWorld::add_force_at(id, force, point)` accumulated until the next
`step` (rapier `add_force_at_point` with reset), and a `Floating` flag on `BodyDesc` that keeps the body
awake while the sea state at it exceeds 1 cm amplitude (calm water lets it sleep like any body).
`sf_water_bevy` (feature `physics`) runs the floaters system in `FixedUpdate` before the physics step.

**Budget and range**: floaters simulate within the physics active ring (R0); at most 32 awake floaters
(pontoons ≤ 8 each) ≤ 0.1 ms CPU; past the budget the farthest freeze (as physics.md's sleep rules).
Floaters crossing an origin rebase need nothing special (`WorldPos` queries; physics rebases).

**Characters**: not pontoon bodies. The kinematic mover's `Swim { surface }` (locomotion.md) takes
the surface from the query; wading speed scales by immersion (adapter curve).

## Wading, wetness and interaction hooks

- `WaterQuery::immersion(capsule)` gives depth, submerged fraction and the surface point for actors.
- **Character wetness** (weather/precipitation-surface.md "Surface state", `sf_surface`): wet line = the highest immersion
  over the last minutes, drying per hour; the character material darkens and smooths below it.
- **Dynamic ripples** (iWave/Crest dynamic waves): a 256² wave-equation grid at 0.125 m (32 m) around
  the camera, toroidal, on the GPU; actors' feet and legs, floating bodies, thrown items and raindrops
  inject; the water draw adds its normals (and a small displacement on High). Wakes behind boats and
  rings around a wading leg come from this.
- **Splashes and cues**: entering the water above a speed emits `splash { size }` (effects later);
  footsteps take the `water` variant below 0.3 m depth, `wade` deeper; `underwater` and
  `surface_break` cues for the camera.
- **AI**: the query's `depth` lets pathing treat deep water as swim-only (M7).

## Weather coupling

| Weather input | Water response |
|---|---|
| `WindState` speed/direction | spectrum (blended `h0`), foam coverage, ripple chop |
| Precipitation intensity (rain) | ripple atlas normals, splash sprites on water, rain mist above water (froxels) |
| Lightning | flash in reflections (the flash light is in the clustered lights) |
| Cloud shadows | darker water inside and on its specular (`DirectionalLightTexture`) |
| Fog | the water pass applies froxel + aerial fog; ground fog banks lie on water too |
| Ash/blight (`surface_rates.ash`) | optional surface film tint on still water (`surface_tint`), cleared by rain |

## Performance budgets (1440p; RTX 4060 Laptop / RTX 3070 desktop, GPU ms)

Added to weather.md's table; budgets, not measurements. Each feature lands with an `sf_diag` counter
(`water/fft/gpu_ms`, `water/pass/gpu_ms`, …) and an A/B (`--off water|sspr|underwater|ripples`).

| Feature | Low | Medium | High | Ultra |
|---|---|---|---|---|
| FFT cascades + foam (amortized) | 2 × 128², 0.05 | 3 × 128², 0.08 | 4 × 256², 0.15 / 0.12 | 4 × 256² every frame, 0.2 |
| Water-level clipmap + dynamic ripples | 0.02 (no ripples) | 0.05 | 0.08 | 0.1 |
| Snapshot copies | 0.1 | 0.1 | 0.1 / 0.08 | 0.1 |
| SSPR | off (env map) | ½ res, 0.15 | full, 0.3 / 0.25 | full, 0.3 |
| Dynamic reflection probe (amortized) | off | 128², 0.08 | 256², 0.15 / 0.12 | 256², 0.15 |
| Water draw (clipmap 5/6/7/7 rings) + velocity | 0.3 | 0.4 | 0.5 / 0.4 | 0.6 |
| Underwater (when below) | analytic 0.1 | analytic 0.1 | froxel medium +0.1 | +0.15 |
| Caustics in the hook | off | 0.03 | 0.05 | 0.05 |
| **Total water (above water)** | **≈ 0.5** | **≈ 0.9** | **≈ 1.35 / 1.1** | **≈ 1.55** |

Together with weather.md's ≈ 3.3 ms (High), a Bitter Coast rainstorm view is ≈ 4.6 ms of weather +
water on High: over a third of the 8.3 ms frame target before the world draws. High is the target on
the 3070 with DLSS (resolution-independent passes: FFT, clipmap, CPU twin; the rest scales). Over-budget
features are cut per preset, not the frame.

CPU: twin FFT ≤ 0.05 ms per step; queries ≤ 2 µs each; 32 floaters ≤ 0.1 ms; clipmap raster
scheduling ≤ 0.02 ms.

## Crate split (rule 7)

| Crate | Bevy? | Contents | Depends on |
|---|---|---|---|
| `sf_water` | no | records, `WaterQuery` + body BVH, spectrum (JONSWAP/TMA/spreading), CPU twin (`rustfft`), buoyancy math (pontoon caps, drag, hull clipping), `WaterSource`/`WaterOpticsSource` hooks; feature `bake`: `water.shore`, `water.hydro`, `water.flow`, `water.caustics`, `water.pontoons` | `sf_coords`, `sf_records`, `sf_terrain` (heights), `sf_wind` (`WindState`), `glam`, `rustfft`, `sf_bake` (feature) |
| `sf_water_bevy` | thin Rust, heavy WESL | `WaterPlugin`: FFT compute, clipmap mesh, snapshot + SSPR + water draw + velocity + underwater nodes, water-level clipmap, dynamic ripples, WESL module `sf_water::sample` (surface hook, precipitation, froxels), camera medium; feature `physics`: floaters system | `sf_water`, `sf_wind_bevy`, `sf_terrain_bevy` (clipmap binding), `sf_world_bevy` (origin); `sf_physics_bevy` (feature) |

Wide and shallow: `sf_water` is a leaf the controller, surface and AI can read without Bevy. The
surface hook, precipitation and fog read the water-level clipmap through a WESL module and a bind
group resource, not a crate dependency on the water plugin (the app wires the resource, as it does for
`FarShadowInput`). Bevy features: `bevy_pbr`, `bevy_core_pipeline`, `bevy_render`.

## Verification

- **Unit tests (Bevy-free)**: spectrum energy vs the analytic JONSWAP integral; CPU twin vs a reference
  DFT; twin vs GPU golden displacement within 1 cm (golden captured once from a framework example);
  displaced-height inversion converges in 3 iterations for steepness ≤ 0.3; origin phase: an origin
  shift leaves sampled heights bit-identical; body BVH priority; shore EDT vs brute force; priority-flood
  on synthetic basins (no sinks left, lakes kept); flow direction toward the outlet on a synthetic
  channel, zero in a dead-end canal; pontoon k-means volumes sum to the hull volume; a box of density
  500 floats at half draft ± 2 %, density 2 000 sinks; buoyancy across a physics rebase unchanged.
- **Framework examples** (synthetic): `water_ocean` (sea states calm → storm, shore on a generated
  island, flight to the horizon), `water_bodies` (a lake above the sea, a pool, a generated river to
  the sea), `water_float` (boxes of different densities, a hull with pontoons and with hull clipping).
- **Game scenes**: the adapter spec's WA-* checks and the M6 demo scenes (ROADMAP "M6").
- **Perf**: `perf suite` coast views at 1440p per preset; budgets above; A/B per feature.

Moved to [history](archive/water-history.md#as-built-w1-2026-10-13-m6-w1).

## Open questions

1. W2 decides the water draw's plumbing: `ExtendedMaterial<StandardMaterial, WaterExt>` in the
   transparent phase (sorted first, depth write via `specialize`) vs a full `Material` in our own phase.
   Criterion: keeps Bevy's view bindings (lights, shadows, env map, cloud texture) with the least code.
2. Water and the depth-only shadow passes: water casts no shadow (correct for clear water); murky
   water should darken the bed via the hook instead. Check that it reads right in the swamp.
3. RT reflections from the water pass on the Solari path: ray queries against Solari's TLAS + world
   cache lookups. Needs Solari's bindings to be reachable from our pass (a patch like the sunset patch
   in sky-lighting.md). After M6's raster path.
4. Ocean far field: does the horizon skirt need its own far-shadow term (SHM tier 2) for ridge shadows
   on the sea at sunset? Probably yes, one fetch.
5. Ship sailing and swimming creatures' buoyancy: vehicle spec and M7 AI, not M6.

Moved to [history](archive/water-history.md#industry-comparison).

Moved to [history](archive/water-history.md#as-built-w2-2026-10-13-m6-w2).
