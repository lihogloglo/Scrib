# Weather, atmosphere and distant shadows

Status: draft (2026-10-11). "Near-term visual fixes": distant shadows built (shadows2), bug B1-B3 built (nifpop, 2026-10-12); volumetric clouds built (clouds, 2026-10-12, "Clouds as built"); start now
in M2; the rest is M6.
Crates (new): `sf_wind`, `sf_weather`, `sf_atmos`, `sf_surface` (Bevy-free cores);
`sf_wind_bevy`, `sf_weather_bevy`, `sf_atmos_bevy`, `sf_surface_bevy`, `sf_precip_bevy` (thin plugins).
Touches: `sf_sky`/`sf_sky_bevy` (proposed changes at the end; owned by sky-lighting.md),
`sf_terrain_bevy` (surface hook, far shadow), `sf_lod_bevy` (far shadow, impostor wind),
`sf_model` (wind vertex data, rest-pose cook), `sf_world_bevy` (uncooked preload).
Adapter: `../../../bevyrrowind/docs/specs/weather-adapter.md`.
Note: "proposed changes at the end" now lives in [weather/history.md](weather/history.md).

## Topic files

| File | Contents |
|---|---|
| [weather/state.md](weather/state.md) | Weather state machine and presets as data; WE1 and WE3 as built (`sf_weather`, `sf_weather_bevy`) |
| [weather/clouds.md](weather/clouds.md) | Volumetric clouds: model, genera, scale, motion, shadows, sun occlusion; as built |
| [weather/fog.md](weather/fog.md) | Aerial perspective, height fog, ground fog, froxel fog, god rays; as built |
| [weather/wind.md](weather/wind.md) | Wind field, `sf_wind` / `sf_wind_bevy`; WD1 as built |
| [weather/precipitation-surface.md](weather/precipitation-surface.md) | Precipitation and surface state (wetness, snow) |
| [weather/distant-shadows.md](weather/distant-shadows.md) | Distant shadows, shadow bias, sun cascades, near-term visual fixes (Bug A, Bug B, aerial) |
| [weather/distant-weather.md](weather/distant-weather.md) | Distant weather (WE4): design, cost, slices, owner decisions |
| [weather/history.md](weather/history.md) | Research summary, reuse decision, Megalophobia scavenge, proposed changes, WE4a-e as built |

See [history](weather/history.md#we4a-as-built-we4a-2026-10-19-sf_weatherweathermap-cpu-side) (WE4a-e as built).

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

See [history](weather/history.md#research-summary-2026-10-11) (research summary, reuse decision, Megalophobia scavenge).

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

See [history](weather/history.md#proposed-changes-to-sky-lightingmd-owned-by-the-dlss-session-apply-after-it-finishes) (proposed changes to sky-lighting.md and other specs).

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

