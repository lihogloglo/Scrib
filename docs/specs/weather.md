# Weather, atmosphere and distant shadows

Status: draft (2026-10-11). "Near-term visual fixes": distant shadows built (shadows2), bug B1-B3 built (nifpop, 2026-10-12); volumetric clouds built (clouds, 2026-10-12, "Clouds as built"); start now
in M2; the rest is M6.
Crates (new): `sf_wind`, `sf_weather`, `sf_atmos`, `sf_surface` (Bevy-free cores);
`sf_wind_bevy`, `sf_weather_bevy`, `sf_atmos_bevy`, `sf_surface_bevy`, `sf_precip_bevy` (thin plugins).
Touches: `sf_sky`/`sf_sky_bevy` (proposed changes at the end; owned by sky-lighting.md),
`sf_terrain_bevy` (surface hook, far shadow), `sf_lod_bevy` (far shadow, impostor wind),
`sf_model` (wind vertex data, rest-pose cook), `sf_world_bevy` (uncooked preload).
Adapter: `../../../bevyrrowind/docs/specs/weather-adapter.md`.

## Goal

Owner's request, 2026-10-11: "RDR2-type weather. Beautiful Nubis-like clouds that change with the
weather. Rain that makes things wet and makes puddles. Wind that makes trees sway and grass bend.
The whole thing. Distant terrain shadows. Ground fog. It's a moody place, so god rays etc."

What that means for the framework:

1. **One weather state** with blended presets, defined as data (records from mods). It drives the sky,
   clouds, fog, wind, precipitation, surface state, sound cues and gameplay tags, so all of them agree.
2. **Volumetric clouds** (Nubis model) that change shape and cover with the weather. They cast shadows
   on the ground and into the fog, and fade into the aerial perspective over the far field.
3. **Fog at every scale**: aerial perspective (Bevy's atmosphere), exponential height fog, ground fog
   banks, and froxel volumetric fog. Sun and point lights scatter in it, shadowed by the shadow maps,
   the terrain and the clouds. God rays fall out of this; they are not a separate effect.
4. **One wind field** sampled the same way by grass, trees, impostors, cloth, particles, clouds and fog.
   Bake steps write the per-vertex data that trees need.
5. **Precipitation** (rain, snow, ash, custom kinds) that roofs and trees block (an occlusion map), with
   splashes and ripples.
6. **Surface state**: wetness, puddles, snow and ash. It accumulates where precipitation reaches and dries
   over time. Every world material shows it through one shader hook.
7. **Distant shadows**: the terrain and the far field are shadowed to the horizon, and no object
   suddenly gets its shadow as the camera approaches.

Non-goals: water bodies, waves and buoyancy (WISHLIST §4; [water.md](water.md), M6); character wetness
(designed here, built with characters); persistent snow deformation (terrain.md stack layer 3, M6
deformation work); voxel hero clouds (Nubis³; open question 3); audio playback (we only emit cues).

## Research summary (2026-10-11)

Reuse order (CLAUDE.md rule 8): Bevy, then crates, then our own. The sources come first, then what
each one means for us.

**Clouds.**
- Schneider and Vos, *The Real-time Volumetric Cloudscapes of Horizon Zero Dawn*, SIGGRAPH 2015
  Advances ([pdf](https://advances.realtimerendering.com/s2015/The%20Real-time%20Volumetric%20Cloudscapes%20of%20Horizon%20-%20Zero%20Dawn%20-%20ARTR.pdf)).
  This is the canonical density model: a Perlin-Worley base shape, Worley detail erosion, a height
  gradient per cloud type, and a weather map (coverage, type, precipitation). Clouds are raymarched in
  a spherical shell over a flat world. They are drawn at quarter resolution, and 1/16 of the pixels
  update each frame with temporal reprojection.
- Schneider, *Nubis: Authoring Real-Time Volumetric Cloudscapes with the Decima Engine*, SIGGRAPH 2017
  Advances ([pdf](https://advances.realtimerendering.com/s2017/Nubis%20-%20Authoring%20Realtime%20Volumetric%20Cloudscapes%20with%20the%20Decima%20Engine%20-%20Final.pdf)).
  Authoring comes from the weather map and type profiles, plus the cloud shadow and lighting
  approximations (Beer-powder, multiple-scattering octaves).
- Schneider, *Nubis, Evolved*, SIGGRAPH 2022
  ([pdf](https://advances.realtimerendering.com/s2022/SIGGRAPH2022-Advances-NubisEvolved-NoVideos.pdf)),
  and *Nubis³*, SIGGRAPH 2023
  ([pdf](https://advances.realtimerendering.com/s2023/Nubis%20Cubed%20(Advances%202023).pdf)).
  These cover clouds you can fly through, compressed voxel clouds with SDF-accelerated marching, and
  0.2 to 5 ms on PS5 depending on view. We take the 2015/2017 model for the global deck. Voxel clouds
  are an option for one hero volume (the Red Mountain plume), not the whole sky.
- Hillaire, *Physically Based Sky, Atmosphere and Cloud Rendering in Frostbite*, SIGGRAPH 2016 PBS
  course ([page](https://www.ea.com/frostbite/news/physically-based-sky-atmosphere-and-cloud-rendering)).
  It gives the energy-conserving analytic step integral (no banding at low step counts), the
  multiple-scattering octave approximation, and how clouds integrate with the atmosphere LUTs.
- Bauer, *Creating the Atmospheric World of Red Dead Redemption 2: A Complete and Integrated Solution*,
  SIGGRAPH 2019 Advances course (ACM SIGGRAPH 2019 Courses). Bauer presents sky, clouds, fog and
  volumetrics as **one integrated system** that a weather state drives, rather than separate effects.
  This is the "RDR2 look" the owner wants. We could only confirm the talk's outline, not its slides.
  The parts we rely on are also in the Nubis and Frostbite sources: the multiple-scattering octaves,
  low-resolution temporal clouds, and froxel fog with a separate far raymarch. Our structure follows
  the talk's thesis, not its implementation.
- UE5 *Volumetric Clouds*, *Sky Atmosphere*, *Exponential Height Fog* and *Volumetric Fog*
  (dev.epicgames.com docs). This is the same split we use: the sky LUTs (Hillaire 2020), a cloud
  component raymarched at reduced resolution with temporal reprojection, cloud shadows on the
  directional light and in the fog, analytic height fog for the far range, and froxel fog for the near
  range.

**Fog and god rays.**
- Wronski, *Volumetric Fog*, SIGGRAPH 2014 Advances (Assassin's Creed IV). Hillaire, *Towards Unified
  and Physically-Based Volumetric Lighting in Frostbite*, SIGGRAPH 2015 Advances
  ([page](https://www.ea.com/frostbite/news/physically-based-unified-volumetric-rendering-in-frostbite)).
  Both describe a camera-aligned froxel grid: density and lighting per froxel (sun with shadow maps,
  local lights, ambient), then front-to-back integration and temporal reprojection. Opaque and
  transparent surfaces then sample the result. UE's Volumetric Fog uses the same technique. God rays
  are the shadowed part of the in-scattering, not a screen-space hack.
- Hillaire, *A Scalable and Production Ready Sky and Atmosphere Rendering Technique*, EGSR 2020
  ([pdf](https://sebh.github.io/publications/egsr2020.pdf)). Bevy 0.20's `Atmosphere` implements this,
  including the aerial-view LUT.

**Wind.**
- Rockenbeck, *Blowing from the West: Simulating Wind in Ghost of Tsushima*, GDC 2021
  ([vault](https://gdcvault.com/play/1027124/Blowing-from-the-West-Simulating)). One wind system drives
  thousands of cloth pieces, hundreds of thousands of particles and millions of grass blades, and the
  wind is a gameplay signal. We have the abstract only. The design point we take: one field, many
  consumers. Wohllaib, *Procedural Grass in Ghost of Tsushima*, GDC 2021
  ([vault](https://gdcvault.com/play/1027033/Advanced-Graphics-Summit-Procedural-Grass)): per-blade
  Bézier bend from the wind, already ARCHITECTURE §7.1.
- SpeedTree wind (Global, Branch, Leaf levels; SpeedTree Modeler docs) and UE *Pivot Painter Tool 2.0*
  (docs). Tree motion is hierarchical: the trunk bends, branches sway around their pivots, and leaves
  flutter. Pivots, axes and hierarchy levels are baked into vertex data or textures offline, and a
  vertex shader evaluates them. Morrowind trees carry no such data, so a bake step derives it.

**Rain, wetness, puddles.**
- Tatarchuk, *Artist-Directable Real-Time Rain Rendering in City Environments*, SIGGRAPH 2006 course
  (ATI ToyShop). Rain is streak particles in camera-relative layers, with occlusion from a top-down
  depth map, splashes and ripples. The industry still does it this way (RDR2, Watch Dogs, UE Niagara
  rain).
- Lagarde, the "Water drop" series, 2012-2013 (seblagarde.wordpress.com): 1 *Observe rainy world*,
  2a/2b *Dynamic rain and its effects*, 3a *Physically based wet surfaces*, 3b *Other wet surfaces*.
  Porous materials darken (albedo²-like) and get smoother. Puddles are a water layer
  (F0 ≈ 0.02, roughness ≈ 0, flat normal, ripple normals). The puddle level follows the height map,
  and accumulation and drying are separate rates. Remember Me used a rain occlusion map.

**Distant shadows.**
- UE: cascaded shadow maps out to *Dynamic Shadow Distance*, *Far Shadow Cascades* for selected
  casters, then *Distance Field Shadows* out to *Distance Field Shadow Distance*
  ([docs](https://dev.epicgames.com/documentation/en-us/unreal-engine/distance-field-soft-shadows-in-unreal-engine)).
  UE5 adds Virtual Shadow Maps. The point is that shadows beyond the cascades come from a cheaper,
  coarser representation, never from more cascades.
- Horizon mapping: Max, *Horizon Mapping: Shadows for Bump-Mapped Surfaces* (1988). Sloan and Cohen,
  *Interactive Horizon Mapping* (EGWR 2000). For a heightfield and a single sun direction, the
  horizon reduces to one "shadow height" per texel, which any receiver can compare against. Terrain
  engines and flight simulators use this for far terrain shadows.
- Cloud shadows: Nubis 2017 (a 2D shadow map from the weather map), UE Volumetric Clouds (cloud
  shadows on the directional light and in the fog).

**Bevy 0.20 (read in `~/.cargo/registry/src/*/bevy_*-0.20.0`).**
- `Atmosphere` + `AtmosphereSettings`: transmittance, multiscattering, sky-view and **aerial-view**
  LUTs. The `render_sky` pass runs after opaque and reads the main depth texture. For geometry pixels it
  composites aerial-view in-scattering and segment transmittance (`aerial_view_lut_max_distance`
  32 km, 32³ by default). **Aerial perspective already applies to everything**, far field included,
  with no depth prepass needed. Its slices are ~1 km deep, so it is haze, not near fog.
- `DistanceFog` (`Linear`, `Exponential`, `ExponentialSquared`, `Atmospheric`): applied per material,
  with **no height falloff**.
- `VolumetricFog` (camera) + `VolumetricLight` (directional/point/spot) + `FogVolume` (box, optional
  3D density texture with scroll offset). It raymarches per pixel per volume, with single scattering
  and 64 steps by default, and reads the main depth texture. Megalophobia measured its limits on large
  boxes (below): the sun term dims with box size, edges are hard, there is a box-intersection bug
  (#22574), and each volume costs one full-screen pass. It is not a froxel volume, has no temporal
  integration, and transparent surfaces can't sample it.
- `DirectionalLightTexture` (feature `pbr_light_textures`, through clustered decals): it multiplies a
  directional light by an R-channel texture projected in light space, tiled or single, for every PBR
  material. This is the cheapest correct hook for **cloud shadows** on every material, ours and
  `StandardMaterial` alike.
- Shadows: at most 4 cascades per light (`MAX_CASCADES_PER_LIGHT`). Past the last cascade's far bound
  `fetch_directional_shadow` returns 1.0, which means **lit, with a hard cut and no fade**
  (`shadows.wesl`). Overlap blending happens only between cascades.
- `ScreenSpaceReflections` requires `DepthPrepass` + `DeferredPrepass`. No height fog, clouds,
  precipitation, wind or GPU particles in Bevy.
- `pbr_lighting::directional_light` and `shadows::fetch_directional_shadow` are public WESL functions.
  We use them for the far-shadow correction (below).

**Crates (crates.io, 2026-10-11).**

| Crate | Latest | Bevy | License | Verdict |
|---|---|---|---|---|
| `bevy_weather` | 0.2.0 | 0.19.1 | MIT/Apache | **Reference.** Full weather plugin: curved cloud shell, CPU cloud shadows as `DirectionalLightTexture` (1.5-20 ms CPU, amortized), `DistanceFog` + fog volumes, GPU rain and snow, a pure-function weather driver (clock + seed). One maintainer, 10 commits, 1 star. Not on 0.20. We'd replace its fog, shadows and coordinates anyway. |
| `bevy_fast_clouds` | 0.19.0 | 0.19 | MIT/Apache | Wavelet cloud volumes, 18 recent downloads. Not Nubis, not on 0.20. Reference only. |
| `bevy-volumetric-clouds` | 0.2.0 | 0.18 | MIT | HZD-style skybox clouds at a fixed 1080p target, **ignores the depth buffer**, no atmosphere integration. Rejected. |
| `bevy_hanabi` | 0.19.0 (main: 0.20.0-dev, still Bevy 0.19) | 0.19 | MIT/Apache | GPU particles. Not on 0.20 yet. Precipitation doesn't need a particle simulator (below). Revisit for splashes and spell VFX when it ports. |
| `bevy_enoki` | 0.8.0 | **0.20** | MIT | 2D sprite particles only. Rejected for 3D. |
| `bevy_firework` | 0.10.0 | 0.19 | MIT/Apache | CPU particles. Wrong scale for rain. |
| `bevy_feronia` | 0.8.4 | 0.18 | MIT/Apache | Foliage scattering + wind shaders. Reference for grass wind, stale Bevy. |

## Reuse decision

| Need | Bevy | Crate | Ours, and why |
|---|---|---|---|
| Sky, sun, aerial perspective | **`Atmosphere` (used)** | n/a | Weather drives its `haze` and medium terms (sky-lighting.md). |
| Clouds | none | `bevy_weather` (0.19, bus factor 1, CPU shadows, no floating origin) | **Ours** in `sf_atmos_bevy`. We port the owner's Megalophobia deck math (below), on a shell with temporal reprojection and the atmosphere LUTs. |
| Cloud shadows on surfaces | **`DirectionalLightTexture` (used)** | n/a | We generate the texture on the GPU (a few µs, vs bevy_weather's ms on the CPU). |
| Near fog + god rays | `VolumetricFog` (prototype only, near-term step N3) | none | **Ours: froxel fog.** Bevy's raymarch is per pixel per box, has no temporal reprojection, transparent surfaces can't sample it, and it has known large-volume artifacts. |
| Height fog, far fog | `DistanceFog` (no height) | none | **Ours**: analytic exponential height fog (UE formula), plus a low-resolution far raymarch shared with the cloud pass. |
| Wind | none | `bevy_feronia` (grass only, 0.18) | **Ours** (`sf_wind`): CPU and GPU twins of one deterministic field. |
| Precipitation | none | `bevy_hanabi` (not on 0.20) | **Ours**: closed-form instanced streaks in camera-wrapped volumes. No simulation, so no particle crate is needed. |
| Wetness, puddles, snow, ash | none | none | **Ours** (`sf_surface`): surface-state clipmap + the shader hook terrain already calls. |
| Distant shadows | CSM (4 cascades max, hard end) | none | CSM retuned + **ours**: a terrain shadow-height map and a far-shadow correction in our materials. |

## Megalophobia (the owner's Bevy project): what to scavenge

`D:\Gamedev\Megalophobia`, Bevy **0.19**, author: the owner (all 10 commits by `lihogloglo`). No
LICENSE file. `THIRD_PARTY_NOTICES.md` lists only `src/gen/erosion.rs` (MPL-2.0, Rune Skovbo
Johansen / Luke Mitchell), CC BY greeble meshes and NASA imagery. None of these are in the files below,
and none of them may move. The files below carry no third-party markers. They cite papers (Schneider,
Hillaire, Wrenninge), not code.

| File | What it is | Take | Change for us |
|---|---|---|---|
| `src/clouds.rs`, `assets/shaders/cloud_deck.wgsl`, `cloud_density.wgsl` | Nubis density (Perlin-Worley + weather map + Worley erosion, height-fraction sampling), Hillaire analytic step integral, multiple-scattering octaves, Beer-powder, two-lobe HG phase, CPU tiling noise generators (Perlin lattice, tiling Worley), per-texture scroll wrapping | **Density model, lighting math, noise generators, scroll rule** ("each texture wraps in its own tile": the bug that taught it is documented) | Slab → spherical shell. Full-res transparent mesh material → quarter-res render-graph pass with temporal reprojection. Fixed sky/ground tints → atmosphere LUTs (sun transmittance, sky ambient, aerial perspective). `#ifdef DEPTH_PREPASS` depth clamp → main depth texture (exteriors have no prepass, far-field.md). Static dither → per-frame blue noise (TAA/DLSS). Startup noise bake → `atmos.noise` bake step. Coordinates → origin-relative + `WorldOrigin` phase. |
| `src/cloud_shadow.rs`, `cloud_shadow.wgsl` | A 60 km alpha-masked quad drawn into the sun's cascades so fog shafts see clouds | **The out-of-range-is-lit rule** and the dithered-threshold penumbra idea | Replaced by a 2D cloud shadow map (`DirectionalLightTexture` + fog/cloud sampling). Our cascades end at ~500 m, and a quad in every cascade costs a shadow pass per cascade. |
| `src/ground_fog.rs`, `ground_fog.wgsl` | Raymarched slab: warped, advected bank noise over an exponential profile + a haze column shadow-sampled for shafts | **The bank density function** | Becomes a density term of the froxel volume and of the far raymarch, not its own pass. |
| `assets/shaders/grass_wind.wgsl` (+ `grass.rs` notes) | Two-octave curl-noise gust field advected downwind, gust envelope with calm bias, swirl capped so blades never lean upwind, `time_prev` for motion vectors, one shared function for forward/prepass/shadow | **The gust field, nearly as is** | WGSL + Rust twin in `sf_wind`. Origin phase instead of a camera focus point. Uniform from `sf_wind_bevy`. |
| `src/volumetrics.rs` | `FogVolume` setup + tiling 3D noise | Lessons only (why `FogVolume` fails at scale) | n/a |

**Measured cost there** (RTX 4060 Laptop, dev build, window resolution not recorded, TRACKER.md
session 11): cloud deck **5.8 ms** (full resolution, 64 + 6 steps, 3 octaves), ground fog slab
**2.5 ms**. That is the reason for our quarter-resolution, temporally amortized design. With the same
math the expected cost is 0.5 to 1 ms at 1440p (budgets below).

**Provenance gate:** before any of this code enters superfargoth, the owner states in writing (a
§10 entry) that they license these Megalophobia files as MIT OR Apache-2.0. Ports are rewritten for
0.20/WESL anyway, but they are derived works of those files.

## Canonical technique (overview)

RDR2/UE-style **integrated atmosphere**. One `WeatherFrame` per frame (Bevy-free, deterministic)
parameterizes these passes (raster path; Solari notes per section):

```
CPU:  SkyClock + climate + seed ──► sf_weather::evaluate ──► WeatherFrame
        ├─► sf_sky::Weather (haze, cover dimming)          ├─► sf_wind::WindState (+ local sources)
        ├─► AtmosParams (clouds, fog, lightning)           ├─► PrecipParams, SurfaceRates
        └─► cues (sound loops, thunder, gameplay tags)

GPU (per frame, exterior camera):
  compute: wind uniform/buffer · cloud shadow map (512², amortized) · shadow-height map (amortized)
           · surface-state clipmap update · froxel density+lighting → integrate (temporal)
  shadow passes (CSM, 4 cascades to ~500 m) · occlusion camera (amortized, precipitation)
  opaque (world materials: surface hook + far-shadow correction + wind in vertex)
  Bevy render_sky (sky + aerial perspective)
  sf_atmos composite: clouds + far fog (¼ res, temporal, upsampled) · froxel fog apply on opaque
  transparent (water, glass: sample froxels) · precipitation streaks · splashes
  post (bloom, tonemap, DLSS/TAA)
```

## Crate split (rule 7)

| Crate | Bevy? | Contents | Depends on |
|---|---|---|---|
| `sf_wind` | no | `WindSettings`, `WindState`, `WindSource` (directional, radial, vortex), CPU `velocity_at(WorldPos, t)`, the WGSL module as a string, `wind.paint` bake step (feature `bake`) | `sf_coords`, `glam` |
| `sf_weather` | no | `WeatherPreset`, `Climate` records, state machine, forecast, blending, `WeatherFrame`, lightning schedule, cue list | `sf_records`, `sf_wind`, `sf_sky` (for `Weather`, `KeyCurve`) |
| `sf_atmos` | no | Cloud/fog parameter structs + GPU layouts, tiling noise generators, `atmos.noise` bake step, CPU cloud density and shadow-height evaluation (tests, gameplay queries such as "is it under cloud") | `glam`, `sf_bake` (feature `bake`) |
| `sf_surface` | no | Accumulation model (rates, drying, permeability), coarse persistent world grid (save data), character wetness model | `sf_coords` |
| `sf_wind_bevy` | thin | Uploads wind uniform + source buffer, `WindPhase`, WESL `sf_wind::sample` | `sf_wind`, `sf_world_bevy` (origin) |
| `sf_weather_bevy` | thin | `WeatherPlugin`: clock → evaluate → feeds sky, wind, atmos, precip, surface; lightning light; cue messages; Luau/script hook later | `sf_weather`, `sf_sky_bevy` |
| `sf_atmos_bevy` | heavy shaders, thin Rust | Cloud pass, cloud shadow map, far fog, froxel fog, shadow-height map, `sf_far_shadow` WESL, `DirectionalLightTexture` wiring | `sf_atmos`, `sf_wind_bevy`, `sf_terrain_bevy` (clipmap binding) |
| `sf_surface_bevy` | thin | Surface-state clipmap, occlusion camera, the shared `surface_state.wesl` (moves here from `sf_terrain_bevy`), `WorldMaterial` (= `ExtendedMaterial<StandardMaterial, SurfaceExt>`) | `sf_surface`, `sf_wind_bevy` |
| `sf_precip_bevy` | thin | Streak/flake/ash volumes, splashes, ripple normal atlas | `sf_surface_bevy` (occlusion), `sf_wind_bevy` |

Wide and shallow. `sf_wind` is tiny and stable, so the many wind consumers don't depend on the
weather crate, which changes often. `sf_sky` stays the owner of `Weather` for the sky. Its `Wind`
type moves to `sf_wind` and is re-exported (proposed change below). Bevy features: `sf_atmos_bevy`
needs `bevy_pbr` (+ `pbr_light_textures`), `bevy_core_pipeline`, `bevy_render`. The others need
`bevy_pbr` or less.

## Weather: state machine and presets as data

**Records** (`sf_records` kinds; adapters generate them, mods patch them field by field):

```rust
pub struct WeatherPreset {
    pub id: RecordId,
    pub clouds: CloudLayers,          // per layer: coverage, type (stratus 0 → cumulus 1), density,
                                      // base/top (m above sea level), erosion, wind factor
    pub sky: SkyTerms,                // haze multiplier, dust/ash absorption tint, overcast dimming,
                                      // optional sun/ambient tint KeyCurves over sun elevation
    pub fog: FogTerms,                // height fog: density, falloff (m), base height, albedo, phase g;
                                      // ground banks: density, height, noise scale; KeyCurves over
                                      // time of day (morning fog)
    pub precipitation: Vec<PrecipTerm>, // kind: RecordRef<PrecipKind> (rain, snow, ash, …), intensity
    pub wind: WindTerms,              // speed range, gustiness, gust length, direction bias/lock
    pub surface: SurfaceRates,        // wet/puddle/snow/ash accumulation and drying per hour
    pub lightning: Option<Lightning>, // strikes per minute, flash intensity, sound set
    pub cues: Vec<Cue>,               // ambient loops, precipitation loops (RecordRef to sound records)
    pub tags: Vec<(RecordRef, f32)>,  // gameplay tags with strength (e.g. a disease-exposure tag)
    pub transition: TransitionTiming, // per-channel lead/lag: clouds first, precipitation last
}
pub struct Climate {                  // "region weather"
    pub id: RecordId,
    pub weights: Vec<(RecordRef<WeatherPreset>, f32)>,
    pub change_interval_hours: (f32, f32),
    pub transition_hours: f32,
    pub storm_source: Option<WindSource>, // e.g. radial wind away from a volcano
}
pub struct PrecipKind { pub id: RecordId, pub look: PrecipLook /* streak | flake | dust */,
    pub fall_speed: f32, pub size: [f32; 2], pub texture: VfsPath, pub splash: Option<VfsPath>,
    pub accumulates: SurfaceChannel /* wet | snow | ash | none */ }
```

Durations are in **game hours** on `SkyClock`, so time scale and sleeping work.

**Which climate applies.** An adapter hook `ClimateSource::climate_at(ws, WorldPos) -> RecordRef`
(Morrowind: the cell's region). A bake step `weather.climate` rasterizes it into a coarse per-worldspace
grid with distance to the nearest border. The camera's weather is the blend of the climates within a
**border band** (default 256 m), so crossing a region border is a spatial crossfade, not a pop. RDR2 and
UE weather volumes behave the same way.

**State and forecast.** Per climate: `ClimateWeather { current, next, change_at, transition_start }`.
The next preset is a **pure function** of `(world seed, climate id, change index)` (weighted pick from
a counter-based hash), as in bevy_weather's driver. So saves store only the seed plus explicit
overrides, and a forecast can be queried. Scripted overrides (`force(climate, preset, hours)`,
weight edits from quests) are records in the runtime overlay, so saves keep them.

**Blending.** `WeatherFrame = blend(presets, weights, t)`: continuous fields interpolate per channel
with the preset's lead/lag (clouds 0-60% of the transition, wind 20-80%, precipitation and fog 40-100%).
Discrete kinds (precipitation, sound loops) crossfade by intensity. Rain fades out while snow fades
in; they never morph. Surface rates add.

**Outputs.** `WeatherFrame { sky: sf_sky::Weather, wind: WindState, clouds, fog, precip, surface_rates,
lightning_events, cues, tags, visibility_m }`. `visibility_m` (Koschmieder from fog + precipitation)
feeds AI perception and the dev HUD.

**Interiors** have no weather. Their fog comes from an adapter-supplied fog-only preset per interior
(an environment volume, M8). Interiors marked as exterior-like get their climate's weather.

**Lightning**: the strike schedule is a Poisson process from the hash, so it is deterministic and
forecastable. A strike = a 0.1-0.3 s flash (a temporary point light up in the cloud + cloud emission +
the directional sky term) and a thunder cue delayed by distance / 343 m/s.

### Weather core as built (WE1, we1, 2026-10-18): `sf_weather`

**Reuse check (rule 8).** Bevy 0.20 has no weather. No maintained crate does a forecastable weather state
machine (crates.io/web, 2026-10-18: Bevy state-machine crates `seldom_state`, `moonshine_behavior` are
entity FSMs with stored state; `bevy_weather` 0.2 is on Bevy 0.19 and its driver was the reference
above). The core is ~1 000 lines of plain Rust, so: ours. Randomness is a counter-based hash
(SplitMix64 finalizer over `(seed, record key, salt, counter)`), not a `rand` RNG: stateless, O(1) per
slot, nothing to save. Record keys are FNV-1a of the folded id, so load order and added mods never
reshuffle a forecast.

**Crate** (Bevy-free; deps `glam`, `serde`, `smallvec`, `sf_records`, `sf_sky`, `sf_wind`, and
**`sf_atmos`**, not in the plan above: the frame carries the renderer's `CloudWeather` and `FogSettings`
directly instead of mirror structs, and `visibility_m` uses `FogSettings::extinction_at`; `sf_atmos` is
`glam` + `rayon` only). `sf_records::RecordId` gained serde (its folded string) for record references.

**Records** (kinds `weather.preset`, `weather.climate`, `weather.precip`; `register_kinds`; all
`#[serde(default)]`, so a record file or patch names only the fields it sets). As the sketch above,
with these differences: the record id is the database key (no `id` field); references are `RecordId`
(no typed `RecordRef<T>` exists yet); `CloudTerms` is `CloudWeather` as data with a cloud-altitude
`speed` (the direction is the ground wind's); `FogTerms` holds the height fog and bank layer terms plus
`height_by_hour` / `banks_by_hour` multiplier curves, `SkyTerms` a `haze_by_hour` curve (empty curve = 1);
`WindTerms { speed: [min, max], gustiness, gust_length, direction_deg, direction_jitter_deg, storm }`;
`PrecipKind` has an `extinction` (1/m at intensity 1) for visibility; `Climate` has `cues` (region
ambient loops) and `one_shots` (`OneShotSet`, random one-shot ambience: sounds with a chance per draw, rest silence,
interval in **real seconds**; the audio layer schedules the draws, a "scatterer" as in FMOD Studio), and
`WeatherPreset.climate_ambience` (0..1, cue channel) scales the climate's loops and one-shots, so storms hush
birds (we2c1). `WeatherBook::from_db` resolves everything into indices once (rebuild when weather
records change); unknown references become `warnings`, not panics.

**State machine.** Per climate, slot `k` starts at `phase + k·mean + jitter·(2u_k − 1)` with `mean` the
middle of `change_interval_hours` and `jitter` a quarter of its range, so every interval stays inside the
range and the slot containing any time is found in O(1) (no summing of past intervals). Slot `k`'s preset
is a weighted draw from `hash(seed, climate, k)`. `WeatherState { seed, overrides }` is the save: an
`Override::Force { climate, preset, start, transition_hours, until }` (default: until the first slot start
one minimum interval later) or `Override::Weights { climate, from, weights }` (slots starting at or after
`from`). Overrides are **timestamped entries in the state, not record patches**: a patch has no "from
when", so editing weights through the record overlay would rewrite the past and the current weather.
A **segment** is one transition (at a slot start, a forced change, or a force's end) from the blend just
before it (recursively, so a change interrupting a change starts where it was) to one preset.
`forecast(climate, from, n)` lists segments; `current` is the preset being turned into.

**Blending.** Six channels with windows from `TransitionTiming` (fractions of the transition, smoothstep
inside): clouds 0-60 %, sky 0-60 %, wind 20-80 %, fog 40-100 %; precipitation (also lightning rate,
surface rates, tags) in 40-100 % and **out 0-60 %**, cues the same. Continuous channels use the incoming
preset's window (weights sum to 1); discrete channels fade each outgoing preset by its own `*_out` window
and the incoming one by its in-window, summed per precipitation kind or sound, so rain stops before the
sky clears, and rain → snow is two crossfading entries, never a mixed kind. Per preset the wind wanders
inside its speed range and ±jitter direction by smooth value noise (3 h lattice) keyed by climate and
preset; presets blend as velocity vectors; `gust_length` is discrete (the heaviest preset's, one swap at
mid-transition, since `sf_wind` snaps it to lattice divisors).

**Border band.** `border_weight(d, band)` = ½ on the border, 1 from `band/2` inside (smoothstep);
`border_blend(own, neighbor, d, band)` returns the climate weights `evaluate` takes. The grid lookup
(`weather.climate` bake, `ClimateSource`) is WE2/WE3.

**Lightning.** A non-homogeneous Poisson process by thinning (Lewis and Shedler 1979): candidates at the
cap rate (`LIGHTNING_CAP_PER_HOUR` = 120 per game hour) are drawn per 1/30 h bucket (Poisson count by CDF
inversion, uniform times, all hashed), each kept with probability `rate(t) · climate weight / cap`, where
`rate(t)` is the precipitation-channel blend of `Lightning::per_hour`. Strikes carry time, azimuth,
distance, flash, and a thunder sound. Deterministic and independent of frame boundaries; steps over 1 h
(sleep, skips) report none. **Rates are per game hour** (every weather duration is game time); the adapter
converts Morrowind's real-time frequency with its time scale.

**Frame** (`evaluate` / `evaluate_into`, which reuses allocations): `sky: sf_sky::Weather` (cloud cover
= `CloudWeather::mean_cover`, haze, dust, dimming, and **`sky.wind`, the wind target** for
`WindField.state.wind`; the transition windows are its easing), `clouds: CloudWeather` (moving with the
wind direction at the cloud speed), `fog: FogSettings` (weather fields; look fields default, the plugin
keeps its own; banks drift with the ground wind), `precip`, `surface`, `lightning` (strikes in
`[prev_hours, hours)`), `lightning_per_hour`, `cues` (weather and region loops), `one_shots` (climate sets on, with volume), `tags`, `storms` (a
climate's `storm_source` with strength = the wind-channel blend of `WindTerms::storm` × climate weight;
WE3 adds/updates the `WindSource`), `visibility_m` = min(3.912 / (fog outside banks at the camera
altitude + Σ precipitation intensity × extinction), 100 km). No climate (interiors): calm defaults.

**Verified** (`cargo test -p sf_weather`, 10 tests): forecast purity (same seed same slots, load order
irrelevant, intervals in range, `slot_at` inverts `slot_start`), draw frequencies within 4σ of the
weights over 8 000 slots, save/restore (RON) with a force and a weight edit gives identical frames over
220 h, lead/lag (clouds half way at 30 %, no rain before 40 %, rain gone at 60 % of the way out,
continuous weights sum to 1), rain → snow crossfade (only the two kinds, monotone, no coverage pops),
border walk (2 m steps, no pops in coverage, snow or wind), lightning (count within 4σ of 30/h over
400 h, exponential gaps (CV ≈ 1), identical strikes at 30 s and 8.6 min steps), wind easing and
visibility, records through `sf_records` with a mod patch. **`evaluate` cost: 0.0006 ms** (release, 2
climates mid-transition, lightning on; gate 0.05 ms).

**Open (logged).** Overrides are never pruned (a long game with many scripted changes grows the save;
prune entries older than one interval + transition, keeping the last weight edit). `gust_length` swaps
once mid-transition (crossfade two wind frames if it shows). Lightning flash position is camera-relative
(no world anchor). Interior fog-only presets (M8). `storm_intensity` for NPC animation: derive from
`storms` when needed.

## Clouds

**Model** (Nubis 2017, Megalophobia math). The weather map (R coverage, G type, B wetness/darkness,
A anvil) is a 2D texture that weather writes (below). The shape volume is 128³ RG8 (Perlin-Worley,
Worley FBM) and the detail volume 32³ R8 (Worley FBM). Both tile and come from the `atmos.noise` bake
step, which is deterministic and cached. Type height profiles follow Nubis/Megalophobia
`combine_density`/`erode_density`.

**Geometry.** A spherical shell around the planet center that `Atmosphere`'s transform defines
(`inner_radius` + base/top from `CloudLayers`). Rays intersect it analytically. Morrowind's flat world
under a curved shell gives the correct horizon falloff, as in HZD. Up to two layers (low cumulus/stratus
deck, high cirrus as a 2D textured layer at 6-8 km: one fetch, as in Nubis/UE).

**Coordinates.** Shaders see origin-relative positions (big_space). Noise is sampled at
`rel + WorldOrigin.phase_16384`. Tile sizes divide 16 384 m (weather 16 384 m, shape 4 096 m, detail
256 m). Wind scroll offsets are wrapped **per texture in its own tile** on the CPU in f64. That is the
Megalophobia rule; one shared offset makes coverage jump.

**Driving from weather.** The weather map is not a static texture. A small compute pass synthesizes it
each time weather changes enough (or every N frames): base coverage noise (2 octaves, advected by
`WindState` at cloud altitude) thresholded by `clouds.coverage`, type from `clouds.type`, plus storm
cells (a tiling cellular pattern whose density follows precipitation intensity). Transitions are then
smooth, because only scalars change, and clouds visibly build up before rain. Storm cells can also be
anchored by the adapter (an ash plume over a volcano: a coverage boost around a world position).

**Rendering.** An `sf_atmos_bevy` render-graph node after Bevy's `render_sky`:
1. Raymarch at **quarter resolution** (half each axis), updating one pixel of each 2×2 block per frame
   (Ultra: every pixel at half res) with per-frame blue-noise jitter. 48-96 primary steps, adaptive:
   big steps in empty space, small inside cloud. Early out at transmittance < 0.01. 5-6 light samples
   with growing steps (cone-like), 3 multiple-scattering octaves.
2. Lighting: sun radiance × atmosphere **transmittance LUT** at the sample's altitude (sunset clouds
   turn red for free), ambient from the **sky-view LUT** (upper hemisphere) and a ground-bounce term
   (ground albedo × sun irradiance), Beer-powder, two-lobe HG. The moon works the same way through the
   second directional light. Lightning adds emission inside the strike's cloud.
3. Aerial perspective between camera and cloud: aerial-view LUT up to its range (32 km), transmittance
   LUT segment beyond. Clouds then fade into the same haze as the far field behind them.
4. Depth: the march stops at the scene depth from the **main depth texture** (bound like Bevy's
   `render_sky` does; no prepass). Mountains and far-field proxies cut into low cloud correctly.
5. Temporal reprojection (cloud motion + camera reprojection by the cloud's depth-weighted distance,
   neighborhood clamp), then a depth-aware upsample to full resolution. Composite: `color × T + L`.

The far fog raymarch (below) runs in the same node and resolution, so the two share reprojection.

**Cloud shadows.** A compute pass writes a **512² R8 cloud shadow map** covering 16 km around the
camera, projected along the sun direction (optical depth through the layer, 8 samples, low detail).
It updates 1/4 of the texels per frame. Three consumers:
- every PBR material, through `DirectionalLightTexture` on the sun (and moon). The light's
  `Transform` is re-centered on the camera each frame and scaled to the coverage, and `tiled: false`;
- froxel fog and far fog (shafts under cloud gaps: the RDR2 look);
- the surface model (rain under the clouds, not under a clear hole).

Past its coverage it reads as "average cover" (the weather map's mean), never black. That is
Megalophobia's out-of-range-is-lit rule, generalized.

**Interaction with the far field.** The far field is opaque geometry in the depth buffer, so clouds,
cloud shadows and aerial perspective need nothing special from it. Proxies receive cloud shadows
through `DirectionalLightTexture` (`StandardMaterial`) or the shared WESL (impostors). Clouds cast no
CSM shadows (`NotShadowCaster`). Cloud shadows come only from the shadow map.

**Solari path:** the same cloud pass composites after Solari's output. Cloud shadows reach ray-traced
lighting only through the sun's `DirectionalLightTexture`, if Solari honors it (unverified; open
question 5).

### Clouds as built (clouds session, 2026-10-12)

**Crates.** `sf_atmos` (Bevy-free, 10 tests): `CloudSettings` = `CloudWeather` (coverage, type,
density, wetness, storm, erosion, base, thickness, wind: what weather drives, `lerp` for transitions)
+ `CloudLook` (extinction, fill, phase, tiles, reach, shadow strength) + `CloudQuality` (steps,
octaves, checkerboard, shadow map); presets `CloudPreset::{Clear, Fair, Overcast, Storm}`; the
Megalophobia noise generators (128³ RG8 shape, 32³ R8 detail) plus a 512² weather-noise map; the
density/weather functions as the CPU twin of the WESL; `CloudScroll` (wind travel wrapped at 16 384 m,
per-texture offsets that include the origin phase). `sf_atmos_bevy` (thin Rust, WESL): `CloudsPlugin`,
`Clouds` (resource the weather writes), `CloudCamera`, `CloudShadowLight`, `CloudsOrigin`.

**Deviations from the plan above (and why).**
- *Weather map:* no compute pass writes one. A fixed tiling weather-noise texture (R coverage field,
  G type field, B storm cells) is thresholded per sample by the `CloudWeather` scalars
  (`sf_atmos::weather_at`, same math in WESL). Same behavior (only scalars change, clouds grow where
  the field is high, storm cells add cover and wetness), one fetch, no pass. A written map comes back
  when adapter anchors (Red Mountain plume, open question 3) or painted climates need it.
- *Noise* is generated at startup on a background thread (226 ms, clouds appear when ready), not by
  an `atmos.noise` bake step yet (next step: cache it through `sf_bake`).
- *Composite order:* Bevy's `render_sky` is private, so the composite is ordered after every system
  that runs before `main_transparent_pass_3d` (found in the Core3d graph at `finish`): named scaffold
  `SkyOrderScaffold` in `render.rs`, removal path a public sky set upstream.
- *Atmosphere bindings:* group 0 replicates `render_sky`'s bind group from bevy_pbr's public resources
  (LUTs, uniforms, depth), so the cloud WESL imports `bevy_pbr::atmosphere::functions` unchanged.

**Passes (per `CloudCamera`, raster path).** Shadow map before the main pass; after the opaque pass:
trace (one pixel of each 2×2 half-res block per frame, 32-80 adaptive steps by segment length, IGN
jitter per frame, 5 growing light steps, 3 octaves, Beer-powder, two-lobe HG, sun or moon (brightest
light) through the transmittance LUT at the sample, sky fill from the sky-view LUT, ground bounce,
aerial perspective from the aerial-view LUT and beyond it the sky's in-scatter in proportion to the
transmittance reached), resolve (reprojection by transmittance-weighted cloud distance, minus the
wind step, plus the origin shift; 3×3 neighborhood clamp on this frame's samples; fresh pixels blend
50 %), composite (depth-aware bilinear: a tap whose cloud lies behind the pixel's geometry is dropped;
blend `One, SrcAlpha`). The march ends at the main depth buffer and at the ground sphere, so nothing
draws under the horizon or on the sea. MSAA layouts exist but only `Msaa::Off` was run.

**Cloud shadows.** 512² (1024² since S1, below) R8 render target, 16 km square in the sun's light plane centered on the
camera (snapped to the 32 m texel grid), 8 samples through the shell per texel, edge fading to the
mean cover; given to the sun as a **tiled** `DirectionalLightTexture` (Bevy reads 0 = black outside
an untiled texture, which breaks the out-of-range rule). The light's `GlobalTransform` gets the
scale/translation after propagation (Bevy builds cascades from its rotation only). Since S1
(clouds3) the map is toroidal and time-sliced, 6 samples per texel: see "S1 cost" below. Moon: none yet.

**Floating origin.** The app writes `CloudsOrigin` (absolute position of the origin cell) in its
origin sync; noise offsets carry `origin mod tile` (tiles divide 16 384 m), and the resolve shifts
the history by the origin delta, so neither clouds nor their history jump on shifts.

**App (Bevyrrowind).** On by default in exteriors with the sky; `--off clouds`; `--clouds
clear|fair|cumulus|overcast|storm`; viewer keys F10 (on/off) and C (cycle presets). Heights halved for
the compressed world (`CLOUD_HEIGHT_SCALE` 0.5: fair base 750 m, top 1,450 m), shape tile 2,048 m,
detail 128 m. Visual checks CL-1..CL-6 (awaiting owner).

**Measured** (release, 2560×1440 native, RTX 4060 Laptop on AC, Red Mountain camera at 16:30,
2 × 12 s each, back to back; `shots/clouds/ab.sh`, results `shots/clouds/perf/`): frame median
`--off clouds` 8.97 / 8.98 ms; clear 9.59 / 9.60 (+0.62); fair 9.98 / 9.91 (+0.97); overcast
9.90 / 9.89 (+0.92); storm 9.86 / 9.84 (+0.88). Against the High budget (clouds + far fog 0.9 ms,
shadow and weather maps 0.05 ms) the clouds alone sit at the budget with no far fog yet. The fixed
part (+0.6 ms with almost no cloud) is the next target: amortize the shadow map, skip empty-space
samples by weather coverage before the shape fetch, and a cheaper composite. No `sf_diag` GPU
counter yet (`atmos/clouds/gpu_ms`).

**Not built yet.** High cirrus layer, far fog in the same node, lightning emission, quality presets
(Low 1/16 res), Solari path (open question 5), MSAA run, `atmos.noise` bake step, GPU timer.

### Cloud genera, scale, motion, shadows, sun occlusion (S1, clouds2 session, 2026-10-13)

Owner feedback (CL-1, CL-3 FAILED): two overcasts + one pattern + clear; one cloud covers the
island; nothing moves; no shadow edge on the ground; the sun seems to draw over the clouds.

**What was wrong.**
1. *Shadows:* Bevy 0.20 defines `LIGHT_TEXTURES` only when clustered decals are usable
   (`bevy_pbr` mesh.rs), which needs the `pbr_clustered_decals` cargo feature. We enabled only
   `pbr_light_textures`, so the sun's `DirectionalLightTexture` was extracted but **never
   sampled**: no cloud shadows at all, anywhere. Fix: `sf_atmos_bevy` enables both features.
   Also: the map now uses the detailed (eroded) density, full extinction (the cookie scales
   only the sun's direct light; sky fill stays, so `shadow_strength` 0.9 is physical), a
   bilinear repeating sampler, and 1024² over 16 km (16 m texels).
2. *Scale:* the weather tile was 16 384 m with its coarsest coverage octave at a quarter
   tile: cloud-field blobs of ~4 km over an island ~5 km across, and the app scaled only the
   heights. Real cumulus are 0.5-2 km wide (humilis/mediocris), congestus towers 1-3 km,
   cumulonimbus cells 5-10 km, cirrus at 6-12 km (WMO International Cloud Atlas). Now the real
   scale defaults are weather 8 192 m (2 km blobs), shape 2 048 m, detail 256 m, cirrus
   16 384 m, storm cells a third of the weather tile; and a compressed world scales **sizes
   and heights together** (`CloudLook::scaled` + `CloudWeather::scaled_heights`, powers of
   two so tiles still divide 16 384 m). Bevyrrowind uses 0.5: fair cumulus a few hundred
   meters wide, base 600 m.
3. *Motion:* 9 m/s is a real-world speed; beside a 30x game clock it reads frozen. The
   framework keeps physical preset winds; the adapter sets the speed (Bevyrrowind:
   Morrowind's per-weather `Cloud Speed` x 10 m/s, cloudy 20 m/s). Shapes evolve
   (Schneider 2017's advected weather map, done without a pass): the cloud field is two
   octaves of the weather map (the second transposed) drifting `evolve` (0.35) of the wind
   apart, contrast-restored (`sf_atmos::evolve_field`), so clouds form and dissolve as they
   drift; the billows rise through the shape volume (its vertical axis tiles: a `rise`
   phase) and the detail octave boils 3x faster.
4. *Variety:* nine genus presets (`CloudPreset`): clear, fair (cumulus humilis/mediocris),
   cumulus (congestus), stratocumulus, altocumulus, altostratus, cirrus, overcast
   (stratus/nimbostratus), storm (cumulonimbus cells + high veil). The volumetric layer holds
   one low or mid genus at its own base/thickness/type/erosion; the **high layer** is the
   planned 2-D cirrus sheet (Nubis/UE practice): a sphere at `cirrus_height` (8 km real),
   one fetch of the weather map's new A channel (domain-warped fibres along X, tiling),
   lit as thin ice (two-lobe HG g 0.8/0.2 + sky fill), alpha from the slant optical depth,
   composited behind the low layer, moving at 2x the wind (parallax), faded past the reach
   (no mips), and in the shadow map. `CloudWeather` gained `cirrus` and `cirrus_height`.
5. *Sun through clouds:* the composite order was right (no "render_sky not found" warning;
   the composite runs after `render_sky`), but the march stops at T < 0.01 and kept that
   1 %: the HDR sun disk is ~10^5 times the sky, so 1 % still burned through and bloomed.
   Now the remainder is snapped to zero (`T' = (T − 0.01) / 0.99`, Nubis): behind thick
   cloud the disk is gone; thin edges still let it through (correct, silver lining).

**Weather mapping** (Bevyrrowind, weather-adapter.md): Clear → clear, Cloudy → fair,
Foggy → altostratus, Overcast / Rain / Snow / Blizzard → overcast, Thunderstorm → storm,
Ashstorm / Blight → stratocumulus (fast). Cumulus, altocumulus and cirrus are reachable by
genus name and for mods.

**Test scenes.** Framework: `cargo run --release -p sf_atmos_bevy --example clouds -- --preset
<genus> --scale 0.5 --view sky|sun|shadow|up [--screenshot f.png]` (24 km plane, pillars every
500 m for scale; C cycles genera). App: `clouds-*` scenes, visual checks CL-1..CL-6.

**Cost.** Per density sample one more 2-D fetch (second weather octave); per pixel two more
(cirrus); the shadow map is 4x the texels with the detail fetch. Measured: see the S1
session log line in ROADMAP (shared machine).

**S1 cost (clouds3, 2026-10-13).** GPU timers: every cloud pass is a `RenderDiagnosticsPlugin`
span (`clouds_shadow`, `clouds_trace`, `clouds_resolve`, `clouds_composite`; the perf report's
"GPU passes" table). Per-pass GPU ms are the A/B measure: on the shared machine the frame was
CPU-bound (render thread ~20 ms), so frame-median deltas against `--off clouds` were noise.
Release, 2560×1440, RTX 4060 Laptop, Balmora camera toward Red Mountain at 16:30
(`shots/clouds3/ab.sh`, results `shots/clouds3/perf/`; shared machine: other agents compiled):

| preset | before: trace / shadow / composite / resolve = total | after (High) |
|---|---|---|
| fair | 1.02 / 0.44 / 0.19 / 0.05 = 1.70 | 0.68 / 0.04 / 0.19 / 0.05 = 0.95 |
| cumulus | 0.72 / 0.54 / 0.19 / 0.05 = 1.49 | 0.55 / 0.04 / 0.18 / 0.05 = 0.83 |
| overcast | 0.53 / 0.59 / 0.19 / 0.05 = 1.35 | 0.29 / 0.09 / 0.19 / 0.05 = 0.61 (8 slices) |
| storm | 0.53 / 0.69 / 0.19 / 0.05 = 1.47 | 0.38 / 0.09 / 0.19 / 0.05 = 0.71 (8 slices) |

Against High (clouds + far fog 0.9, shadow + weather maps 0.05): the shadow map is in budget
(0.04 at 16 slices); clouds alone are 0.91 on fair (at the line, no far fog yet), 0.79 cumulus,
≤ 0.62 overcast/storm. What changed:
1. *Shadow map:* **toroidal** (the light texture repeats; the light sits at a world anchor that
   moves in 2 km cells; each texel holds the repeat of its sky column nearest the camera, edge
   fade by distance from the camera), so texels stay fixed in the world and the map can be
   **time-sliced** (as reflection probes and far cascades are): one band of `size / 16` rows per
   frame (`CloudQuality::shadow_slices`, High 16, Ultra 4; `sf_atmos::ShadowSlicer`). Stale
   bands are ≤ 16 frames old: ≤ 5 m of drift at cloudy speed against 16 m texels. A full redraw
   on an anchor move, a light turn of more than 4 m at the camera since the oldest band (a
   time-of-day jump), or a weather jump (a preset switch, not a blend). Each texel now marches
   its column from sea level (before: from camera height). 6 samples per texel (Ultra 8).
2. *Trace:* the step is a continuous function of the span (`thickness / 20`, stretched only to
   keep the count in 32..64): an integer count dividing the span evenly jumps at every count
   change, which drew the faint **horizontal bands** in thick decks. Empty-space skipping
   (Schneider 2015): twice the step with the cheap density outside cloud, step back on a hit,
   back to coarse after 4 empty samples. Lighting LOD: two light samples (stretched to the same
   reach) where T < 0.3 or past 15 % of the reach; the first light sample reuses the primary
   sample's weather (a tenth of the layer away). High `max_steps` 80 → 64.
3. *Cumulus* coverage 0.40 → 0.34 (more sky between towers from the ground).

**Next.** Crisper cauliflower billows (curl-noise distortion of the detail lookup, height-
dependent erosion remap; tops read as soft vertical columns today), anvils (weather map A is
now cirrus: anvils need a written map), a cheaper composite (0.19 ms, a constant full-resolution
cost), far fog in the cloud node (no budget left on fair: trim the trace first), Low/Medium
presets measured.

## Fog: aerial perspective, height fog, ground fog, volumetric fog, god rays

Four ranges, one look:

| Range | Technique | Where |
|---|---|---|
| 0 to ~384 m (High) | **Froxel volumetric fog**: 160×90×64 at 1440p (16 px tiles, exponential slices), RGBA16F scattering/extinction | `sf_atmos_bevy` compute |
| froxel end to ~8 km | **Far fog raymarch** at ¼ res, 16 steps, in the cloud node | same node as clouds |
| analytic, all distances | **Exponential height fog** (UE formula: closed-form integral of `ρ0·e^(−(h−h0)/H)` along the ray, with sun in-scattering lobe), used for the far fog density and as the Low preset | shared WESL |
| 1 km+ | **Aerial perspective** (Bevy `Atmosphere`), haze driven by weather | Bevy |

**Froxel pass** (Wronski 2014, Hillaire 2015):
1. Density per froxel = height fog (preset) + ground fog banks (Megalophobia bank function: warped
   value noise advected by `sf_wind`, thresholded, over an exponential profile, origin-phased) +
   precipitation mist (intensity-scaled extinction, e.g. rain haze, ash) + local `FogVolume`-like
   records (boxes/ellipsoids with density + noise: swamps, caves, the Ghostfence, spell clouds).
2. Lighting per froxel: sun/moon × atmosphere transmittance × **CSM shadow** (near) ×
   **shadow-height map** (terrain, to the horizon) × **cloud shadow map**, with HG phase. Plus the
   sky ambient from the sky-view LUT, plus clustered point/spot lights (Bevy's cluster buffers, with
   a cap per froxel). A sun shaft is just a froxel lit through a gap.
3. Temporal reprojection (jittered slice depth, 90-95% history), then front-to-back integration into
   an in-scatter/transmittance volume.
4. Apply: a full-screen pass on opaque pixels (main depth) after `render_sky` and before the
   transparent pass. The transparent world materials sample the volume in their fragment shader
   (a `sf_atmos::fog` WESL function the water and glass materials call).

**God rays** come from the froxels near and the far fog farther out: shafts between buildings at
sunrise (CSM), shafts from ridges (shadow-height map) and shafts under cloud gaps (cloud shadow map).
No screen-space radial blur.

**Ground fog** (Bitter Coast morning): a time-of-day `KeyCurve` on the preset (dense from 4 to 9 h)
× climate. Banks drift with the wind. The far fog continues them past the froxels, so a valley of fog
reads from a ridge.

**Bevy `DistanceFog`**: off on the exterior camera when `sf_atmos` runs, or both would fog surfaces.
Kept for interiors until the environment volumes of M8.

**Solari path:** froxel lighting uses the raster CSM. On the ray-traced path the sun's shadow maps are
off (sky-lighting.md), so the froxels use the shadow-height map + cloud shadows only (or a cheap
low-resolution CSM kept for the fog alone). Open question 6.

### Fog as built (M6 F2, m6-f2, 2026-10-13): height fog, ground-fog banks, far fog

Reuse check: Bevy 0.20 has `DistanceFog` (per-material, distance-only, no height, no noise; it would also fog
the sky twice with the atmosphere) and `VolumetricFog`/`FogVolume` (box volumes; Megalophobia measured it grey
and dim at our scales, and it is the froxel tier's job, F3). Neither does height or bank fog over km views, and no
maintained crate targets 0.20, so the fog is ours, in the cloud node as planned.

- **Model** (`sf_atmos::fog`, CPU twin + tests): two exponential layers over sea level, `FogSettings { height,
  banks, albedo, phase_g, ambient, far, sky_distance, wind }`. Height fog = UE's exponential height fog
  (`FogLayer::optical_depth`, closed form written as a difference of end densities so steep rays never overflow;
  density capped at e² × below the base). Banks = a second, thin layer (12 m falloff in the app) × the Megalophobia
  bank mask: the weather map's G channel at two scales (1 km and 256 m tiles), domain-warped by a third sample
  (2 km tile, 220 m), stretched around its median (`NOISE_MID`/`NOISE_GAIN`, measured: the channel's 5-95 % spread
  is 0.29-0.43) and thresholded (coverage 0.42 + 0.18 ramp: ~half the ground in banks, a third in clear gaps,
  test `banks_cover_part_of_the_ground`). `FogDrift` advects banks with the ground wind (warp at 0.35×, wisps at
  1.35×), wrapped at 16 384 m with the origin phase like the clouds' scroll.
- **Render** (`sf_atmos_bevy`, same node as the clouds; runs with the clouds off too):
  1. *Far fog* in `clouds_trace`: per traced pixel, 16 quadratic steps over the ray's part inside the bank layer,
     out to `far` (6 km) or the depth buffer, store the bank **modulation** = the noise's share of the layer's
     optical depth, weighted by what the eye still sees (G channel of the distance target, now `Rg32Float`).
     `clouds_resolve` reprojects and clamps it with the clouds.
  2. `clouds_fog_light` (new, 66×1 `Rgba32Float`, once per frame): sky fill per view azimuth (sky-view LUT toward
     the horizon + zenith) and the key light (sun or moon, as seen at the camera) for the composite.
  3. `clouds_composite`, full resolution: both layers' closed-form optical depth to **this pixel's own depth**
     (sky pixels: `sky_distance`, 20 km), the bank layer × the bilaterally upsampled modulation, lit by the key
     light (HG g 0.6 over an isotropic floor) and the sky fill × albedo; composed as `T·(clouds) + L` in front of
     the clouds, so stars, moons and clouds behind dim with it. Depth edges are exact (the analytic part is per
     pixel); the reduced-resolution part only shapes the banks.
- Bevy `DistanceFog` was already off on the exterior camera; interiors keep their look. The sea scaffold became
  opaque (the composite runs before transparents and fogs by depth). **Transparents (particles, glass) are not
  fogged yet:** F3's `sf_atmos::fog` function for materials.
- Weather: `exterior.rs` `SkyLook::fog` per weather + `ground_fog(hour)` (weather-adapter.md "Fog"); `--fog <x>`
  scales, `--off fog` drops it. `sf_sky::Weather` stays the medium's input; the fog lives beside the clouds'
  settings because the node that draws it reads `sf_atmos` only (WE3's `WeatherPlugin` drives both).
- Also: Bevy's `sky_view_lut_samples` 16 → 32 on sky cameras (the F1 sea-horizon seam; a few µs).
- **Cost** (release, 2560×1440, RTX 4060 Laptop, `fg-balmora-foggy-noon`, `--off fog` A/B; shared machine: another
  `bevyrrowind.exe` used the GPU in every run): `clouds_fog_light` 0.004 ms; bank march +0.05-0.17 ms in
  `clouds_trace`; composite +0.36 ms when it walked the lights and sampled the LUTs per pixel, within noise once they
  moved to the fog-light texture. The 0.1 ms far-fog gate needs a quiet re-measure. Levers if over: fewer bank steps
  (8), skip the march where the bank layer's analytic depth is negligible, half-rate modulation.

### Froxel fog as built (M6 F3, m6-f3/m6-f3b, 2026-10-13): god rays, lit fog, transparents

**Bevy's `VolumetricFog` (decision, re-checked for 0.20).** Not used, on exteriors or as a stopgap (N3 stays
skipped, owner decision 7). `VolumetricFog` + `VolumetricLight` + `FogVolume` raymarch each pixel through each box
volume (`step_count` 64 by default) at full resolution: cost scales with pixels × steps × volumes (3.7 M pixels at
1440p vs 0.92 M froxels at High), there is no temporal reprojection (only per-frame jitter for TAA), transparent
materials can't sample it, it has no height-fog or bank density and no terrain or cloud shadow terms, and large
volumes show the artifacts of #22574. Our froxels reuse F2's density terms and the shadow maps we already have.
Revisit `FogVolume` only for small local volumes in interiors (M9 environment volumes), where its box shape fits.

- **Model** (`sf_atmos::froxel`, CPU twin + tests): `FroxelSettings { size, near, far, history, max_lights, jitter }`,
  presets Medium 128×72×48 to 256 m, **High 160×90×64 to 384 m (default)**, Ultra 160×90×128 to 512 m. Slices are
  exponential in distance along the view ray (not view z), `near` 0.5 m, so a pixel's slice depends only on its
  own distance (~10 % of the distance per slice at High: 38 m deep at the far end). `integrate_step` = Hillaire
  2015's energy-conserving slab integral (test against the homogeneous closed form).
- **Render** (`sf_atmos_bevy`, compute, in the cloud node after `clouds_fog_light`, before the trace):
  1. `atmos_froxel_inject` (8×8×1 groups, one thread per froxel): a Halton (2,3,5) jittered point inside the
     froxel; density = F2's height layer + bank layer × the bank mask (`fog_bank.wesl`, now a shared library with
     its own weather-map binding; the cloud trace calls the same function). Light = the key light seen at the
     camera (`clouds_fog_light`) × HG phase × **CSM** (Bevy's cascades and comparison sampler, one hard tap: a
     froxel is far coarser than a texel) × **terrain shadow-height map** (`FogShadowHeights`, the same rule as
     `sf_lod_bevy`'s far shadow: penumbra, distance bias, border fade) × **cloud shadow map**, + sky fill ×
     ambient, + **point lights**: the 32 nearest visible `PointLight`s whose range reaches the froxels (CPU pick
     per frame, unshadowed: Bevy's cube maps aren't sampled; a named limit), with Bevy's range falloff. Temporal:
     the point is reprojected into last frame's volume (origin shifts applied), history weight 0.9 inside the
     screen and the range, ping-pong `Rgba16Float` volumes.
     **Temporal sampling (shimmer, 2026-10-19):** the first build used ONE Halton(2,3,5) offset for the whole
     volume (16-frame cycle) and read the history at the jittered point. Every froxel erred in step, a
     history of 0.9 keeps ~26 % of a 16-frame cycle, and screen TAA cannot clamp a coherent change: the fog
     pulsed (owner report; measured ~1-2 % of frame luminance, static camera, clock paused, worst where shadow
     and bank edges sit in the volume). As built: `FroxelJitter::PerFroxel` (default): the frame offset is R3
     (Roberts 2018, never repeats) rotated per froxel by a pcg3d hash (Cranley-Patterson), so the residual is
     spatially uncorrelated noise the history and TAA/DLSS average; the history is read at the froxel centre
     (a still camera reads texel centres, no drift). A/B `--off fogdither` (old), `--off fogjitter` (centres).
     Cost: one integer hash + one centre ray per froxel (not benchmarked; negligible next to the CSM tap).
  2. `atmos_froxel_integrate` (one thread per tile column): front to back into (in-scatter, transmittance), into
     the shared `FroxelVolume` image.
  3. `clouds_composite`: the froxels from the camera to `min(depth, far)`, then F2's analytic layers only from the
     froxel end (`layer_depth` from the point at `far`), so the near fog isn't counted twice; the far bank
     modulation still shapes the analytic part. Sky pixels: froxels to `far`, analytic beyond.
  The froxels need the cascades bound (`ViewShadowBindings`); without them (or `Fog::froxels = None`, the Low
  preset) the composite fogs everything analytically as in F2.
- **Transparents** (`FroxelVolume`, `froxel_apply.wesl`, `FroxelFogMaterial`): the integrated volume is a shared
  3D `Image` that blended materials sample at their own screen position and distance (UE/Frostbite's
  translucency path). `froxel_fog_apply` handles blend, premultiplied (in-scatter × alpha), additive and
  multiplicative (transmittance only: the fog light isn't added twice). `FroxelFogMaterial` =
  `ExtendedMaterial<StandardMaterial, FroxelFogExt>`; the plugin keeps every such material's volume and params
  current. Past the froxel range the ray's medium carries on as the froxels saw it (`froxel_fog_lookup`: `T_far^(d/far)`, in-scatter × `(1 − T)/(1 − T_far)`; fogfix 2026-10-19), the stand-in for the composite's analytic far layers (exact for level rays through height fog, over looking up, under looking down; W2's water draws its own far fog). **As built in the app (fogfix):** `transparent_fog.rs` swaps every blended/additive/premultiplied/multiply `StandardMaterial` (NIF leaves without an alpha-test ref, the Ghostfence field, glows, glass, particles) for one fogged `FroxelFogMaterial` copy per source and copies NIF controller changes over; `--off transfog` is the A/B. Audit: the order (prepass → opaque + alpha-mask incl. impostors → composite → transparent → TAA/post) is the canonical split; the bug was the unwired translucent half. Open: Morrowind leaves (blend + test GREATER 0) are blended, so they skip depth, shadows and motion vectors; mapping them to alpha-test (UE "Masked" foliage) needs a re-bake and an owner look call. Light sprites (`sf_lod_bevy`, additive, night) are not fogged yet. Cost: one trilinear 3D fetch per translucent fragment (not benchmarked).
- Synthetic scene: `cargo run --release -p sf_atmos_bevy --example clouds -- --view glass --fog 400 [--time 23]`
  (glass panes beside stone panes at 20/60/150/300 m, a lantern by each).
- Weather (app): clear and fair keep a thin valley haze (40 km / 30 km visibility at sea level, ×1.5 at dawn by
  the ground-fog curve) so dawn shafts have a medium; `--off froxels` (alias `godrays`) is the Low-preset A/B.
- **Cost** (release, 2560×1440, RTX 4060 Laptop, `fg-balmora-foggy-noon`, A/B `--off froxels`, quiet machine for the
  froxel run): `atmos_froxel_inject` 0.122 ms, `atmos_froxel_integrate` 0.023 ms, composite +0.01 ms: **≈ 0.16 ms**
  against the High budget of 0.7 ms; timed GPU total 10.35 vs 10.21-10.23 ms, frame median 15.53 vs 15.31 ms (within
  noise). Bug found on the way: a render-world-only volume image was re-created every frame (+4 ms CPU); fixed.
  Not yet measured: the 3070, flight at 60 m/s for temporal smear, F2's quiet `--off fog` gate.
- Solari path: without sun cascades the CSM term is 1, so near shafts come only from the terrain map and clouds. Open
  question 6 stands (a fog-only low-res CSM vs SHM + clouds only).

## Wind

**Field** (`sf_wind`; the CPU and WGSL twins agree to 1e-4, checked by a test that runs the WGSL on
the GPU headless and compares; see "Wind as built" below):
- **Global**: direction + mean speed from `WindState` (weather). Speed eases with `WindTerms`.
- **Gusts**: Megalophobia's two-octave curl-noise field advected downwind. It produces discrete gust
  cells that form, travel, swirl and die, plus a calm bias for patchy stillness and a capped swirl so
  nothing leans upwind. Sampled at `rel + phase_1024` (gust scale ≤ 1 024 m divides the period).
- **Local sources**: up to 32 `WindSource`s in a storage buffer (directional gust, radial burst for
  explosions and spells, vortex, a storm source radial from a world point). Each has a falloff and a
  lifetime. Gameplay adds and removes them.
- **Altitude**: speed × a log profile above ground. Cloud wind uses its own altitude speed from the
  preset.
- **Later (open question 2)**: a Tsushima-style low-resolution 3D wind texture around the camera with
  advection/injection, if the analytic field + sources looks too uniform in practice.

**Consumers.** All read either `sf_wind::sample(rel_pos, t)` in WGSL or `velocity_at` on the CPU:

| Consumer | Use |
|---|---|
| Grass (ARCHITECTURE §7.1) | blade lean = field direction × gust envelope. Bézier bend + flutter. The same function in forward, prepass, shadow and motion-vector passes (`time_prev`), the Megalophobia lesson |
| Trees (cooked L0) | hierarchical sway from baked vertex data (below) |
| Impostors | per-instance trunk bend: the quad's top vertices shear by the field at the instance + a per-model stiffness from the atlas header |
| Cloth (M4) | CPU `velocity_at` per cloth body (aerodynamic force) |
| Precipitation | streak velocity = fall + wind. Snow/ash also flutter |
| Clouds, fog banks | scroll/advection (cloud-altitude speed for clouds) |
| Water (M6 water spec) | wind speed/direction for FFT wave spectra |
| Audio | wind loudness/gust cues |

**Bake-time wind data** (`wind.paint`, a bake step in `sf_wind` (feature `bake`) over `mesh.cook`,
guided by an adapter hook in the style of `FarSource`:
`fn wind_profile(&self, model, summary) -> Option<WindProfile>`):
- For models the adapter marks as foliage (Morrowind: `flora_*`, like the impostor rule): per vertex
  `wind0 = (pivot.xyz relative to the model, stiffness)` and `wind1 = (level 0 trunk/1 branch/2 leaf,
  phase, flutter weight, height fraction)`.
- Derivation without authored data: trunk = the connected component(s) containing the lowest
  vertices. Branches and leaf cards = the other connected components, after welding by position. Pivot
  = the component's vertex nearest the trunk axis. Alpha-masked materials → leaf level. Stiffness from
  the component's size and height (Pivot Painter conventions). Deterministic; keyed by `mesh.cook`
  and the profile's version.
- Stored as a separate product (one vertex stream per cooked LOD) that `sf_model_bevy` appends as two
  optional attributes at load, so `mesh.cook` stays unchanged. **The ray-tracing variant keeps the exact
  position/normal/uv/tangent layout** (Solari rejects other layouts, sky-lighting.md open question
  3): `sf_model_bevy` drops wind attributes from the RT LOD, so ray-traced trees are static.
- Merged far-field meshes (L1-L3) carry no wind. At 400 m+ sway is sub-pixel. Impostors use the
  per-instance bend above.

### Wind as built (WD1 part 1, wd1a, 2026-10-18): `sf_wind`

**Reuse check (rule 8).** Bevy 0.20 has no wind. `bevy_feronia` 0.8 (Bevy 0.18) is grass-only shader
wind with no CPU twin. Noise crates (`noise`, `fastnoise-lite`) are CPU-only and their float hashes
can't be mirrored bit for bit in WGSL. So: our own, porting Megalophobia's `grass_wind.wgsl` (MIT OR
Apache-2.0 by the owner, ARCHITECTURE §10 2026-10-12; provenance in `sf_wind`'s crate docs and the
shader header).

**API** (Bevy-free; deps `glam`, `serde`, `sf_coords`):
- `Wind` (moved from `sf_sky`, re-exported there unchanged: `velocity` at the reference height,
  `gustiness`, `gust_length`, `lerp`, plus a stateless `velocity_at(pos, t)` for constant wind).
- `WindSettings` (swirl 0.9, gust bias 0.42, band 0.3 (Megalophobia's), gust travel 1.0 × mean speed,
  roughness z0 0.1 m, reference height 10 m (WMO; weather-adapter.md maps `Wind Speed` at 10 m), profile
  cap 2).
- `WindState` (saveable): `wind`, clock, the two octaves' drift integrated in f64 and wrapped to
  `PERIOD` (so a speed change never jumps the field), up to `MAX_SOURCES` = 32 `WindSource`s
  (`Directional { direction }`, `Radial`, `Vortex { axis }`; center `WorldPos`, strength, inner/outer
  radius smoothstep falloff, attack/duration/release envelope; `add_source` returns `None` when full;
  expired sources drop in `advance`).
- `WindFrame` = `state.frame(&settings, origin)`: `WindParams` (6 × vec4, 96 B uniform) + `WindSourceGpu`s
  (3 × vec4, 48 B each, ≤ 1.5 KB storage), each with `to_array()` for the upload, and the CPU sampler
  `sample(rel, above_ground)`, `global`, `gust`, `profile`. `state.velocity_at(&settings, WorldPos, h)` builds
  a frame around the point's cell.
- `sf_wind::WESL`: `src/shaders/sf_wind.wesl`, pure functions, no imports, no bindings: `wind_global(params,
  rel, above_ground)`, `wind_gust(params, rel_xz) -> WindGust { dir, curl, gust }`, `wind_profile`,
  `wind_source_count`, `wind_source(source, rel)`. Consumers bind the two buffers and sum the sources.

**Model.** `v = dir × speed × (1 + gustiness·(2·gust − 1)) × profile(h) + Σ sources`. `dir` is the axis
turned by the capped curl (at most `asin(swirl)` off it, so never upwind); `gust` the smoothstepped
two-octave potential. Profile: `ln(h/z0 + 1) / ln(h_ref/z0 + 1)`, 0 at the ground, capped. Sources:
directional `dir·s·fall`, radial `r·s·fall / max(|r|, 0.5 m)`, vortex `(axis × r)·s·fall / max(|axis × r|,
0.5 m)`; sources get no altitude profile and the never-upwind cap applies to the weather wind only.
Grass flutter (Megalophobia's harmonic `wind_press`) is a consumer detail, not part of the field.

**Changes from Megalophobia, and why.**
1. **Integer lattice hash** (PCG, Jarzynski and Olano 2020) instead of `fract(dot(...))`: the float hash
   multiplies values to ~1e4 before `fract`, so one ulp of FMA difference moves the noise by ~1e-3 and the
   twin could never hold 1e-4. PCG on wrapped integer cells is bit-exact on CPU and GPU.
2. **Origin phase.** The lattices tile `PERIOD` = 1 024 m (4 grid cells; gust cells snap to `1024/n`, the
   second octave to `1024/round(n/0.37)`). The CPU computes `(origin mod 1024 − drift) mod 1024` in f64 from
   the origin's cell (exact) and uploads it **split** into whole lattice cells (`cells`, integers, added to the
   cell index) and a fraction, so the shader's float math is only `rel/cell + frac`. A first version added the
   meter offset to `rel` and lost 1.3e-4 m/s across a recenter (ulp of 1 200 m); split, the error is the
   rounding of `rel` itself.
3. **Drift integrated on the CPU** per octave (Megalophobia multiplied time by a fixed speed).

**Twin verification (choice).** `tests/wgsl_twin.rs` runs `WESL` + a small compute harness on the GPU with
headless `wgpu` 30 (Bevy 0.20's version; dev-dependency only, so no library pulls it), default adapter,
falling back to the software adapter (WARP on Windows), skipping with a message if neither exists. Not
"naga + mirrored golden table": naga has no interpreter, and a table generated by the CPU twin is
circular; a table captured from the GPU would go stale with every shader edit. The GPU run is the real
thing and costs ~0.5 s. Golden points: 2 048 scattered over ±600 m and −1 to 400 m above ground, ±4 cells of
lattice boundaries, the source centers. Tolerance 1e-4 **relative above 1 m/s** (`|Δv| / max(|v|, 1)`):
GPUs contract `rel·inv + frac` into an FMA, a 1-ulp difference that the steep gust envelope amplifies.
Measured (RTX 4060 Laptop): worst 1.1e-5 relative (2.4e-4 m/s absolute at a 39 m/s peak), gust envelope
4.3e-6.

**Other tests** (`cargo test -p sf_wind`): never upwind (swirl 0.99, gustiness 1, three directions,
20 steps × 500 points, also the turn angle ≤ asin(swirl)); phase-exact across a ~390 m origin recenter
330 km out (≤ 5e-5 m/s, measured 2.9e-5: the rounding of `rel`); tiles the period; a speed change keeps
the gust pattern; log profile; sources (radial burst envelope and expiry, vortex handedness, directional,
capacity 32); patchiness; `Wind::velocity_at` stays within `mean·(1 ± gustiness)` (the old `sf_sky` test,
still passing through the re-export).

**Open (logged, not built).** Changing `gust_length` across a snap step (`n` changes) swaps the lattice:
crossfade two frames if a weather blend shows it. No eviction when 32 sources are live. Vortex falloff is
spherical (no tornado height). Motion vectors need last frame's `WindParams` (wd1b: upload a `prev` copy or
let consumers pass it). Cloud wind keeps its own altitude speed (clouds read only the direction).
`WindTerms` easing belongs to `sf_weather` (WE1).

### Wind as built (WD1 part 2, wd1b, 2026-10-18): `sf_wind_bevy`

**Reuse check (rule 8).** Bevy 0.20's `ShaderBuffer` asset (`bevy_render::storage`) is the upload: a
same-size rewrite is a `queue.write_buffer` into the existing GPU buffer, so bind groups (and every
material that binds it with `#[storage(n, read_only)]`) stay valid. WESL modules go through
`bevy_shader` as usual. Nothing to build but the glue.

**Plugin** (`WindPlugin { wind, settings }`; deps `bevy_app/asset/ecs/math/platform/render/shader/time`,
`sf_coords`, `sf_wind`; **not** `sf_world_bevy` as planned above: the app writes the origin, like
`sf_atmos_bevy::CloudsOrigin`, which keeps big_space and the streaming graph out of the crate):
- `WindField` resource: `state: WindState` (weather writes `state.wind`, gameplay `add_source`),
  `settings`, and after `WindSystems::Advance` (`PostUpdate`) `frame()` / `prev_frame()` / `sample(rel, h)`
  (the CPU twin for cloth, particles, audio). `WindOrigin(WorldPos)`: written by the app before
  `PostUpdate`'s advance. `WindStats`: CPU ms of the advance + upload.
- Each frame: `state.advance(dt)` (virtual time, so pausing stops the wind), `frame(origin)`, and the
  previous state framed at the **current** origin; pack and rewrite the buffer.
- **One buffer, not uniform + storage** (the handoff's plan): `WindBuffer(Handle<ShaderBuffer>)` holds
  `WindData { params, prev_params, sources: array<WindSource, 32>, prev_sources: array<WindSource, 32> }`,
  3 264 B, fixed size (so "bind at least one element" never arises and the size never changes). One
  binding slot per consumer, and materials get it through `AsBindGroup` with no custom bind group.
- Without rendering (`Assets<Shader>`/`Assets<ShaderBuffer>` absent, a server) only the CPU field runs.

**WESL.** `sf_wind::WESL` is inserted as a UUID-handle `Shader` at path `embedded://sf_wind/field.wesl`;
bevy_shader turns `embedded://a/b.wesl` into module `a::b`, so consumers `import sf_wind::field::...`.
`sf_wind_bevy::wind` (`src/wind.wesl`, `load_shader_library!`) declares
`@group(constants::SF_WIND_GROUP) @binding(constants::SF_WIND_BINDING) var<storage, read> wind: WindData`
and `wind_sample(rel, above_ground)` / `wind_sample_prev(...)` (global + loop over the live sources).
The slot comes from two UInt shader defs (`wind_shader_defs(group, binding)`), which bevy_shader exposes
as `constants::*` (the pattern bevy_pbr uses for `MATERIAL_BIND_GROUP`).

**Motion vectors (choice).** The buffer carries last frame's field (`prev_params`, `prev_sources`);
prepass shaders call `wind_sample_prev` beside `wind_sample`. This is how Bevy handles every other
vertex animation (previous joint matrices for skinning, previous morph weights): a second copy of last
frame's inputs, not "sample again at `t − dt`" (the field is integrated, so last frame can't be
recomputed from a time). Last frame is re-framed at the current origin, so a consumer passes the same
origin-relative position to both and an origin shift adds no false motion.

**Upload cost.** Advance + two `frame()` + pack + buffer write, 32 live sources (worst case), measured
by `cargo test -p sf_wind_bevy upload_cost` (the dev/test profile here is optimized): **0.0025 ms** mean over
2 000 frames (gate 0.02 ms); the `wind` example logs 0.003-0.007 ms per frame live (RTX 4060 laptop).

**Example** `cargo run -p sf_wind_bevy --example wind`: 21×21 CPU-twin arrows (gizmos, 2 m) over a
ground shaded by the GPU `wind_sample` (calm dark blue, mean green, 2× mean yellow); Space radial burst,
V vortex, Up/Down speed, G gustiness; `--screenshot out.png --frames N --burst F`.

**App** (bevyrrowind `exterior.rs`): the weather's `Wind` feeds `WindField.state.wind` (until WE1's
`sf_weather` writes it), the terrain origin feeds `WindOrigin`; fog drift reads the field's mean
velocity; clouds take the field's direction at their own altitude speed (Morrowind `Cloud Speed`).

**Open.** No consumer samples the GPU field in game yet (grass, trees, precipitation: their lanes). Cloud
and fog drift take the mean wind only (no gusts at that scale, by design).

### WE3 as built (part 1, we3a, 2026-10-19): `sf_weather_bevy`

**Reuse check (rule 8).** Bevy has no weather system; the crate is glue between `sf_weather` (logic), `sf_sky_bevy`
(clock, sky medium) and `sf_wind_bevy` (field). Deps: `bevy_app/ecs/platform`, `sf_diag_bevy` (counters),
`sf_sky_bevy`, `sf_weather`, `sf_wind_bevy`, `smallvec`. Not on `sf_atmos_bevy`: clouds and fog read the frame from
their own consumers (part 2).

- `WeatherPlugin { book: Arc<WeatherBook>, state: WeatherState }` inserts `WeatherRecords`, `WorldWeather` (the save
  data: seed + overrides; scripts and debug flags `force` through it), `WeatherSite` (camera climates with spatial
  weights + altitude, **written by the app**: only it knows its regions), `CurrentWeather { frame, hours, valid }`,
  `WeatherOverrides { haze, dust }` (debug pins), `WeatherStats`.
- `WeatherSystems::{Evaluate, Apply}`, chained in `Update` after `SkySystems::Clock`, before `SkySystems::Apply`.
  Evaluate: game hours = `SkyTime.days × hours_per_day`; lightning window `(prev, now]` clamped to
  `MAX_STEP_HOURS` (0.25 h, a clock jump skips strikes) and empty on the first frame or when the clock goes back;
  an empty site skips (keeps the last weather, e.g. interiors). Apply: sky `haze`/`dust`/`dust_tint` written only on
  change (the sky's rate-limited rebuild sees no churn); `WindField.state.wind` = `frame.wind()`; one radial source
  per storming climate, added once and retuned in place (new `WindState::source_mut`) so its attack never restarts.
- **Not driven yet:** the sky's `cloud_cover` / `overcast_dimming` (they change the lit look; part 2 with the cloud
  genus blend), clouds, fog, precipitation, lightning light, cues, border blend.
- **Replication (co-op, net.md):** the frame is a pure function of (records, `WorldWeather`, clock, site); a host
  replicates the seed + overrides and the clock, each peer evaluates its own site.
- Counters: `weather/eval/ms`, `weather/apply/ms`, `weather/strikes`. Cost: evaluate 0.0027 ms mean, whole
  `app.update` with the plugin 0.005 ms (test `cost_per_frame`, opt-level 1; gate 0.05 ms).

### WE3 as built (part 2a, we3b1, 2026-10-19): clouds, fog, sky cover

- `sf_weather_bevy` now also depends on `sf_atmos_bevy` (the `Clouds` and `Fog` resources are the consumers; a
  separate glue crate would be one more node for three small systems). `WeatherSystems::Apply` runs `apply_sky`,
  `apply_clouds`, `apply_fog`, `apply_wind`; all write only on change, and each is a no-op without its resource.
- **Clouds:** `Clouds.settings.weather` = `frame.clouds` (the per-channel genus blend: coverage, type, density,
  wetness, heights, cirrus, and the wind at cloud altitude = ground wind direction × the presets' blended `speed`).
  The look (`CloudLook`, noise tiles) stays the app's, so its `scaled(0.5)` halving survives. No pops: every term is a
  smoothstep blend on the clouds window (0-60 %), and the scroll integrates the wind (`CloudScroll` travel), so a speed
  change never jumps positions.
- **Sky:** `cloud_cover` (= the drawn layer's `mean_cover`) and `overcast_dimming` join haze/dust/tint.
- **Fog:** `fog_settings(frame.fog, scale, look)`: height fog, bank layer + coverage, albedo, phase, ambient and wind
  from the frame (hour curves already applied by `evaluate`), the renderer's look fields (reach, sky distance, bank
  noise tiles/warp) kept. Fog follows on its own window (40-100 %): clouds lead, fog lags.
- `WeatherOverrides` gains `clouds: Option<CloudWeather>` (a pinned genus; its wind stays the weather's) and
  `fog_scale: Option<f32>` (density multiplier).
- Tests (6 green): `frame_drives_clouds_and_fog` (clouds = frame, fog ×2 scale, look fields kept, cover/dimming, pin
  keeps the wind); `transition_glides_clouds_lead` (clear → overcast over 2 h in 400 steps: max per-step change within a
  smoothstep's bound, clouds pass half-way before the fog). Cost unchanged (gate 0.05 ms).


### WE3 as built (part 2b, we3b2, 2026-10-19): precipitation, lightning, cues, border blend

- **New crate `sf_precip_bevy`** (thin; `PrecipPlugin`, `Precip` resource, `PrecipMaterial`, `precip.wesl`). The
  technique of "Precipitation" below minus occlusion: three nested camera-centered boxes (8 / 24 / 64 m, 3000 / 6000 /
  8000 drops at intensity 1), one draw each per kind slot (2 slots, so a crossfade draws both; 6 entities). Drop
  position closed-form in the vertex shader: `cam + (fract((seed·L + offset − cam) / L) − ½)·L`, the offset being
  velocity (fall + wind × follow) integrated on the CPU and wrapped to `[0, L)` (a wind change never jumps drops).
  A drop shows while its rank (evenly spread, shuffled) < intensity. Streaks stretch along the velocity
  (`STREAK_S` 0.035 s of travel) and face the camera; flakes are soft discs that flutter. Minimum width 1 px
  (flakes 1.5) with alpha dimmed to keep energy; faded at the box faces, inside the nested box, and within 1 m of
  the camera. Far boxes are more transparent. Alpha blended, no prepass, no shadows, `NoFrustumCulling`.
  **No particle crate:** `bevy_hanabi` is not on Bevy 0.20 (crate table above) and the closed form needs no simulator.
- `PrecipKind.tint` (linear rgb + opacity) is new. Looks: `Streak` = rain (wind follow 0.8); `Flake` = snow
  (flutter 0.15 m); `Dust` = blowing grit **drawn as wind streaks** (flutter 0.3 m, follow 1): ash storms at 10-12 m/s
  read as near-horizontal blowing ash (owner, Next up 6a), not specks.
- `sf_weather_bevy::fx`: `apply_precip` maps `frame.precip` into `Precip` slots (a kind keeps its slot while it falls:
  `assign_slots`), wind = `frame.wind().velocity`, brightness = `precip_brightness(sun elevation, overcast dimming,
  flash)` (night floor 0.04). Written only on change.
- **Lightning:** `LightningStrike::{flash_at, thunder_delay_s, thunder_volume}` in `sf_weather` (pure: decay over
  `fade_s` + two re-strokes placed by the strike time; delay = distance / `SPEED_OF_SOUND` 343). `apply_lightning`:
  the latest strike restarts `LightningFlash` (real time); a shadowless `DirectionalLight` (`LightningLight`, spawned on
  the first strike, hidden between flashes) from the strike's azimuth at 0.95 rad elevation, `LIGHTNING_LUX` 40 000 ×
  flash; **cloud emission stand-in:** the clouds' `look.ambient` × (1 + 6 × flash), restored after (a true in-cloud
  emission at the strike position is still open). Rain brightens with the flash.
- **Cues:** `WeatherSound` message: `Loop { sound, volume, region }` on start / stop (volume 0) / change ≥ 0.02;
  `OneShot { sound, volume, azimuth, distance_m }` for thunder when its delay has passed. **No audio backend in the
  workspace yet**, so nothing plays them (host's job, logged). Climate one-shot sets are not emitted yet.
- **Border blend (app):** `weather_site` finds the nearest grid square of another climate among the 8 around the
  camera (distance to that square), and writes `border_blend(own, neighbor, d, band)`, band = min(`BORDER_BAND_M`,
  2 squares) ≈ 234 m in Morrowind; written only on a weight change ≥ 1e-3. Interiors switch `Precip.enabled` off.
- Tests: sf_precip_bevy 3 (mesh/ranks, wrap, nesting/light), sf_weather `thunder_delay_and_flash_are_pure`,
  sf_weather_bevy `frame_drives_precip_cues_and_lightning`, `slots_keep_their_kind_through_a_crossfade`.
- **Not built:** occlusion map (rain under roofs / trees outdoors), splashes, ripples, fog on the streaks (transparent
  fog path, fogfix), textures (`PrecipKind.texture` unused: procedural shapes), cloud shadow-map rate modulation,
  in-cloud emission at the strike, audio playback.


## Precipitation

**Technique** (Tatarchuk 2006; RDR2 and Watch Dogs follow the same structure):
- **Volumes**: 3 nested camera-centered boxes (near 8 m, mid 24 m, far 64 m). Each is one instanced
  draw of N quads. A drop's position is closed-form: `p = wrap(hash(id) · size + v·t, camera box)`,
  with `v = fall + wind`. No simulation, no CPU work, no spawn/despawn. The far layer uses thinner, more
  transparent streaks, and the fog provides the far "rain curtain".
- **Looks** (`PrecipKind`): rain streaks (stretched along screen-space velocity, refraction-free
  luminance from the sky ambient + nearby lights, lit by lightning), snow/ash flakes (flutter from the
  wind field, soft round sprites, ash in a dark grey-brown with the sun scattering forward),
  dust (dense, small).
- **Occlusion**: a top-down depth map along the precipitation direction (default 1024² over 128 m,
  R32F). It comes from an **occlusion camera**: an orthographic `Camera3d` rendering opaque casters
  into a small target, no atmosphere or post, refreshed when the camera moves 1/8 of the coverage or
  every 8 frames (amortized). A render-graph node copies its depth into the persistent texture.
  Streaks below the occluder depth are discarded in the vertex shader. The same map decides which
  surfaces get wet, snowy or ashy (below).
- **Splashes**: N splash sprites per frame at random XZ around the camera, placed at the occlusion
  depth (the top surface there), with an animated sprite sheet. Ripples on puddles and water: a
  tiling animated ripple normal atlas (Lagarde 2a) that the surface hook samples.
- **Drips, wet camera**: out of scope until a camera FX spec.
- Rate = `PrecipTerm.intensity` × the cloud shadow map's local cover (no rain under a clear hole when
  the weather is "showers").

## Surface state

WISHLIST §5 and early decision 5, built on the hook `sf_terrain_bevy` already calls
(`apply_surface_state(&pbr, world_xz)`, a no-op today).

**Data.**
- **Surface-state clipmap** (GPU, RGBA8, 2 levels: 512² at 0.25 m = 128 m, and 512² at 2 m = 1 km,
  toroidal like the height clipmap): R wetness, G puddle water level, B snow depth, A ash depth.
- **Exposure**: a surface point is exposed if its height ≥ the occlusion depth − ε (open sky) or it
  faces up under the open sky (objects: per-pixel check in the hook. The clipmap stores the terrain/top
  surface).
- **Update compute** each frame (a few µs): `wet += rate·exposed·dt − drying(sun, wind, temperature)·dt`,
  puddle level rises after wetness saturates, snow/ash add when their channel's precipitation falls
  and melt/blow off per preset. Rates come from `WeatherFrame.surface_rates` (game hours).
- **Coarse persistent grid** (`sf_surface`, CPU, 32 m cells over the worldspace, in the save): fed by
  the same model at low rate. It seeds the clipmap when regions stream in, so you don't walk into a dry
  town after a storm.
- **Permeability per surface** (`TerrainLayer.surface` record, terrain.md): porosity (how dark and how
  fast), puddle allowed, snow/ash adherence. Objects get it from a material extras key or a default.

**Material response** (Lagarde 3a/3b), in `surface_state.wesl`:
- wet porous: albedo × `mix(1, albedo-dependent darkening, wet·porosity)`, roughness → `mix(r, 0.1, wet)`,
  specular F0 unchanged;
- puddles: where `puddle_level > height_signal` (terrain: the layer height channel already used for
  height blending; objects: an up-facing mask × a world-space noise), a water layer: albedo darkens,
  roughness 0.02, normal → flat + ripple normals × rain intensity, F0 0.02;
- snow/ash: on `N·up > threshold`, height-blended with the layer height (terrain.md height blend),
  albedo/roughness/normal toward the snow/ash material (`PrecipKind` points at its layer). Terrain also
  displaces by depth through the deformation stack (terrain.md layer 3);
- far field: proxies and impostors get the wetness darkening only (cheap, sub-pixel puddles).

**Which materials get the hook.** Terrain (has it), impostors and proxies (`sf_lod_bevy` materials),
and **all cooked world statics**. The streaming `MaterialMapper` (streaming.md) maps cooked materials to
`WorldMaterial = ExtendedMaterial<StandardMaterial, SurfaceExt>` instead of `StandardMaterial`. Bindless
and batching are kept (one extension type). Its fragment builds `PbrInput` the way Bevy's does, calls
the hook, and lights with `apply_pbr_lighting`. The hook is cheap (one clipmap fetch + ALU) and
compiled out per quality preset.

**Reflections.** Wet looks need reflections. The environment map (`AtmosphereEnvironmentMapLight`)
gives the sky everywhere. Puddles that mirror buildings need SSR, which in Bevy 0.20 requires depth +
deferred prepasses. The exterior camera has no depth prepass (far-field.md pitfall 2). Whether the
deferred prepass honors the `VisibilityRange` dither is untested (it has a fragment stage, unlike the
depth-only prepass). Open question 4. The Solari path reflects by ray tracing anyway.

**Characters** (built with characters): per-character wetness uniform: rain exposure (occlusion map at
the head), immersion height from the water query API (WISHLIST §4), drying per hour. The character
material darkens below the waterline height and with exposure.

## Distant shadows

Before 2026-10-12 (raster): 4 cascades to 600 m in the app (300 m in `SkyRenderOptions`' default). Bevy treats everything past the last cascade as fully lit with a
hard cut. Terrain past it has no shadows at all, and objects "acquire" shadows as they cross 300 m
(analysis in "Near-term visual fixes").

**Design: four tiers, each the cheapest representation for its range** (UE's CSM → far → distance
field structure, with a heightfield horizon map instead of distance fields):

| Tier | Range | Casters → receivers | Mechanism |
|---|---|---|---|
| 1 CSM | 0 → `s_max` (**250 m** since shadows F3; was ≥ `d1 + w` + margin = 504 m) | everything near → everything | Bevy cascades (4), retuned |
| 2 Far sun visibility | `0.8·s_max` → horizon | terrain (+ later far-field proxies) → terrain, proxies, impostors, grass, fog, clouds' ground bounce | **shadow-height map** (SHM) + the far-shadow correction in our materials |
| 3 Cloud shadows | 16 km around the camera | clouds → everything | cloud shadow map → `DirectionalLightTexture`, fog |
| 4 Terrain sky visibility (optional) | whole world, static | terrain → ambient on terrain/objects | baked horizon-based ambient occlusion per terrain tile (`terrain.skyvis` bake step, 8 directions) |

**Why `s_max` ≥ `d1 + w`.** Every L0 object (individual placements, out to the far-field handoff `d1`
plus its band) is then always inside the CSM. No object ever crosses a shadow boundary. Only terrain
and proxies do, and tier 2 covers them with a crossfade.

**Shadow-height map** (Max 1988, Sloan and Cohen 2000, reduced to one sun direction):
- 2 levels, toroidal and aligned like the height clipmap: 1024² at 2 m (2 km) and 1024² at 8 m (8 km);
  optional 1024² at 32 m. RG16F: `h_s` (height of the shadow boundary above that XZ) and `d_occ`
  (horizontal distance to the occluder that sets it).
- Compute: for each texel, march toward the sun in XZ over the height clipmap (finer levels near,
  coarser far, 48 exponentially spaced steps to 4 km horizontal),
  `h_s = max_i (h(x + s_i·L̂_xz) − s_i·tan(elevation))`.
- Receiver test: `vis = smoothstep(h_s − w, h_s + w, P.y)`, with penumbra half-width
  `w = d_occ·tan(sun angular radius + softening)`. One fetch per receiver, for terrain, objects,
  grass, fog and clouds alike.
- Update: strips when the clipmap scrolls; a full refresh spread over 8 frames whenever the sun moves
  > 0.05° (at Morrowind's time scale 30 the sun moves ~0.125°/s, so a refresh every ~0.4 s). Below 1°
  sun elevation, fade to unshadowed sun, since the sun itself is fading there.
- Phase 2: far-field proxies cast. A top-down orthographic pass rasterizes L1/L2 proxy heights
  (max blend) into a caster-height texture that the march reads with the terrain. Big structures
  (Vivec cantons, Ghostfence, strongholds) then cast far shadows.

**Far-shadow correction** (Bevy has no far-shadow hook, and past the last cascade it returns 1.0): in
**our** materials (terrain, proxy, impostor, `WorldMaterial`, grass), after `apply_pbr_lighting`:

```
f      = smoothstep(0.8·s_max, s_max, view_depth)      // 0 inside the CSM, 1 beyond
s_csm  = view_depth < s_max ? fetch_directional_shadow(sun, …) : 1.0   // only evaluated in the band
s_far  = shm_visibility(P)                              // tier 2
sun    = directional_light(sun, &lighting_input, true)  // Bevy's own function, unshadowed
color += (mix(s_csm, s_far, f) − s_csm) · sun          // = replace the sun's shadow term
```

This is exact for the shadowing light, because Bevy's sun contribution there is `s_csm · sun`
(cloud texture and atmosphere transmittance are inside `directional_light`). It costs a few ALU
outside the band and one extra light evaluation inside it. It needs the `LightingInput` that
`apply_pbr_lighting` builds internally: `sf_far_shadow.wesl` rebuilds it in ~25 lines mirroring Bevy
0.20 (named scaffold `FAR_SHADOW_LIGHTING_INPUT`, re-checked on every Bevy upgrade). The removal path is
an upstream Bevy hook (a directional-light "far shadow" texture or a cascade end fade; candidate
contribution, like the Solari transmittance patch in sky-lighting.md).

**As built (2026-10-12), replacing the correction above.** Bevy 0.20 already has the hook:
`PbrInput::directional_shadow_factor` multiplies every shadowed directional light's cascade term
(`pbr_functions.wesl`), and past the last cascade that term is 1.0, so the factor *is* the shadow there.
No `FAR_SHADOW_LIGHTING_INPUT` scaffold and no Bevy override. A multiplier can't fade the cascades *out*,
so the cascade end is a **dithered transition** (UE's dithered CSM transitions; Bevy's own
visibility-range crossfade): over `0.85·s_max → s_max` a growing fraction of pixels (ordered 4×4 dither)
takes the far term, and for those our materials move the lighting position just past `s_max` along the
view axis, so Bevy's cascade lookup returns 1.0 (directional lighting doesn't depend on the position;
the material restores it before fog). Object shadows fade out instead of ending on a line at 504 m.
Applies wherever a pixel past `0.85·s_max` can be: terrain, proxies, impostors (L0 objects end at 440 m).
- Terrain (`sf_terrain_bevy/src/shaders/far_shadow.wesl`): per-pixel horizon march over the height
  clipmap (14 steps to 5 km, stops above the highest resident height, skipped where `N·L ≤ 0`) out to
  700 m view distance, then the SHM (one fetch), dithered over 560-700 m.
- SHM (`sf_terrain::shadow::far_shadow_map`, `sf_terrain_bevy::FarShadowMap`): CPU, on the async pool,
  384² × 24 m (9.2 km) around the camera, `Rg16Float` (h_s − base, d_occ), from the resident heights
  resampled at 24 m; refreshed when the light turns 0.25°, the camera moves 1 km, or every 8 s. One
  level instead of the 2 planned: the near range is the march's. Unoccluded texels are floored to
  ground − 2 texels so bilinear filtering stays sane.
- Proxies become `ProxyMaterial = ExtendedMaterial<StandardMaterial, FarShadowExt>` (`proxy.wesl`,
  forward); impostors bind the map too (`far_shadow_map.wesl`). The app copies `FarShadowMap` into
  `sf_lod_bevy::FarShadowInput` (no crate dependency between the two).
- Cost (release, 1440p, AC, RTX 4060 Laptop, 2 × 12 s runs each, frame median; sunset 18:30):
  CSM 504 m / 2048² vs the old 600 m / 2048²: **−0.1 to −0.3 ms**; 4096² was +1.4 ms and is dropped.
  Far term (all materials): Red Mountain view ≈ **+0.0-0.4 ms**, north vista over the Foyada
  (`--camera "0,+250,0,300,0,-2000"`) ≈ **+0.5-0.6 ms** (main opaque pass +0.6-0.8 ms). Net vs the
  pre-session state: +0.0 / +0.4 ms. Over the High budget (0.15 + 0.08) in vistas; open: profile
  the split (terrain march vs map fetches vs proxies) and cut per preset (map-only terrain on Low/Medium).

**Not chosen:** more cascades (4 is Bevy's cap, and far cascades re-render casters with a cost that
grows with distance); distance-field shadows (no mesh SDFs or global SDF in Bevy; a project of its own,
revisit for M7 if proxies need sharper far shadows); virtual shadow maps (not in Bevy); static baked
shadow maps (the sun moves).

**Solari path:** ray-traced shadows have no range limit. Tiers 2 and 4 are off. Tier 3 applies if
Solari honors the light texture.

### Shadow bias and contact shadows (shadows F1 + F2, as built 2026-10-19)

Research: `docs/research/shadows/README.md` (fixes F1, F2). Rule 8: all Bevy 0.20, no new code paths.

- **Contact shadows (F1):** Bevy's `ContactShadows` on the exterior camera (screen-space march of the depth
  prepass toward the light, `bevy_pbr` `calculate_contact_shadow`) plus `contact_shadows_enabled` on the sun
  (`sf_sky_bevy::SunShadow::contact_shadows`) and on the budgeted shadowed point lights only
  (`PointShadowBudget::contact_shadows`; never the fade twins: one march per lit pixel per light). Bevy multiplies
  it into the shadow-map term, so it only darkens, and only where the map says lit. Presets: Low off; Medium
  8 steps; High/Ultra 16 steps; length 0.5 m, thickness 0.1 m. The moon gets none (it has no shadow map and
  would add a second march all day). A/B `--off contactshadows`; live: BRP `DirectionalLight.contact_shadows_enabled`.
- **The prepass it needs:** `ContactShadows` requires `DepthPrepass`; ours adds `MotionVectorPrepass` with it,
  because a depth-only prepass ignores the `VisibilityRange` dither (holes at LOD crossfades, `exterior.rs`
  note). So Medium and up now run the depth + MV prepass (measured 0.4 ms earlier); Low keeps none.
- **Bias (F2):** sun normal bias 1.8 → **0.8 texels** (depth bias 0.02 m kept), `exterior.rs` `SUN_NORMAL_BIAS`;
  per cascade the lift falls from 2.5 / 9 / 34 / 130 cm to 1.1 / 4 / 15 / 56 cm. No acne seen at 15:00 in Seyda
  Neen or Balmora; grazing sun (07:00, the 6a outline) is the owner check SH-7. `--off shadowtune` restores
  Bevy's 1.8 with the old cascades. Point lights keep Bevy's 0.08 m / 0.6 texel (already in range).
- **Filtering (F2):** `ShadowFilteringMethod` per preset on the camera: Low `Hardware2x2`, Medium and up
  `Gaussian`. `Temporal` needs TAA (raster runs none); revisit with F5 (PCSS).
- Seen 2026-10-19 (1080p, 15:00): differences are small at a high sun; cascade 0 already grounds feet and pots.
  Contact shadows mainly darken creases (column bases, wall feet) and the 30-40 m NPC contact. Expected larger
  at low sun and in cascades 2-3; owner sign-off SH-7.

### Sun cascades and the far static map (shadows F3, step a as built 2026-10-19)

Research: `docs/research/shadows/README.md` (F3, owner decision 2026-10-19: drop the 504 m rule). **Supersedes
"Why `s_max` ≥ `d1 + w`"** and A1's range: the CSM no longer has to contain every L0 object.

- **Splits:** `sf_sky_bevy::CascadeSplits`, the practical split (PSSM, GPU Gems 3 ch. 10): each bound is
  `λ·n·(f/n)^(i/N) + (1−λ)·(n + (f−n)·i/N)` with a 1 m split near (the 0.1 m camera near would give a 0.7 m first
  cascade). It builds a plain Bevy `CascadeShadowConfig` (`SkyRenderOptions::cascades` is now the built component),
  so Bevy 0.20 still does the rest: bounding-sphere fit per slice + snapping to whole texels (`bevy_light`
  `cascade.rs` `calculate_cascade`: stable cascades, no shimmer as the camera moves or turns) and the 20 % overlap
  blend between cascades. Rule 8: Bevy's builder can only do "first bound + geometric", so the split is ours (12 lines).
- **Presets** (`exterior.rs` `Quality::sun_cascades`): Low 3 cascades to 200 m (λ 0.85); Medium/High/Ultra 4 to
  250 m (λ 0.9). Maps stay 2048² (4096² was +1.4 ms).
- **Texels** (70° vertical FOV, 16:9, 2048²; `sf_sky_bevy` test `practical_split_cascades_and_texel_sizes`, Bevy's
  sizing): High bounds 9.9 / 26.8 / 75.4 / 250 m, texels **1.4 / 3.8 / 10.5 / 35 cm**; A1 (10 / 36.9 / 136 / 504 m):
  1.4 / 5.2 / 19 / 70 cm. Cascade 3 is 2× sharper, cascades 1-2 1.4-1.8×; with the 0.8-texel normal bias the
  lift in the last cascade falls from 56 to 28 cm.
- **Cost** (dev build, 1440p, `exterior_seyda_neen` bench 2 × 12 s each, RTX 4060 Laptop): Bevy-timed GPU
  17.4 / 16.2 ms vs A1 16.8 / 16.4 ms (noise, ≈ 0); frame median 30.0 / 29.4 ms vs 35.0 / 33.3 ms (the dev build is
  CPU-bound: fewer casters queued in the last cascade). VRAM unchanged (4 × 2048² × 32 bit = 64 MB; Low 48 MB).
- **A/B:** `--off shadowsplits` = the A1 504 m layout (F2 bias kept); `--off shadowtune` = the 600 m one before it.
- **Gap until step b:** L0 objects at 250-440 m get no object shadows: their shadows on terrain fade with the
  dithered cascade end (212-250 m, unchanged code: it reads `s_max` from the light), and object *receivers* (NIF
  statics' materials have no far term) end on Bevy's hard cut at 250 m. The terrain march (to 700 m) and the SHM
  are unchanged.

**Step b (CPU core built 2026-10-19 shf3b; render pass + receivers not built yet): cached far static shadow map (FSM).**
- *Built (shf3b):* `sf_sky::far_shadow` (Bevy-free): `FarShadowSettings` (4096², reach ±2048 m = 1 m texels, 4 km
  depth, sun 0.03°, recentre 1/16 reach = 128 m, 4 strips), `FarShadowView::fit` (sun-aligned ortho, fixed up hint,
  centre snapped to whole texels in the light plane, reverse Z like Bevy's shadow views), `FarShadowCache::update`
  (refresh on sun turn / recentre / `invalidate()` on far-cell arrival, one strip per frame into the *back* map, then
  swap). Two maps, not one: strips drawn into the sampled map would mix two views, so VRAM is **2 × 64 MB = 128 MB**
  at 4096² (or 1 × 64 MB with a whole-map refresh in one frame and no strips). Tests: `far_shadow_*` in `sf_sky`.
- *Bevy 0.20 render hooks found (for the next slice):* cameras run a schedule (`CameraRenderGraph(ScheduleLabel)`,
  `bevy_core_pipeline::schedule`, `Core3dSystems`), so the FSM camera gets its own schedule with one system; Bevy's
  own shadow queue (`bevy_pbr` `render/light.rs` `queue_shadows`, retained bins with `DirtySpecializations`, phase
  `ViewBinnedRenderPhases<Shadow>`, drawn by `per_view_shadow_pass` over `ViewLightEntities`, target
  `ShadowView::depth_attachment`) is the template for the binned depth-only phase. Step 1 check: the Balmora SH-8 vista
  does have sun shadows (backlit at 15:00, sun 48°; identical with `--off shadowsplits`): nothing broken.
- *Bevy first:* Bevy 0.20 has no shadow caching (every shadow view re-renders every frame, maps from the
  `TextureCache`) and no API for an extra or orthographic shadow view. A second shadow-casting `DirectionalLight`
  with zero illuminance would get cascades, but they are fitted to the camera frustum (re-rendered every frame,
  re-fit on every turn), share `DirectionalLightShadowMap`'s size (2048²: ~2.8 m texels over 2 km) and add a
  cascade lookup to every material. Rejected. *Crates:* none for Bevy 0.20. *Ours:* a plain `Camera` (no `Camera3d`,
  so no material queues into the main phases) with an orthographic sun-aligned projection, its own
  `CameraRenderGraph` sub-graph and a custom binned depth-only phase (Bevy's `custom_render_phase` example pattern:
  `SpecializedMeshPipeline`, `SetMeshBindGroup` + `DrawMesh`, GPU preprocessing per view), queuing only
  `FarShadowCaster` entities (static L0 placements + L1 proxies; masked foliage later). Bevy then gives it
  visibility, frustum culling and batching; caching = the camera is `is_active` only on refresh frames, and its
  depth is copied into a persistent 4096² `Depth32` image (64 MB).
- *Refresh:* sun turns > 0.02-0.05°, camera moves 1/16 of the map's reach, or a far cell streams in; texel-snapped
  ortho centre (stable); spread over N frames by drawing one of N horizontal strips per frame (scissor), so the
  amortised cost stays ≈ 0.05-0.2 ms.
- *Which far mechanism wins where:* 0 → 0.85·s_max: CSM only. 0.85·s_max → s_max: dithered CSM → far term (as
  now). Past s_max: `far = min(FSM_objects, terrain)`, where the terrain term is the horizon march to 700 m then the
  SHM (unchanged); the FSM holds **objects only** (no terrain casters: the march/SHM are sharper and cheaper for
  terrain and reach 9.2 km), fades out at its border (~2 km, 4096² at ~1 m texels). Receivers: terrain, proxies,
  impostors through their existing `directional_shadow_factor` hook, and NIF statics (needs a far hook in their
  material extension: coordinate with whoever owns `FroxelFogExt`/`mw_assets`).
- *Verify:* SH-8 (no object-shadow pop at 250 m), SH-1/SH-6 re-run, SH-11; GPU A/B per refresh frame and amortised.

## Near-term visual fixes (before M6)

### Bug A: distant objects have no shadows, then acquire them up close

**Cause** (verified in source):
1. `SkyRenderOptions::cascades` = 4 cascades, `first_cascade_far_bound` 12 m, `maximum_distance`
   **300 m** (`sf_sky_bevy/src/lib.rs`).
2. Bevy 0.20 `fetch_directional_shadow`: when the fragment's view depth is past the last cascade's
   far bound it returns **1.0**, which is lit, with no fade (`bevy_pbr/src/render/shadows.wesl`).
   So everything past 300 m view depth is unshadowed, and the boundary is a hard line that moves with
   the camera.
3. The far-field handoff is `d1` = 400 m (far-field.md). L0 objects live from 0 to 440 m, so between
   300 and 440 m they draw **without** shadows and gain them as they cross 300 m. L1 proxies are
   configured to cast "into far cascades" (`FarFieldSettings::l1_shadows`), but no cascade reaches
   400 m+, so proxies never cast or receive.
4. Terrain past 300 m has no sun shadows at all, so the hills look flat (`shots/farfield/*`).

**Fix, in order** (each step verified by screenshots before the next one starts):

- **A1. Retune the CSM** (config only). This is in `sf_sky_bevy`, owned by the dlss session, so
  coordinate or apply the change after it finishes. `maximum_distance = d1·1.1 + margin` (default
  500 m), derived in the app from `FarFieldSettings` so the two can't drift, `first_cascade_far_bound`
  10 m, `overlap_proportion` 0.2, `DirectionalLightShadowMap` 4096 on high and 2048 on low. Keep
  `l1_shadows` on: proxies now cast in the 400-500 m band. They duplicate the L0 shadow there, and
  shadows combine by `min`, so nothing is visible.
  Verify: `perf suite` balmora 1440p, A/B on the shadow pass GPU time (expect +0.3-0.8 ms). A
  screenshot sequence flying from 700 m to 100 m at a Balmora building: shadow present at every
  distance below 440 m. `balmora_a` re-shot.
- **A2. Shadow-height map** in `sf_atmos_bevy` (or first inside `sf_terrain_bevy`, moved later),
  computed from the height clipmap (above). It needs only the terrain clipmap and the sun direction.
  Verify: CPU test against `sf_atmos`'s reference march on a synthetic ridge, and a GPU readback test.
- **A3. Far-shadow correction** in the terrain material first (it already calls
  `apply_pbr_lighting`), then impostors, then proxies (proxies become an `ExtendedMaterial`, which
  far-field.md's phase-2 far textures need anyway).
  Verify: `redmtn_view` and `summit_vivec` re-shot at 7:00 and 17:30 (long ridge shadows), and a
  fly-through with no shadow line crossing the terrain near 400-500 m. GPU cost of the band: < 0.2 ms.
- **A4.** Cloud shadows wait for the cloud pass (M6).
- **Status (2026-10-12):** A1 done with 2048² (4096² measured +1.4 ms, over budget), range derived in
  the app (`exterior.rs` `sun_cascades`: `(handoff(1) + band(1))·1.1 + 20` = 504 m, first bound 10 m,
  overlap 0.2). A2 + A3 done for terrain, proxies and impostors, through Bevy's
  `directional_shadow_factor` and a dithered cascade end (see "As built" in "Distant shadows").
  The app ran the CSM at 600 m before this (not the 300 m this spec assumed). A/B:
  `--off farshadows` (far term off everywhere), `--off shadowtune` (old 600 m / 20 m cascades).

### Bug B: some objects pop in close (the Ghostfence)

**Cause** (verified in the bake store and code):
1. The Ghostfence wall segments `meshes/x/ex_gg_fence_s_01..04.nif` and `ex_gg_fence_s_h_01.nif` are
   **not cooked**: their `mesh.cook` product is `Skipped { reason: animated node "Tri Object03" }`
   (the energy field's material is animated). The pylons and plain fence pieces (`ex_gg_pylon_*`,
   `ex_gg_fence_0*`) are cooked. 736 of 6,619 models are skipped this way (bake-adapter.md), covering
   ~6% of placements; the viewer's "112 uncooked" counts placements spawned through that path so far.
2. `lod.policy`/`lod.l1` read `mesh.cook` and drop `Skipped` models (`sf_lod/src/bake.rs`:
   `Cooked::Skipped { .. } => Ok(None)`). So uncooked models have **no far-field representation**.
3. With a far field, every placement spawns at R1 (520-648 m) and none in R2. So an uncooked object
   exists only while its cell is in R1.
4. The runtime NIF spawner (`apps/bevyrrowind/src/exterior.rs`, `NifSpawner`, also `LightSpawner` for
   every light placement) **ignores `SpawnCx::max_distance` and `end_band`**: no `VisibilityRange`, so
   no dither-in. And the NIF loads through the `AssetServer` only when spawned, so it appears some
   frames **after** its cell's group is revealed.
   Net effect: big animated statics (the Ghostfence) are invisible past ~520-650 m, then pop in at full
   opacity, late. Lamp models (light placements) also never fade and double-draw over their merged
   L1 copy until the cell leaves R1.

**Fix, in order:**
- **B1. Ranges on NIF-spawned meshes** (`mw_assets` + the app): `NifSpawner`/`LightSpawner` put
  `NifRange { end: cx.max_distance, band: cx.end_band }` on the `WorldAssetRoot`. A system on scene
  spawn inserts `VisibilityRange { start: 0..0, end: e..e + band }` on every `Mesh3d` descendant, with
  `e = end + |node origin − placement origin|`. The dither is measured from each node's origin, and
  the extra offset guarantees no node fades before the proxy is fully drawn (the lead-band invariant,
  far-field.md). Verify: `--far-tint` at 380-460 m from a lamp shows a clean handoff, and the dither
  appears on NIF meshes.
- **B2. Rest-pose cook for far use** (`sf_model` + `mw_bake`): a new product `mesh.rest` for models
  `mesh.cook` skips **only** for animated nodes, particles, billboards or switches (never skinned). It
  is the model at t = 0: material/UV animation is irrelevant to geometry, transform keyframes at
  frame 0, switch → first child, billboards as static quads, particle systems dropped. `lod.policy`
  and `lod.l1` use `mesh.rest` when `mesh.cook` is `Skipped`. Bump the far rules version. Blended
  parts still leave L1 (phase-1 rule), so the energy field itself stays out of the far field until
  proxies support emissive/additive parts (open question 7). Its pylons and frame appear.
  Verify: the bake summary prints the rest-cooked count, `--far-tint` at Red Mountain shows the
  Ghostfence green (L1), and flying at it from 1.5 km shows no pop.
- **B3. Preload uncooked models before reveal** (`sf_world_bevy`): a `PlacementSpawner::prepare(model)
  -> Option<UntypedHandle>` hook that the assets stage calls and waits on, like cooked models. The
  group then reveals with its NIFs loaded. Verify: a fly-through toward Ghostgate shows no late
  appearance after the cell group reveals.
- **B4 (M6/M7). Split cook**: cook the static nodes of animated models with a LOD chain, and keep only
  the dynamic nodes on the NIF path. This is bake-adapter.md's open question "gameplay keyframes block
  cooking" generalized, and it makes the Ghostfence a normal cooked static near the camera. Material UV
  animation becomes a material parameter (`nif-runtime.md` material controllers) instead of blocking
  the cook.

The Ghostfence's white energy field (the farfield-exterior session's "look bug") is a separate
material issue in the NIF material mapping (additive/emissive blend), not part of the pop.

**Status (nifpop session, 2026-10-12): B1-B3 built, B4 planned.**
- **B1 built** as a framework component, not an `mw_assets` one: `sf_world_bevy::SubtreeRange { end, band }`
  (`SubtreeRange::of(cx)`) on the scene root, and `apply_subtree_ranges` (PostUpdate, before
  `CalculateBounds`) gives every new `Mesh3d` below it `VisibilityRange` with the fade `[e, e + band]`,
  `e = end + |node origin − root origin|` (scaled). The app's `NifSpawner` and `LightSpawner` insert it.
  Unit test `subtree_ranges_reach_late_meshes`.
- **B2 built**: `sf_model::cook_rest` + step `mesh.rest` (`MeshRestStep`, registered next to
  `mesh.cook` in `mw_bake::model_baker`). It cooks with the `animated` flags ignored (rest transforms as
  imported; particle nodes carry no mesh, inactive switch children are hidden by the importer); skinned
  stays `Skipped`. `sf_lod`'s `cooked()` reads `mesh.rest` only when `mesh.cook` is `Skipped`, so it is
  baked lazily for exterior far-eligible models only. Versions: `lod.policy` 3, `lod.impostor` 3,
  `lod.l1-3` 4. Bake: **21 models rest-cooked** (lamps and fires: `light_tikitorch00` 1,457 placements,
  `light_torch10`, `light_dae_brazier00`, candles, lanterns, `furn_mist256`, lava sparks; 4,037
  placements over all cells), 0 skinned among them; the far rebuild took 20.5 s, the whole bake 71 s
  with models cached.
- **B3 built**: `PlacementSpawner::prepare(&AssetServer, placement, model) -> Option<UntypedHandle>`.
  Once a load's models resolve, the assets stage asks each placement's spawner
  (`PlacementSpawners::spawner_for`) and waits until the handles' recursive dependencies have loaded or
  failed. `NifSpawner`/`LightSpawner` return the NIF's `WorldAsset`. `StreamStats::prepared` counts them
  (Ghostgate start: 59 NIFs preloaded, 148 uncooked placements, settled in 7.6 s).
- **Ghostfence finding:** the wall segments `ex_gg_fence_s_*` are **activators** (`ACTI`, class
  `Custom(2)`), so they are not far-eligible at all, and every part of them is the alpha-blended field
  (three 0.5-opacity planes, UV-scrolled, emissive 0.3). B2 therefore gives them no proxy (blended
  parts leave L1); B1 keeps their ring range (no handoff, no fade); B3 removes the late appearance.
  Seeing the field from 2 km needs **B5**: blended/emissive L1 submeshes (open question 7) *and* far
  eligibility for scriptless activators (adapter: map an `ACTI` without a script to a far-eligible
  class, or a `FarSource` override). Not built: making them eligible now would fade them out at the
  handoff with nothing replacing them.
- **Look (GF-2), investigated, not changed:** the material maps faithfully (texture
  `tx_gg_fence_01/02`, `Blend` non-additive, alpha 0.5, emissive 0.3 through the base texture). Three
  stacked 0.5 layers give 87.5 % coverage of a pale texture lit by an unclamped sun, which reads as an
  opaque white wall. Candidate differences from the fixed-function original, to check with data before
  changing the shared mapping: (a) with `AmbientDiffuse` vertex colors the fixed-function diffuse alpha
  comes from the vertex color, while we multiply material alpha × vertex alpha; (b) NIF lighting is
  clamped to 1 (texture × min(1, emissive + ambient + diffuse)), Bevy's PBR lighting isn't; (c) the
  environment-map specular on large blended planes. Visual check GF-2 carries the A/B.
- **B4 plan (split cook, M6/M7):** the importer already marks animated nodes per node. (1) `sf_model`
  cooks the static subset (nodes not animated and with no animated ancestor) into the normal LOD chain
  plus a *dynamic remainder* model (the animated subtrees with their parent chain) written as a second
  product, `mesh.dynamic`; `Cooked::Static` gains an optional `dynamic` reference. (2) Material UV/color
  controllers stop blocking: the cooked material carries the controller curve as a material parameter
  (`nif-runtime.md` material controllers), so UV scrolling parts like the Ghostfence field cook
  statically. (3) The runtime spawns the cooked chain through `StaticSpawner` and the remainder through
  the content pack's hook, both under the placement root. (4) Gameplay keyframes (doors, 307 models)
  stay on the NIF path until animation graphs drive cooked nodes. Gate: the bake summary's skipped
  count drops from 736 by the UV/material/particle-only models (~300), bake time unchanged.

### Aerial perspective and haze (now)

Bevy's atmosphere already fogs every opaque pixel with the aerial-view LUT. The shots look clear
because the medium is a clean Earth sky and Vvardenfell views are ≤ 10 km.
- **N1.** Drive `sf_sky::Weather::haze` (already wired: Mie density multiplier, medium rebuilt at
  ≥ 0.02 change) from a temporary per-time-of-day curve in the app (`--haze`) until `sf_weather`
  exists: 2-3 on clear days, 4-8 on Bitter Coast mornings. Verify: `summit_vivec` and `redmtn_view` at
  several values. The owner picks the default.
- **N2.** Add an absorbing dust term to `AtmosphereModel` (a brown-grey Mie-like term with absorption,
  low scale height) so ash regions and storms tint the haze. The weather scales it.
  sky-lighting.md's `AtmosphereModel.terms` already supports extra terms.
- **As built (M6 F1, m6-f1, 2026-10-13).** `Weather` gained `dust` and `dust_tint`; `MediumTerm.weather_scaled`
  became `weather: WeatherScale { None, Haze, Dust }`; `Weather::medium()` → `MediumScales` feeds both the GPU medium
  and the CPU transmittance. `AtmosphereModel::earth()` has a 4th term, dust (per unit: extinction ~9.5e-5 /m at the
  ground, scale height 2 km, single-scattering albedo 0.87/0.81/0.71, Mie g 0.7); `build_medium` drops empty terms.
  Rebuilds: a step (haze 0.05, dust 0.02, tint 0.02) and at most every `SkyRenderOptions::medium_min_interval`
  (0.25 s real time); `MediumStats` counts them. Cost: GPU unchanged (same LUT sizes and passes; terms are baked into
  the LUTs); a rebuild is a ~2.8 ms CPU LUT bake at 256² on the render thread (release, `medium_rebuild_cost`,
  shared machine) plus a 1 MB upload, so ≤ 4 short hitches per second while the weather moves, none when it is still.
  F2/WE3: lower the phase LUT resolution or bake off-thread if the hitch shows in time-lapses.
  **Calibration changed:** "2-3 on clear days" assumed Earth-scale views. Morrowind's far field is 1-5 km away, where
  haze 2-3 is invisible (shots/f1/redmtn_old vs clear); the app now uses haze 20 for clear (aerosol optical depth
  ~0.18, a typical clear continental day), 60 foggy, dust 4 for ash storms (weather-adapter.md "Haze and dust"). Near
  fog at game distances stays F2's analytic height fog, not more haze. Known issue: a purple/white seam on the sea
  horizon at noon with dust (sky-view LUT vs aerial LUT; visual check FG-1).
- **N3 (optional prototype, owner's eyes only).** God rays now with Bevy's own volumetrics:
  `VolumetricLight` on the sun, `VolumetricFog { jitter, step_count: 32 }` on the exterior camera, and
  one camera-following `FogVolume` (≈ 300×60×300 m, low density). Megalophobia found large boxes go
  grey and dim (above), so keep it small. Measure. Throw it away when the froxels land.

## Performance budgets (1440p; RTX 4060 Laptop / RTX 3070 desktop, GPU ms)

| Feature | Low | Medium | High | Ultra |
|---|---|---|---|---|
| CSM to `s_max` (delta vs today's 300 m) | 2048², 400 m: +0.2 / +0.15 | +0.3 / +0.2 | 4096², 500 m: +0.6 / +0.45 | +0.8 / +0.6 |
| Shadow-height map (amortized) | 0.05 | 0.05 | 0.08 / 0.06 | 0.1 |
| Far-shadow correction (materials) | 0.05 | 0.1 | 0.15 / 0.1 | 0.15 |
| Clouds + far fog (¼ res, temporal) | 1/16 res, 32 steps: 0.4 / 0.3 | 0.6 / 0.45 | 0.9 / 0.65 | ½ res every frame: 1.8 / 1.3 |
| Cloud shadow map + weather map | 0.03 | 0.03 | 0.05 | 0.05 |
| Froxel fog (+ apply) | analytic height fog only: 0.1 | 128×72×48: 0.4 / 0.3 | 160×90×64: 0.7 / 0.5 | 160×90×128: 1.1 / 0.8 |
| Precipitation (3 layers, splashes, occlusion camera amortized) | 0.2 | 0.3 | 0.4 / 0.3 | 0.5 |
| Surface state (update + hook ALU in world materials) | wetness only: 0.1 | 0.2 | 0.3 / 0.2 | 0.35 |
| Wind (vertex ALU in foliage/grass) | 0.05 | 0.1 | 0.15 / 0.1 | 0.2 |
| **Total weather + atmosphere** | **≈ 1.2** | **≈ 2.1** | **≈ 3.3 / 2.5** | **≈ 5.0 / 3.8** |

CPU: `sf_weather::evaluate` + wind + uploads ≤ 0.05 ms. The occlusion camera's extraction ≤ 0.1 ms
when it refreshes.

Context: Balmora at 1440p is 11.9 ms today (GPU 8.6 ms timed) against WISHLIST §0's 8.3 ms (120 fps,
3070 class). Weather makes that tension bigger. High is the target on the 3070 with DLSS Super
Resolution (dlss session). The cloud and froxel passes are resolution-independent, so DLSS helps the
rest of the frame. These are budgets, not measurements: each feature lands with an `sf_diag` counter
(`atmos/clouds/gpu_ms`, `atmos/froxels/gpu_ms`, …) and an A/B (`--off clouds|fog|precip|surface|wind`),
and over-budget features are cut per preset, not the frame.

## Verification

- **Unit tests (Bevy-free):** weather forecast determinism and save/restore (seed + overrides →
  same sequence); blending (channel lead/lag, discrete crossfades, border band); wind CPU/WGSL twin
  values and divergence-free gusts; noise tiling (seamless at every tile edge); shadow-height map
  against a brute-force ray test on synthetic terrain; surface accumulation and drying curves; the
  `wind.paint` pivots on a synthetic tree; `mesh.rest` on a model with a UV controller.
- **Screenshots** (`cargo dev view` with `--time`, `--weather <preset>`, `--day`, plus new
  `benchmarks/*.json` cameras; the owner reviews):
  1. **Bitter Coast rainstorm**: Seyda Neen docks, 14:00, thunderstorm. Clouds overcast with
     structure, rain streaks occluded under the docks' roofs, puddles on the paths, wet dark wood,
     splashes, lightning frame.
  2. **Ash storm at Red Mountain**: near Ghostgate looking up the mountain, 16:00, ash storm. Low
     visibility, brown haze, ash streaks blowing away from the summit, ash on up-facing rocks, the
     Ghostfence visible (bug B fixed).
  3. **Sunrise god rays in Balmora**: 6:15, clear/cloudy. Shafts between buildings (CSM) and from the
     western ridge (shadow-height map), red-lit cloud undersides.
  4. **Bitter Coast morning fog**: 6:30, foggy. Ground fog banks in the swamp, trees fading into it,
     fog continuing past the froxel range from a ridge view.
  Plus a flight sequence (60 m/s) for shadow and pop regressions and a weather transition time-lapse
  (clear → overcast → rain over 2 game hours).
- **Perf:** `perf suite` balmora/red_mountain/flight with each weather preset at 1440p. Gates are the
  budget table and WISHLIST §0. A/B per feature.

## Proposed changes to sky-lighting.md (owned by the dlss session; apply after it finishes)

1. **Shadows** paragraph: "4 cascades to `s_max` = far-field `d1·1.1` + margin (default 500 m), first
   bound 10 m; shadow map 4096 high / 2048 low; past the CSM, far shadows come from weather.md's
   shadow-height map". `SkyRenderOptions::cascades` default changes accordingly, or the app derives it
   from `FarFieldSettings`.
2. **Data model**: `Wind` moves to `sf_wind` (re-exported from `sf_sky`, so callers don't change).
   "M6 swaps its analytic gusts for a sampled field" now points to weather.md.
3. **Weather hook** (applied in code by F1, 2026-10-13): `Weather` gains `dust: f32` (absorbing aerosol term scale, N2). `haze` is driven by
   `sf_weather`.
4. **Interfaces**: the sun/moon entities get `DirectionalLightTexture` (cloud shadows) and, optionally,
   `VolumetricLight` (prototype N3). `SkyPlugin` must keep them when re-spawning lights. The light's
   `Transform` is set by `sf_atmos_bevy` (camera-centered).
5. **Non-goals**: "clouds, weather presets and blending, fog, precipitation, surface wetness → weather.md".
   Stars and moon phases (open question 4) stay with sky-lighting.
6. **Raster path**: note that the exterior camera has no depth prepass (far-field.md pitfall 2), which
   matters for SSR on wet surfaces (weather.md open question 4).

## Proposed changes to other specs

- **far-field.md**: `lod.policy`/`lod.l1` fall back to `mesh.rest` (B2). Proxy and impostor materials
  call the far-shadow correction and the surface hook (wetness only). Impostor atlas header gains wind
  stiffness. "Shadows: L1 casts (far cascades)" → "L1 casts in the CSM's last band; past the CSM,
  terrain shadows come from the shadow-height map".
- **streaming.md**: `PlacementSpawner::prepare` (B3). `MaterialMapper` maps to `WorldMaterial` (surface
  hook) at M6.
- **terrain.md**: `surface_state.wesl` moves to `sf_surface_bevy` (the hook signature stays). The
  terrain material adds the far-shadow correction. The new bake step `terrain.skyvis` (tier 4,
  optional). The clipmap is bound for the shadow-height compute.
- **bake.md / bake-adapter.md**: products `mesh.rest` and `wind.paint` (a per-LOD vertex stream appended
  at load).

## Open questions

1. **Megalophobia license** (owner): confirm MIT OR Apache-2.0 for `clouds.rs`, `cloud_deck.wgsl`,
   `cloud_density.wgsl`, `ground_fog.*`, `grass_wind.wgsl` before porting. **Resolved 2026-10-12**: the
   owner licensed the Megalophobia cloud files MIT OR Apache-2.0 (ARCHITECTURE §10); ported in `sf_atmos`.
2. Wind texture (Tsushima-style 3D advection) vs analytic field + sources: decide after grass and
   trees are seen in motion.
3. Red Mountain's ash plume as a hero cloud: a Nubis³-style voxel volume, or a weather-map coverage
   anchor? Start with the anchor.
4. SSR for puddles: test whether `DeferredPrepass` honors the `VisibilityRange` dither. If it does, the
   raster exterior can go deferred + SSR on High. Otherwise wet surfaces reflect only the sky until
   Bevy fixes the prepass.
5. Does Solari honor `DirectionalLightTexture`? If not, cloud shadows on the RT path need a patch or
   a pre-attenuated sun (like the transmittance workaround).
6. Froxel shadows on the Solari path (shadow maps off): a cheap fog-only CSM, or SHM + clouds only?
7. Emissive/additive parts in far proxies (Ghostfence field, lava, lit windows): an emissive L1 submesh
   with vertex-color emission, together with far-field phase-2 textures.
8. Scale of Morrowind clouds: Vvardenfell is compressed (Red Mountain is a few hundred meters), so
   cloud base heights must come from the adapter, not real-world defaults. Calibrate on screenshots.

From J9 (2026-10-12, industry comparison below):

9. **Temporal upscaler vs transparents without motion vectors (P1 when precipitation lands).** Rain
   streaks, splashes, particles and UV-scrolled surfaces (water, lava) have no motion vectors, so DLSS
   and TAA smear or erase them. Standard fixes: a per-pixel **reactive / bias mask** written by those
   draws (FSR 2/3 reactive and transparency-composition masks; DLSS "bias current color" mask; UE TSR
   "responsive AA" / translucency flags), or drawing them **after** the upscale at output resolution
   (UE "after DOF/after TSR" translucency, unverified per version). Bevy 0.20's DLSS node passes
   `bias: None // TODO` (`bevy_anti_alias/src/dlss/node.rs`; `dlss_wgpu` 6.0 accepts the texture), and
   Bevy's own TAA has no reactive input. Options: a small upstream patch that lets a camera supply the
   bias texture + a mask write in the precipitation/particle materials, or a post-upscale precipitation
   pass. Check with a rain screenshot at DLSS performance before choosing.

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`). Most of the
comparison is already in "Research summary" above (with sources); this table maps it to verdicts
and adds what that summary didn't cover.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Weather as one integrated system driven by a state | RDR2 (Bauer 2019), UE5 (Sky Atmosphere + Volumetric Clouds + height/volumetric fog driven together), Horizon (Nubis weather map), Bethesda (weather records with transitions: fog, colors, sounds, precipitation) | `WeatherFrame` from presets + climate state machine | **match** |
| Volumetric clouds | Nubis 2015/2017 deck model (Decima); Nubis Evolved 2022 / Nubis³ 2023 voxel clouds for fly-through and hero shapes; UE Volumetric Clouds; Frostbite (Hillaire 2016); RAGE (RDR2) | Nubis deck at ¼ res, temporal; voxel only for one hero volume (open question 3) | **match**; Nubis³ voxel clouds deliberately limited to hero use (cost; we don't fly through clouds) |
| Cloud shadows | Nubis 2D shadow map; UE cloud shadows on the light and in fog | 2D map → `DirectionalLightTexture` + fog | **match** |
| Near fog and god rays | froxel volumetric fog (Wronski 2014, Hillaire 2015, UE Volumetric Fog, Decima, RDR2) | froxel fog (ours, Bevy's per-box raymarch rejected) | **match** |
| Far fog / haze | UE exponential height fog (analytic), aerial perspective LUT | analytic height fog + Bevy aerial perspective | **match** |
| Precipitation | camera-relative layers + top-down occlusion depth (Tatarchuk 2006; RDR2, Watch Dogs, UE Niagara rain) | closed-form instanced streaks, occlusion camera | **match** |
| Wet surfaces, puddles, ripples | Lagarde 2012–13 (Remember Me), UE/Unity wetness material functions, RDR2 | `sf_surface` clipmap + hook (Lagarde model) | **match** |
| Snow/ash accumulation | up-facing blend + height blend (universal), deformation trails (RDR2, Horizon, Tsushima) | same, deformation via terrain layer 3 | **match** |
| Wind | one field, many consumers (Tsushima 2021); SpeedTree / Pivot Painter hierarchical tree wind | `sf_wind` field + baked pivots | **match** |
| Lightning | flash as a light + sky term, delayed thunder (universal) | Poisson schedule, light + cue | **match** |
| Distant shadows | UE far cascades + distance-field shadows, VSM; heightfield horizon maps | CSM + shadow-height map + horizon march | **match** (different representation, same tiering) |
| Reflections on wet surfaces | SSR + probes (UE, Unity), RT (RDR2 PC no; UE Lumen) | env map only on raster; SSR blocked (open question 4) | **gap already open** (question 4) |
| Transparents under temporal upscaling | reactive/transparency masks (FSR 2+, DLSS bias mask), TSR responsive AA, post-upscale translucency | none (Bevy DLSS passes no bias mask) | **GAP (P1 with precipitation)**: open question 9 |
| Rain on the camera lens | screen drops in RDR2, Far Cry, UE samples (common, optional) | out of scope until a camera FX spec | **differ** (deliberately deferred) |

**Sources** (checked 2026-10-12, beyond "Research summary"):
- AMD FidelityFX Super Resolution 2, reactive mask and transparency & composition mask: https://gpuopen.com/manuals/fidelityfx_sdk/techniques/super-resolution-temporal/ (page not re-read this session; widely documented, mark unverified on detail)
- `dlss_wgpu` 6.0.0 `DlssSuperResolutionRenderParameters::bias` ("Optional per-pixel bias to make DLSS more reactive") and Bevy 0.20 `bevy_anti_alias/src/dlss/node.rs` (`bias: None // TODO`), read in the local registry.
- UE TSR "responsive AA" material flag: unverified (from memory of UE 5 docs).

## Distant weather (WE4)

Research and design, we4r, 2026-10-19 (docs only, nothing built). Owner want: see weather in other regions
from afar (a storm over the Ashlands seen from Balmora, rain shafts under distant clouds). Today `evaluate`
runs once, at the camera's climates, and the clouds threshold one fixed noise map by **uniform** scalars
("Clouds as built", deviation "no compute pass writes one"). WE4 brings the written weather map back.

### How shipped games do it

- **Horizon Zero Dawn** (Schneider and Vos 2015, [pdf](https://advances.realtimerendering.com/s2015/The%20Real-time%20Volumetric%20Cloudscapes%20of%20Horizon%20-%20Zero%20Dawn%20-%20ARTR.pdf),
  slides 38-45): the weather system writes a 2-D map "over our section of world map": R coverage, G
  precipitation, B type. Precipitation turns a texel into cumulonimbus at 70 % coverage and darkens it
  (absorption up), and **the same signal starts the rain effects** at the player: local rain is the map at
  the player, not a separate state. Clouds draw to 35 km; past 15 km the map is biased toward cumulus so
  the horizon stays interesting. Cutscenes swap in painted maps.
- **Horizon Forbidden West** (Schneider, *Nubis Evolved*, SIGGRAPH 2022,
  [pdf](https://advances.realtimerendering.com/s2022/SIGGRAPH2022-Advances-NubisEvolved-NoVideos.pdf);
  *The Real-Time Volumetric Superstorms of Horizon Forbidden West*, GDC 2022,
  [vault](https://www.gdcvault.com/play/1027688/The-Real-Time-Volumetric-Superstorms)): 2-D "Nubis data
  fields" over 16 × 16 km tiles, decompressed vertically by a profile model. **Lightning**: generate a
  position, play a flash animation, add an in-cloud glow
  `pow(1 − d/radius, 12) × height gradient × (1 − saturate(5 × density))` to the light energy, and a
  **stable mask** so the temporal upscaler doesn't smear or reject the flash.
- **UE5 Volumetric Clouds** ([docs](https://dev.epicgames.com/documentation/unreal-engine/volumetric-cloud-material-in-unreal-engine)):
  a world-placed layout texture ("global pattern", scale in km, offset and rotation) with one cloud type per
  channel (A = nimbostratus, used by the storm mode with lightning), plus a mask texture that adds or removes
  cloud in a region. Spatial weather is texture placement, not per-texel state. Third-party UE weather
  (Ultra Dynamic Weather, [Fab](https://www.fab.com/listings/84fda27a-c79f-49c9-8458-82401fb37cfb)) uses
  **weather override volumes**: a local preset inside a volume, blended at its edge (our border band).
- **Microsoft Flight Simulator 2020/2024**: live weather is a gridded model (Meteoblue, METARs blended in)
  sampled periodically and interpolated into volumetric cloud layers, rain and wind
  ([guide](https://simtuts.com/guides/msfs-2024-live-weather-guide),
  [forum study](https://forums.flightsimulator.com/t/investigating-how-msfs-draws-clouds-for-weather/482387)).
  Community sources only; no Asobo talk found. Lesson: a coarse grid interpolated per sample is enough for
  clouds; sharp convective cells need a cell signal on top (ours: the storm-cell channel).
- **RDR2** (Bauer, SIGGRAPH 2019 Advances, [video](https://www.youtube.com/watch?v=IYxtqlD1-Ac)): as noted in
  "Research summary", only the outline is confirmed (one integrated system, storm and lightning timing). In
  play, regional weather (snow in the Grizzlies, storm cells with lightning over the plains) is visible from
  afar; the mechanism is unconfirmed, so we take nothing specific from it.
- **Far Cry 6** (Weick and Zhou, GDC 2022, [vault](https://gdcvault.com/play/1027725/Simulating-Tropical-Weather-in-Far)):
  arbitrary weather-state transitions in a day cycle, rain and lightning as particles; the abstract says
  nothing spatial. No Wildlands or Far Cry 5 weather talk found. Frostbite (Hillaire 2016) uses the same
  weather-texture model as Nubis.
- **Distant rain shafts**: no primary source found (searched 2026-10-19). Two practices exist: (a) extend the
  cloud medium below the base where the map's precipitation is high (a low-albedo column with fall streaks,
  in the same march), which UE users build in the cloud material; (b) billboard or cylinder **curtain
  impostors** under cells (older flight sims; flat when close). Rain as a participating medium is also
  argued in [this thesis](https://studenttheses.uu.nl/handle/20.500.12932/48068). We take (a), inside the
  far-fog march we already have.

**Common thread.** One 2-D map around the player carries coverage, type and precipitation; the near
effects (rain particles, wetness, sky darkening) read **the same map at the player**, so near and far can't
disagree. Lightning is a positioned event that lights the cloud from inside.

### Design

**1. Per-climate evaluation, not per texel.** `evaluate` is linear in the climate weights for every
continuous channel and sums discrete ones by weight (WE1 "Blending"). So the map never evaluates per texel:
evaluate each climate in range **once at weight 1** into a **climate table** (≤ 64 rows: the cloud scalars,
fog density scale, precipitation intensity and look, lightning rate), then blend rows per texel by the
border weights. Morrowind has ~9 Vvardenfell regions + 6 Solstheim + Mournhold: ≤ 16 evaluates ×
0.0006 ms (release) ≈ 0.01 ms per frame. Upload ≤ 4 KB.

**2. Climate raster.** The `weather.climate` bake (WE3's border grid) already stores per cell its climate,
the nearest other climate and the distance to that border. WE4 makes it the single source: a per-worldspace
raster of `(own, neighbor, distance)`, uploaded once (Morrowind: ~64 × 64 cells, 12 KB). Huge worldspaces
use a toroidal window into it, re-blitted when the map recenters.

**3. Weather map (GPU).** World-anchored, toroidal (like the cloud shadow map), recentered in 1/8-map
steps on the camera, addressed with the origin phase (`CloudsOrigin`, f64 on the CPU), so floating-origin
shifts move nothing.
- Size: 256² texels. Texel ≤ border band / 2, so the band spans ≥ 4 texels: real scale 128 m (32 km,
  covers a 16 km cloud reach), Morrowind 64 m (16 km: all of Vvardenfell from anywhere on it, Red
  Mountain to the sea). Past the map: clamp to the edge texels (the out-of-range rule).
- Textures: A `Rgba16Float` coverage, type, wetness, storm; B `Rgba16Float` base, thickness, density,
  erosion; C `Rgba8Unorm` cirrus, precipitation intensity, fog scale, precipitation look. ~1.3 MB.
- A **resolve compute pass** writes them from raster + table: per texel, `border_blend` weights (the
  `sf_weather` function, WGSL twin) over the two table rows. Runs when the table changed past an epsilon or
  the map recentered (at most every frame); 65 k threads.
- **Uniform fast path**: when every row in range is the same climate state (the common clear day), a
  flag keeps today's uniform scalars and skips the map fetches.

**4. Clouds read the map.** `weather_sample(p)` takes coverage/type/wetness/storm (and base/thickness)
from one bilinear fetch of A (and B) at `p.xz` instead of uniforms; `sf_atmos::weather_at` keeps its math
(the map sample replaces the scalars), so the CPU twin and gameplay "is this in cloud" still agree. The
shell becomes the **envelope** of the map's base/top (a neighbour's cumulonimbus is taller than our fair
deck); `h` is normalized by the local base/thickness and the step comes from the local thickness, so a thin
local deck doesn't pay for the distant tower. The **cloud shadow map** samples the same map (shadows match
the deck), and storm cells keep their own cell channel (MSFS's lesson). Adapter anchors (open question 3,
the Red Mountain plume) become a coverage boost written in the resolve. Clouds drift with the camera's
wind through **fixed** regional weather: a cell drifting out of a stormy region thins over the band (as
Morrowind's per-region weather implies; moving storm systems are owner question 2).

**5. Local = map at the camera.** The camera's `WeatherSite` weights come from the same raster sample and
`border_blend` the GPU uses, on the CPU (`WeatherMap::site_at(pos)`), so `CurrentWeather.frame` (sky
dimming, wind, near rain, cues, `visibility_m`) equals the map at the camera by construction. Crossing a
border is the band crossfade WE3 already plans; nothing pops because there is one code path. Test: the map
texel at the camera equals `evaluate(site)` for every continuous channel along a 2 m border walk.

**6. Precipitation shafts.** In `clouds_trace`, a short march (8-12 steps, quarter res, temporal like the
banks) over the ray's part **below the local cloud base**, out to the far-fog reach (6 km; all of
Vvardenfell), only when the map's per-frame precipitation maximum is above zero. Medium: map C
precipitation at the sample, sheared by the wind with height (slanted curtains), × the vertical profile
(full under the base; a `virga` fraction per preset fades it before the ground) × fall streaks (the detail
volume stretched vertically, scrolled at `PrecipKind.fall_speed`); extinction = intensity ×
`PrecipKind.extinction` (the same number `visibility_m` uses, so a shaft is as opaque as standing in it).
Lit by the key light × cloud shadow map (dark under the storm, bright at its sunlit edge) + sky fill; tint
by look (rain grey-blue, snow white, ash brown). **Hand-off**: weight `ramp(32 m, 128 m, t)`; inside it the
near particle boxes (to 64 m) and the camera's uniform precipitation extinction take over. At the camera
both describe the same intensity, so walking into a storm adds particles in front of a curtain that was
already there.

**7. Far fog by location.** The analytic height fog stays the camera's (closed form, per pixel). The
existing 16-step bank march adds the **difference** `(fog_scale(x) − fog_scale(camera))` × height-fog
density, clamped so the total stays ≥ 0: the Bitter Coast's fog reads from a dry ridge, and the term is
zero at the camera, so near fog is unchanged.

**8. Distant lightning (pure, replicated).** Strikes become **world-anchored** (closes WE1's "flash
position is camera-relative"). Per climate, the Poisson schedule runs at weight 1 with the authored
`per_hour` defined as the rate seen within a reference radius R (default 3 km real, scaled by the adapter),
so the density per km² is `per_hour / (π R²)` and a player inside the region sees the authored rate. Strike
`k` takes a cell from the climate's raster cells by `hash(seed, climate, k)`, jittered, kept if the map's
storm × coverage there is > 0.5 (4 tries, else dropped). Everything is a function of (seed, overrides,
clock, raster), so co-op peers see the same flash at the same place without messages. Render: up to 4
active flashes in a uniform; the trace adds the Nubis 2022 glow term; the resolve skips the history clamp
inside a flash's screen bound (the stable mask) so 0.1-0.3 s flashes neither smear nor vanish; a sky and
ambient bump falls off with distance; thunder delays by distance / 343 m/s and is culled past ~3 R.

**Morrowind adapter notes.** Regions are irregular cell sets: the adapter rasterizes `CELL` region names
(WE2's `ClimateSource`) per 117 m cell into the raster, with a distance transform for the border distance.
Cells without a region (most sea) take the **nearest region** by dilation (owner question 1), so the
coast's storm continues out to sea instead of stopping at a clear ocean. `ChangeWeather` / `ModRegion`
already act per climate, so a scripted storm in a neighbouring region shows from afar for free. Debug: BRP
`weather.force {region, weather}` and `--force-region <id>=<weather>` for the WE-3 check.

### Cost (estimates; GPU ms at 1440p, RTX 4060 Laptop, High)

| Part | Estimate | Note |
|---|---|---|
| CPU: climate table (≤ 16 evaluates) + upload | ≤ 0.02 | inside the 0.05 CPU gate |
| Resolve pass (256², on change) | ≤ 0.01 | inside "cloud shadow map + weather map" 0.05 |
| Trace: 1-2 map fetches per density sample | +0.03-0.08 | 0 on the uniform fast path |
| Trace: envelope shell toward a taller neighbour | +0.05-0.10 | views toward the storm only |
| Shadow map reads the map | < 0.01 | time-sliced |
| Shafts (8-12 steps, rain in view) | +0.03-0.06 | skipped with no precipitation in the map |
| Lightning glow (≤ 4 flashes) | ≤ 0.02 | only while a flash is active |
| **Total, storm in view** | **≈ +0.15-0.25** | **gate +0.15 on High; ≤ 0.02 uniform** |

Clouds on fair already sit at the 0.9 ms High line (S1 cost), so WE4 must be paid for: the fast path
covers uniform days; the levers are a per-ray map lookup on Low/Medium (sample the map at 3 points along
the in-layer segment and lerp), A-only fetches (B every 8 samples), and 8 shaft steps.

### Slices (1-2 steps each; one build agent each)

- **WE4a** (built, we4a 2026-10-19: "WE4a as built" below) `sf_weather::WeatherMap` (Bevy-free): climate raster type over WE3's border grid, per-climate
  table (`evaluate` at weight 1), `site_at(pos)` and `sample(pos)` (CPU twin of the resolve).
  *Verify:* tests: map at the camera = `evaluate(site)` (1e-5, continuous channels) on a 2 m border walk;
  same seed → same table; table cost ≤ 0.02 ms for 16 climates (release). **Needs WE3's border grid (we3b2).**
- **WE4b** `sf_atmos_bevy` weather map: textures, resolve pass, clouds + shadow map read it, envelope
  shell, uniform fast path, plume anchor hook. *Verify:* `clouds` example `--split clear,storm` (two
  climates side by side) screenshot from the clear side; GPU-timer A/B uniform vs split (gate above).
- **WE4c** App: region raster (dilated sea), `site_at` replaces the camera lookup, BRP/flag force per
  region, scene `we3-ridge`, `docs/visual-checks.md` **WE-3**. *Verify (WE-3):* from the ridge above
  Balmora at 14:00, own region Clear, forced Thunderstorm in the Ascadian Isles and Ashstorm in the
  Ashlands: a dark deck over each neighbour, clear sky overhead, no seam at the border band; walk across:
  no pop (screenshots at 400/200/100/0 m).
- **WE4d** Shafts + far fog by location. **Needs WE3 precipitation (we3b2)** for the near hand-off and
  `PrecipKind` looks. *Verify:* WE-3 shots show slanted curtains under the storm, ash brown in the Ashlands;
  the approach sequence shows no double rain at the hand-off; FG-2 Bitter Coast fog seen from a dry ridge;
  shafts ≤ 0.06 ms.
- **WE4e** World-anchored lightning + glow + stable mask. **Needs WE3 lightning/thunder (we3b2).**
  *Verify:* tests: two sites far apart get identical world strikes for one seed; strike rate inside a
  region within 4σ of `per_hour`; WE-3 at 23:00: flashes inside the distant deck, thunder delayed by
  distance.

WE4a-c need only WE3's border grid; d and e wait for we3b2's precipitation and lightning. When WE4b lands,
log the reversal of "Clouds as built"'s "no written weather map" deviation in ARCHITECTURE §10.

### Owner questions (WE4)

1. **Sea weather**: regionless cells (most sea) take the nearest region's weather (recommended), or a
   fixed Clear sea?
2. **Moving storms**: keep weather fixed to regions (Morrowind's model; clouds thin as they drift over a
   border; recommended), or add advected storm cells that cross borders (more realistic, no Morrowind
   basis, later)?
3. **Cost tier**: WE4 adds ≈ 0.15-0.25 ms when a storm is in view, with clouds already at the High line.
   Accept on High (trim elsewhere), or use the per-ray map lookup (cheaper, softer region edges) on High too?

### WE4 owner decisions (2026-10-19)

- **Regions first, moving storms later:** WE4 shows each region's own weather from afar, blended at borders; storm
  cells drifting across borders are a later slice on the same map.
- **Regionless sea cells take the nearest region's weather** (blended), not a default clear sky.
- **Cost:** full weather map on High and Ultra; Medium and Low use the cheaper per-ray lookup (coarser, no rain shafts).

### WE4a as built (we4a, 2026-10-19): `sf_weather::WeatherMap` (CPU side)

**Reuse check (rule 8).** Bevy has no spatial weather; the map is ~400 lines over `sf_weather`'s own `evaluate` and
`border_blend`, so it lives in that crate (module `map`, no new deps: `glam`, `smallvec`, `sf_atmos::fog`).

- **`ClimateRaster`** (world X, Z origin, square size, `width × height` of `Option<ClimateIx>`, band): the adapter's
  region raster. Unowned squares (sea) take the **nearest** owned square's climate (owner decision 1): Euclidean
  dilation by vector propagation (Danielsson 1980: a BFS that carries each square's source square and relaxes on a
  closer one; ties to the lower climate index, then the lower source), so two regions meet at sea on their Voronoi
  border and blend there like on land. `site_at(p)` = own square's climate + the nearest other climate among the squares
  within half the band (point-to-square distance), through `border_blend`: the same rule as the app's `ClimateMap::border_at`
  (one ring when the band ≤ 2 squares), generalized to any band. Past the raster: clamped to the edge squares.
  Deviation from Design §2: no stored per-square `(neighbor, distance)`; the distance is exact per query from the
  neighborhood (WE4b's resolve does the same 3 × 3 search per texel, trivial at 65 k threads).
- **`ClimateRow`** (the table): each raster climate evaluated **once at weight 1** (`update_table`; lightning window
  empty: strikes stay the camera frame's) into 31 continuous channels (`row::*`: cloud genus terms + cloud speed, dimming,
  haze, dust, ground wind, gustiness, height fog + bank terms, total precipitation, precipitation extinction, lightning
  rate, surface rates) and the heaviest precipitation's `look`. `ClimateRow::visibility_m(alt)` recomputes the frame's
  Koschmieder visibility from the blended terms (nonlinear, so not blended itself).
- **`WeatherMap`** (`new(raster, size, texel_m)`, default `MAP_SIZE` 256): `sample(p)` / `sample_site(site)` = the
  weight-normalized sum of rows (the CPU twin of the resolve); `update(book, state, hours, hour_of_day, camera)` refreshes
  the table, then the **world-anchored toroidal window**: world texel `t = floor(p / texel_m)` (absolute f64 position,
  so floating-origin shifts move nothing) stored at `t mod size`, window = camera texel snapped to 1/8-map steps − size/2.
  A full resolve when the table moved past `MAP_EPSILON` (1e-3, relative over 1) since the last resolve, else only the
  strips entering the window on a recenter. `MapTexel` = the spec's A (coverage, genus, wetness, storm), B (base, thickness,
  density, erosion), C (cirrus, precipitation, **height-fog density**, **overcast dimming = darkness**) + `look`.
  Deviation: C carries absolute fog density and dimming instead of "fog scale" and the look byte (the look is a separate
  field); WE4b decides the packing. **Uniform skip**: when every row's map channels agree (all regions Clear: their
  winds differ, the map channels don't), `update` writes nothing, `texel_at` is `None` and `uniform_row()` stands for the map.
- **Near = far:** the camera's site is meant to come from `raster.site_at(camera)` (WE4c replaces the app's
  `ClimateMap::border_at` with it); `CurrentWeather.frame` = `evaluate(site)` and `map.sample(camera)` agree by construction.
- **Verified** (`cargo test -p sf_weather`, 5 new, 17 total): `raster_fills_sea_with_the_nearest_region` (islands + sea:
  sea columns split at the midpoint, ½/½ at the sea border, edge clamp, empty raster → `None`);
  `map_at_the_camera_is_the_current_weather` (2 m walk across a 200 m band, two climates mid-transition: all 31 channels
  of `sample` = `ClimateRow::from_frame(evaluate(site_at))` within 1e-5 relative, same look, visibility within 1e-4, no
  coverage/precipitation step ≥ 0.02, resolved texel = sample at its center); `map_is_a_pure_function_of_seed_overrides_clock_raster`;
  `uniform_day_skips_and_recentering_writes_only_new_strips` (clear + clear: 0 texels; storm: 32² once; same inputs: nothing;
  one step east: 4 × 32 texels, every texel = its world sample); `map_cost`.
- **Cost** (release, 16 climates, 64 × 64 raster): **table 0.0034 ms** (estimate ≤ 0.02 incl. upload; dev opt-level 1:
  0.0053 ms). CPU resolve of the full 256² window 3.5 ms release (5.4 ms dev): fine for tests and a one-off, not per frame;
  the per-frame resolve is WE4b's GPU pass (≤ 0.01 ms estimate). Recentering strips (256 × 32) would be ~0.4 ms on the CPU.

**Next (WE4b, ordered):** (1) `sf_atmos_bevy` resource with the raster (R16Uint own-climate texture, uploaded once)
and the table (storage buffer of rows, ≤ 64 × 31 f32, re-uploaded when `MapUpdate.table_changed`); (2) WGSL resolve
compute pass, twin of `ClimateRaster::site_at` + `WeatherMap::blend` (3 × 3 search, `border_blend`), writing A/B/C
textures at `storage_index`, full on table change, strips on recenter, skipped when uniform; a GPU-vs-CPU texel test
(readback ≤ 1e-3) like `sf_wind`'s twin; (3) `weather_sample(p)` in the cloud trace reads A (and B) by world texel with
the origin phase, uniform fast-path flag keeps today's scalars; (4) envelope shell (max base + thickness over the table)
and per-sample local base/thickness; (5) cloud shadow map reads the same map; (6) `clouds` example `--split clear,storm`
+ GPU timer A/B; (7) log the "no written weather map" reversal in ARCHITECTURE §10.

### WE4b as built, steps 1-2 (we4b, 2026-10-19): GPU map, clouds and cloud shadow map read it

**Reuse check (rule 8).** Bevy has no spatial weather texture or cloud weather hook; the resolve is one ~100-line compute
shader over our own raster/table, so it is ours. The WGSL lives in the Bevy-free `sf_weather` (`MAP_WGSL`, `src/map.wgsl`)
so the twin test runs it in raw wgpu like `sf_wind`'s; `sf_atmos_bevy` now depends on `sf_weather` (Bevy-free, small).

- **CPU side** (`sf_weather::map`): `ClimateRaster::gpu_cells` (each square's table row index, `u32` storage buffer:
  deviation from "R16Uint texture", 16 KB for 64², simpler indexing), `WeatherMap::gpu_rows` (A, B, C per row),
  `gpu_geometry` → `MapGpuGeometry::params(window, rects)` (96-byte uniform; raster origin split into integer + fractional
  world texels so f32 precision doesn't depend on world size), `resolve_rects(window, size, from)` (≤ 2 strips; `None` =
  full), `set_cpu_texels(false)` (`update` only tracks table/window; the GPU owns the texels), `set_rows` + `update_window`
  (a table without a `WeatherBook`, for examples/tools).
- **Resolve** (`map.wgsl`, entry `resolve`, 8 × 8, dispatch `(size/8, size/8, rects)`): per texel the 3 × 3 (reach)
  point-to-square search, `border_weight`, blend of two rows, written to a `Rgba16Float` 2-D array (layers A, B, C;
  deviation: C is `Rgba16Float` too, it carries absolute fog density) at `t mod size`. Ties: lower distance, then lower
  row index (= lower `ClimateIx`, the CPU's order).
- **Bevy** (`sf_atmos_bevy::weather_map`): main-world `CloudWeatherMap::sync(&map, update)` (raster re-upload on new
  geometry, rows when the table was resolved anew); render world uploads, schedules full / strip / no resolve (a resolve
  that never ran, e.g. no camera, forces a full one next frame), Core3d pass `weather_map` (GPU timer) before
  `clouds_shadow`. Uniform day or no map: `map_on = 0`, the clouds keep today's uniform scalars.
- **Clouds** (`cloud_density.wesl`): `map_a(p)` = one bilinear fetch of layer A by world texel (`map_offset` = origin /
  texel − window in f64 on the CPU, `map_wrap` = window mod size, Repeat sampler, clamped to the window's edge texels);
  `weather_sample_a(p, a)` feeds `weather_at` with the map's coverage, genus, wetness (darkness) and storm instead of the
  uniforms. `CloudQuality::map_per_sample` (High/Ultra `true`): every density sample (trace, light march, shadow map);
  `false` (Medium/Low, owner decision): `ray_a` = 3 fetches along the in-layer segment (trace) or the shadow texel's
  light ray, lerped by the sample's fraction; the light march reuses the primary sample's value.
- **Verified**: `cargo test -p sf_weather --test map_gpu_twin` (RTX 4060 Laptop): 3 climates over sea, 64² map of 32 m,
  full resolve and a recenter (2 strips covering exactly the entering texels) match the CPU twin within **9.2e-4**
  (`MapTexel::max_diff`, f16 storage; gate 1e-3). `clouds --split clear,storm` (Clear west of x = 2.5 km, Storm east,
  400 m band): zenith clear with the storm deck beyond the border, ground lit on the clear side (shadow map reads the
  map); visual check **WE-4b** awaiting owner.
- **Cost** (1280 × 720, RTX 4060 Laptop, High, averaged GPU timers): resolve **0.017 ms** (full 256², only on change);
  `clouds_trace` uniform storm 0.255 ms vs split per sample 0.253 ms vs per ray 0.250 ms; `clouds_shadow` 0.019 / 0.019 /
  0.025 ms. The extra fetch is lost in the noise at this resolution; re-measure at 1440p in the app (WE4c).
- **Not yet** (next, in order): (3/4) envelope shell + local base/thickness/density/erosion from layer B (the shell is
  still the uniform `Clouds` weather's; the example uses the east climate's heights), plume anchor hook; WE4c app wiring
  (`site_at`, `sf_weather_bevy` drives `CloudWeatherMap`); cirrus stays uniform (layer C unused by the clouds).

### WE4b as built, steps 3-4 (we4b2, 2026-10-19): local layer heights and cirrus from the map

- **Envelope shell.** `CloudWeatherMap` keeps `envelope(rows)` = lowest base, highest top (base + thickness) and most cirrus
  over the table's rows; the render world grows the uniform (camera) layer to it (`Clouds.shell_base`, `shell_thickness`,
  `cirrus_max`). A texel is a convex blend of rows, so every local layer lies inside the shell. The trace and the cloud
  shadow map march the shell (the shadow map's span cap is 4 × the shell's thickness); off the map it is the uniform layer.
- **Local layer from B.** Each density sample reads layer B (base, thickness, density, erosion) next to A, by the same
  rule: per sample on High/Ultra (`map_layer(p, 1)`, one more bilinear fetch of the same texel), else `ray_b` = 3 fetches
  along the in-layer segment, lerped (Medium/Low). `height_fraction(p, b)` and `cloud_density(p, h, w, b, detailed)`
  use the local base/thickness (detail octave z, density, erosion); the uniform `Clouds.base/thickness/density/erosion`
  are now only the no-map / uniform-day layer. The light march keeps the primary sample's B (the border band, ≥ 400 m,
  is wider than the march's reach at the steps it takes); the trace step `ds` stays the camera layer's thickness / 20
  (clamped by the step counts), so near clouds keep their sampling under a tall neighbour. Shader helpers are
  generic per layer (`map_layer`, `uniform_layer`, `local_layer`, `ray_layer`, `ray_local`, `_a`/`_b` wrappers).
- **Cirrus from C.** The sheet stays one sphere at the camera layer's `cirrus_height` (one ray-sphere hit); its cover is
  layer C's cirrus at the hit point (any map mode: one fetch per ray), gated by `cirrus_max`. The shadow map's cirrus
  term reads it too (`cirrus_optical_depth` → `cirrus_hit`).
- **Example**: `clouds --split` now sets the plugin's uniform layer to the **west** (camera) climate, as the app will.
- **Verified** (1280 × 720, RTX 4060 Laptop, High, averaged GPU timers, `clouds`): `--preset storm` (uniform) trace
  **0.210 ms**, shadow 0.020; `--split clear,storm` per sample trace **0.239 ms** (+0.03, the storm deck now 8.9 km tall
  beyond the border while the clear side is a 700 m layer), shadow 0.019, resolve 0.018; `--per-ray` trace 0.249 ms
  (no saving at this resolution; the 6 per-ray fetches + lerp cost about what the per-sample fetch does), shadow 0.019.
  All inside the WE4 gate (+0.15 High). Shots: from the clear side the storm rises as a wall at the border band with its
  tops against blue sky (`--scale 0.3` to see the tops in frame), clear zenith, cirrus streaks; no seam line in the
  sky; the `shadow` view shows only soft patches on the clear-side ground (no band at the border). Visual check WE-4b
  covers it (owner).
- **Not done** (by scope): plume anchor hook; cirrus height per climate (one sheet); a per-ray B saving on Medium/Low
  (could reuse `ray_a`'s 3 points with a 2-layer fetch helper if Medium/Low trace time matters at 1440p).

### WE4c as built (we4c, 2026-10-19): the game runs the world weather map

- **`sf_weather_bevy::WorldWeatherMap`** (`map: WeatherMap`, `camera: Option<DVec2>`): built from the app's
  `ClimateRaster` with `MAP_SIZE` 256 and `MAP_TEXEL_M` 64 m (a quarter of the 256 m band; 16.4 km window), CPU texels
  off. System `update_map`, chained after `evaluate` in `WeatherSystems::Evaluate`: `map.update(book, state, hours,
  hour, camera)` then `CloudWeatherMap::sync`; counter `weather/map/ms`. `site_at` / `climates` helpers.
- **App** (bevyrrowind): `WeatherSetup::raster` = the region cells' bounding box + 4 sea squares, rows along +Z (south),
  unowned squares dilated to the nearest region; without region cells one square of the fallback climate. `weather_site`
  writes `map.camera` and takes the site from `site_at` (the old `ClimateMap::border_at` and its 2-square band cap are
  gone; the band is now `BORDER_BAND_M` 256 m). Live control forces every raster climate, or one region's
  (`bevyrrowind/set {"weather", "region"}`). `Quality::cloud_map_per_sample`: High/Ultra per sample, Medium/Low per ray.
- **Verified:** builds, clippy clean (our code); scene `we3-ridge` with Ashlands = ashstorm, Ascadian Isles =
  thunderstorm: per-region forces apply, the zenith stays clear, the forced regions get clouds.
- **Bug fixed (we4c2, 2026-10-19): distant decks cut short, grey columns.** Cause, from the numbers: (1) **past-the-edge
  rule**: the resolve clamped positions past the raster per axis (and the trace clamped past the 16 km window per axis),
  so each edge square stretched into a stripe of constant width out to the 50 km cloud reach. Morrowind's raster is the
  island (~6 km, 117 m cells) + 4 sea squares, so a region touching the edge narrowly (Ascadian Isles' dilated sea)
  became a ~1 km stripe running to the horizon, drawn as a tall column tapering into the distance (8.9 km storm deck).
  (2) **step length**: the shell (700-8900 m with ashstorm + thunderstorm) puts the camera inside it, so every grazing ray
  spans ~50 km and `ds` = span / `max_steps` ≈ 780 m (coarse 1.56 km): near decks drew as banded slabs, narrow ones were
  stepped over. Fix: past the edge, positions are **projected toward the raster's (window's) center onto its edge**
  (`ClimateRaster::site_at`, `map.wgsl`, `map_layer`): each edge square owns a wedge that widens outward (a coastal
  region's weather reaches the horizon in its direction; the twin test still matches). The trace step is
  **`ds = local thickness / 20 + g·t`** (distance-proportional, as Nubis/UE scale steps with view distance), `g` sized so
  `max_steps` coarse steps grow t from the start to `end`, capped at span / `min_steps`: a thin deck keeps 35 m steps, an
  8 km tower 400 m, far samples ~2 km. `clouds --split clear,storm` (1280 × 720, High): trace **0.215 ms** (was 0.239),
  deck to ~1° above the horizon. Game `we3-ridge` (`shots/we4c2/`): the Ascadian Isles storm is a wide wall reaching the
  horizon, clear zenith, no striations; the Ashlands deck ends ~3 km out because the region does (geometry: a 2-3 km region
  under a 700 m base can't fill the sky below ~10°). **Not done:** 1440p re-time (trace, map pass, High per sample vs
  Medium per ray; no GPU pass timers over BRP and the dev window opened at 720p); the window's own radial projection
  recenters with the window (32-texel steps), so the sky past 8 km can shift on a recenter (unchecked).
