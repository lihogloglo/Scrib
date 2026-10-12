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

Non-goals: water bodies, waves and buoyancy (WISHLIST §4; the water spec, M6); character wetness
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

## Wind

**Field** (`sf_wind`; the CPU and WGSL twins agree to 1e-4, checked by a test that runs the WGSL on
the CPU through `naga` + a reference or by matching golden values):
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
| 1 CSM | 0 → `s_max` ≥ `d1 + w` + margin (default **500 m**) | everything near → everything | Bevy cascades (4), retuned |
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
3. **Weather hook**: `Weather` gains `dust: f32` (absorbing aerosol term scale, N2). `haze` is driven by
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
