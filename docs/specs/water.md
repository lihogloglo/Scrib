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

## Research summary (2026-10-13)

Reuse order (CLAUDE.md rule 8): Bevy, then crates, then our own.

**Bevy 0.20** (read in `~/.cargo/registry/src/*/bevy_*-0.20.0`):
- No water. `StandardMaterial` has screen-space **transmission** (`specular_transmission`, `ior`,
  `thickness`, attenuation): the `Transmissive3d` phase snapshots the opaque colour and draws
  transmissive meshes. It samples colour only (no scene depth without a depth prepass), and its system
  is ordered only "after opaque, before transparent", the same as the atmosphere's `render_sky`, so
  whether the snapshot contains the sky and aerial perspective is unspecified. It writes no motion
  vectors. Not enough for depth colour, shore fades or DLSS, but the mechanism (snapshot, then draw) is
  the one we use.
- `ScreenSpaceReflections` needs `DepthPrepass` + `DeferredPrepass`; the exterior camera has neither
  (far-field.md pitfall 2; weather.md open question 4).
- What we do reuse: the `Atmosphere` aerial-view LUT and environment map (sky reflection), the PBR
  lighting functions (GGX sun specular, clustered point lights for torches reflecting at night,
  `fetch_directional_shadow`), `DirectionalLightTexture` cloud shadows, the core-3d schedule (our
  passes are systems ordered `after(render_sky)`, `before(main_transparent_pass_3d)`), and the main
  depth texture the atmosphere already makes bindable (`configure_camera_depth_usages`).
- `bevy_solari` 0.20: BLAS geometry is flagged `OPAQUE`; specular/mirror lobes are traced
  (`initial_path.wesl`). Water can't be a transparent RT surface; on the RT path it stays a raster
  layer drawn over Solari's output (below).

**Crates (crates.io and GitHub, 2026-10-13).**

| Crate | Latest | Bevy | License | State | Verdict |
|---|---|---|---|---|---|
| `bevy-aqua` | 0.1.3 (2026-08-31) | 0.19 | MIT OR Apache-2.0 | 167 commits, last 2026-09-23, 29 stars, 98 downloads, one maintainer (+1 contributor), "AI-assisted development" disclosed; no Bevy 0.20 branch (0.20 shipped 2026-10-08) | **Reference and code source, not a dependency** (below) |
| `bevy_water` | 0.18.1 (2026-02-02) | 0.18 | MIT OR Apache-2.0 | dynamic ocean material (sum-of-waves in a `StandardMaterial` extension), one level | Reference only: no FFT, no bodies, two Bevy versions behind |
| `rustfft` | 6.x | n/a | MIT OR Apache-2.0 | maintained, the Rust FFT | **Use** in `sf_water` for the CPU twin (check the version at W1) |
| `bevy_hanabi` | 0.19 line | 0.19 | MIT/Apache | not on 0.20 | Not needed (spray is ours or later) |

**`bevy-aqua` evaluation** (README, repo, crates.io, 2026-10-13). What it has: five camera-centred
displacement cascades with LOD blending; analytic (Crest-style) or Tessendorf FFT waves from sea
state, wind and fetch; shallow-water attenuation; Beer-Lambert transmission, refraction, planar mirror
or cubemap reflections, optional SSR (needs a depth prepass); persistent whitecaps and shoreline foam;
bounded ponds, lakes and river corridors with per-body optics (rivers analytic, others flat);
procedural caustics; GPU `WaveQuery` (256 probes per frame, ~1 frame readback latency); optional
`bevy_hanabi` spray; opt-in underwater (homogeneous medium, no multi-camera, waterline not closed).
Inputs: one `Ocean` resource (one sea level), one global `BedHeightMap` image (replaced by handle,
no in-place edits), bodies must be flat and should not move (moving rebuilds shore fields).

Against our constraints:

| Our constraint | bevy-aqua 0.1.3 | Cost to make it fit |
|---|---|---|
| Bevy 0.20 | 0.19 | port (render API moved to schedule-based core-3d in 0.20; WESL) |
| Floating origin (`big_space`, 256 m cells) | world XZ in `f32`, no origin phase | rewrite every world-space sample (cascades, foam, bodies, queries) |
| Streamed terrain (toroidal height clipmap) | one global `BedHeightMap` | a window copy per camera step, or sample our clipmap |
| CPU, synchronous, deterministic query (§10) | GPU readback, ~1 frame late, 256/frame | write the CPU twin ourselves |
| No depth prepass on the exterior camera | refraction/SSR expect one | restructure to a snapshot pass |
| Planar reflections | second camera re-rendering `ReflectedInWater` roots | re-renders the streamed world (draw cost ×~2); replace |
| Many worldspaces (interiors with their own water) | one `Ocean` resource | multi-worldspace bodies |
| Spray | `bevy_hanabi` (not on 0.20) | drop |

Most of its integration surface would be rewritten; what transfers unchanged is the wave model
(spectrum, FFT, cascade blending, foam accumulation, shallow attenuation) and its optics shader math.
**Decision: build `sf_water` ourselves and port bevy-aqua's wave/foam/optics code where it fits
(MIT OR Apache-2.0: allowed, keep its copyright notice in the ported files and in
`THIRD_PARTY_NOTICES`).** GodotOceanWaves (MIT, the source ARCHITECTURE §7.2 named) is the second
reference for the spectrum and cascades. Revisit trigger: if bevy-aqua ships on Bevy 0.20 with
origin-relative coordinates and a CPU query, re-evaluate it for the ocean surface only (log it in §10).

**Papers and engines** (the canonical techniques):
- Tessendorf, *Simulating Ocean Water*, SIGGRAPH course notes 2001: FFT ocean from a statistical
  spectrum, horizontal "choppy" displacement, the Jacobian of the displacement as the foam signal.
- Horvath, *Empirical Directional Wave Spectra for Computer Graphics*, DigiPro 2015: JONSWAP with
  fetch, TMA shallow-water correction, Donelan-Banner/Hasselmann directional spreading, swell. What
  GodotOceanWaves and most current game oceans implement.
- Bruneton, Neyret, Holzschuch, *Real-time Realistic Ocean Lighting using Seamless Transitions from
  Geometry to BRDF*, Eurographics 2010: far ocean shading from the wave slope variance (roughness
  grows with distance instead of aliasing sparkles).
- Dupuy and Bruneton, *Real-time Animation and Rendering of Ocean Whitecaps*, SIGGRAPH Asia 2012
  technical brief: whitecap coverage from the Jacobian, accumulated with decay.
- Losasso and Hoppe, *Geometry Clipmaps*, SIGGRAPH 2004; Strugar, *Continuous Distance-Dependent
  Level of Detail* (CDLOD), 2009: camera-centred nested grids with vertex morphing (our terrain uses
  the same family).
- Bowles, *Crest: Novel Ocean Rendering Techniques in an Open Source Framework*, SIGGRAPH 2017
  Advances, and Crest (Unity, MIT): LOD data cascades for shape, foam, dynamic waves and sea-floor
  depth; queries by GPU readback, later also baked FFT data for CPU queries.
- Unity HDRP Water System (2022+): up to three bands (swell, chop/agitation, ripples), pools use
  ripples only, rivers ripples + chop; script queries by GPU readback by default with a **CPU
  simulation** option ("full CPU simulation", "evaluate ripples" off by default) and an iterative
  search for the displaced height. Same split as ours.
- Unreal Engine Water plugin (UE 4.26+/5): water body actors (Ocean, Lake, River, Custom) on splines,
  a water zone that renders all bodies into a water info texture (height, depth, velocity), the
  Single Layer Water shading model (water drawn in its own pass after opaque, reading scene colour and
  depth: absorption, scattering, refraction), Gerstner "Water Waves" assets, and a Buoyancy component
  that approximates a hull with **pontoon spheres** (the docs' example: four pontoons on a cube).
  `GetWaterHeight` returns height, depth, normal and velocity: the same query contract as ours.
- Vlachos, *Water Flow in Portal 2*, SIGGRAPH 2010: flow maps with two-phase normal advection.
- Kerner, *Water interaction model for boats in video games* (Gamasutra 2015): per-triangle hull
  clipping with hydrostatic and drag forces; the accurate alternative to pontoons.
- Tessendorf, *Interactive Water Surfaces* (iWave), Game Programming Gems 4, 2004: a small local
  wave-equation grid for ripples and wakes (Crest's "dynamic waves" is the same idea).
- Cichocki, *Optimized pixel-projected reflections for planar reflectors*, SIGGRAPH 2017 Advances:
  screen-space planar reflections (SSPR) in one compute pass for a known plane.
- Guardado and Sánchez-Crespo, *Rendering Water Caustics*, GPU Gems 2004: caustics from refracting
  light through the wave normals onto a plane; games ship a tiling animated caustics texture.
- Barnes, Lehman and Mulla, *Priority-Flood*, Computers & Geosciences 2014 (depression filling,
  watershed labels); O'Callaghan and Mark 1984 (D8 flow accumulation); Génevaux et al., *Terrain
  Generation Using Procedural Models Based on Hydrology*, SIGGRAPH 2013 (river networks from
  drainage); Leopold and Maddock 1953 (hydraulic geometry: width grows with discharge^0.5).
- Felzenszwalb and Huttenlocher, *Distance Transforms of Sampled Functions*, 2012: exact linear-time
  Euclidean distance transform (shore distance at bake).

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
(Horvath 2015), from `WindState` (weather.md) speed and direction and a per-body fetch. Cascades are
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
   foam; rain ripple normals (weather.md's ripple atlas × rain intensity); dynamic ripple normals;
   flow-advected normals (rivers). Then fog: aerial perspective from Bevy's aerial-view LUT and the
   froxel fog through `sf_atmos::fog` (weather.md "Fog", step 4), so far water fades like terrain.
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
- **Character wetness** (weather.md "Surface state", `sf_surface`): wet line = the highest immersion
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

## As built (W1, 2026-10-13, m6-w1)

`crates/sf_water` (Bevy-free; deps `glam`, `rustfft` 6.4, `serde`, `sf_coords`, `sf_records`, `sf_world`,
`sf_terrain`). No bake steps and no buoyancy math yet (W3, W6, W7).

- `records`: `WaterBody` (`sf.water_body`), `WaterKind`, `WaterShape` (Infinite / Polygon with holes / Ribbon
  of `RiverPoint` / Bounds), `WaveTerms` (first cascade + amplitude; per-kind defaults), `WaterOptics`
  (`sf.water_optics`, physical defaults for clear water, `visibility()`), `WaterBody::from_worldspace`
  (`sea_level` → Ocean/Infinite with terrain, Pool/Bounds without).
- `spectrum`: `SpectrumParams` (wind, fetch, depth, amplitude, choppiness) with JONSWAP + TMA (Kitaigorodskii,
  Horvath's approximation) + Donelan-Banner spreading + finite-depth dispersion; `Cascades` (power-of-two
  tiles, band boundary `size(i+1)/split`, split 2); `H0::generate` (deterministic in seed, cascade, params; noise
  keyed by **wave vector**, so any two grid sizes share every coefficient), `H0::blend` (weather mix),
  `band_variance` (analytic). Swell (Horvath's second spectrum) is not in yet. `WindInput` is the named scaffold
  for `sf_wind::WindState` (WD1): swap the field type when WD1 lands.
- `twin`: `CascadeTwin` (per-step time evolution of the active bins only, 4 packed complex 2D inverse FFTs
  giving height, height rate, Dx, Dz, their rates and slopes; Catmull-Rom sampling), `WaveTwin` (1-4 cascades
  summed, displaced-height inversion with 3 fixed-point iterations), `tile_coords(WorldPos, size)` (exact
  integer cell phase: the render origin never enters), `origin_phase(origin_cell, size)` (what the GPU adds
  to camera-relative XZ), `reference_height` (direct sum, for tests and goldens).
- `query`: `WaterQuery` (`sample`, `sample_many`, `immersion`), `WaterWorld` (per worldspace, `step(t)` evaluates
  every twin), `WorldspaceWater` (bodies, 2D median-split BVH over finite bodies + unbounded bodies by priority,
  optional `BedHeights` (impl for `TerrainHeights`), optional `WaveTwin`). The ocean is dry where the bed is above
  the displaced surface; depth is `f32::INFINITY` where no bed is known (interiors, unstreamed terrain).
- Hooks: `WaterSource` (bodies, `river_policy`, `ocean_spectrum`), `DefaultWaterSource`, `WaterOpticsSource`,
  `RiverPolicy`.

**Differences from the text above** (all logged in ARCHITECTURE §10): the query has no `t` argument (it reads the
last `step`); bands split at `size/2`; cubic twin sampling; physical amplitudes (bevy-aqua normalises every sea
state to a fixed RMS height; we don't).

**Tests** (`cargo test -p sf_water`, 11): spectrum energy vs the analytic JONSWAP integral within 3 % (realised
variance within 15 %, Donelan-Banner integrates to 1 ± 1e-3; Hs vs the JONSWAP fetch law within 40 %: the peak
and energy fits are separate regressions, ~30 % apart); cascades power-of-two and gap-free; determinism (same
seed → same `h0` bits; twin state a function of `t` only); 64² and 256² grids share every coefficient and the
whole band; twin vs direct DFT: 2.4e-7 m on nodes, **2.4 mm** between nodes and vs the 256² grid (Hs 1.9 m; was
7.7 cm bilinear with `/4` bands); inversion: 3 iterations within 2.4 mm of 40 (choppiness 1, 18 m/s, two
cascades), converged fit 2.6e-6 m; origin shifts and far cells (cell 100 002) give bit-identical samples, the GPU
formula lands within 1e-3 m of the CPU's tile coordinates for any origin; weather blend continuous; BVH priority
(ocean, lake above the sea, pool inside the lake, 60 ponds, a river with height and flow by segment), dry land
cuts the ocean, immersion; record JSON round trip.

**Cost** (`cargo run --release -p sf_water --example water_bench`, 4060 Laptop machine, 201 bodies): twin step
**0.027 ms** for cascade 0 (192 active bins; budget 0.05), 0.055 ms with cascade 1 (376 bins: High's "rough sea"
option is 10 % over, so evaluate cascade 1 every other step or accept it); sample **0.25 µs** (budget 2), 0.43
µs with two cascades.

**W2 handoff (ocean surface + water pass)**:
1. GPU spectrum: upload `H0::generate(params, cascades, i, 256 or 128, seed)` per cascade (RG32F per texel, layout
   `iz * n + ix`, signed frequency mod n). Use the **same seed and `Cascades`** as the twin: the twin is then the
   GPU's cascade 0 by construction. Time evolution on the GPU must match `CascadeTwin::evaluate`: `ω` from
   `SpectrumParams::omega` (finite depth if set), `h̃ = h0·e^{iωt} + conj(h0(−k))·e^{−iωt}`, `D̃ = λ·i·k̂·h̃` (sign:
   crests sharpen). The CPU reduces `ω·t` mod 2π in f64; the GPU has no f64, so W2 adds Tessendorf's repeat
   period `T` (quantise `ω` to multiples of 2π/T in `CascadeTwin::new` too, upload `t mod T`) and keeps the
   twin test green.
2. Sampling position: `fract((origin_phase(origin_cell, size) + rel_xz) / size)`, origin cell from the same
   place `sync_origin` reads (water-adapter.md "What replaces `SeaPlaneScaffold`").
3. Golden: capture GPU displacement at a few points from `water_ocean` and compare with
   `WaveTwin::sample` (target 1 cm; the twin is 2.4 mm from the exact field).
4. Weather: blend with `H0::blend` (CPU) and re-upload when `w` moves 2 %; rebuild the twin from the same blended
   `h0` (`WaveTwin::from_h0`).
5. App wiring (owner of `exterior.rs`): `WaterWorld` as a resource, `step(t)` in `FixedUpdate` before physics,
   one `WorldspaceWater` per active worldspace from `mw_water::MwWaterSource`, bed = the resident
   `TerrainHeights`.

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

## Industry comparison

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Ocean waves | Tessendorf FFT cascades (Crest, HDRP 3 bands, Sea of Thieves, Atlas, bevy-aqua); Gerstner sums (UE Water Waves, bevy_water) | FFT cascades, JONSWAP/TMA (Horvath 2015) | **match** (FFT branch; Gerstner kept only as Low fallback if FFT is over budget) |
| Wave LOD | camera-centred LOD cascades / clipmaps (Crest, aqua, UE water quadtree) | geometry clipmap + CDLOD morph, horizon skirt | **match** |
| Far ocean shading | slope-variance BRDF transition (Bruneton 2010) | same | **match** |
| Whitecaps / foam | Jacobian with persistent decay (Tessendorf, Dupuy-Bruneton, Crest, HDRP) | same | **match** |
| Water bodies at any height | UE water bodies (ocean/lake/river/custom) + water zone info texture; HDRP surfaces (ocean/river/pool); Crest water bodies | records + water-level clipmap | **match** |
| Water shading pass | UE Single Layer Water (after opaque, scene colour + depth); HDRP water pass | snapshot + water draw | **match** |
| Reflections | SSR + probes (UE, HDRP), planar (older; aqua), RT (UE Lumen, HDRP RTR) | SSPR + camera-centred dynamic probe + env map; RT later | **match** (SSPR instead of SSR because our exterior has no deferred prepass) |
| Refraction, absorption | screen-space refraction + Beer-Lambert + scattering (all) | same | **match** |
| Caustics | projected animated texture / light function (UE, Crest), RT caustics (HDRP option) | baked atlas in the surface hook | **match** (no RT caustics) |
| Underwater | volumetric underwater fog + waterline meniscus (Crest, HDRP underwater volume, UE post) | froxel water medium (High) / analytic, meniscus | **match** |
| Shoreline | sea-floor depth (Crest), distance to shore (UE water info), shore foam | baked shore EDT + live depth, openness | **match** |
| Rivers | authored splines with flow (UE, HDRP rivers, Portal 2 flow maps) | authored or baked from hydrology | **differ** (games author rivers; we derive flow because the first content pack has none: a bake step, not runtime simulation) |
| Gameplay queries | GPU readback (Crest, HDRP default, aqua) or CPU simulation (HDRP CPU option, Crest baked FFT, UE CPU Gerstner) | CPU twin of the long cascades | **match** (CPU path, deterministic) |
| Buoyancy | pontoons (UE Buoyancy component, Crest), hull triangle clipping (Kerner 2015, many boat games) | pontoons default, hull clipping per body | **match** |
| Interaction ripples | local wave-equation sims (iWave, Crest dynamic waves, HDRP deformers) | iWave grid around the camera | **match** |
| Temporal AA on water | motion vectors from displacement, reactive masks (UE translucency velocity, FSR reactive) | velocity redraw + reactive mask (weather.md question 9) | **gap until the DLSS bias patch** (P1 with precipitation) |
| Fluid simulation (flooding, flowing over terrain) | rare in open worlds (shallow-water sims in UE Fluid Flow, Cities: Skylines) | none | **differ** (non-goal) |

**Sources** (checked 2026-10-13): bevy-aqua README and repo (https://github.com/sayhisam1/bevy-aqua, commits
API, branches), crates.io `bevy-aqua`, `bevy_water`; UE Buoyancy component docs
(https://dev.epicgames.com/documentation/en-us/unreal-engine/water-buoyancy-component-in-unreal-engine)
and Water System overview; Unity forum thread on HDRP water CPU simulation
(https://discussions.unity.com/t/water-level-in-water-system/1688088) and the HDRP water blog
(https://unity.com/blog/engine-platform/new-hdrp-water-system-in-2022-lts-and-2023-1); Bevy 0.20 and
bevy_solari 0.20 sources in the local registry. Papers as cited above (from the literature, not
re-read this session); UE water info texture and Single Layer Water details from the UE docs as
remembered: mark unverified on detail.

## As built (W2, 2026-10-13, m6-w2)

`crates/sf_water_bevy` (thin Rust, WESL; minimal bevy_* deps + `sf_water`). `WaterPlugin`; host inputs `OceanSpectrum`
(params, cascades, seed: the twin's), `WaterFrame` (time, origin cell, render-space level), `WaterLook`
(`from_optics`), `WaterFogInput` (froxel volume), `ClipmapSettings`; markers `WaterCamera`, `OceanSurface`;
`WaterSystems::Drive` (PostUpdate). `sf_water`: `REPEAT_PERIOD` 256 s, `omega_repeat`/`repeat_cycles` (the twin now
evolves with them), `H0::gpu_texels`; test `waves_repeat_after_the_repeat_period`.
- **FFT** (`water_fft.wesl`): 4 × 256² every frame: spectrum (4 packed complex signals: h, Dx, Dz, slopes, Jxx, Jzz,
  Jxz), radix-2 inverse FFT rows then columns in workgroup memory (one line per workgroup), assemble into two RGBA16F
  arrays (Dx,h,Dz,foam / sx,sz,Jxx,Jzz) + persistent Jacobian foam (storage buffer with decay). Phase
  `2π·fract(cycles·(t mod T)/T)`.
- **Golden** (`cargo run --release -p sf_water_bevy --example water_ocean -- --golden`, 14 m/s, t 37.5 s): GPU cascade 0
  vs `CascadeTwin` on the 64² nodes **max 1.9 mm, rms 0.2 mm** (Hs ≈ 2 m; includes f16 storage). Exits 0/1.
- **Clipmap**: one static mesh (7 rings × 128², 0.25 m, hole 30 cells, skirt to 32 km); the vertex shader snaps each
  ring to 2× its spacing around the camera and CDLOD-morphs its outer 12 cells; a ring discards where the finer one
  covers it, one fine cell short of the edge (there both are the same surface: no seams). Displacement fades per
  cascade with distance (10-16 × its shortest wavelength) and to 0 at 900 m; normals fade per cascade by pixel
  footprint, the lost variance goes into roughness (Bruneton-lite; no mips yet).
- **Water pass**: snapshot (colour copy + depth resolve to R32F, fullscreen, ordered after everything that runs before
  `main_transparent_pass_3d`, i.e. `render_sky` and the `sf_atmos` composite) + a compute copy of Bevy's aerial-view
  LUT; water draw = `ExtendedMaterial<StandardMaterial, WaterExt>` in the transparent phase, depth-writing, cull none
  (open question 1 decided: keeps Bevy's lights, shadows and environment map with the least code); refraction with
  in-front rejection, Beer-Lambert + single-scattering body colour, Bevy PBR with F0 0.02, whitecaps (cascades 0-1),
  contact foam (vertical depth below 0.35 m × contact), then aerial perspective (LUT in-scatter, analytic air
  extinction) and froxel fog. **Velocity**: after the transparent pass the clipmap is redrawn procedurally into the
  prepass motion-vector texture where the water is nearest (current vs previous displacement texture and view); only
  when the camera has `MotionVectorPrepass` (DLSS, `--prepass`).
- **GPU cost** (example, 1600×900, 4060 Laptop, **shared machine**: other agents' builds were running): `water_fft`
  0.22 ms (High budget 0.15: over; amortise the 256 m cascade every 2nd frame, or 3 cascades on Medium),
  `water_snapshot` 0.04 ms (budget 0.1), transparent pass holding the water draw 0.53 ms (draw budget 0.5). Velocity not
  timed (the example has no prepass). Not yet re-measured in the app at 1440p.
- **Handoff (W3/W4 and W2 leftovers)**: shore attenuation + `water.shore` (W3; waves run up beaches unattenuated now),
  optics tuning (W1 placeholders read milky turquoise), SSPR + dynamic probe (W4: environment map only now; the
  material's PBR call is where SSPR plugs in), underwater (W5; back faces flip the normal only), FFT amortisation and
  the Low 128² path, normal mips, F2 analytic height fog on water when froxels are off (water gets aerial + froxels
  only), weather `H0::blend` re-upload (WE3; `upload_spectrum` regenerates on any `OceanSpectrum` change, on the main
  thread, tens of ms), pipeline warm-up (the FFT pipelines take ~1-2 s to compile on DX12; the water is flat until
  then), in-app DLSS smear check and fast flight across origin shifts (WA-8).
